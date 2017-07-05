extern crate webrender_api;
extern crate lithium_core;

use webrender_api::{DisplayListBuilder, ColorF, GradientStop, LayoutPoint, LayoutSize, LayoutRect, ComplexClipRegion};
use lithium_core::gui::scene::Command;
use lithium_core::theme::ElementStyle;
use lithium_core::theme::element_style::{BackgroundImage, LengthOrPercentage, ColorStop, Border, BorderStyle, PositionCoordinate, RadialGradientShape, border, corner};
use lithium_core::{Color, Vec2, Rect};

fn convert_color(color: Color) -> ColorF {
    let (r, g, b, a) = color.to_sRGB();
    ColorF::new(r, g, b, a)
}

fn to_layout_point(v: Vec2<f64>) -> LayoutPoint {
    LayoutPoint::new(v.x as f32, v.y as f32)
}

fn to_layout_size(v: Vec2<f64>) -> LayoutSize {
    LayoutSize::new(v.x as f32, v.y as f32)
}

pub fn build(layout_size: Vec2<f64>, commands: &[Command], builder: &mut DisplayListBuilder) {
    for command in commands.iter().rev() {
        match command {
            &Command::CloseElement(ref element) => {
                build_element(layout_size, element.place, &element.style, builder);
            }
            &Command::StartElement => {

            }
            &Command::Text(ref _text) => {}
            &Command::Mesh(ref _mesh) => {}
        }
    }
}

fn build_element(layout_size: Vec2<f64>, place: Rect<f64>, style: &ElementStyle, builder: &mut DisplayListBuilder) {
    use webrender_api::{ExtendMode, BorderWidths, BorderDetails, NormalBorder, BorderRadius};
    use lithium_core::theme::element_style::AngleOrCorner;

    let rect = LayoutRect::new(
        to_layout_point(place.top_left()),
        to_layout_size(place.size()),
    );

    let layout_rect = LayoutRect::new(
        LayoutPoint::zero(),
        to_layout_size(layout_size),
    );

    let size = place.size();
    let width = place.width();
    let height = place.height();

    let radii = if style.border_radius.iter().any(
        |&radius|
            radius != Vec2::new(LengthOrPercentage::Length(0.0), LengthOrPercentage::Length(0.0)) &&
            radius != Vec2::new(LengthOrPercentage::Percentage(0.0), LengthOrPercentage::Percentage(0.0))
    ) {
        let mut radii = [Vec2::zero(); 4];

        for i in 0..4 {
            radii[i] = length_or_percentage_vec(style.border_radius[i], size);
        }

        // https://drafts.csswg.org/css-backgrounds/#corner-overlap
        let f = (width/(radii[corner::TOP_LEFT].x + radii[corner::TOP_RIGHT].x))
            .min(width/(radii[corner::BOTTOM_LEFT].x + radii[corner::BOTTOM_RIGHT].x))
            .min(height/(radii[corner::TOP_LEFT].y + radii[corner::BOTTOM_LEFT].y))
            .min(height/(radii[corner::TOP_RIGHT].y + radii[corner::BOTTOM_RIGHT].y));

        if f < 1.0 {
            for i in 0..4 {
                radii[i] *= f;
            }
        }

        radii
    } else {
        [Vec2::zero(); 4]
    };

    let radii = BorderRadius {
        top_left: to_layout_size(radii[corner::TOP_LEFT]),
        top_right: to_layout_size(radii[corner::TOP_RIGHT]),
        bottom_left: to_layout_size(radii[corner::BOTTOM_LEFT]),
        bottom_right: to_layout_size(radii[corner::BOTTOM_RIGHT]),
    };

    let clip_region = ComplexClipRegion::new(LayoutRect::new(
        to_layout_point(place.top_left()),
        to_layout_size(place.size()),
    ), radii);
    builder.push_clip_node(None, layout_rect, layout_rect, std::iter::once(clip_region), None);

    for background in &style.background_layers {
        match background.image {
            BackgroundImage::None => {},
            BackgroundImage::LinearGradient(ref gradient) => {
                let (dir, start, mut end) = match gradient.direction {
                    AngleOrCorner::Angle(angle) => {
                        let dir = Vec2::from_angle((angle as f64 - 90.0).to_radians());

                        let (start, end) = match (dir.x > 0.0, dir.y > 0.0) {
                            (false, false) => (place.bottom_right(), place.top_left()),
                            (false, true)  => (place.top_right(), place.bottom_left()),
                            (true, false)  => (place.bottom_left(), place.top_right()),
                            (true, true)   => (place.top_left(), place.bottom_right()),
                        };

                        (dir, start, end)
                    },
                    AngleOrCorner::TopLeft => {
                        let dir = Vec2::new(-height, -width).normalize();
                        (dir, place.bottom_right(), place.top_left())
                    },
                    AngleOrCorner::TopRight => {
                        let dir = Vec2::new(height, -width).normalize();
                        (dir, place.bottom_left(), place.top_right())
                    },
                    AngleOrCorner::BottomLeft =>  {
                        let dir = Vec2::new(-height, width).normalize();
                        (dir, place.top_right(), place.bottom_left())
                    },
                    AngleOrCorner::BottomRight => {
                        let dir = Vec2::new(height, width).normalize();
                        (dir, place.top_left(), place.bottom_right())
                    },
                };

                end = start + dir*Vec2::dot(end - start, dir);

                let stops = convert_stops(&gradient.stops, (end - start).norm());

                let extend_mode = if gradient.repeating {
                    ExtendMode::Repeat
                } else {
                    ExtendMode::Clamp
                };

                let gradient = builder.create_gradient(to_layout_point(start - place.top_left()), to_layout_point(end - place.top_left()), stops, extend_mode);
                builder.push_gradient(
                    rect,
                    rect,
                    gradient,
                    rect.size,
                    LayoutSize::zero(),
                );
            }
            BackgroundImage::RadialGradient(ref gradient) => {
                let center = position(gradient.position, place);

                let radius = match gradient.shape {
                    // FIXME: do the actual calculation here.
                    RadialGradientShape::Circle(_extent) => Vec2::new(50.0, 50.0),
                    // FIXME: do the actual calculation here.
                    RadialGradientShape::Ellipse(_extent) => Vec2::new(50.0, 50.0),
                    RadialGradientShape::Ellipse2(radius) => length_or_percentage_vec(radius, size),
                };

                let stops = convert_stops(&gradient.stops, radius.y);

                let extend_mode = if gradient.repeating {
                    ExtendMode::Repeat
                } else {
                    ExtendMode::Clamp
                };

                let gradient = builder.create_radial_gradient(to_layout_point(center - place.top_left()), to_layout_size(radius), stops, extend_mode);
                builder.push_radial_gradient(
                    rect,
                    rect,
                    gradient,
                    rect.size,
                    LayoutSize::zero(),
                );
            }
            BackgroundImage::Image(ref _image) => {
            }
        }
    }

    if style.background_color.a != 0.0 {
        builder.push_rect(
            rect,
            rect,
            convert_color(style.background_color),
        );
    }

    builder.pop_clip_node();

    if style.border.iter().any(|border| border.style != BorderStyle::None) {
        let widths = BorderWidths {
            left: style.border[border::LEFT].width,
            right: style.border[border::RIGHT].width,
            top: style.border[border::TOP].width,
            bottom: style.border[border::BOTTOM].width,
        };

        let details = BorderDetails::Normal(NormalBorder {
            left: convert_border(style.border[border::LEFT]),
            right: convert_border(style.border[border::RIGHT]),
            top: convert_border(style.border[border::TOP]),
            bottom: convert_border(style.border[border::BOTTOM]),
            radius: radii,
        });

        builder.push_border(rect, rect, widths, details);
    }
}

fn position(position: Vec2<PositionCoordinate>, place: Rect<f64>) -> Vec2<f64> {
    Vec2::new(position_coordinate(position.x, place.left, place.right), position_coordinate(position.y, place.top, place.bottom))
}

fn position_coordinate(position: PositionCoordinate, start: f64, end: f64) -> f64 {
    match position {
        PositionCoordinate::Length(len) => start + len as f64,
        PositionCoordinate::LengthOpposite(len) => end - len as f64,
        PositionCoordinate::Percentage(percentage) => start + percentage as f64*(end - start),
    }
}

fn length_or_percentage(value: LengthOrPercentage, max: f64) -> f64 {
    match value {
        LengthOrPercentage::Length(len) => len as f64,
        LengthOrPercentage::Percentage(p) => (p as f64)*max,
    }
}

fn length_or_percentage_vec(radius: Vec2<LengthOrPercentage>, size: Vec2<f64>) -> Vec2<f64> {
    let x = length_or_percentage(radius.x, size.x);
    let y = length_or_percentage(radius.y, size.y);
    Vec2::new(x, y)
}

fn convert_border(border: Border) -> webrender_api::BorderSide {
    webrender_api::BorderSide {
        color: convert_color(border.color),
        style: match border.style {
            BorderStyle::None => webrender_api::BorderStyle::None,
            BorderStyle::Dotted => webrender_api::BorderStyle::Dotted,
            BorderStyle::Dashed => webrender_api::BorderStyle::Dashed,
            BorderStyle::Solid => webrender_api::BorderStyle::Solid,
            BorderStyle::Double => webrender_api::BorderStyle::Double,
            BorderStyle::Groove => webrender_api::BorderStyle::Groove,
            BorderStyle::Ridge => webrender_api::BorderStyle::Ridge,
            BorderStyle::Inset => webrender_api::BorderStyle::Inset,
            BorderStyle::Outset => webrender_api::BorderStyle::Outset,
        },
    }
}

fn convert_stops(stops: &[ColorStop], gradient_length: f64) -> Vec<GradientStop> {
    let mut stops: Vec<_> = stops.iter().map(|stop| GradientStop {
        offset: match stop.position {
            None => std::f32::NAN,
            Some(LengthOrPercentage::Percentage(p)) => p,
            Some(LengthOrPercentage::Length(length)) => length/(gradient_length as f32),
        },
        color: convert_color(stop.color),
    }).collect();

    if stops[0].offset.is_nan() {
        stops[0].offset = 0.0;
    }

    if stops.last().unwrap().offset.is_nan() {
        stops.last_mut().unwrap().offset = 1.0;
    }

    let mut i = 1;
    while i < stops.len() - 1 {
        if stops[i].offset.is_nan() {
            let mut j = i + 1;
            while stops[j].offset.is_nan() {
                j = j + 1;
            }

            let step = (stops[j].offset - stops[i - 1].offset)/(j - i + 1) as f32;
            for k in i..j {
                stops[k].offset = stops[i - 1].offset + step*(k - i) as f32;
            }

            i = j + 1;
        } else {
            i += 1;
        }
    }

    for i in 1..stops.len() {
        if stops[i].offset < stops[0].offset {
            stops[i].offset = stops[0].offset;
        }
    }

    stops
}