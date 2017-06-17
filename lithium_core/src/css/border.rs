use cssparser::{Parser, ParseError};
#[cfg(test)]
use cssparser::ParserInput;
use Color;
use theme::ElementStyle;
use theme::element_style::{BorderStyle, LengthOrPercentage, border, corner};
use super::{parse_color, parse_length_or_percentage, parse_length};

const THIN_BORDER: f32 = 1.0;
const MEDIUM_BORDER: f32 = 2.0;
const THICK_BORDER: f32 = 3.0;

pub fn border<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

pub fn border_bottom<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

pub fn border_left<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

pub fn border_right<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

pub fn border_top<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

pub fn border_bottom_color<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    border_side_color(parser, element_style, border::BOTTOM)
}

pub fn border_left_color<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    border_side_color(parser, element_style, border::LEFT)
}

pub fn border_right_color<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    border_side_color(parser, element_style, border::RIGHT)
}

pub fn border_top_color<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    border_side_color(parser, element_style, border::TOP)
}

pub fn border_side_color<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle, side: usize) -> Result<(), ParseError<'i, ()>> {
    element_style.border[side].color = parse_color(parser)?;
    Ok(())
}

pub fn border_bottom_left_radius<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    border_corner_radius(parser, element_style, corner::BOTTOM_LEFT)
}

pub fn border_bottom_right_radius<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    border_corner_radius(parser, element_style, corner::BOTTOM_RIGHT)
}

pub fn border_top_left_radius<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    border_corner_radius(parser, element_style, corner::TOP_LEFT)
}

pub fn border_top_right_radius<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    border_corner_radius(parser, element_style, corner::TOP_RIGHT)
}

pub fn border_corner_radius<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle, corner: usize) -> Result<(), ParseError<'i, ()>> {
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

pub fn border_bottom_style<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    border_side_style(parser, element_style, border::BOTTOM)
}

pub fn border_left_style<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    border_side_style(parser, element_style, border::LEFT)
}

pub fn border_top_style<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    border_side_style(parser, element_style, border::TOP)
}

pub fn border_right_style<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    border_side_style(parser, element_style, border::RIGHT)
}

pub fn border_side_style<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle, border: usize) -> Result<(), ParseError<'i, ()>> {
    element_style.border[border].style = parse_border_style(parser)?;
    Ok(())
}

pub fn border_bottom_width<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    border_side_width(parser, element_style, border::BOTTOM)
}

pub fn border_top_width<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    border_side_width(parser, element_style, border::TOP)
}

pub fn border_left_width<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    border_side_width(parser, element_style, border::LEFT)
}

pub fn border_right_width<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    border_side_width(parser, element_style, border::RIGHT)
}

pub fn border_side_width<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle, border: usize) -> Result<(), ParseError<'i, ()>> {
    element_style.border[border].width = parse_border_width(parser)?;
    Ok(())
}

pub fn border_color<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
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

pub fn border_image<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

pub fn border_image_outset<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

pub fn border_image_repeat<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

pub fn border_image_slice<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

pub fn border_image_source<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

pub fn border_image_width<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

pub fn border_radius<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
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

pub fn border_style<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
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

pub fn border_width<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
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
fn test_four_values() {
    for &(a, b) in &[
        ("1px",       [1.0, 1.0, 1.0, 1.0]),
        ("1px 2px",     [1.0, 2.0, 1.0, 2.0]),
        ("1px 2px 3px",   [1.0, 2.0, 3.0, 2.0]),
        ("1px 2px 3px 4px", [1.0, 2.0, 3.0, 4.0]),
    ]
    {
        assert_eq!(parse_four_values(&mut Parser::new(&mut ParserInput::new(a)), parse_length).unwrap(), b);
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
