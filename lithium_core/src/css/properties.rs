use std;
use cssparser::{Parser, ParseError, Token};
#[cfg(test)]
use cssparser::ParserInput;
use {Color, Vec2};
use theme::ElementStyle;
use theme::element_style::{Overflow, LengthOrPercentage, Filter, Shadow};
use super::parse_color;
use super::{border, background, parse_length};

pub fn parse_property<'i, 'tt>(parser: &mut Parser<'i, 'tt>, property: &str, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    let f = match_ignore_ascii_case! { property,
        "color" => color,
        "opacity" => opacity,
        "overflow" => overflow,
        "overflow_x" => overflow_x,
        "overflow_y" => overflow_y,
		"visibility" => visibility,

		"background" => background::background,
		"background-attachment" => background::background_attachment,
		"background-blend-mode" => background::background_blend_mode,
		"background-color" => background::background_color,
		"background-image" => background::background_image,
		"background-position" => background::background_position,
		"background-repeat" => background::background_repeat,
		"background-clip" => background::background_clip,
		"background-origin" => background::background_origin,
		"background-size" => background::background_size,

		"border" => border::border,
		"border-bottom" => border::border_bottom,
		"border-bottom-color" => border::border_bottom_color,
		"border-bottom-left-radius" => border::border_bottom_left_radius,
		"border-bottom-right-radius" => border::border_bottom_right_radius,
		"border-bottom-style" => border::border_bottom_style,
		"border-bottom-width" => border::border_bottom_width,
		"border-color" => border::border_color,
		"border-image" => border::border_image,
		"border-image-outset" => border::border_image_outset,
		"border-image-repeat" => border::border_image_repeat,
		"border-image-slice" => border::border_image_slice,
		"border-image-source" => border::border_image_source,
		"border-image-width" => border::border_image_width,
		"border-left" => border::border_left,
		"border-left-color" => border::border_left_color,
		"border-left-style" => border::border_left_style,
		"border-left-width" => border::border_left_width,
		"border-radius" => border::border_radius,
		"border-right" => border::border_right,
		"border-right-color" => border::border_right_color,
		"border-right-style" => border::border_right_style,
		"border-right-width" => border::border_right_width,
		"border-style" => border::border_style,
		"border-top" => border::border_top,
		"border-top-color" => border::border_top_color,
		"border-top-left-radius" => border::border_top_left_radius,
		"border-top-right-radius" => border::border_top_right_radius,
		"border-top-style" => border::border_top_style,
		"border-top-width" => border::border_top_width,
		"border-width" => border::border_width,

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
        "blur" => Filter::Blur(parser.parse_nested_block(|parser| parse_length(parser))?),
        "brightness" => Filter::Brightness(parser.parse_nested_block(|parser| parse_number_or_percentage(parser))?),
        "contrast" => Filter::Contrast(parser.parse_nested_block(|parser| parse_number_or_percentage(parser))?),
        "drop-shadow" => Filter::DropShadow(parser.parse_nested_block(|parser| parse_shadow(parser))?),
        "grayscale" => Filter::Grayscale(parser.parse_nested_block(|parser| parse_number_or_percentage(parser))?),
        "hue-rotate" => Filter::HueRotate(parser.parse_nested_block(|parser| parse_angle(parser))?),
        "invert" => Filter::Invert(parser.parse_nested_block(|parser| parse_number_or_percentage(parser))?),
        "opacity" => Filter::Invert(parser.parse_nested_block(|parser| parse_number_or_percentage(parser))?),
        "sepia" => Filter::Invert(parser.parse_nested_block(|parser| parse_number_or_percentage(parser))?),
        "saturate" => Filter::Invert(parser.parse_nested_block(|parser| parse_number_or_percentage(parser))?),
        _ => return Err(ParseError::Custom(())),
    })
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

fn parse_number_or_percentage<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> Result<f32, ParseError<'i, ()>> {
    if let Ok(number) = parser.try(|parser| parser.expect_number()) {
        Ok(number)
    } else {
        Ok(parser.expect_percentage()?)
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
    use super::parse_length_or_percentage;

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
fn test_filter() {
    for &(a, b) in &[
        ("blur(5px)", Filter::Blur(5.0)),
        ("brightness(0.4)", Filter::Brightness(0.4)),
        ("drop-shadow(16px 16px 20px blue)", Filter::DropShadow(Shadow {
            position: Vec2::new(16.0, 16.0),
            blur: 20.0,
            spread: 0.0,
            color: Color::from_rgb24(0, 0, 255),
            inset: false
        })),
    ]
    {
        assert_eq!(parse_filter(&mut Parser::new(&mut ParserInput::new(a))).unwrap(), b);
    }
}