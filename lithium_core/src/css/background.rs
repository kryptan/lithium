// https://drafts.csswg.org/css-backgrounds/

use cssparser::Parser;
#[cfg(test)]
use cssparser::ParserInput;
use theme::ElementStyle;
use theme::element_style::{LengthOrPercentage, BackgroundImage, BackgroundLayer, BackgroundAttachment,
    BackgroundSize, BackgroundRepeat, BackgroundBox, PositionCoordinate};
use {Vec2, Color};
use super::{CssResult, syntax, error, value, image};

// [ <bg-layer> , ]* <final-bg-layer>
//
// https://developer.mozilla.org/en-US/docs/Web/CSS/background
pub fn background<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    let mut color = None;

    let layers = parser.parse_comma_separated(|parser| background_layer(parser, &mut color))?;
    element_style.background_layers = layers;

    if let Some(color) = color {
        element_style.background_color = color;
    }

    Ok(())
}

// <bg-layer> = <bg-image> || <position> [ / <bg-size> ]? || <repeat-style> || <attachment> || <box>{1,2}
// <final-bg-layer> = <bg-image> || <position> [ / <bg-size> ]? || <repeat-style> || <attachment> || <box> || <box> || <'background-color'>
pub fn background_layer<'i, 'tt>(parser: &mut Parser<'i, 'tt>, background_color: &mut Option<Color>) -> CssResult<'i, BackgroundLayer> {
    let (image, position_and_size, repeat_style, attachment, clip_and_origin, color) = syntax::one_or_more_of_6(parser,
        parse_background_image,
        background_position_and_size,
        repeat_style,
        parse_attachment,
        parse_background_clip_and_origin,
        value::color,
    )?;
    
    if background_color.is_some() {
        return error("color set in non final background layer");
    }
    *background_color = color;

    let mut layer = BackgroundLayer::default();

    if let Some(image) = image {
        layer.image = image;
    }

    if let Some(attachment) = attachment {
        layer.attachment = attachment;
    }

    if let Some((position, size)) = position_and_size {
        layer.position = position;
        layer.size = size;
    }

    if let Some((repeat_x, repeat_y)) = repeat_style {
        layer.repeat_x = repeat_x;
        layer.repeat_y = repeat_y;
    }

    if let Some((clip, origin)) = clip_and_origin {
        layer.clip = clip;
        layer.origin = origin;
    }

    Ok(layer)
}

pub fn background_blend_mode<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> CssResult<'i, ()> {    
    error("unimplemented")
}

// <color>
//
// https://drafts.csswg.org/css-backgrounds/#the-background-color
pub fn background_color<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    element_style.background_color = value::color(parser)?;
    Ok(())
}

// <image> | none
//
// https://drafts.csswg.org/css-backgrounds/#the-background-image
fn parse_background_image<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> CssResult<'i, BackgroundImage> {
    syntax::one_of_2(parser,
        |parser| {
            parser.expect_ident_matching("none")?;
            Ok(BackgroundImage::None)
        },
        image::image,
    )
}

// <bg-position> = [
//   [ left | center | right | top | bottom | <length-percentage> ]
// |
//   [ left | center | right | <length-percentage> ]
//   [ top | center | bottom | <length-percentage> ]
// |
//   [ center | [ left | right ] <length-percentage>? ] &&
//   [ center | [ top | bottom ] <length-percentage>? ]
// ]
//
// https://drafts.csswg.org/css-backgrounds-3/#background-position
fn bg_position<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> CssResult<'i, Vec2<PositionCoordinate>> {
    //  center | [ left | right ] <length-percentage>?
    fn two_values<'i, 'tt>(parser: &mut Parser<'i, 'tt>, left: &'static str, right: &'static str) -> CssResult<'i, PositionCoordinate> {
        syntax::one_of_2(parser,
            // center
            |parser| {
                parser.expect_ident_matching("center")?;
                Ok(PositionCoordinate::Percentage(0.5))
            },
            // [ left | right ] <length-percentage>?
            |parser| {
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
                
                let offset = syntax::maybe(parser, value::length_or_percentage)?.unwrap_or(LengthOrPercentage::Percentage(0.0));

                Ok(match (opposite, offset) {
                    (false, LengthOrPercentage::Percentage(x)) => PositionCoordinate::Percentage(x),
                    (true,  LengthOrPercentage::Percentage(x)) => PositionCoordinate::Percentage(1.0 - x),
                    (false, LengthOrPercentage::Length(x))     => PositionCoordinate::Length(x),
                    (true,  LengthOrPercentage::Length(x))     => PositionCoordinate::LengthOpposite(x),
                })
            }
        )
    };

    syntax::one_of_5(parser,
        // left | center | right | top | bottom
        |parser| {
            let ident = parser.expect_ident()?;
            Ok(match_ignore_ascii_case! { ident.as_ref(),
                "left"   => Vec2::new(PositionCoordinate::Percentage(0.0), PositionCoordinate::Percentage(0.5)),
                "center" => Vec2::new(PositionCoordinate::Percentage(0.5), PositionCoordinate::Percentage(0.5)),
                "right"  => Vec2::new(PositionCoordinate::Percentage(1.0), PositionCoordinate::Percentage(0.5)),
                "top"    => Vec2::new(PositionCoordinate::Percentage(0.5), PositionCoordinate::Percentage(0.0)),
                "bottom" => Vec2::new(PositionCoordinate::Percentage(0.5), PositionCoordinate::Percentage(1.0)),
                _ => return error(""),
            })
        },

        // <length>
        |parser| Ok(Vec2::new(PositionCoordinate::Length(value::length(parser)?), PositionCoordinate::Percentage(0.5))),

        // <percentage>
        |parser| Ok(Vec2::new(PositionCoordinate::Percentage(parser.expect_percentage()?), PositionCoordinate::Percentage(0.5))),

        // [ left | center | right | <length-percentage> ] [ top | center | bottom | <length-percentage> ]
        |parser| {
            // left | center | right | <length-percentage>
            let x = value::one_value_position(parser, "left", "right")?;

            // top | center | bottom | <length-percentage>
            let y = value::one_value_position(parser, "top", "bottom")?;

            Ok(Vec2::new(x, y))
        },

        // [ center | [ left | right ] [ <length-percentage> ]? ] && [ center | [ top | bottom ] [ <length-percentage> ]? ]
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

// <attachment> = scroll | fixed | local
//
// https://drafts.csswg.org/css-backgrounds/#the-background-attachment
fn parse_attachment<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> CssResult<'i, BackgroundAttachment> {
    let ident = parser.expect_ident()?;
    Ok(match_ignore_ascii_case! { ident.as_ref(),
        "scroll" => BackgroundAttachment::Scroll,
        "fixed" => BackgroundAttachment::Fixed,
        "local" => BackgroundAttachment::Local,
        _ => return error(""),
    })
}

// <bg-position> [ / <bg-size> ]? 
fn background_position_and_size<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> CssResult<'i, (Vec2<PositionCoordinate>, BackgroundSize)> {
    let position = bg_position(parser)?;
    let size = syntax::maybe(parser, |parser| {
        parser.expect_delim('/')?;
        bg_size(parser)
    })?.unwrap_or(BackgroundSize::default());
    
    Ok((position, size))
}

// <bg-size> = [ <length-percentage> | auto ]{1,2} | cover | contain
//
// https://drafts.csswg.org/css-backgrounds/#bg-size
fn bg_size<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> CssResult<'i, BackgroundSize> {
    syntax::one_of_2(parser,
        // [ <length-percentage> | auto ]{1,2}
        |parser| {
            let x = value::length_or_percentage_auto(parser)?;
            let y = syntax::maybe(parser, value::length_or_percentage_auto)?.unwrap_or(x);
            Ok(BackgroundSize::Size(Vec2::new(x, y)))
        },

        // cover | contain
        |parser| {
            let ident = parser.expect_ident()?;
            Ok(match_ignore_ascii_case! { ident.as_ref(),
                "cover" => BackgroundSize::Cover,
                "contain" => BackgroundSize::Contain,
                _ => return error(""),
            })
        },
    )
}

fn parse_background_repeat<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> CssResult<'i, BackgroundRepeat> {
    let ident = parser.expect_ident()?;
    Ok(match_ignore_ascii_case! { ident.as_ref(),
        "repeat" => BackgroundRepeat::Repeat,
        "space" => BackgroundRepeat::Space,
        "round" => BackgroundRepeat::Round,
        "no-repeat" => BackgroundRepeat::NoRepeat,
        _ => return error(""),
    })
}

// <repeat-style> = repeat-x | repeat-y | [repeat | space | round | no-repeat]{1,2}
//
// https://drafts.csswg.org/css-backgrounds/#the-background-repeat
fn repeat_style<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> CssResult<'i, (BackgroundRepeat, BackgroundRepeat)> {
    syntax::one_of_2(parser,
        |parser| {
            let ident = parser.expect_ident()?;
            Ok(match_ignore_ascii_case! { ident.as_ref(),
                "repeat-x" => (BackgroundRepeat::Repeat, BackgroundRepeat::NoRepeat),
                "repeat-y" => (BackgroundRepeat::NoRepeat, BackgroundRepeat::Repeat),
                _ => return error(""),
            })
        },
        |parser| {
            let repeat_x = parse_background_repeat(parser)?;
            let repeat_y = parser.try(parse_background_repeat).unwrap_or(repeat_x);
            
            Ok((repeat_x, repeat_y))
        }
    )
}

fn parse_box<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> CssResult<'i, BackgroundBox> {
    let ident = parser.expect_ident()?;
    Ok(match_ignore_ascii_case! { ident.as_ref(),
        "border-box" => BackgroundBox::BorderBox,
        "padding-box" => BackgroundBox::PaddingBox,
        _ => return error(""),
    })
}

fn parse_background_clip_and_origin<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> CssResult<'i, (BackgroundBox, BackgroundBox)> {
    let clip = parse_box(parser)?;

    let origin = if let Ok(origin) = parser.try(parse_box) {
        origin
    } else {
        BackgroundBox::PaddingBox
    };

    Ok((clip, origin))
}

// FIXME: order dependent
pub fn background_image<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    let images = parser.parse_comma_separated(image::image)?;
    element_style.background_layers = images.into_iter().map(|image| {
        BackgroundLayer {
            image,
            .. BackgroundLayer::default()
        }
    }).collect();

    Ok(())
}

// FIXME: order dependent
pub fn background_attachment<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    let attachments = parser.parse_comma_separated(parse_attachment)?;
    for (i, layer) in element_style.background_layers.iter_mut().enumerate() {
        layer.attachment = attachments[i % attachments.len()];
    }

    Ok(())
}

// FIXME: order dependent
pub fn background_position<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    let positions = parser.parse_comma_separated(bg_position)?;
    for (i, layer) in element_style.background_layers.iter_mut().enumerate() {
        layer.position = positions[i % positions.len()];
    }

    Ok(())
}

// FIXME: order dependent
pub fn background_repeat<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    let repeat_styles = parser.parse_comma_separated(repeat_style)?;
    for (i, layer) in element_style.background_layers.iter_mut().enumerate() {
        layer.repeat_x = repeat_styles[i % repeat_styles.len()].0;
        layer.repeat_y = repeat_styles[i % repeat_styles.len()].1;
    }

    Ok(())
}

// FIXME: order dependent
pub fn background_clip<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    let clips = parser.parse_comma_separated(parse_box)?;
    for (i, layer) in element_style.background_layers.iter_mut().enumerate() {
        layer.clip = clips[i % clips.len()];
    }

    Ok(())
}

// FIXME: order dependent
pub fn background_origin<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    let origins = parser.parse_comma_separated(parse_box)?;
    for (i, layer) in element_style.background_layers.iter_mut().enumerate() {
        layer.origin = origins[i % origins.len()];
    }

    Ok(())
}

// FIXME: order dependent
pub fn background_size<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    let sizes = parser.parse_comma_separated(bg_size)?;
    for (i, layer) in element_style.background_layers.iter_mut().enumerate() {
        layer.size = sizes[i % sizes.len()];
    }

    Ok(())
}

#[test]
fn test_repeat_style() {
    for &(a, b) in &[
        ("repeat-x",        (BackgroundRepeat::Repeat,   BackgroundRepeat::NoRepeat)),
        ("repeat-y",        (BackgroundRepeat::NoRepeat, BackgroundRepeat::Repeat)),
        ("repeat",          (BackgroundRepeat::Repeat,   BackgroundRepeat::Repeat)),
        ("space",           (BackgroundRepeat::Space,    BackgroundRepeat::Space)),
        ("round no-repeat", (BackgroundRepeat::Round,    BackgroundRepeat::NoRepeat)),
    ]
    {
        assert_eq!(repeat_style(&mut Parser::new(&mut ParserInput::new(a))).unwrap(), b);
    }
}
