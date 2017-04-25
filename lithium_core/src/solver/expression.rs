use std::ops::{Add, Sub, Neg, Mul, Div};
use std::iter;
use super::Var;

/// Expression which can be used as part of the constraint.
///
/// Any expression has the form: *k<sub>0</sub>v<sub>0</sub> + k<sub>1</sub>v<sub>1</sub> + ... k<sub>n</sub>v<sub>n</sub> + c*,
///
/// where:
///
/// *v<sub>i</sub>* are variables,
///
/// *k<sub>i</sub>* are coefficients,
///
/// *c* is constant.
pub trait Expression: 'static {
    type Iter: Iterator<Item=Term>;

    /// Get constant *c*.
    fn constant(&self) -> f64;

    /// Get terms *k<sub>i</sub>v<sub>i</sub>*.
    fn terms(&self) -> Self::Iter;
}

/// Single term in a larger expression.
///
/// Expression represented by this struct is `coefficient*variable`.
#[derive(Copy, Clone, Debug)]
pub struct Term {
    pub variable: Var,
    pub coefficient: f64,
}

/// This type is used internally when constructing constraints using the `constraint!` macro.
///
/// There is no need to use this type directly.
///
/// Expression represented by this struct is `term + rest`.
#[derive(Copy, Clone, Debug)]
pub struct Sum<T> {
    pub term: Term,
    pub rest: T,
}

//#############
// Expression #
//#############

impl Expression for f64 {
    type Iter = iter::Empty<Term>;

    fn constant(&self) -> f64 {
        *self
    }

    fn terms(&self) -> Self::Iter {
        iter::empty()
    }
}

impl Expression for Var {
    type Iter = iter::Once<Term>;

    fn constant(&self) -> f64 {
        0.0
    }

    fn terms(&self) -> Self::Iter {
        iter::once(Term {
            coefficient: 1.0,
            variable: *self,
        })
    }
}

impl<T> Expression for Sum<T>
    where T: Expression
{
    type Iter = iter::Chain<iter::Once<Term>, T::Iter>;

    fn constant(&self) -> f64 {
        self.rest.constant()
    }

    fn terms(&self) -> Self::Iter {
        iter::once(self.term).chain(self.rest.terms())
    }
}

/*         Neg
┌────────┬─────────────────┐
│    f64 │       std       │
│    Var │ Neg for Var     │
│ Sum<T> │ Neg for Sum<T>  │
└────────┴─────────────────┘ */

impl Neg for Var {
    type Output = Sum<f64>;

    fn neg(self) -> Self::Output {
        Sum {
            term: Term {
                coefficient: -1.0,
                variable: self,
            },
            rest: 0.0,
        }
    }
}

impl<T> Neg for Sum<T>
    where T: Neg
{
    type Output = Sum<T::Output>;

    fn neg(self) -> Self::Output {
        Sum {
            term: Term {
                coefficient: -self.term.coefficient,
                variable: self.term.variable,
            },
            rest: -self.rest,
        }
    }
}

/*                                   Add
┌────────┬─────────────────────┬─────────────────────┬────────────────────────┐
│        │                 f64 │                 Var │                 Sum<T> │
├────────┼─────────────────────┼─────────────────────┼────────────────────────┤
│    f64 │         std         │ Add<f64>    for Var │ Add<f64>    for Sum<T> │
│    Var │ Add<Var>    for f64 │ Add<Var>    for Var │ Add<Var>    for Sum<T> │
│ Sum<T> │ Add<Sum<T>> for f64 │ Add<Sum<T>> for Var │ Add<Sum<U>> for Sum<T> │
└────────┴─────────────────────┴─────────────────────┴────────────────────────┘
*/

impl Add<f64> for Var {
    type Output = Sum<f64>;

    fn add(self, other: f64) -> Self::Output {
        Sum {
            term: Term {
                coefficient: 1.0,
                variable: self,
            },
            rest: other,
        }
    }
}

impl<T> Add<f64> for Sum<T>
    where T: Add<f64>
{
    type Output = Sum<T::Output>;

    fn add(self, other: f64) -> Self::Output {
        Sum {
            term: self.term,
            rest: self.rest + other,
        }
    }
}

impl Add<Var> for f64 {
    type Output = Sum<f64>;

    fn add(self, other: Var) -> Self::Output {
        other + self
    }
}

impl Add<Var> for Var {
    type Output = Sum<Sum<f64>>;

    fn add(self, other: Var) -> Self::Output {
        Sum {
            term: Term {
                coefficient: 1.0,
                variable: self,
            },
            rest: Sum {
                term: Term {
                    coefficient: 1.0,
                    variable: other,
                },
                rest: 0.0,
            },
        }
    }
}

impl<T> Add<Var> for Sum<T> {
    type Output = Sum<Sum<T>>;

    fn add(self, other: Var) -> Self::Output {
        Sum {
            term: Term {
                coefficient: 1.0,
                variable: other,
            },
            rest: self,
        }
    }
}

impl<T> Add<Sum<T>> for f64
    where T: Add<f64>
{
    type Output = Sum<T::Output>;

    fn add(self, other: Sum<T>) -> Self::Output {
        other + self
    }
}

impl<T> Add<Sum<T>> for Var {
    type Output = Sum<Sum<T>>;

    fn add(self, other: Sum<T>) -> Self::Output {
        other + self
    }
}

impl<T, U> Add<Sum<U>> for Sum<T>
    where T: Add<U>
{
    type Output = Sum<Sum<T::Output>>;

    fn add(self, other: Sum<U>) -> Self::Output {
        Sum {
            term: self.term,
            rest: Sum {
                term: other.term,
                rest: self.rest + other.rest,
            }
        }
    }
}

/*                                   Sub
┌────────┬─────────────────────┬─────────────────────┬────────────────────────┐
│        │                 f64 │                 Var │                 Sum<T> │
├────────┼─────────────────────┼─────────────────────┼────────────────────────┤
│    f64 │         std         │ Sub<f64>    for Var │ Sub<f64>    for Sum<T> │
│    Var │ Sub<Var>    for f64 │ Sub<Var>    for Var │ Sub<Var>    for Sum<T> │
│ Sum<T> │ Sub<Sum<T>> for f64 │ Sub<Sum<T>> for Var │ Sub<Sum<U>> for Sum<T> │
└────────┴─────────────────────┴─────────────────────┴────────────────────────┘ */

impl Sub<f64> for Var {
    type Output = Sum<f64>;

    fn sub(self, other: f64) -> Self::Output {
        Sum {
            term: Term {
                coefficient: 1.0,
                variable: self,
            },
            rest: -other,
        }
    }
}

impl<T> Sub<f64> for Sum<T>
    where T: Sub<f64>
{
    type Output = Sum<T::Output>;

    fn sub(self, other: f64) -> Self::Output {
        Sum {
            term: self.term,
            rest: self.rest - other,
        }
    }
}

impl Sub<Var> for f64 {
    type Output = Sum<f64>;

    fn sub(self, other: Var) -> Self::Output {
        Sum {
            term: Term {
                coefficient: -1.0,
                variable: other,
            },
            rest: self,
        }
    }
}

impl Sub<Var> for Var {
    type Output = Sum<Sum<f64>>;

    fn sub(self, other: Var) -> Self::Output {
        Sum {
            term: Term {
                coefficient: 1.0,
                variable: self,
            },
            rest: Sum {
                term: Term {
                    coefficient: -1.0,
                    variable: other,
                },
                rest: 0.0,
            },
        }
    }
}

impl<T> Sub<Var> for Sum<T> {
    type Output = Sum<Sum<T>>;

    fn sub(self, other: Var) -> Self::Output {
        Sum {
            term: Term {
                coefficient: -1.0,
                variable: other,
            },
            rest: self,
        }
    }
}

impl<T> Sub<Sum<T>> for f64
    where
        T: Sub<f64>,
        Sum<T::Output>: Neg
{
    type Output = <Sum<T::Output> as Neg>::Output;

    fn sub(self, other: Sum<T>) -> Self::Output {
        -(other - self)
    }
}

impl<T> Sub<Sum<T>> for Var
    where T: Neg
{
    type Output = Sum<Sum<T::Output>>;

    fn sub(self, other: Sum<T>) -> Self::Output {
        -other + self
    }
}

impl<T, U> Sub<Sum<T>> for Sum<U>
    where U: Sub<T>
{
    type Output = Sum<Sum<U::Output>>;

    fn sub(self, other: Sum<T>) -> Self::Output {
        Sum {
            term: self.term,
            rest: Sum {
                term: Term {
                    coefficient: -other.term.coefficient,
                    variable: other.term.variable,
                },
                rest: self.rest - other.rest,
            }
        }
    }
}

/*                               Mul
┌────────┬─────────────────────┬──────────────────┬─────────────────────┐
│        │                 f64 │              Var │              Sum<T> │
├────────┼─────────────────────┼──────────────────┼─────────────────────┤
│    f64 │         std         │ Mul<f64> for Var │ Mul<f64> for Sum<T> │
│    Var │ Mul<Var>    for f64 │        -         │          -          │
│ Sum<T> │ Mul<Sum<T>> for f64 │        -         │          -          │
└────────┴─────────────────────┴──────────────────┴─────────────────────┘ */

impl Mul<f64> for Var {
    type Output = Sum<f64>;

    fn mul(self, other: f64) -> Self::Output {
        Sum {
            term: Term {
                coefficient: other,
                variable: self,
            },
            rest: 0.0,
        }
    }
}

impl<T> Mul<f64> for Sum<T>
    where T: Mul<f64>
{
    type Output = Sum<T::Output>;

    fn mul(self, other: f64) -> Self::Output {
        Sum {
            term: Term {
                coefficient: other * self.term.coefficient,
                variable: self.term.variable,
            },
            rest: self.rest*other,
        }
    }
}

impl Mul<Var> for f64 {
    type Output = Sum<f64>;

    fn mul(self, other: Var) -> Self::Output {
        other * self
    }
}

impl<T> Mul<Sum<T>> for f64
    where T: Mul<f64>
{
    type Output = Sum<T::Output>;

    fn mul(self, other: Sum<T>) -> Self::Output {
        other * self
    }
}

/*                         Div
┌────────┬─────┬──────────────────┬─────────────────────┐
│        │ f64 │              Var │              Sum<T> │
├────────┼─────┼──────────────────┼─────────────────────┤
│    f64 │ std │ Div<f64> for Var │ Div<f64> for Sum<T> │
└────────┴─────┴──────────────────┴─────────────────────┘ */

impl Div<f64> for Var {
    type Output = Sum<f64>;

    fn div(self, other: f64) -> Self::Output {
        Sum {
            term: Term {
                coefficient: 1.0 / other,
                variable: self,
            },
            rest: 0.0,
        }
    }
}

impl<T> Div<f64> for Sum<T>
    where T: Mul<f64>
{
    type Output = Sum<T::Output>;

    fn div(self, other: f64) -> Self::Output {
        self*(1.0/other)
    }
}
