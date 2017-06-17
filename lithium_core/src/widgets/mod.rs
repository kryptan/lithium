use {Gui, Rect, Var};

pub mod button;
pub mod click_area;
pub mod dummy;

pub use self::click_area::ClickArea;
pub use self::dummy::Dummy;

pub trait Widget {
    fn appear(&mut self, gui: &mut Gui) -> Rect<Var>;
}
