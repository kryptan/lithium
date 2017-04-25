use std::f64;
use std::collections::HashMap;
use std::cmp::Ordering;
use Id;
use self::row::{Row, Symbol, SymbolKind};
use self::expression::{Expression, Term};

pub mod expression;
mod row;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Var(Id);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
pub enum Strength {
    Weak,
    Medium,
    Strong,
}

impl From<Id> for Var {
    fn from(id: Id) -> Self {
        Var(id)
    }
}

impl Var {
    pub fn compare_by_id(self, other: Var) -> Ordering {
        self.0.cmp(&other.0)
    }
}

pub struct Constraint<E: Expression> {
    pub expr: E,
    pub positive: bool,
}

#[derive(Copy, Clone)]
pub struct ConstraintTag {
    marker: Symbol,
    other: Symbol,
}

/// A constraint solver which uses the cassowary algorithm.
#[derive(Clone, Default)]
pub struct Solver {
    rows: HashMap<Symbol, Row>,
    objective: Row,
}

impl Solver {
    /// Add a constraint to the solver.
    pub fn add_constraint(&mut self, positive: bool, constant: f64, terms: &[Term], strength: Strength) -> ConstraintTag {
        let strength = match strength {
            Strength::Weak => 0.001,
            Strength::Medium => 1.0,
            Strength::Strong => 1000.0,
        };

        let (mut row, tag) = self.create_row(positive, constant, terms, strength);
        let subject = Self::choose_subject(&row, tag);

        row.solve_for_symbol(subject);
        self.substitute(subject, &row);
        self.rows.insert(subject, row);

        // Optimizing after each constraint is added performs less aggregate work due to a smaller average system size. It also ensures the solver
        // remains in a consistent state.
        self.optimize();

        tag
    }

    /// Remove a constraint from the solver.
    pub fn remove_constraint(&mut self, tag: ConstraintTag) {
        // Remove the error effects from the objective function *before* pivoting, or substitutions into the objective will lead to incorrect solver
        // results.
        self.remove_constraint_effects(tag);

        // If the marker is basic, simply drop the row. Otherwise, pivot the marker into the basis and then drop the row.
        if self.rows.remove(&tag.marker).is_none() {
            let (leaving, mut row) = self.get_marker_leaving_row(tag.marker);
            row.solve_for_symbols(leaving, tag.marker);
            self.substitute(tag.marker, &row);
        }

        // Optimizing after each constraint is removed ensures that the solver remains consistent. It makes the solver api easier to use at a small
        // trade-off for speed.
        self.optimize();
    }

    pub fn get_value(&self, var: Var) -> Option<f64> {
        self.rows.get(&Symbol::from_var(var)).map(|row| row.constant)
    }

    /// Create a new Row object for the given constraint.
    ///
    /// The terms in the constraint will be converted to cells in the row. Any term in the constraint with a coefficient of zero is ignored. If the
    /// symbol for a given cell variable is basic, the cell variable will be substituted with the basic row. The necessary slack and error variables
    /// will be added to the row. If the constant for the row is negative, the sign for the row will be inverted so the constant becomes positive.
    ///
    /// The tag will be updated with the marker and error symbols to use for tracking the movement of the constraint in the tableau.
    fn create_row(&mut self, positive: bool, constant: f64, terms: &[Term], strength: f64) -> (Row, ConstraintTag) {
        let mut row = Row::new(constant);
        // Substitute the current basic variables into the row.
        for term in terms {
            if near_zero(term.coefficient) {
                continue;
            }

            let symbol = Symbol::from_var(term.variable);
            if let Some(other_row) = self.rows.get(&symbol) {
                row.add_row(other_row, term.coefficient);
            } else {
                row.add_symbol(symbol, term.coefficient);
            }
        }

        // Add the necessary slack and error variables.
        let tag = if positive {
            let slack = Symbol::new(SymbolKind::Slack);
            let error = Symbol::new(SymbolKind::Error);
            row.add_symbol(slack, -1.0);
            row.add_symbol(error, 1.0);
            self.objective.add_symbol(error, strength);
            ConstraintTag {
                marker: slack,
                other: error,
            }
        } else {
            let error_plus = Symbol::new(SymbolKind::Error);
            let error_minus = Symbol::new(SymbolKind::Error);
            row.add_symbol(error_plus, -1.0); // v = error_plus - error_minus
            row.add_symbol(error_minus, 1.0); // v - error_plus + error_minus = 0
            self.objective.add_symbol(error_plus, strength);
            self.objective.add_symbol(error_minus, strength);
            ConstraintTag {
                marker: error_plus,
                other: error_minus,
            }
        };

        // Ensure the row has a positive constant.
        if row.constant < 0.0 {
            row.reverse_sign();
        }

        (row, tag)
    }

    /// Choose the best subject for using as the solve target for the row.
    ///
    /// The symbol is chosen according to the following precedence:
    ///
    /// 1) The first external variable.
    ///
    /// 2) A negative slack or error variable.
    fn choose_subject(row: &Row, tag: ConstraintTag) -> Symbol {
        for symbol in row.cells.keys() {
            if symbol.kind == SymbolKind::External {
                return *symbol;
            }
        }
        if row.coefficient_for(tag.marker) < 0.0 {
            return tag.marker;
        }
        if row.coefficient_for(tag.other) < 0.0 {
            return tag.other;
        }

        // At least one of `tag.marker` or `tag.other` should have negative coefficient because they were added in `create_row` with +1.0 and -1.0
        // coefficients.
        unreachable!()
    }

    /// Substitute the symbol with the given row.
    ///
    /// This method will substitute all instances of the symbol in the tableau and the objective function with the given row.
    fn substitute(&mut self, symbol: Symbol, row: &Row) {
        for other_row in self.rows.values_mut() {
            other_row.substitute(symbol, row);
        }
        self.objective.substitute(symbol, row);
    }

    /// Optimize the system for the given objective function.
    ///
    /// This method performs iterations of Phase 2 of the simplex method until the objective function reaches a minimum.
    fn optimize(&mut self) {
        loop {
            let entering = Self::get_entering_symbol(&self.objective);
            if let Some(entering) = entering {
                let (leaving, mut row) = self.get_leaving_row(entering);
                // pivot the entering symbol into the basis
                row.solve_for_symbols(leaving, entering);
                self.substitute(entering, &row);
                self.rows.insert(entering, row);
            } else {
                return;
            }
        }
    }

    /// Compute the entering variable for a pivot operation.
    ///
    /// This method will return first symbol in the objective function which has a coefficient less than zero. If no symbol meets the criteria, it
    /// means the objective function is at a minimum, and  `None` is returned.
    ///
    /// Could return an External symbol
    fn get_entering_symbol(objective: &Row) -> Option<Symbol> {
        for (symbol, value) in &objective.cells {
            if *value < 0.0 {
                return Some(*symbol);
            }
        }
        None
    }

    /// Compute the row which holds the exit symbol for a pivot.
    ///
    /// This method will return the row in the row map which holds the exit symbol.
    ///
    /// If no appropriate exit symbol is found it indicates that the objective function is unbounded.
    ///
    /// Never returns a row for an External symbol.
    fn get_leaving_row(&mut self, entering: Symbol) -> (Symbol, Row) {
        let mut ratio = f64::INFINITY;
        let mut found = None;
        for (symbol, row) in &self.rows {
            if symbol.kind != SymbolKind::External {
                let temp = row.coefficient_for(entering);
                if temp < 0.0 {
                    let temp_ratio = -row.constant / temp;
                    if temp_ratio < ratio {
                        ratio = temp_ratio;
                        found = Some(*symbol);
                    }
                }
            }
        }
        found.map(|symbol| (symbol, self.rows.remove(&symbol).unwrap())).unwrap()
    }

    /// Compute the leaving row for a marker variable.
    ///
    /// This method will return a row from the row map which holds the given marker variable. The row will be chosen according to the following
    /// precedence:
    ///
    /// 1) The row with a restricted basic variable and a negative coefficient for the marker with the smallest ratio of -constant/coefficient.
    ///
    /// 2) The row with a restricted basic variable and the smallest ratio of constant/coefficient.
    ///
    /// 3) The last unrestricted row which contains the marker.
    ///
    /// If the marker does not exist in any row this indicates an internal solver error since the marker *should* exist somewhere in the tableau.
    fn get_marker_leaving_row(&mut self, marker: Symbol) -> (Symbol, Row) {
        let mut r1 = f64::INFINITY;
        let mut r2 = r1;
        let mut first = None;
        let mut second = None;
        let mut third = None;
        for (&symbol, row) in &self.rows {
            let c = row.coefficient_for(marker);
            if c == 0.0 {
                continue;
            }
            if symbol.kind == SymbolKind::External {
                third = Some(symbol);
            } else if c < 0.0 {
                let r = -row.constant / c;
                if r < r1 {
                    r1 = r;
                    first = Some(symbol);
                }
            } else {
                let r = row.constant / c;
                if r < r2 {
                    r2 = r;
                    second = Some(symbol);
                }
            }
        }

        first.or(second).or(third).and_then(|symbol| {
            self.rows.remove(&symbol).map(|row| (symbol, row))
        }).unwrap()
    }

    /// Remove the effects of a constraint on the objective function.
    fn remove_constraint_effects(&mut self, tag: ConstraintTag) {
        if tag.marker.kind == SymbolKind::Error {
            self.objective.cells.remove(&tag.marker);
        }

        if tag.other.kind == SymbolKind::Error { // FIXME: always true?
            self.objective.cells.remove(&tag.other);
        }
    }
}

fn near_zero(value: f64) -> bool {
    const EPS: f64 = 1e-8;
    value.abs() < EPS
}
