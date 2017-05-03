extern crate lithium_core;
extern crate lithium_glutin;

use lithium_core::gui::scene::Theme;

fn main() {
    let theme = Theme::empty();
    
    lithium_glutin::spawn_window("Lithium", theme, |window| {
        window.show(lithium_core::widgets::ClickArea::new())
    }).join().unwrap();
}
