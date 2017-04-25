use std::ops::{Add, Sub, Mul, Div};

mod vec2;
mod rect;

pub use self::vec2::Vec2;
pub use self::rect::Rect;

pub fn rescale_to_01<T>(value: T, from: (T, T)) -> T
    where T: Copy + Sub<T, Output=T> + Div<T, Output=T>
{
    (value - from.0)/(from.1 - from.0)
}

pub fn rescale_from_01<T>(value: T, to: (T, T)) -> T
    where T: Copy + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T>
{
    to.0 + value*(to.1 - to.0)
}

pub fn rescale<T>(value: T, from: (T, T), to: (T, T)) -> T
    where T: Copy + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T>
{
    rescale_from_01(rescale_to_01(value, from), to)
}

pub fn rescale_vec_to_01<T>(value: Vec2<T>, from: Rect<T>) -> Vec2<T>
    where T: Copy + Sub<T, Output=T> + Div<T, Output=T>
{
    Vec2::new(rescale_to_01(value.x, (from.left, from.right)), rescale_to_01(value.y, (from.top, from.bottom)))
}

pub fn rescale_vec_from_01<T>(value: Vec2<T>, to: Rect<T>) -> Vec2<T>
    where T: Copy + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T>
{
    Vec2::new(rescale_from_01(value.x, (to.left, to.right)), rescale_from_01(value.y, (to.top, to.bottom)))
}

pub fn rescale_vec<T>(value: Vec2<T>, from: Rect<T>, to: Rect<T>) -> Vec2<T>
    where T: Copy + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T>
{
    rescale_vec_from_01(rescale_vec_to_01(value, from), to)
}
