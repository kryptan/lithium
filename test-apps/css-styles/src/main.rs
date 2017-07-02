#[macro_use]
extern crate lithium_core;
extern crate lithium_glutin;
extern crate regex;

use lithium_core::{Id, Rect, Var, Gui, Theme, Widget};
use lithium_core::widgets::Dummy;
use lithium_core::theme;
use lithium_core::theme::StyleVariant;
use regex::Regex;

fn main() {
    let css = include_str!("../style.css");
    let theme = Theme::from_css_str(css);

    let re = Regex::new(r"\.([a-z0-9-]+) \{").unwrap();
    let style_variants: Vec<StyleVariant> = re.captures_iter(css).map(|capture| theme::style_variant(&capture[1])).collect();

    lithium_glutin::spawn_window("Lithium CSS Styles", theme, |window| {
        window.show(Window::new(style_variants))
    }).join().unwrap();
}

struct Window {
    id: Id,
    divs: Vec<(StyleVariant, Dummy)>,
}

impl Widget for Window {
    fn appear(&mut self, gui: &mut Gui) -> Rect<Var> {
        let place = Rect::from(self.id);

        let mut left = place.left;

        for &mut (style_variant, ref mut div) in &mut self.divs {
            gui.styled(style_variant, |gui| {
                let inner_place = div.appear(gui);
                add_constraints!(gui.layout, [
                    (inner_place.top) == 20.0,
                    (inner_place.left) == left + 20.0,
                    (inner_place.size().x) == 160.0,
                    (inner_place.size().y) == 90.0,
                ]);

                left = inner_place.right;
            });
        }

        place
    }
}

impl Window {
    pub fn new(variants: Vec<StyleVariant>) -> Self {
        let divs = variants.iter().map(|&style_variant| (style_variant, Dummy::new())).collect();

        Window {
            id: Id::unique(),
            divs: divs,
        }
    }
}
