use std;
use cssparser;
use cssparser::{Parser, ParseError, Token};
#[cfg(test)]
use cssparser::ParserInput;
use {Color, Vec2};
use theme::ElementStyle;
use theme::element_style::{Overflow, BorderStyle, BackgroundPicture, BackgroundImage, LengthOrPercentage, Filter, Shadow, border, corner};

const THIN_BORDER: f32 = 1.0;
const MEDIUM_BORDER: f32 = 2.0;
const THICK_BORDER: f32 = 3.0;

pub fn parse_property<'i, 'tt>(parser: &mut Parser<'i, 'tt>, property: &str, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    let f = match_ignore_ascii_case! { property,
        "color" => color,
        "opacity" => opacity,
        "overflow" => overflow,
        "overflow_x" => overflow_x,
        "overflow_y" => overflow_y,
		"visibility" => visibility,
		"background" => background,
		"background-attachment" => background_attachment,
		"background-blend-mode" => background_blend_mode,
		"background-color" => background_color,
		"background-image" => background_image,
		"background-position" => background_position,
		"background-repeat" => background_repeat,
		"background-clip" => background_clip,
		"background-origin" => background_origin,
		"background-size" => background_size,
		"border" => border,
		"border-bottom" => border_bottom,
		"border-bottom-color" => border_bottom_color,
		"border-bottom-left-radius" => border_bottom_left_radius,
		"border-bottom-right-radius" => border_bottom_right_radius,
		"border-bottom-style" => border_bottom_style,
		"border-bottom-width" => border_bottom_width,
		"border-color" => border_color,
		"border-image" => border_image,
		"border-image-outset" => border_image_outset,
		"border-image-repeat" => border_image_repeat,
		"border-image-slice" => border_image_slice,
		"border-image-source" => border_image_source,
		"border-image-width" => border_image_width,
		"border-left" => border_left,
		"border-left-color" => border_left_color,
		"border-left-style" => border_left_style,
		"border-left-width" => border_left_width,
		"border-radius" => border_radius,
		"border-right" => border_right,
		"border-right-color" => border_right_color,
		"border-right-style" => border_right_style,
		"border-right-width" => border_right_width,
		"border-style" => border_style,
		"border-top" => border_top,
		"border-top-color" => border_top_color,
		"border-top-left-radius" => border_top_left_radius,
		"border-top-right-radius" => border_top_right_radius,
		"border-top-style" => border_top_style,
		"border-top-width" => border_top_width,
		"border-width" => border_width,
		"box-shadow" => box_shadow,
		"filter" => filter,
        _ => return Err(ParseError::Custom(()))
    };

    f(parser, element_style)
}

fn color<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    element_style.font_color = parse_color(parser)?;
    Ok(())
}

fn opacity<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    element_style.opacity = parser.expect_number()?;
    Ok(())
}

fn overflow<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    let overflow = overflow_any(parser)?;
    element_style.overflow = overflow;
    Ok(())
}

fn overflow_x<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    element_style.overflow = overflow_any(parser)?;
    Ok(())
}

fn overflow_y<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    element_style.overflow = overflow_any(parser)?;
    Ok(())
}

fn overflow_any<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> Result<Overflow, ParseError<'i, ()>> {
    let ident = parser.expect_ident()?;

    match_ignore_ascii_case! { ident.as_ref(),
        "visible" | "initial" => Ok(Overflow::Visible),
        "hidden" => Ok(Overflow::Hidden),
        _ => Err(ParseError::Custom(())),
    }
}

fn visibility<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    let ident = parser.expect_ident()?;

    match_ignore_ascii_case! { ident.as_ref(),
        "visible" | "initial" => element_style.visible = true,
        "hidden" => element_style.visible = false,
        _ => return Err(ParseError::Custom(())),
    }

    Ok(())
}

fn background<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

fn background_attachment<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

fn background_blend_mode<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

fn background_color<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    let parsed_color = parse_color(parser)?;
    if let Some(image) = element_style.background_images.last_mut() {
        if let Some(BackgroundPicture::Color(ref mut color)) = image.image {
            *color = parsed_color;
            return Ok(());
        }
    }

    element_style.background_images.push(BackgroundImage {
        image: Some(BackgroundPicture::Color(parsed_color)),
        .. BackgroundImage::default()
    });

    Ok(())
}

fn background_image<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

fn background_position<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

fn background_repeat<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

fn background_clip<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

fn background_origin<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

fn background_size<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

fn border<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

fn border_bottom<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

fn border_left<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

fn border_right<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

fn border_top<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

fn border_bottom_color<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    border_side_color(parser, element_style, border::BOTTOM)
}

fn border_left_color<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    border_side_color(parser, element_style, border::LEFT)
}

fn border_right_color<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    border_side_color(parser, element_style, border::RIGHT)
}

fn border_top_color<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    border_side_color(parser, element_style, border::TOP)
}

fn border_side_color<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle, side: usize) -> Result<(), ParseError<'i, ()>> {
    element_style.border[side].color = parse_color(parser)?;
    Ok(())
}

fn border_bottom_left_radius<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    border_corner_radius(parser, element_style, corner::BOTTOM_LEFT)
}

fn border_bottom_right_radius<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    border_corner_radius(parser, element_style, corner::BOTTOM_RIGHT)
}

fn border_top_left_radius<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    border_corner_radius(parser, element_style, corner::TOP_LEFT)
}

fn border_top_right_radius<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    border_corner_radius(parser, element_style, corner::TOP_RIGHT)
}

fn border_corner_radius<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle, corner: usize) -> Result<(), ParseError<'i, ()>> {
    if Ok(()) == parser.try(|parser| parser.expect_ident_matching("initial")) {
        element_style.border_radius[corner].x = LengthOrPercentage::Length(0.0);
        element_style.border_radius[corner].y = LengthOrPercentage::Length(0.0);
    } else {
        element_style.border_radius[corner].x = parse_length_or_percentage(parser)?;

        if let Ok(value) = parser.try(parse_length_or_percentage) {
            element_style.border_radius[corner].y = value;
        } else {
            element_style.border_radius[corner].y = element_style.border_radius[corner].x;
        }
    }

    Ok(())
}

fn border_bottom_style<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    border_side_style(parser, element_style, border::BOTTOM)
}

fn border_left_style<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    border_side_style(parser, element_style, border::LEFT)
}

fn border_top_style<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    border_side_style(parser, element_style, border::TOP)
}

fn border_right_style<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    border_side_style(parser, element_style, border::RIGHT)
}

fn border_side_style<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle, border: usize) -> Result<(), ParseError<'i, ()>> {
    element_style.border[border].style = parse_border_style(parser)?;
    Ok(())
}

fn border_bottom_width<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    border_side_width(parser, element_style, border::BOTTOM)
}

fn border_top_width<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    border_side_width(parser, element_style, border::TOP)
}

fn border_left_width<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    border_side_width(parser, element_style, border::LEFT)
}

fn border_right_width<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    border_side_width(parser, element_style, border::RIGHT)
}

fn border_side_width<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle, border: usize) -> Result<(), ParseError<'i, ()>> {
    element_style.border[border].width = parse_border_width(parser)?;
    Ok(())
}

fn border_color<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    if Ok(()) == parser.try(|parser| parser.expect_ident_matching("initial")) {
        for i in 0..4 {
            element_style.border[i].color = Color::black();
        }
    } else {
        let colors = parse_four_values(parser, parse_color)?;

        for i in 0..4 {
            element_style.border[i].color = colors[i];
        }
    }

    Ok(())
}

fn border_image<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

fn border_image_outset<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

fn border_image_repeat<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

fn border_image_slice<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

fn border_image_source<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

fn border_image_width<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

fn border_radius<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    if Ok(()) == parser.try(|parser| parser.expect_ident_matching("initial")) {
        for i in 0..4 {
            element_style.border_radius[i].x = LengthOrPercentage::Length(0.0);
            element_style.border_radius[i].y = LengthOrPercentage::Length(0.0);
        }
    } else {
        let lens = parse_four_values(parser, parse_length_or_percentage)?;

        for i in 0..4 {
            element_style.border_radius[i].x = lens[i];
            element_style.border_radius[i].y = lens[i];
        }
    }

    Ok(())
}

fn border_style<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    if Ok(()) == parser.try(|parser| parser.expect_ident_matching("initial")) {
        for i in 0..4 {
            element_style.border[i].style = None;
        }
    } else {
        let styles = parse_four_values(parser, parse_border_style)?;

        for i in 0..4 {
            element_style.border[i].style = styles[i];
        }
    }

    Ok(())
}

fn border_width<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    if Ok(()) == parser.try(|parser| parser.expect_ident_matching("initial")) {
        for i in 0..4 {
            element_style.border[i].width = MEDIUM_BORDER;
        }
    } else {
        let widths = parse_four_values(parser, parse_border_width)?;

        for i in 0..4 {
            element_style.border[i].width = widths[i];
        }
    }

    Ok(())
}

fn box_shadow<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    element_style.box_shadows.clear();

    if Ok(()) == parser.try(|parser| parser.expect_ident_matching("none")) ||
       Ok(()) == parser.try(|parser| parser.expect_ident_matching("initial"))
    {
        return Ok(());
    }

    if let Ok(shadows) = parser.parse_comma_separated(parse_shadow) {
        element_style.box_shadows = shadows;
    }

    Ok(())
}

fn filter<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    element_style.filters.clear();

    if Ok(()) == parser.try(|parser| parser.expect_ident_matching("none")) ||
       Ok(()) == parser.try(|parser| parser.expect_ident_matching("initial"))
    {
        return Ok(());
    }

    while let Ok(filter) = parser.try(parse_filter) {
        element_style.filters.push(filter);
    }

    Ok(())
}

fn parse_filter<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> Result<Filter, ParseError<'i, ()>> {
    let function = parser.expect_function()?;

    Ok(match_ignore_ascii_case! { function.as_ref(),
        "blur" => Filter::Blur(parse_length(parser)?),
        "brightness" => Filter::Brightness(parse_number_or_percentage(parser)?),
        "contrast" => Filter::Contrast(parse_number_or_percentage(parser)?),
        "drop-shadow" => Filter::DropShadow(parse_shadow(parser)?),
        "grayscale" => Filter::Grayscale(parse_number_or_percentage(parser)?),
        "hue-rotate" => Filter::HueRotate(parse_angle(parser)?),
        "invert" => Filter::Invert(parse_number_or_percentage(parser)?),
        "opacity" => Filter::Invert(parse_number_or_percentage(parser)?),
        "sepia" => Filter::Invert(parse_number_or_percentage(parser)?),
        "saturate" => Filter::Invert(parse_number_or_percentage(parser)?),
        _ => return Err(ParseError::Custom(())),
    })
}

fn parse_color<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> Result<Color, ParseError<'i, ()>> {
    match cssparser::Color::parse(parser)? {
        cssparser::Color::RGBA(rgba) => Ok(Color::from_rgba32(rgba.red, rgba.green, rgba.blue, rgba.alpha)),
        cssparser::Color::CurrentColor => Err(ParseError::Custom(())),
    }
}

fn parse_shadow<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> Result<Shadow, ParseError<'i, ()>> {
    let mut color: Option<Color> = None;
    let mut inset: bool = false;

    let mut position: Option<Vec2<f32>> = None;
    let mut blur = 0.0;
    let mut spread = 0.0;

    while !parser.is_exhausted() {
        if Ok(()) == parser.try(|parser| parser.expect_ident_matching("inset")) {
            if inset {
                return Err(ParseError::Custom(()));
            }

            inset = true;
        } else if let Ok(parsed_color) = parser.try(parse_color) {
            if color.is_some() {
                return Err(ParseError::Custom(()));
            }
            color = Some(parsed_color);
        } else if let Ok(h_shadow) = parser.try(parse_length) {
            if position.is_some() {
                return Err(ParseError::Custom(()));
            }

            let v_shadow = parse_length(parser)?;

            blur = parser.try(parse_length).unwrap_or(0.0);
            spread = parser.try(parse_length).unwrap_or(0.0);

            position = Some(Vec2::new(h_shadow, v_shadow));
        } else {
            return Err(ParseError::Custom(()));
        }
    }

    if let Some(position) = position {
        Ok(Shadow {
            color: color.unwrap_or(Color::black()),
            position: position,
            inset,
            blur,
            spread,
        })
    } else {
        return Err(ParseError::Custom(()));
    }

}

fn parse_border_style<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> Result<Option<BorderStyle>, ParseError<'i, ()>> {
    let ident = parser.expect_ident()?;

    Ok(match_ignore_ascii_case! { ident.as_ref(),
        "none" | "initial" | "hidden" => None,
         "dotted" => Some(BorderStyle::Dotted),
         "dashed" => Some(BorderStyle::Dashed),
         "solid" => Some(BorderStyle::Solid),
         "double" => Some(BorderStyle::Double),
         "groove" => Some(BorderStyle::Groove),
         "ridge" => Some(BorderStyle::Ridge),
         "inset" => Some(BorderStyle::Inset),
         "outset" => Some(BorderStyle::Outset),
        _ => return Err(ParseError::Custom(())),
    })
}

fn parse_border_width<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> Result<f32, ParseError<'i, ()>> {
    if let Ok(ident) = parser.try(|parser| parser.expect_ident()) {
        Ok(match_ignore_ascii_case! { ident.as_ref(),
            "thin" => THIN_BORDER,
            "medium" | "initial" => MEDIUM_BORDER,
            "thick" => THICK_BORDER,
            _ => return Err(ParseError::Custom(())),
        })
    } else {
        parse_length(parser)
    }
}

fn parse_length_or_percentage<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> Result<LengthOrPercentage, ParseError<'i, ()>> {
    if let Ok(length) = parser.try(parse_length) {
        Ok(LengthOrPercentage::Length(length))
    } else {
        Ok(LengthOrPercentage::Percentage(parser.expect_percentage()?))
    }
}

fn parse_number_or_percentage<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> Result<f32, ParseError<'i, ()>> {
    if let Ok(number) = parser.try(|parser| parser.expect_number()) {
        Ok(number)
    } else {
        Ok(parser.expect_percentage()?)
    }
}

fn parse_length<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> Result<f32, ParseError<'i, ()>> {
    if let Token::Dimension(value, unit) = parser.next()? {
        Ok(value.value * match_ignore_ascii_case! { unit.as_ref(),
            "px" => 1.0,
            "cm" => 96.0/2.54,
            "mm" => 96.0*0.1/2.54,
            "q" => 96.0*0.25/2.54,
            "in" => 96.0,
            "pc" => 96.0/6.0,
            "pt" => 96.0/72.0,
            _ => return Err(ParseError::Custom(()))
        })
    } else {
        Err(ParseError::Custom(()))
    }
}

fn parse_angle<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> Result<f32, ParseError<'i, ()>> {
    if let Token::Dimension(value, unit) = parser.next()? {
        Ok(value.value * match_ignore_ascii_case! { unit.as_ref(),
            "deg" => 1.0,
            "grad" => 360.0/400.0,
            "rad" => 360.0/(2.0*std::f32::consts::PI),
            "turn" => 360.0,
            _ => return Err(ParseError::Custom(()))
        })
    } else {
        Err(ParseError::Custom(()))
    }
}

fn parse_four_values<'i, 'tt, F, R>(parser: &mut Parser<'i, 'tt>, f: F) -> Result<[R; 4], ParseError<'i, ()>>
  where
    F: Copy + for<'tt2> Fn(&mut Parser<'i, 'tt2>) -> Result<R, ParseError<'i, ()>>,
    R: Copy
{
    let value1 = f(parser)?;
    
    if let Ok(value2) = parser.try(f) {
        if let Ok(value3) = parser.try(f) {
            if let Ok(value4) = parser.try(f) {
                Ok([value1, value2, value3, value4])
            } else {
                Ok([value1, value2, value3, value2])
            }
        } else {
            Ok([value1, value2, value1, value2])
        }
    } else {
        Ok([value1, value1, value1, value1])
    }
}

#[test]
fn test_angles() {
    for &(a, b) in &[
        ("360deg",   360.0),
        ("185.5deg", 185.5),
        ("1turn",    360.0),
        ("1rad",      57.295776),
        ("1grad",      0.9)]
    {
        assert_eq!(parse_angle(&mut Parser::new(&mut ParserInput::new(a))).unwrap(), b);
    }
}

#[test]
fn test_lengths() {
    for &(a, b) in &[
        ("96px",   "1in"),
        ("1cm",   "10mm"),
        ("0.25cm", "1q"),
        ("25.4mm", "1in"),
        ("1in",   "72pt"),
        ("1in",    "6pc"),
    ]
    {
        assert_eq!(parse_length(&mut Parser::new(&mut ParserInput::new(a))).unwrap(), parse_length(&mut Parser::new(&mut ParserInput::new(b))).unwrap());
    }

    assert_eq!(parse_length(&mut Parser::new(&mut ParserInput::new("100px"))).unwrap(), 100.0);
    
    for &(a, b) in &[
        ("1px",   LengthOrPercentage::Length(1.0)),
        ("50%",   LengthOrPercentage::Percentage(0.5)),
        ("50.5%", LengthOrPercentage::Percentage(0.505)),
    ]
    {
        assert_eq!(parse_length_or_percentage(&mut Parser::new(&mut ParserInput::new(a))).unwrap(), b);
    }

    for &(a, b) in &[
        ("0.6",   0.6),
        ("50%",   0.5),
        ("50.5%", 0.505),
    ]
    {
        assert_eq!(parse_number_or_percentage(&mut Parser::new(&mut ParserInput::new(a))).unwrap(), b);
    }
}

#[test]
fn test_four_values() {
    for &(a, b) in &[
        ("1",       [1.0, 1.0, 1.0, 1.0]),
        ("1 2",     [1.0, 2.0, 1.0, 2.0]),
        ("1 2 3",   [1.0, 2.0, 3.0, 2.0]),
        ("1 2 3 4", [1.0, 2.0, 3.0, 4.0]),
    ]
    {
        assert_eq!(parse_four_values(&mut Parser::new(&mut ParserInput::new(a)), parse_number_or_percentage).unwrap(), b);
    }
}

#[test]
fn test_border_width() {
    for &(a, b) in &[
        ("1px",     1.0),
        ("thin",    THIN_BORDER),
        ("thick",   THICK_BORDER),
        ("medium",  MEDIUM_BORDER),
    ]
    {
        assert_eq!(parse_border_width(&mut Parser::new(&mut ParserInput::new(a))).unwrap(), b);
    }
}

#[test]
fn test_shadow() {
    for &(a, b) in &[
        ("60px -16px teal",          Shadow { color: Color::from_css_hex(b"008080"), position: Vec2::new(60.0, -16.0), inset: false, blur: 0.0, spread: 0.0 }),
        ("10px 5px 5px black",       Shadow { color: Color::black(),                 position: Vec2::new(10.0,   5.0), inset: false, blur: 5.0, spread: 0.0 }),
        ("10px 5px 5px 1.5px black", Shadow { color: Color::black(),                 position: Vec2::new(10.0,   5.0), inset: false, blur: 5.0, spread: 1.5 }),
    ]
    {
        assert_eq!(parse_shadow(&mut Parser::new(&mut ParserInput::new(a))).unwrap(), b);
    }
}

#[test]
fn test_color() {
    for &(a, b) in &[
        ("olive",   Color::from_css_hex(b"808000")),
        ("#123456", Color::from_css_hex(b"123456")),
        ("rgb(178, 81, 25)", Color::from_rgb24(178, 81, 25)),
    ]
    {
        assert_eq!(parse_color(&mut Parser::new(&mut ParserInput::new(a))).unwrap(), b);
    }
}