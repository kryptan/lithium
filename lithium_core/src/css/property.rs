use cssparser::Parser;
#[cfg(test)]
use cssparser::ParserInput;
use {Color, Vec2};
use theme::ElementStyle;
use theme::element_style::{Overflow, Filter, Shadow};
use super::{CssResult, syntax, border, background, value, error};

pub fn property<'i, 'tt>(parser: &mut Parser<'i, 'tt>, property: &str, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    let f = match_ignore_ascii_case! { property,
        "color" => color,
        "opacity" => opacity,
        "overflow" => overflow,
        "overflow_x" => overflow_x,
        "overflow_y" => overflow_y,
		"visibility" => visibility,
		"box-shadow" => box_shadow,
		"filter" => filter,
		"isolation" => isolation,

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
		"border-bottom" => border::bottom,
		"border-bottom-color" => border::bottom_color,
		"border-bottom-left-radius" => border::bottom_left_radius,
		"border-bottom-right-radius" => border::bottom_right_radius,
		"border-bottom-style" => border::bottom_style,
		"border-bottom-width" => border::bottom_width,
		"border-color" => border::color,
		"border-image" => border::border_image,
		"border-image-outset" => border::image_outset,
		"border-image-repeat" => border::image_repeat,
		"border-image-slice" => border::image_slice,
		"border-image-source" => border::image_source,
		"border-image-width" => border::image_width,
		"border-left" => border::left,
		"border-left-color" => border::left_color,
		"border-left-style" => border::left_style,
		"border-left-width" => border::left_width,
		"border-radius" => border::radius,
		"border-right" => border::right,
		"border-right-color" => border::right_color,
		"border-right-style" => border::right_style,
		"border-right-width" => border::right_width,
		"border-style" => border::style,
		"border-top" => border::top,
		"border-top-color" => border::top_color,
		"border-top-left-radius" => border::top_left_radius,
		"border-top-right-radius" => border::top_right_radius,
		"border-top-style" => border::top_style,
		"border-top-width" => border::top_width,
		"border-width" => border::width,

        _ => return error("invalid or unsupported property")
    };

    f(parser, element_style)
}

fn color<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    element_style.font_color = value::color(parser)?;
    Ok(())
}

fn opacity<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    element_style.opacity = parser.expect_number()?;
    Ok(())
}

fn overflow<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    element_style.overflow = overflow_any(parser)?;
    Ok(())
}

fn overflow_x<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    element_style.overflow = overflow_any(parser)?;
    Ok(())
}

fn overflow_y<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    element_style.overflow = overflow_any(parser)?;
    Ok(())
}

fn overflow_any<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> CssResult<'i, Overflow> {
    let ident = parser.expect_ident()?;

    match_ignore_ascii_case! { ident.as_ref(),
        "visible" => Ok(Overflow::Visible),
        "hidden" => Ok(Overflow::Hidden),
        _ => error("invalid overflow value"),
    }
}

fn visibility<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    let ident = parser.expect_ident()?;

    match_ignore_ascii_case! { ident.as_ref(),
        "visible" => element_style.visible = true,
        "hidden" => element_style.visible = false,
        _ => return error("invalid visibility value"),
    }

    Ok(())
}

fn isolation<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    let ident = parser.expect_ident()?;

    match_ignore_ascii_case! { ident.as_ref(),
        "auto" => element_style.isolate = false,
        "isolate" => element_style.isolate = true,
        _ => return error("invalid isolation value"),
    }

    Ok(())
}

// none | <shadow>#
//
// https://developer.mozilla.org/en/docs/Web/CSS/box-shadow
fn box_shadow<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    element_style.box_shadows = syntax::one_of_2(parser,
        |parser| {
            parser.expect_ident_matching("none")?;
            Ok(Vec::new())
        },
        |parser| parser.parse_comma_separated(shadow)
    )?;

    Ok(())
}

// none | <filter-function-list>
//
// where 
// <filter-function-list> = [ <filter-function> | <url> ]+
//
// https://developer.mozilla.org/en/docs/Web/CSS/filter
fn filter<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    element_style.filters = syntax::one_of_2(parser,
        |parser| {
            parser.expect_ident_matching("none")?;
            Ok(Vec::new())
        },
        |parser| syntax::one_or_more(parser, filter_function)
    )?;

    Ok(())
}

// <filter-function> = <blur()> | <brightness()> | <contrast()> | <drop-shadow()> | <grayscale()> | <hue-rotate()> | <invert()> | <opacity()> | <sepia()> | <saturate()>
// 
// where 
// <blur()> = blur( <length> )
// <brightness()> = brightness( <number-percentage> )
// <contrast()> = contrast( [ <number-percentage> ] )
// <drop-shadow()> = drop-shadow( <length>{2,3} <color>? )
// <grayscale()> = grayscale( <number-percentage> )
// <hue-rotate()> = hue-rotate( <angle> )
// <invert()> = invert( <number-percentage> )
// <opacity()> = opacity( [ <number-percentage> ] )
// <sepia()> = sepia( <number-percentage> )
// <saturate()> = saturate( <number-percentage> )
fn filter_function<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> CssResult<'i, Filter> {
    let function = parser.expect_function()?;

    Ok(match_ignore_ascii_case! { function.as_ref(),
        "blur" => Filter::Blur(parser.parse_nested_block(value::length)?),
        "brightness" => Filter::Brightness(parser.parse_nested_block(value::number_or_percentage)?),
        "contrast" => Filter::Contrast(parser.parse_nested_block(value::number_or_percentage)?),
        "drop-shadow" => Filter::DropShadow(parser.parse_nested_block(shadow)?), // FIXME: syntax is different
        "grayscale" => Filter::Grayscale(parser.parse_nested_block(value::number_or_percentage)?),
        "hue-rotate" => Filter::HueRotate(parser.parse_nested_block(value::angle)?),
        "invert" => Filter::Invert(parser.parse_nested_block(value::number_or_percentage)?),
        "opacity" => Filter::Opacity(parser.parse_nested_block(value::number_or_percentage)?),
        "sepia" => Filter::Sepia(parser.parse_nested_block(value::number_or_percentage)?),
        "saturate" => Filter::Saturate(parser.parse_nested_block(value::number_or_percentage)?),
        _ => return error("invalid filter"),
    })
}

// <shadow> = inset? && <length>{2,4} && <color>?
fn shadow<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> CssResult<'i, Shadow> {
    let (inset, lengths, color) = syntax::all_of_3(parser,
        |parser| syntax::maybe(parser, |parser| Ok(parser.expect_ident_matching("inset")?)),
        |parser| syntax::between(parser, 2, 4, value::length),
        |parser| syntax::maybe(parser, value::color),
    )?;

    Ok(Shadow {
        color: color.unwrap_or(Color::black()),
        position: Vec2::new(lengths[0], lengths[1]),
        inset: inset == Some(()),
        blur: lengths.get(2).cloned().unwrap_or(0.0),
        spread: lengths.get(3).cloned().unwrap_or(0.0),
    })
}

#[test]
fn test_lengths() {
    use theme::element_style::LengthOrPercentage;

    for &(a, b) in &[
        ("96px",   "1in"),
        ("1cm",   "10mm"),
        ("0.25cm", "1q"),
        ("25.4mm", "1in"),
        ("1in",   "72pt"),
        ("1in",    "6pc"),
    ]
    {
        assert_eq!(value::length(&mut Parser::new(&mut ParserInput::new(a))).unwrap(), value::length(&mut Parser::new(&mut ParserInput::new(b))).unwrap());
    }

    assert_eq!(value::length(&mut Parser::new(&mut ParserInput::new("100px"))).unwrap(), 100.0);
    
    for &(a, b) in &[
        ("1px",   LengthOrPercentage::Length(1.0)),
        ("50%",   LengthOrPercentage::Percentage(0.5)),
        ("50.5%", LengthOrPercentage::Percentage(0.505)),
    ]
    {
        assert_eq!(value::length_or_percentage(&mut Parser::new(&mut ParserInput::new(a))).unwrap(), b);
    }

    for &(a, b) in &[
        ("0.6",   0.6),
        ("50%",   0.5),
        ("50.5%", 0.505),
    ]
    {
        assert_eq!(value::number_or_percentage(&mut Parser::new(&mut ParserInput::new(a))).unwrap(), b);
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
        assert_eq!(shadow(&mut Parser::new(&mut ParserInput::new(a))).unwrap(), b);
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
        assert_eq!(filter_function(&mut Parser::new(&mut ParserInput::new(a))).unwrap(), b);
    }
}