use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use num_traits::{Zero, Float};

/// 2D vector. Can be used to represent position, displacement, size, etc.
#[derive(Copy, Clone, Default, Eq, PartialEq, Debug)]
pub struct Vec2<T> {
    /// Horizontal coordinate where negative direction is left and positive is right.
    pub x: T,

    /// Vertical coordinate where negative direction is up and positive is down.
    pub y: T,
}

impl<T> Vec2<T> {
    /// Construct new vector.
    pub fn new(x: T, y: T) -> Self {
        Vec2 {
            x: x,
            y: y,
        }
    }

    pub fn into_array(self) -> [T; 2] {
        [self.x, self.y]
    }
}

impl<T: Zero> Vec2<T> {
    pub fn zero() -> Self {
        Vec2 {
            x: T::zero(),
            y: T::zero(),
        }
    }
}

impl<T> Vec2<T>
    where T: Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Copy
{
    /// Dot product of two vectors.
    ///
    /// <code>Vec2::dot(**a**, **b**)</code> = <strong>a</strong><sub>x</sub>⋅<strong>b</strong><sub>x</sub> + <strong>a</strong><sub>y</sub>⋅<strong>b</strong><sub>y</sub> = |<strong>a</strong>|⋅|<strong>b</strong>|⋅cos(θ)
    ///
    /// where θ is the angle between **a** and **b**.
    pub fn dot(a: Vec2<T>, b: Vec2<T>) -> T {
        a.x*b.x + a.y*b.y
    }

    /// 2D cross product (also known as perp dot product) of two vectors.
    ///
    /// <code>Vec2::cross(**a**, **b**)</code> = <strong>a</strong><sub>x</sub>⋅<strong>b</strong><sub>y</sub> - <strong>a</strong><sub>y</sub>⋅<strong>b</strong><sub>x</sub> = |<strong>a</strong>|⋅|<strong>b</strong>|⋅sin(θ)
    ///
    /// where θ is the angle between **a** and **b**.
    pub fn cross(a: Vec2<T>, b: Vec2<T>) -> T {
        a.x*b.y - a.y*b.x
    }

    pub fn norm_squared(self) -> T {
        self.x*self.x + self.y*self.y
    }
}

impl<T: Float> Vec2<T> {
    pub fn norm(self) -> T {
        self.norm_squared().sqrt()
    }

    pub fn from_angle(a: T) -> Self {
        Vec2::new(a.cos(), a.sin())
    }

    pub fn normalize(self) -> Self {
        self/self.norm()
    }
}

macro_rules! impl_from {
    ($from:ty, $to:ty) => {
        impl From<Vec2<$from>> for Vec2<$to> {
            #[inline]
            fn from(from: Vec2<$from>) -> Vec2<$to> {
                Vec2::new(from.x as $to, from.y as $to)
            }
        }
    }
}

impl_from!(f32, f64);
impl_from!(f64, f32);
impl_from!(u32, f64);
impl_from!(i32, f64);

impl<T> From<(T, T)> for Vec2<T> {
    fn from(from: (T, T)) -> Vec2<T> {
        Vec2::new(from.0, from.1)
    }
}

impl<T: Copy> From<[T; 2]> for Vec2<T> {
    fn from(from: [T; 2]) -> Vec2<T> {
        Vec2::new(from[0], from[1])
    }
}

macro_rules! impl_bin_op {
    ($op:ident, $method:ident, $op_assign:ident, $method_assign:ident) => {
        impl<A, B, R> $op<Vec2<B>> for Vec2<A>
            where A: $op<B, Output=R>
        {
            type Output = Vec2<R>;

            fn $method(self, other: Vec2<B>) -> Vec2<R> {
                Vec2::new(self.x.$method(other.x), self.y.$method(other.y))
            }
        }

        impl<A, B> $op_assign<Vec2<B>> for Vec2<A>
            where A: $op_assign<B>
        {
            fn $method_assign(&mut self, other: Vec2<B>) {
                self.x.$method_assign(other.x);
                self.y.$method_assign(other.y);
            }
        }
    }
}

impl_bin_op!(Add, add, AddAssign, add_assign);
impl_bin_op!(Sub, sub, SubAssign, sub_assign);

macro_rules! impl_scalar_op {
    ($op:ident, $method:ident, $op_assign:ident, $method_assign:ident) => {
        impl<A, B, R> $op<B> for Vec2<A>
            where
                A: $op<B, Output=R>,
                B: Copy,
        {
            type Output = Vec2<R>;

            fn $method(self, other: B) -> Vec2<R> {
                Vec2::new(self.x.$method(other), self.y.$method(other))
            }
        }

        impl<A, B> $op_assign<B> for Vec2<A>
            where
                A: $op_assign<B>,
                B: Copy,
        {
            fn $method_assign(&mut self, other: B) {
                self.x.$method_assign(other);
                self.y.$method_assign(other);
            }
        }
    }
}

impl_scalar_op!(Mul, mul, MulAssign, mul_assign);
impl_scalar_op!(Div, div, DivAssign, div_assign);

impl<T> Mul<Vec2<T>> for f64
   where f64: Mul<T>
{
    type Output = Vec2<<f64 as Mul<T>>::Output>;

    fn mul(self, other: Vec2<T>) -> Self::Output {
        Vec2::new(self*other.x, self*other.y)
    }
}
