use Vec2;
use super::{MouseButton, Key};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Event {
    MouseMoved(Vec2<f64>),
    MouseEntered,
    MouseLeft,
    MouseButton(MouseButton, bool),
    Touch(TouchEvent),
    Key(Key, bool),
    Char(char),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TouchEvent {
    pub phase: TouchPhase,
    pub position: Vec2<f64>,
    pub id: u64,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum TouchPhase {
    /// User has touched the screen.
    Started,

    /// Touch position has moved.
    Moved,

    /// User has ended touching the screen.
    Ended,

    /// Touch was cancelled by some other means, for example window has lost focus.
    Cancelled,
}
