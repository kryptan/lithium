use {Id, Var};
use std::collections::HashMap;
use super::near_zero;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Symbol {
    id: Id,
    pub kind: SymbolKind,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum SymbolKind {
    External,
    Slack,
    Error,
}

#[derive(Clone, Default)]
pub struct Row {
    pub cells: HashMap<Symbol, f64>,
    pub constant: f64
}

impl Symbol {
    pub fn new(kind: SymbolKind) -> Self {
        Symbol {
            id: Id::unique(),
            kind: kind,
        }
    }

    pub fn from_var(var: Var) -> Self {
        Symbol {
            id: var.0,
            kind: SymbolKind::External,
        }
    }
}

impl Row {
    pub fn new(constant: f64) -> Row {
        Row {
            cells: HashMap::new(),
            constant: constant
        }
    }

    pub fn add_symbol(&mut self, symbol: Symbol, k: f64) {
        use std::collections::hash_map::Entry;

        match self.cells.entry(symbol) {
            Entry::Vacant(entry) => {
                if !near_zero(k) {
                    entry.insert(k);
                }
            },
            Entry::Occupied(mut entry) => {
                *entry.get_mut() += k;
                if near_zero(*entry.get_mut()) {
                    entry.remove();
                }
            }
        }
    }

    pub fn add_row(&mut self, other: &Row, k: f64) {
        self.constant += other.constant * k;
        for (other_symbol, other_k) in &other.cells {
            self.add_symbol(*other_symbol, other_k * k);
        }
    }

    pub fn reverse_sign(&mut self) {
        self.constant = -self.constant;
        for k in self.cells.values_mut() {
            *k = -*k;
        }
    }

    pub fn solve_for_symbol(&mut self, symbol: Symbol) {
        let k = -1.0 / self.cells.remove(&symbol).unwrap();
        self.constant *= k;
        for (_, v) in &mut self.cells {
            *v *= k;
        }
    }

    pub fn solve_for_symbols(&mut self, lhs: Symbol, rhs: Symbol) {
        self.add_symbol(lhs, -1.0);
        self.solve_for_symbol(rhs);
    }

    pub fn coefficient_for(&self, symbol: Symbol) -> f64 {
        self.cells.get(&symbol).cloned().unwrap_or(0.0)
    }

    pub fn substitute(&mut self, symbol: Symbol, row: &Row) {
        if let Some(k) = self.cells.remove(&symbol) {
            self.add_row(row, k)
        }
    }
}
