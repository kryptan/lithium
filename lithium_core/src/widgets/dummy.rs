use {Id, Gui, Rect, Var};
use super::Widget;

pub struct Dummy {
    id: Id,
}

impl Widget for Dummy {
    fn appear(&mut self, gui: &mut Gui) -> Rect<Var> {
        let place = Rect::from(self.id);
    	gui.element(self.id, element_kind!("Dummy"), |_gui| {});
        place
    }
}

impl Dummy {
    pub fn new() -> Self {
        Dummy {
            id: Id::unique(),
        }
    }
}
