// https://drafts.csswg.org/css-backgrounds/

#![cfg_attr(feature = "cargo-clippy", allow(needless_range_loop))]

use cssparser::Parser;
#[cfg(test)]
use cssparser::ParserInput;
use theme::ElementStyle;
use theme::element_style::{BorderStyle, border, corner};
use Color;
use super::{CssResult, error, syntax, value};

const THIN_BORDER: f32 = 1.0;
const MEDIUM_BORDER: f32 = 2.0;
const THICK_BORDER: f32 = 3.0;

// <br-width> || <br-style> || <color>
//
// https://drafts.csswg.org/css-backgrounds/#the-border-shorthands
pub fn border<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    let (width, style, color) = syntax::one_or_more_of_3(parser, line_width, line_style, value::color)?;

    for i in 0..4 {
        if let Some(width) = width {
            element_style.border[i].width = width;
        }

        if let Some(style) = style {
            element_style.border[i].style = style;
        }

        if let Some(color) = color {
            element_style.border[i].color = color;
        }
    }

    Ok(())
}

pub fn bottom<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    side(parser, element_style, border::BOTTOM)
}

pub fn left<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    side(parser, element_style, border::LEFT)
}

pub fn right<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    side(parser, element_style, border::RIGHT)
}

pub fn top<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    side(parser, element_style, border::TOP)
}

// <line-width> || <line-style> || <color>
//
// https://drafts.csswg.org/css-backgrounds/#the-border-shorthands
fn side<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle, side: usize) -> CssResult<'i, ()> {
    let (width, style, color) = syntax::one_or_more_of_3(parser, line_width, line_style, value::color)?;

    element_style.border[side].width = width.unwrap_or(MEDIUM_BORDER);
    element_style.border[side].style = style.unwrap_or(BorderStyle::None);
    element_style.border[side].color = color.unwrap_or(Color::black());

    Ok(())
}

pub fn bottom_color<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    side_color(parser, element_style, border::BOTTOM)
}

pub fn left_color<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    side_color(parser, element_style, border::LEFT)
}

pub fn right_color<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    side_color(parser, element_style, border::RIGHT)
}

pub fn top_color<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    side_color(parser, element_style, border::TOP)
}

pub fn side_color<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle, side: usize) -> CssResult<'i, ()> {
    element_style.border[side].color = value::color(parser)?;
    Ok(())
}

pub fn bottom_left_radius<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    corner_radius(parser, element_style, corner::BOTTOM_LEFT)
}

pub fn bottom_right_radius<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    corner_radius(parser, element_style, corner::BOTTOM_RIGHT)
}

pub fn top_left_radius<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    corner_radius(parser, element_style, corner::TOP_LEFT)
}

pub fn top_right_radius<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    corner_radius(parser, element_style, corner::TOP_RIGHT)
}

// <length-percentage>{1,2}
//
// https://drafts.csswg.org/css-backgrounds/#the-border-radius
fn corner_radius<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle, corner: usize) -> CssResult<'i, ()> {
    element_style.border_radius[corner].x = value::length_or_percentage(parser)?;
    element_style.border_radius[corner].y = parser.try(value::length_or_percentage).unwrap_or(element_style.border_radius[corner].x);

    Ok(())
}

pub fn bottom_style<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    side_style(parser, element_style, border::BOTTOM)
}

pub fn left_style<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    side_style(parser, element_style, border::LEFT)
}

pub fn top_style<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    side_style(parser, element_style, border::TOP)
}

pub fn right_style<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    side_style(parser, element_style, border::RIGHT)
}

pub fn side_style<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle, side: usize) -> CssResult<'i, ()> {
    element_style.border[side].style = line_style(parser)?;
    Ok(())
}

pub fn bottom_width<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    side_width(parser, element_style, border::BOTTOM)
}

pub fn top_width<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    side_width(parser, element_style, border::TOP)
}

pub fn left_width<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    side_width(parser, element_style, border::LEFT)
}

pub fn right_width<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    side_width(parser, element_style, border::RIGHT)
}

pub fn side_width<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle, side: usize) -> CssResult<'i, ()> {
    element_style.border[side].width = line_width(parser)?;
    Ok(())
}

pub fn border_image<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    error("unimplemented")
}

pub fn image_outset<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    error("unimplemented")
}

pub fn image_repeat<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    error("unimplemented")
}

pub fn image_slice<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    error("unimplemented")
}

pub fn image_source<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    error("unimplemented")
}

pub fn image_width<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    error("unimplemented")
}

// <length-percentage>{1,4} [ / <length-percentage>{1,4} ]?
//
// https://drafts.csswg.org/css-backgrounds/#the-border-radius
pub fn radius<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    let radii = four_values(parser, value::length_or_percentage)?;
    let radii2 = syntax::maybe(parser, |parser| {
        parser.expect_delim('/')?;
        four_values(parser, value::length_or_percentage)
    })?;

    for i in 0..4 {
        element_style.border_radius[i].x = radii[i];
        element_style.border_radius[i].y = radii2.unwrap_or(radii)[i];
    }

    Ok(())
}

// <color>{1,4} 
//
// https://drafts.csswg.org/css-backgrounds/#the-border-color
pub fn color<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    let colors = four_values(parser, value::color)?;

    for i in 0..4 {
        element_style.border[i].color = colors[i];
    }

    Ok(())
}

// <line-style>{1,4}
//
// https://drafts.csswg.org/css-backgrounds/#the-border-style
pub fn style<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    let styles = four_values(parser, line_style)?;

    for i in 0..4 {
        element_style.border[i].style = styles[i];
    }

    Ok(())
}

// <line-width>{1,4}
//
// https://drafts.csswg.org/css-backgrounds/#the-border-width
pub fn width<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    let widths = four_values(parser, line_width)?;

    for i in 0..4 {
        element_style.border[i].width = widths[i];
    }

    Ok(())
}

// <line-style> = none | hidden | dotted | dashed | solid | double | groove | ridge | inset | outset
fn line_style<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> CssResult<'i, BorderStyle> {
    let ident = parser.expect_ident()?;

    Ok(match_ignore_ascii_case! { ident.as_ref(),
        "none" | "hidden" => BorderStyle::None,
         "dotted" => BorderStyle::Dotted,
         "dashed" => BorderStyle::Dashed,
         "solid" => BorderStyle::Solid,
         "double" => BorderStyle::Double,
         "groove" => BorderStyle::Groove,
         "ridge" => BorderStyle::Ridge,
         "inset" => BorderStyle::Inset,
         "outset" => BorderStyle::Outset,
        _ => return error("invalid border style"),
    })
}

// <line-width> = <length> | thin | medium | thick
fn line_width<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> CssResult<'i, f32> {
    syntax::one_of_2(parser,
        value::length,
        |parser| {
            let ident = parser.expect_ident()?;
            Ok(match_ignore_ascii_case! { ident.as_ref(),
                "thin" => THIN_BORDER,
                "medium" | "initial" => MEDIUM_BORDER,
                "thick" => THICK_BORDER,
                _ => return error("invalid width"),
            })
        }
    )
}

fn four_values<'i, 'tt, F, R>(parser: &mut Parser<'i, 'tt>, f: F) -> CssResult<'i, [R; 4]>
  where
    F: for<'tt2> Fn(&mut Parser<'i, 'tt2>) -> CssResult<'i, R>,
    R: Copy
{
    let value1 = f(parser)?;
    
    if let Ok(value2) = parser.try(&f) {
        if let Ok(value3) = parser.try(&f) {
            if let Ok(value4) = parser.try(&f) {
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
fn test_four_values() {
    for &(a, b) in &[
        ("1px",       [1.0, 1.0, 1.0, 1.0]),
        ("1px 2px",     [1.0, 2.0, 1.0, 2.0]),
        ("1px 2px 3px",   [1.0, 2.0, 3.0, 2.0]),
        ("1px 2px 3px 4px", [1.0, 2.0, 3.0, 4.0]),
    ]
    {
        assert_eq!(four_values(&mut Parser::new(&mut ParserInput::new(a)), value::length).unwrap(), b);
    }
}

#[test]
fn test_line_width() {
    for &(a, b) in &[
        ("1px",     1.0),
        ("thin",    THIN_BORDER),
        ("thick",   THICK_BORDER),
        ("medium",  MEDIUM_BORDER),
    ]
    {
        assert_eq!(line_width(&mut Parser::new(&mut ParserInput::new(a))).unwrap(), b);
    }
}
