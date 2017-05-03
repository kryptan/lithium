use {Id, Gui, Rect, Var};
use gui::input::ButtonState;
use super::Widget;

pub struct ClickArea {
    id: Id,
    clicked: bool,
}

impl Widget for ClickArea {
    fn appear(&mut self, gui: &mut Gui) -> Rect<Var> {
        let place = Rect::from(self.id);
        let place_value = gui.layout.prev_value_rect(place);

        if let Some(mouse) = gui.input.mouse_grabbed_by(self.id) {
            if mouse.primary_button.is_pressed() {
                gui.input.grab_mouse(self.id);
            } else if mouse.primary_button == ButtonState::JustReleased && place_value.contains(mouse.position) {
                self.clicked = true;
            }
        } else if let Some(mouse) = gui.input.get_mouse(|pos| place_value.contains(pos)) {
            if mouse.primary_button == ButtonState::JustPressed {
                gui.input.grab_mouse(self.id);
            }
        }

        place
    }
}

impl ClickArea {
    pub fn new() -> Self {
        ClickArea {
            id: Id::unique(),
            clicked: false,
        }
    }

    pub fn clicked(&mut self) -> bool {
        let prev = self.clicked;
        self.clicked = false;
        prev
    }
}
