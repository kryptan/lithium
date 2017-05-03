use {Id, Gui, Rect, Var};
use layout;
use super::Widget;
use super::ClickArea;

pub struct Button<T: Widget> {
    id: Id,
    click_area: ClickArea,

    pub label: T,
}

impl<T: Widget> Widget for Button<T> {
    fn appear(&mut self, gui: &mut Gui) -> Rect<Var> {
    	gui.element(self.id, element_kind!("Button"), |gui| {
            let click_area_place = self.click_area.appear(gui);
            let label_place = self.label.appear(gui);
            
            layout::center(gui, click_area_place, label_place);

            click_area_place
        })
    }
}

impl<T: Widget> Button<T> {
    pub fn new(label: T) -> Self {
        Button {
            id: Id::unique(),
            click_area: ClickArea::new(),
            label
        }
    }

    pub fn clicked(&mut self) -> bool {
        self.click_area.clicked()
    }
}
