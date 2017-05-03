extern crate webrender_traits;
extern crate lithium_core;

use webrender_traits::{DisplayListBuilder, ColorF};
use webrender_traits::{LayoutPoint, LayoutSize, LayoutRect};
use lithium_core::gui::scene::Command;
use lithium_core::Color;

fn lithium_color_to_webrender(color: Color) -> ColorF {
    let (r, g, b, a) = color.to_sRGB();
    ColorF::new(r, g, b, a)
}

pub fn build(commands: &[Command], builder: &mut DisplayListBuilder) {
    for command in commands.iter().rev() {
        match command {
            &Command::CloseElement(ref element) => {
                let rect = LayoutRect::new(
                    LayoutPoint::new(element.place.left as f32, element.place.right as f32),
                    LayoutSize::new(element.place.width() as f32, element.place.height() as f32),
                );

                let clip_region_token = builder.push_clip_region(
                    &rect,
                    std::iter::empty(),
                    None
                );

                builder.push_rect(
                    rect,
                    clip_region_token,
                    lithium_color_to_webrender(element.style.background)
                );
            }
            &Command::StartElement => {

            }
            &Command::Text(ref _text) => {}
            &Command::Mesh(ref _mesh) => {}
        }
    }
}