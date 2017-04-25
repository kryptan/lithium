use {Id, Gui, Rect};
use gui::input::ButtonState;
use super::Widget;

pub struct ClickArea {
    id: Id,
    clicked: bool,
}

impl Widget for ClickArea {
    fn id(&self) -> Id {
    	self.id
    }

    fn appear(&mut self, gui: &mut Gui) {
        let place = gui.layout.prev_value_rect(Rect::from(self.id));

        if let Some(mouse) = gui.input.mouse_grabbed_by(self.id) {
            if mouse.primary_button.is_pressed() {
                gui.input.grab_mouse(self.id);
            } else if mouse.primary_button == ButtonState::JustReleased && place.contains(mouse.position) {
                self.clicked = true;
            }
        } else if let Some(mouse) = gui.input.get_mouse(|pos| place.contains(pos)) {
            if mouse.primary_button == ButtonState::JustPressed {
                gui.input.grab_mouse(self.id);
            }
        }
    }
}

impl ClickArea {
    pub fn clicked(&mut self) -> bool {
        let prev = self.clicked;
        self.clicked = false;
        prev
    }
}
