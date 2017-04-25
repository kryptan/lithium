use std::ops::{Add, Sub, Mul};

use super::Vec2;

/// Axis-aligned rectangle which is usually used to represent some region of the screen.
///
/// It is most commonly instantiated as `Rect<f64>` or `Rect<Var>`.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Rect<T> {
    pub left: T,
    pub right: T,
    pub top: T,
    pub bottom: T,
}

impl<T> Rect<T> {
    /// Construct new rectangle from the top-left and bottom-right corners.
    pub fn from_corners(top_left: Vec2<T>, bottom_right: Vec2<T>) -> Self {
        Rect {
            left: top_left.x,
            top: top_left.y,
            right: bottom_right.x,
            bottom: bottom_right.y,
        }
    }

    /// Get vertex `i`.
    ///
    /// Vertices are numbered in clockwise direction starting from the top left.
    ///
    /// <div style="position: relative; width: 6em; height: 6em;">
    ///     <div style="border: 2px solid black; width: 4em; height: 4em; position: absolute; top: 1em; left: 1em;"> </div>
    ///     <div style="position: absolute; top:    0; left:  0;"> 0 </div>
    ///     <div style="position: absolute; top:    0; right: 0;"> 1 </div>
    ///     <div style="position: absolute; bottom: 0; right: 0;"> 2 </div>
    ///     <div style="position: absolute; bottom: 0; left:  0;"> 3 </div>
    /// </div>
    pub fn vertex(self, i: u8) -> Vec2<T> {
        match i % 4 {
            0 => Vec2::new(self.left, self.top),
            1 => Vec2::new(self.right, self.top),
            2 => Vec2::new(self.right, self.bottom),
            3 => Vec2::new(self.left, self.bottom),
            _ => unreachable!(),
        }
    }

    pub fn top_left(self) -> Vec2<T> {
        Vec2::new(self.left, self.top)
    }

    pub fn top_right(self) -> Vec2<T> {
        Vec2::new(self.right, self.top)
    }

    pub fn bottom_left(self) -> Vec2<T> {
        Vec2::new(self.left, self.bottom)
    }

    pub fn bottom_right(self) -> Vec2<T> {
        Vec2::new(self.right, self.bottom)
    }
}

macro_rules! impl_from {
    ($from:ty, $to:ty) => {
        impl From<Rect<$from>> for Rect<$to> {
            #[inline]
            fn from(from: Rect<$from>) -> Rect<$to> {
                Rect {
                    left: from.left as $to,
                    right: from.right as $to,
                    top: from.top as $to,
                    bottom: from.bottom as $to,
                }
            }
        }
    }
}

impl_from!(f32, f64);
impl_from!(f64, f32);
impl_from!(u32, f64);
impl_from!(i32, f64);

impl<T> Rect<T>
    where T: Add<T, Output=T> + Sub<T, Output=T> + Copy
{
    pub fn from_top_left_and_size(top_left: Vec2<T>, size: Vec2<T>) -> Self {
        Self::from_corners(top_left, top_left + size)
    }

    pub fn from_center_and_half_size(center: Vec2<T>, half_size: Vec2<T>) -> Self {
        Self::from_corners(center - half_size, center + half_size)
    }
}

impl<T, R> Rect<T>
    where T: Sub<T, Output=R>
{
    pub fn size(self) -> Vec2<R> {
        Vec2::new(self.right, self.bottom) - Vec2::new(self.left, self.top)
    }

    pub fn width(self) -> R {
        self.right - self.left
    }

    pub fn height(self) -> R {
        self.bottom - self.top
    }
}

impl<T, U, R> Rect<T>
  where
    U: Mul<f64, Output=R>,
    T: Add<T, Output=U>
{
    pub fn center(self) -> Vec2<R> {
        (Vec2::new(self.left, self.top) + Vec2::new(self.right, self.bottom))*0.5
    }
}

impl<T: PartialOrd> Rect<T>
{
    pub fn contains(&self, p: Vec2<T>) -> bool {
        self.left <= p.x && p.x < self.right && self.top <= p.y && p.y < self.bottom
    }
}

impl<A, B, R> Add<Vec2<B>> for Rect<A>
  where
    A: Add<B, Output=R>,
    B: Copy
{
    type Output = Rect<R>;

    fn add(self, vec: Vec2<B>) -> Rect<R> {
        Rect {
            left: self.left + vec.x,
            right: self.right + vec.x,
            top: self.top + vec.y,
            bottom: self.bottom + vec.y,
        }
    }
}

impl<A, B, R> Sub<Vec2<B>> for Rect<A>
  where
    A: Sub<B, Output=R>,
    B: Copy
{
    type Output = Rect<R>;

    fn sub(self, vec: Vec2<B>) -> Rect<R> {
        Rect {
            left: self.left - vec.x,
            right: self.right - vec.x,
            top: self.top - vec.y,
            bottom: self.bottom - vec.y,
        }
    }
}
