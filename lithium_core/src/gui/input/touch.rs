use Vec2;
use super::ButtonState;

#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct Touch {
    pub id: u64,
    pub position: Vec2<f64>,
    pub state: ButtonState,
}

impl Touch {
    pub fn advance(&mut self) {
        self.state.advance();
    }
}
