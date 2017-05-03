use {Gui, Rect, Var};

pub mod button;
pub mod click_area;

pub use self::click_area::ClickArea;

pub trait Widget {
    fn appear(&mut self, gui: &mut Gui) -> Rect<Var>;
}
