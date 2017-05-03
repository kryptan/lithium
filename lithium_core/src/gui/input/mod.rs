use {Id, Vec2};

mod keyboard;
mod mouse;
mod touch;
mod event;

pub use self::keyboard::{Keyboard, Key};
pub use self::mouse::{Mouse, MouseButton};
pub use self::touch::{Touch};
pub use self::event::{Event, TouchEvent, TouchPhase};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
pub enum ButtonState {
    JustPressed,
    Pressed,
    JustReleased,
    Released,
}

#[derive(Clone, Default, PartialEq, Debug)]
pub struct Input {
    mouse: Option<Mouse>,
    pub touches: Vec<Touch>,
    pub keyboard: Keyboard,

    mouse_grabber: Option<Id>,
    mouse_returned: bool,
    mouse_grabbed: bool,
}

impl ButtonState {
    fn advance(&mut self) {
        *self = match *self {
            ButtonState::JustPressed => ButtonState::Pressed,
            ButtonState::JustReleased => ButtonState::Released,
            ButtonState::Pressed => ButtonState::Pressed,
            ButtonState::Released => ButtonState::Released,
        }
    }

    pub fn is_pressed(self) -> bool {
        self == ButtonState::JustPressed || self == ButtonState::Pressed
    }
}

impl Default for ButtonState {
    fn default() -> Self {
        ButtonState::Released
    }
}

impl Input {
    pub fn mouse_grabbed_by(&mut self, id: Id) -> Option<Mouse> {
        if self.mouse_grabber == Some(id) {
            self.mouse_grabber = None;
            self.mouse
        } else {
            None
        }
    }

    pub fn grab_mouse(&mut self, id: Id) {
        if self.mouse.is_some() {
            self.mouse_grabber = Some(id);
            self.mouse_grabbed = true;
        }
    }

    pub fn get_mouse<F: Fn(Vec2<f64>) -> bool>(&mut self, f: F) -> Option<Mouse> {
        if self.mouse_returned {
            return None;
        }

        if let Some(mouse) = self.mouse {
            if f(mouse.position) {
                self.mouse_returned = true;
                return Some(mouse);
            }
        }

        return None;
    }

    pub fn advance(&mut self) {
        if let Some(ref mut mouse) = self.mouse {
            mouse.advance();
        }

        self.keyboard.advance();

        for touch in &mut self.touches {
            touch.advance();
        }

        self.touches.retain(|&touch| touch.state != ButtonState::Released);

        self.mouse_returned = false;

        if !self.mouse_grabbed {
            self.mouse_grabber = None;
        }
        self.mouse_grabbed = false;
    }

    pub fn event(&mut self, event: &Event) {
        match event {
            &Event::MouseMoved(position) => {
                if self.mouse.is_none() {
                    self.mouse = Some(Mouse::default());
                }

                if let Some(ref mut mouse) = self.mouse {
                    mouse.position = position;
                }
            },
            &Event::MouseEntered => {
                self.mouse = Some(Mouse::default());
            },
            &Event::MouseLeft => {
                self.mouse = None;
            },
            &Event::MouseButton(button, pressed) => {
                if let Some(ref mut mouse) = self.mouse {
                    if pressed {
                        mouse.press(button);
                    } else {
                        mouse.release(button);
                    }
                }
            }
            &Event::Touch(touch_event) => {
                match touch_event.phase {
                    event::TouchPhase::Started => {
                        if self.touches.iter().all(|&touch| touch.id != touch_event.id) {
                            self.touches.push(Touch {
                                id: touch_event.id,
                                position: touch_event.position,
                                state: ButtonState::JustPressed,
                            });
                        }
                    }
                    event::TouchPhase::Moved => {
                        for touch in &mut self.touches {
                            if touch.id == touch_event.id {
                                touch.position = touch_event.position;
                            }
                        }
                    }
                    event::TouchPhase::Ended => {
                        for touch in &mut self.touches {
                            if touch.id == touch_event.id {
                                touch.state = ButtonState::JustReleased;
                            }
                        }
                    }
                    event::TouchPhase::Cancelled => {
                        self.touches.retain(|&touch| touch.id != touch_event.id)
                    }
                }
            },
            &Event::Key(key, pressed) => {
                if pressed {
                    self.keyboard.press(key);
                } else {
                    self.keyboard.release(key);
                }
            }
            &Event::Char(char) => {
                self.keyboard.enter_char(char);
            },
        }
    }
}