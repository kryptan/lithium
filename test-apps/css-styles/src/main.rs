extern crate lithium_core;
extern crate lithium_glutin;

use lithium_core::{Id, Rect, Var, Gui, Theme, Widget};
use lithium_core::widgets::Dummy;
use lithium_core::theme::StyleVariant;

fn main() {
    let theme = Theme::from_css_str(include_str!("../style.css"));

    let mut style_variants: Vec<StyleVariant> = theme.element_styles.keys().map(|&(style_variant, _element_kind)| style_variant).collect();
    style_variants.sort_by_key(|style_variant| style_variant.0);
    style_variants.dedup();
    
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

        for &mut (style_variant, ref mut div) in &mut self.divs {
            gui.styled(style_variant, |gui| {
                div.appear(gui);
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
