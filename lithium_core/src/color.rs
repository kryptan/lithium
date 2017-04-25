#![allow(non_snake_case)]

/// Color with alpha channel.
///
/// The simplest way to construct color is the `from_rgb24` constructor (or `from_rgba32` if you need alpha). E.g. `Color::from_rgb24(255, 255, 0)`
/// produces yellow.
///
/// CSS hex values are also supported: `Color::from_css_hex(b"751aff")`.
///
/// Note that scaling from 8-bit values to the stored `r`, `g`, `b ` values is non-linear, i.e. they cannot be converted to each other by multiplying
/// or diving by `255`. Use the mentioned above constructors and the `to_rgba32` method for conversion.
///
/// Internally color is stored in the linear [sRGB](https://en.wikipedia.org/wiki/sRGB) color space with RGB values premultiplied by alpha.
///
/// Linear color space was chosen for the following reasons:
///
/// 0. It is the native format for GPU.
/// 1. It doesn't make sense to store non-linear values in the floating point format.
/// 2. It can be easily extended for wide color gamut (WCG) and high dynamic range (HDR) by using values outside of the `[0, 1]` range.
/// 3. Many color operations are easier to perform in linear color space.
///
/// Pre-multiplied by alpha format was chosen for the following reasons:
///
/// 0. TODO: add reasons.
///
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Color {
    /// Linear intensity of the red sRGB primary premultiplied by alpha.
    pub r: f32,

    /// Linear intensity of the green sRGB primary premultiplied by alpha.
    pub g: f32,

    /// Linear intensity of the blue sRGB primary premultiplied by alpha.
    pub b: f32,

    /// Opacity.
    pub a: f32,
}

// sRGB electro-optical transfer function.
//
// https://en.wikipedia.org/wiki/SRGB#The_reverse_transformation
fn sRGB_eotf(e: f32) -> f32 {
    if e < 0.04045 {
        e/12.92
    } else {
        ((e + 0.055)/1.055).powf(2.4)
    }
}

// sRGB opto-electrical transfer function.
//
// https://en.wikipedia.org/wiki/SRGB#The_forward_transformation_.28CIE_XYZ_to_sRGB.29
fn sRGB_oetf(o: f32) -> f32 {
    if o < 0.0031308 {
        o*12.92
    } else {
        1.055*o.powf(1.0/2.4) - 0.055
    }
}

// sRGB electro-optical transfer function (EOTF) which converts 8-bit non-linear value into a linear one in the range [0, 1].
fn sRGB_eotf_8bit(e: u8) -> f32 {
    sRGB_eotf((e as f32)*(1.0/255.0))
}

fn discretize_to_u8(v: f32) -> u8 {
    (v*255.0).max(0.0).min(255.0).round() as u8
}

// sRGB opto-electrical transfer function (OETF) which converts linear value in the range [0, 1] into a 8-bit non-linear one.
fn sRGB_oetf_8bit(o: f32) -> u8 {
    discretize_to_u8(sRGB_oetf(o))
}

impl Color {
    /// Construct color from the 24-bit RGB triple.
    pub fn from_rgb24(r: u8, g: u8, b: u8) -> Self {
        Color {
            r: sRGB_eotf_8bit(r),
            g: sRGB_eotf_8bit(g),
            b: sRGB_eotf_8bit(b),
            a: 1.0,
        }
    }

    /// Construct color from 24-bit RGB and 8-bit alpha.
    ///
    /// `a = 0` is transparent.
    ///
    /// `a = 255` is opaque.
    ///
    /// RGB values passed to this function must not be premultiplied by alpha.
    pub fn from_rgba32(r: u8, g: u8, b: u8, a: u8) -> Self {
        let a = (a as f32)*(1.0/255.0);

        Color {
            r: sRGB_eotf_8bit(r)*a,
            g: sRGB_eotf_8bit(g)*a,
            b: sRGB_eotf_8bit(b)*a,
            a: a,
        }
    }

    /// Construct `Color` from the CSS hex color code.
    ///
    /// For example color `#751aff` can be constructed as `Color::from_css_hex(b"751aff")`.
    ///
    /// Supported formats: `#RGB`, `#RRGGBB`, `#RGBA`, `#RRGGBBAA`.
    pub fn from_css_hex(hex: &[u8]) -> Self {
        parse_hex_color(hex).unwrap_or(Self::error())
    }

    /// Get RGBA values.
    pub fn to_rgba32(self) -> (u8, u8, u8, u8) {
        if self.a.abs() < 0.00001 {
            (0, 0, 0, 0)
        } else {
            (
                sRGB_oetf_8bit(self.r / self.a),
                sRGB_oetf_8bit(self.g / self.a),
                sRGB_oetf_8bit(self.b / self.a),
                discretize_to_u8(self.a)
            )
        }
    }

    /// Construct color from the proper (non-linear) sRGB values in the `[0, 1]` range.
    pub fn from_sRGB(r: f32, g: f32, b: f32) -> Self {
        Color {
            r: sRGB_eotf(r),
            g: sRGB_eotf(g),
            b: sRGB_eotf(b),
            a: 1.0,
        }
    }

    /// Construct color from the proper (non-linear) sRGB values in the `[0, 1]` range.
    ///
    /// RGB values passed to this function must not be premultiplied by alpha.
    pub fn from_sRGB_with_alpha(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color {
            r: sRGB_eotf(r)*a,
            g: sRGB_eotf(g)*a,
            b: sRGB_eotf(b)*a,
            a: a,
        }
    }

    /// Get `[r, g, b, a]` values as an array.
    pub fn as_array(self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    /// Black opaque color: <span style="color:#000">⬛</span>.
    pub fn black() -> Self {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }

    /// White opaque color.
    pub fn white() -> Self {
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }

    /// Transparent (invisible) color.
    pub fn transparent() -> Self {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        }
    }

    /// Color which is used to represent erroneous color which is result of some incorrect operation.
    ///
    /// This is magenta `(255, 0, 255)`: <span style="color:#f0f">⬛</span>.
    pub fn error() -> Self {
        Color {
            r: 1.0,
            g: 0.0,
            b: 1.0,
            a: 1.0,
        }
    }
}

fn parse_hex_color(string: &[u8]) -> Result<Color, ()> {
    fn double(i: u8) -> u8 {
        (i << 4) | i
    }

    let (r, g, b, a) = match string.len() {
        3 => // #RGB
            (
                double(parse_hex_digit(string[0])?),
                double(parse_hex_digit(string[1])?),
                double(parse_hex_digit(string[2])?),
                255
            ),
        4 => // #RGBA
            (
                double(parse_hex_digit(string[0])?),
                double(parse_hex_digit(string[1])?),
                double(parse_hex_digit(string[2])?),
                double(parse_hex_digit(string[3])?),
            ),
        6 => // #RRGGBB
            (
                parse_hex_byte(string[0], string[1])?,
                parse_hex_byte(string[2], string[3])?,
                parse_hex_byte(string[4], string[5])?,
                255
            ),
        8 => // #RRGGBBAA
            (
                parse_hex_byte(string[0], string[1])?,
                parse_hex_byte(string[2], string[3])?,
                parse_hex_byte(string[4], string[5])?,
                parse_hex_byte(string[6], string[7])?,
            ),
        _ => return Err(()),
    };

    Ok(Color::from_rgba32(r, g, b, a))
}

fn parse_hex_byte(digit0: u8, digit1: u8) -> Result<u8, ()> {
    Ok((parse_hex_digit(digit0)? << 4) | parse_hex_digit(digit1)?)
}

fn parse_hex_digit(digit: u8) -> Result<u8, ()> {
    match digit {
        b'0' ... b'9' => Ok(digit - b'0'),
        b'a' ... b'f' => Ok(digit - b'a'),
        b'A' ... b'F' => Ok(digit - b'A'),
        _ => return Err(()),
    }
}
