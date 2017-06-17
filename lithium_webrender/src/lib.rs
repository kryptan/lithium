extern crate webrender_traits;
extern crate lithium_core;

use webrender_traits::{DisplayListBuilder, ColorF};
use webrender_traits::{LayoutPoint, LayoutSize, LayoutRect};
use lithium_core::gui::scene::Command;
use lithium_core::theme::ElementStyle;
use lithium_core::theme::element_style::{BackgroundPicture, LengthOrPercentage};
use lithium_core::{Color, Rect};

fn lithium_color_to_webrender(color: Color) -> ColorF {
    let (r, g, b, a) = color.to_sRGB();
    ColorF::new(r, g, b, a)
}

pub fn build(commands: &[Command], builder: &mut DisplayListBuilder) {
    for command in commands.iter().rev() {
        match command {
            &Command::CloseElement(ref element) => {
                build_element(element.place, &element.style, builder);
            }
            &Command::StartElement => {

            }
            &Command::Text(ref _text) => {}
            &Command::Mesh(ref _mesh) => {}
        }
    }
}

fn build_element(place: Rect<f64>, style: &ElementStyle, builder: &mut DisplayListBuilder) {
    use webrender_traits::{GradientStop, ExtendMode};
    use lithium_core::theme::element_style::AngleOrCorner;

    for background in &style.background_images {
        let rect = LayoutRect::new(
            LayoutPoint::new(place.left as f32, place.right as f32),
            LayoutSize::new(place.width() as f32, place.height() as f32),
        );

        let clip_region_token = builder.push_clip_region(
            &rect,
            std::iter::empty(),
            None
        );

        match background.image {
            BackgroundPicture::Color(color) => {
                builder.push_rect(
                    rect,
                    clip_region_token,
                    lithium_color_to_webrender(color),
                );
            },
            BackgroundPicture::LinearGradient(ref gradient) => {
                let width = rect.size.width as f64;
                let height = rect.size.height as f64;

                let _angle = match gradient.angle_or_corner {
                    AngleOrCorner::Angle(angle) => angle as f64,
                    AngleOrCorner::TopLeft => (-height).atan2(-width),
                    AngleOrCorner::TopRight => (-height).atan2(width),
                    AngleOrCorner::BottomLeft => height.atan2(-width),
                    AngleOrCorner::BottomRight => height.atan2(width),
                };
               
                let start = LayoutPoint::new(0.0, 0.0);
                let end = LayoutPoint::new(1.0, 1.0);
                
                let gradient_length = (end - start).length();

                let stops = gradient.stops.iter().map(|stop| GradientStop {
                    offset: match stop.length_or_percentage {
                        LengthOrPercentage::Percentage(p) => p,
                        LengthOrPercentage::Length(length) => length/gradient_length,
                    },
                    color: lithium_color_to_webrender(stop.color),
                }).collect();

                let extend_mode = if gradient.repeating {
                    ExtendMode::Repeat
                } else {
                    ExtendMode::Clamp
                };

                let gradient = builder.create_gradient(start, end, stops, extend_mode);
                builder.push_gradient(
                    rect,
                    clip_region_token,
                    gradient,
                    rect.size,
                    LayoutSize::zero(),
                );
            }
            BackgroundPicture::RadialGradient(ref _gradient) => {
            }
            BackgroundPicture::Image(ref _image) => {
            }
        }
    }
}
