use {Id, Rect, Gui};
use layout;
use super::Widget;
use super::ClickArea;

pub struct Button<T: Widget> {
    id: Id,
    click_area: ClickArea,

    pub label: T,
}

impl<T: Widget> Widget for Button<T> {
    fn id(&self) -> Id {
    	self.id
    }

    fn appear(&mut self, gui: &mut Gui) {
        let place = Rect::from(self.id);

    	layout::center(gui, place, Rect::from(self.label.id()));

    	gui.element(self.id, element_kind!("Button"), |gui| {
            self.click_area.appear(gui);
            self.label.appear(gui);
        });
    }
}

impl<T: Widget> Button<T> {
    pub fn clicked(&mut self) -> bool {
        self.click_area.clicked()
    }
}
