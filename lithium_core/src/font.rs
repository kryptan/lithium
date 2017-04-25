use std::fmt::Debug;
use Vec2;

pub struct Segment {
    pub byte_start: usize,
    pub byte_end: usize,

    pub position_start: f64,
    pub position_end: f64,

    pub glyph_index: u32,
    pub position: Vec2<f64>,
}

pub trait Font: Debug {
    fn shape(&self, text: &str, out: &mut Vec<Segment>);
}

#[derive(Debug)]
pub struct ErrorFont;

impl Font for ErrorFont {
    fn shape(&self, _text: &str, _out: &mut Vec<Segment>) {}
}

/*
pub trait FontRender {
    type Glyph: Glyph;

    fn get_glyph(&self, glyph_index: u32, position: Vec2<f64>, size: Vec2<f64>) -> Self::Glyph;
}

pub trait Glyph {
    fn bounding_box() -> Rect<i32>;

    fn render<O: FnMut(u32, u32, f64)>(&self, o: O);
}
*/