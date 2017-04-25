use Vec2;
use super::ButtonState;

#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct Mouse {
    pub position: Vec2<f64>,

    pub primary_button: ButtonState,
    pub middle_button: ButtonState,
    pub secondary_button: ButtonState,
    pub x1_button: ButtonState,
    pub x2_button: ButtonState,
}

impl Mouse {
    pub fn advance(&mut self) {
        self.primary_button.advance();
        self.middle_button.advance();
        self.secondary_button.advance();
        self.x1_button.advance();
        self.x2_button.advance();
    }

    pub fn button_mut(&mut self, button: MouseButton) -> &mut ButtonState {
        match button {
            MouseButton::Primary => &mut self.primary_button,
            MouseButton::Secondary => &mut self.middle_button,
            MouseButton::Middle => &mut self.secondary_button,
            MouseButton::X1 => &mut self.x1_button,
            MouseButton::X2 => &mut self.x2_button,
        }
    }

    #[inline]
    pub fn press(&mut self, button: MouseButton) {
        *self.button_mut(button) = ButtonState::JustPressed;
    }

    #[inline]
    pub fn release(&mut self, button: MouseButton) {
        *self.button_mut(button) = ButtonState::JustReleased;
    }

}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
#[repr(u8)]
pub enum MouseButton {
    /// Primary mouse button which is usually left.
    Primary,

    /// Secondary mouse button which is usually right.
    ///
    /// This button is most commonly used to open context menus.
    Secondary,

    /// Middle mouse button. On most mouses it is triggered by pressing the wheel.
    Middle,

    /// Button present on some mouses which causes back navigation in a browser.
    X1,

    /// Button present on some mouses which causes forward navigation in a browser.
    X2,
}
