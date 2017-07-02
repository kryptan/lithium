// https://drafts.csswg.org/css-values-4/

use std;
use cssparser;
use cssparser::{Parser, Token};
use theme::element_style::{LengthOrPercentage, PositionCoordinate};
use {Vec2, Color};
use super::{CssResult, syntax, error};

// https://drafts.csswg.org/css-values-4/#absolute-lengths
pub fn length<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> CssResult<'i, f32> {
    number_with_unit_or_zero(parser, |unit| {
        Some(match_ignore_ascii_case! { unit.as_ref(),
            "px" => 1.0,
            "cm" => 96.0/2.54,
            "mm" => 96.0*0.1/2.54,
            "q" => 96.0*0.25/2.54,
            "in" => 96.0,
            "pc" => 96.0/6.0,
            "pt" => 96.0/72.0,
            _ => return None,
        })
    })
}

// <length> | <percentage>
pub fn length_or_percentage<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> CssResult<'i, LengthOrPercentage> {
    syntax::one_of_2(parser,
        |parser| Ok(LengthOrPercentage::Length(length(parser)?)),
        |parser| Ok(LengthOrPercentage::Percentage(parser.expect_percentage()?)),
    )
}

// <number> | <percentage>
pub fn number_or_percentage<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> CssResult<'i, f32> {
    syntax::one_of_2(parser,
        |parser| Ok(parser.expect_number()?),
        |parser| Ok(parser.expect_percentage()?),
    )
}

// <length> | <percentage> | auto
pub fn length_or_percentage_auto<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> CssResult<'i, Option<LengthOrPercentage>> {
    syntax::one_of_2(parser,    
        |parser| Ok(Some(length_or_percentage(parser)?)),
        |parser| {
            parser.expect_ident_matching("auto")?;
            Ok(None)
        },
    )
}

// https://drafts.csswg.org/css-values-4/#angles
pub fn angle<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> CssResult<'i, f32> {
    if let Token::Dimension{ value, unit, .. } = parser.next()? {
        Ok(value*match_ignore_ascii_case! { unit.as_ref(),
            "deg" => 1.0,
            "grad" => 360.0/400.0,
            "rad" => 360.0/(2.0*std::f32::consts::PI),
            "turn" => 360.0,
            _ => return error("invalid angle unit"),
        })
    } else {
        error("expected angle")
    }
}

pub fn color<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> CssResult<'i, Color> {
    match cssparser::Color::parse(parser)? {
        cssparser::Color::RGBA(rgba) => Ok(Color::from_rgba32(rgba.red, rgba.green, rgba.blue, rgba.alpha)),
        cssparser::Color::CurrentColor => error("current color is not supported"),
    }
}

// left | center | right | <length-percentage>
pub fn one_value_position<'i, 'tt>(parser: &mut Parser<'i, 'tt>, left: &'static str, right: &'static str) -> CssResult<'i, PositionCoordinate> {
    syntax::one_of_5(parser,
        |parser| Ok(PositionCoordinate::Percentage(parser.expect_percentage()?)),
        |parser| Ok(PositionCoordinate::Length(length(parser)?)),
        |parser| {
            parser.expect_ident_matching(left)?;
            Ok(PositionCoordinate::Percentage(0.5))
        },
        |parser| {
            parser.expect_ident_matching("center")?;
            Ok(PositionCoordinate::Percentage(0.5))
        },
        |parser| {
            parser.expect_ident_matching(right)?;
            Ok(PositionCoordinate::Percentage(0.5))
        },
    )
}

// <position> = [
//   [ left | center | right ] || [ top | center | bottom ]
// |
//   [ left | center | right | <length-percentage> ]
//   [ top | center | bottom | <length-percentage> ]?
// |
//   [ [ left | right ] <length-percentage> ] &&
//   [ [ top | bottom ] <length-percentage> ]
// ]
//
// https://drafts.csswg.org/css-values-4/#position
pub fn position<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> CssResult<'i, Vec2<PositionCoordinate>> {
    // [ left | right ] <length-percentage>?
    fn two_values<'i, 'tt>(parser: &mut Parser<'i, 'tt>, left: &'static str, right: &'static str) -> CssResult<'i, PositionCoordinate> {
        let opposite = syntax::one_of_2(parser,
            |parser| {
                parser.expect_ident_matching(left)?;
                Ok(false)
            },
            |parser| {
                parser.expect_ident_matching(right)?;
                Ok(true)
            },
        )?;
        
        let offset = length_or_percentage(parser)?;

        Ok(match (opposite, offset) {
            (false, LengthOrPercentage::Percentage(x)) => PositionCoordinate::Percentage(x),
            (true,  LengthOrPercentage::Percentage(x)) => PositionCoordinate::Percentage(1.0 - x),
            (false, LengthOrPercentage::Length(x))     => PositionCoordinate::Length(x),
            (true,  LengthOrPercentage::Length(x))     => PositionCoordinate::LengthOpposite(x),
        })
    };

    syntax::one_of_3(parser,
        // [ left | center | right ] || [ top | center | bottom ]
        |parser| {
            let (x, y) = syntax::one_or_more_of_2(parser,
                |parser| {
                    let ident = parser.expect_ident()?;
                    Ok(match_ignore_ascii_case! { ident.as_ref(),
                        "left"   => PositionCoordinate::Percentage(0.0),
                        "center" => PositionCoordinate::Percentage(0.5),
                        "right"  => PositionCoordinate::Percentage(1.0),
                        _ => return error(""),
                    })
                },
                |parser| {
                    let ident = parser.expect_ident()?;
                    Ok(match_ignore_ascii_case! { ident.as_ref(),
                        "top"    => PositionCoordinate::Percentage(0.0),
                        "center" => PositionCoordinate::Percentage(0.5),
                        "bottom" => PositionCoordinate::Percentage(1.0),
                        _ => return error(""),
                    })
                },
            )?;

            Ok(Vec2::new(x.unwrap_or(PositionCoordinate::Percentage(0.5)), y.unwrap_or(PositionCoordinate::Percentage(0.5))))
        },

        // [ left | center | right | <length-percentage> ] [ top | center | bottom | <length-percentage> ]?
        |parser| {
            // left | center | right | <length-percentage>
            let x = one_value_position(parser, "left", "right")?;

            // top | center | bottom | <length-percentage>
            let y = syntax::maybe(parser, |parser| one_value_position(parser, "top", "bottom"))?.unwrap_or(PositionCoordinate::Percentage(0.5));

            Ok(Vec2::new(x, y))
        },

        // [ [ left | right ] <length-percentage> ] && [ [ top | bottom ] <length-percentage> ]
        |parser| {
            let (x, y) = syntax::all_of_2(parser,
                // center | [ left | right ] <length-percentage>?
                |parser| two_values(parser, "left", "right"),

                // center | [ top | bottom ] <length-percentage>?
                |parser| two_values(parser, "top", "bottom"),
            )?;
            
            Ok(Vec2::new(x, y))
        },
    )
}

// https://drafts.csswg.org/css-values-4/#zero-value
pub fn zero<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> CssResult<'i, ()> {
    if parser.expect_number()? == 0.0 {
        Ok(())
    } else {
        error("expected zero")
    }
}

fn number_with_unit_or_zero<'i, 'tt, F>(parser: &mut Parser<'i, 'tt>, f: F) -> CssResult<'i, f32>
    where F: Fn(&str) -> Option<f32>
{
    syntax::one_of_2(parser,
        |parser| {
            if let Token::Dimension{ value, unit, .. } = parser.next()? {
                if let Some(unit) = f(&unit) {
                    Ok(value * unit)
                } else {
                    error("unknown unit")
                }
            } else {
                error("expected dimension")
            }
        },
        |parser| {
            zero(parser)?;
            Ok(0.0)
        }
    )
}

#[test]
fn test_angles() {
    use cssparser::ParserInput;

    for &(a, b) in &[
        ("360deg",   360.0),
        ("185.5deg", 185.5),
        ("1turn",    360.0),
        ("1rad",      57.295776),
        ("1grad",      0.9)]
    {
        assert_eq!(angle(&mut Parser::new(&mut ParserInput::new(a))).unwrap(), b);
    }
}

#[test]
fn test_color() {
    use cssparser::ParserInput;

    for &(a, b) in &[
        ("olive",   Color::from_css_hex(b"808000")),
        ("#123456", Color::from_css_hex(b"123456")),
        ("rgb(178, 81, 25)", Color::from_rgb24(178, 81, 25)),
    ]
    {
        assert_eq!(color(&mut Parser::new(&mut ParserInput::new(a))).unwrap(), b);
    }
}
