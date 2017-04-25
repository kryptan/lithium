use std::ops::Range;
use std::mem::swap;
use std::cmp::Ordering;
use solver::{Solver, Var, Constraint, ConstraintTag, Strength};
use solver::expression::{Expression, Term};
use {Vec2, Rect};
use util::f64_as_u64;

pub struct Layout {
    size: Vec2<f64>,
    solver: Solver,
    current_strength: Strength,

    previous_constraints: Constraints,
    new_constraints: Constraints,
}

impl Default for Layout {
    fn default() -> Self {
        Layout {
            size: Vec2::new(0.0, 0.0),
            solver: Solver::default(),
            current_strength: Strength::Medium,

            previous_constraints: Constraints::default(),
            new_constraints: Constraints::default(),
        }
    }
}

impl Layout {
    pub fn solve(&mut self) {
        {
            let terms = &self.new_constraints.terms;
            // FIXME: use sort_unstable_by when it is stable.
            self.new_constraints.constraints.sort_by(|a, b| {
                compare_constraints(a, terms, b, terms)
            });
        }

        let mut previous_i = 0;
        let mut new_i = 0;

        while previous_i < self.previous_constraints.constraints.len() && new_i < self.new_constraints.constraints.len() {
            let previous_constraint = &self.previous_constraints.constraints[previous_i];
            let new_constraint = &mut self.new_constraints.constraints[new_i];

            match compare_constraints(previous_constraint, &self.previous_constraints.terms, new_constraint, &self.new_constraints.terms) {
                Ordering::Equal => {
                    previous_i += 1;
                    new_i += 1;
                }
                Ordering::Greater => {
                    new_constraint.tag = Some(self.solver.add_constraint(
                        new_constraint.positive,
                        new_constraint.constant,
                        &self.new_constraints.terms[new_constraint.terms.clone()],
                        new_constraint.strength,
                    ));
                    new_i += 1;
                }
                Ordering::Less => {
                    self.solver.remove_constraint(previous_constraint.tag.unwrap());
                    previous_i += 1;
                }
            }
        }

        for constraint in &mut self.new_constraints.constraints[new_i..] {
            constraint.tag = Some(self.solver.add_constraint(
                constraint.positive,
                constraint.constant,
                &self.new_constraints.terms[constraint.terms.clone()],
                constraint.strength,
            ));
        }

        for constraint in &self.previous_constraints.constraints[previous_i..] {
            self.solver.remove_constraint(constraint.tag.unwrap());
        }

        self.previous_constraints.clear();
        swap(&mut self.previous_constraints, &mut self.new_constraints);
    }

    pub fn constraint<E: Expression>(&mut self, constraint: Constraint<E>) {
        self.new_constraints.push(constraint, self.current_strength);
    }

    pub fn resize(&mut self, size: Vec2<f64>) {
        self.size = size;
    }

    pub fn size(&self) -> Vec2<f64> {
        self.size
    }

    pub fn keep(&mut self, var: Var) {
        let prev_value = self.prev_value(var);

        add_constraints!(self, [
            (var) == prev_value,
        ]);
    }

    pub fn prev_value<E: Expression>(&self, expr: E) -> f64 {
        expr.constant() + expr.terms().map(|term| term.coefficient*self.solver.get_value(term.variable).unwrap_or(0.0)).sum::<f64>()
    }

    pub fn prev_value_vec<E: Expression>(&self, v: Vec2<E>) -> Vec2<f64> {
        Vec2::new(self.prev_value(v.x), self.prev_value(v.y))
    }

    pub fn prev_value_rect<E: Expression>(&self, rect: Rect<E>) -> Rect<f64> {
        Rect {
            left: self.prev_value(rect.left),
            top: self.prev_value(rect.top),
            right: self.prev_value(rect.right),
            bottom: self.prev_value(rect.bottom),
        }
    }
}

struct ConstraintInfo {
    terms: Range<usize>,
    constant: f64,
    positive: bool, // otherwise zero
    strength: Strength,
    tag: Option<ConstraintTag>,
}

#[derive(Default)]
struct Constraints {
    terms: Vec<Term>,
    constraints: Vec<ConstraintInfo>,
}

impl Constraints {
    fn push<E>(&mut self, constraint: Constraint<E>, strength: Strength)
        where E: Expression
    {
        let terms_before = self.terms.len();
        self.terms.extend(constraint.expr.terms());
        let terms_after = self.terms.len();

        self.constraints.push(ConstraintInfo {
            terms: terms_before..terms_after,
            constant: constraint.expr.constant(),
            positive: constraint.positive,
            strength: strength,
            tag: None,
        });
    }

    fn clear(&mut self) {
        self.terms.clear();
        self.constraints.clear();
    }
}

fn compare_constraints(a: &ConstraintInfo, a_terms: &[Term], b: &ConstraintInfo, b_terms: &[Term]) -> Ordering {
    match f64_as_u64(a.constant).cmp(&f64_as_u64(b.constant)) {
        Ordering::Equal => {},
        other => return other,
    };

    match a.terms.len().cmp(&b.terms.len()) {
        Ordering::Equal => {},
        other => return other,
    };

    match a.positive.cmp(&b.positive) {
        Ordering::Equal => {},
        other => return other,
    };

    match a.strength.cmp(&b.strength) {
        Ordering::Equal => {},
        other => return other,
    };

    for (a_term_index, b_term_index) in a.terms.clone().zip(b.terms.clone()) {
        let a_term = &a_terms[a_term_index];
        let b_term = &b_terms[b_term_index];

        match f64_as_u64(a_term.coefficient).cmp(&f64_as_u64(b_term.coefficient)) {
            Ordering::Equal => {},
            other => return other,
        };

        match a_term.variable.compare_by_id(b_term.variable) {
            Ordering::Equal => {},
            other => return other,
        }
    }

    Ordering::Equal
}
