use {Id, Gui};

pub mod button;
pub mod click_area;

pub use self::click_area::ClickArea;

pub trait Widget {
	// FIXME: replace with at associated constant when they stabilize.
    fn id(&self) -> Id;

    fn appear(&mut self, gui: &mut Gui);
}
