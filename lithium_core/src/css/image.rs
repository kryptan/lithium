// https://drafts.csswg.org/css-images/

use cssparser::Parser;
use theme::element_style::{LengthOrPercentage, BackgroundImage, LinearGradient, RadialGradient,
    AngleOrCorner, ColorStop, RadialGradientShape, RadialGradientExtent, PositionCoordinate};
use Vec2;
use super::{CssResult, syntax, error, value};

// <image> = <url> | <image-set()> | <cross-fade()> | <gradient>
//
// https://drafts.csswg.org/css-images/#image-values
pub fn image<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> CssResult<'i, BackgroundImage> {
    gradient(parser)

    // FIXME: parse other variants
    /*
    syntax::one_of_n(parser,
        gradient,
    )*/
}

// <gradient> =
//   <linear-gradient()> | <repeating-linear-gradient()> |
//   <radial-gradient()> | <repeating-radial-gradient()>
//
// https://drafts.csswg.org/css-images/#gradients
fn gradient<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> CssResult<'i, BackgroundImage> {
    let function = parser.expect_function()?;

    Ok(match_ignore_ascii_case! { function.as_ref(),
        "linear-gradient" => BackgroundImage::LinearGradient(parser.parse_nested_block(|parser| linear_gradient(parser, false))?),
        "radial-gradient" => BackgroundImage::RadialGradient(parser.parse_nested_block(|parser| radial_gradient(parser, false))?),
        "repeating-linear-gradient" => BackgroundImage::LinearGradient(parser.parse_nested_block(|parser| linear_gradient(parser, true))?),
        "repeating-radial-gradient" => BackgroundImage::RadialGradient(parser.parse_nested_block(|parser| radial_gradient(parser, true))?),
        _ => return error(""),
    })
}

// linear-gradient() = linear-gradient(
//   [ <zero> | <angle> | to <side-or-corner> ]? ,
//   <color-stop-list>
// )
//
// <side-or-corner> = [left | right] || [top | bottom]
//
// https://drafts.csswg.org/css-images/#linear-gradient-syntax
fn linear_gradient<'i, 'tt>(parser: &mut Parser<'i, 'tt>, repeating: bool) -> CssResult<'i, LinearGradient> {
    // [ <zero> | <angle> | to <side-or-corner> ]?
    let direction = syntax::maybe(parser, angle_or_corner)?.unwrap_or(AngleOrCorner::Angle(0.0));

    // ,
    parser.expect_comma()?;

    // <color-stop-list>
    let stops = color_stop_list(parser)?;
    
    Ok(LinearGradient {
        direction,
        stops,
        repeating,
    })
}

// radial-gradient() = radial-gradient(
//   [ <ending-shape> || <size> ]? [ at <position> ]?,
//   <color-stop-list>
// )
//
// https://drafts.csswg.org/css-images/#radial-gradient-syntax
fn radial_gradient<'i, 'tt>(parser: &mut Parser<'i, 'tt>, repeating: bool) -> CssResult<'i, RadialGradient> {
    // [ <ending-shape> || <size> ]?
    let shape = syntax::maybe(parser, radial_gradient_shape)?
        .unwrap_or(RadialGradientShape::Ellipse2(Vec2::new(LengthOrPercentage::Percentage(1.0), LengthOrPercentage::Percentage(1.0))));

    // [ at <position> ]?
    let position = syntax::maybe(parser, |parser| {
        parser.expect_ident_matching("at")?;
        value::position(parser)
    })?.unwrap_or(Vec2::new(PositionCoordinate::Percentage(0.5), PositionCoordinate::Percentage(0.5)));

    // ,
    parser.expect_comma()?;

    // <color-stop-list>
    let stops = color_stop_list(parser)?;

    Ok(RadialGradient {
        shape,
        position,
        stops,
        repeating,
    })
}

// <ending-shape> || <size>
//
// https://drafts.csswg.org/css-images/#radial-gradient-syntax
fn radial_gradient_shape<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> CssResult<'i, RadialGradientShape> {
    css_enum!(Shape, Circle = "circle", Ellipse = "ellipse");

    enum Size {
        Extent(RadialGradientExtent),
        Circle(f32),
        Ellipse(Vec2<LengthOrPercentage>),
    }

    let (shape, size) = syntax::one_or_more_of_2(parser,
        Shape::parse,
        |parser| {
            syntax::one_of_3(parser,
                |parser| {
                    let ident = parser.expect_ident()?;
                    Ok(Size::Extent(match_ignore_ascii_case! { ident.as_ref(),
                        "closest-side" => RadialGradientExtent::ClosestSide,
                        "farthest-side" => RadialGradientExtent::FarthestCorner,
                        "closest-corner" => RadialGradientExtent::ClosestCorner,
                        "farthest-corner" => RadialGradientExtent::FarthestCorner,
                        _ => return error(""),
                    }))
                },

                // <length>
                |parser| {
                    Ok(Size::Circle(value::length(parser)?))
                },

                // <length-percentage>{2}
                |parser| {
                    Ok(Size::Ellipse(Vec2::new(value::length_or_percentage(parser)?, value::length_or_percentage(parser)?)))
                },
            )
        },
    )?;

    let shape2 = match size {
        Some(Size::Circle(_))  => Some(Shape::Circle),
        Some(Size::Ellipse(_)) => Some(Shape::Ellipse),
        _ => None,
    };

    let shape = match (shape, shape2) {
        (Some(a), Some(b)) if a == b => a,
        (Some(_), Some(_)) => return error("invalid radial gradient shape"),
        (Some(a), None) | (None, Some(a)) => a,
        (None, None) => Shape::Ellipse,
    };

    Ok(match (shape, size) {
        (Shape::Circle,  Some(Size::Extent(extent))) => RadialGradientShape::Circle(extent),
        (Shape::Ellipse, Some(Size::Extent(extent))) => RadialGradientShape::Ellipse(extent),
        (Shape::Circle,  Some(Size::Circle(radius))) => RadialGradientShape::Ellipse2(Vec2::new(LengthOrPercentage::Length(radius), LengthOrPercentage::Length(radius))),
        (Shape::Ellipse, Some(Size::Ellipse(size)))  => RadialGradientShape::Ellipse2(size),
        (Shape::Ellipse, None) => RadialGradientShape::Ellipse(RadialGradientExtent::FarthestCorner),
        _ => return error(""), // unreachable
    })
}

// <zero> | <angle> | to <side-or-corner>
//
// <side-or-corner> = [left | right] || [top | bottom]
fn angle_or_corner<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> CssResult<'i, AngleOrCorner> {
    css_enum!(Horizontal, Left = "left", Right = "right");
    css_enum!(Vertical, Top = "top", Bottom = "bottom");

    syntax::one_of_3(parser,
        // <zero>
        |parser| {
            value::zero(parser)?;
            Ok(AngleOrCorner::Angle(0.0))
        },

        // <angle>
        |parser| Ok(AngleOrCorner::Angle(value::angle(parser)?)),

        // to [left | right] || [top | bottom]
        |parser| {
            // to
            parser.expect_ident_matching("to")?;

            // [left | right] || [top | bottom]
            let (horizontal, vertical) = syntax::one_or_more_of_2(parser,
                Horizontal::parse,
                Vertical::parse,
            )?;

            Ok(match (horizontal, vertical) {
                (Some(Horizontal::Left),  Some(Vertical::Top)   ) => AngleOrCorner::TopLeft,
                (Some(Horizontal::Right), Some(Vertical::Top)   ) => AngleOrCorner::TopRight,
                (Some(Horizontal::Left),  Some(Vertical::Bottom)) => AngleOrCorner::BottomLeft,
                (Some(Horizontal::Right), Some(Vertical::Bottom)) => AngleOrCorner::BottomRight,

                // If the argument is to top, to right, to bottom, or to left, the angle of the gradient line is 0deg, 90deg, 180deg, or 270deg, respectively.
                (None,                    Some(Vertical::Top)   ) => AngleOrCorner::Angle(0.0),
                (Some(Horizontal::Right), None                  ) => AngleOrCorner::Angle(90.0),
                (None,                    Some(Vertical::Bottom)) => AngleOrCorner::Angle(180.0),
                (Some(Horizontal::Left),  None                  ) => AngleOrCorner::Angle(270.0),

                (None, None) => return error(""), // should not happen
            })
        },
    )
}

// <color-stop-list> = <color-stop>#{2,}
//
// https://drafts.csswg.org/css-images/#color-stop-syntax
fn color_stop_list<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> CssResult<'i, Vec<ColorStop>> {
    let stops = parser.parse_comma_separated(color_stop)?;

    if stops.len() < 2 {
        error("too few gradient stops")
    } else {
        Ok(stops)
    }
}

// <color-stop> = <color> <length-percentage>?
//
// https://drafts.csswg.org/css-images/#color-stop-syntax
fn color_stop<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> CssResult<'i, ColorStop> {
    let color = value::color(parser)?;
    let position = syntax::maybe(parser, value::length_or_percentage)?;

    Ok(ColorStop {
        color,
        position,
    })
}
