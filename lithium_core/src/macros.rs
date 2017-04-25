
#[macro_export]
macro_rules! element_kind {
    ($e:expr) => {
        // FIXME: hash string
        $crate::gui::scene::ElementKind(0)
    }
}

#[macro_export]
macro_rules! add_constraints {
    ($layout:expr, [$(($left:expr) $cmp:tt $right:expr,)*]) => {
        $($layout.constraint(constraint![($left) $cmp $right]);)*
    };

    ($layout:expr, [$(($left:expr) $cmp:tt $right:expr),*]) => {
        $($layout.constraint(constraint![($left) $cmp $right]);)*
    };
}

#[macro_export]
macro_rules! constraint {
    [($x:expr) <= $y:expr] => {
        $crate::solver::Constraint {
            expr: -$x + $y,
            positive: true,
        }
    };
    [($x:expr) >= $y:expr] => {
        $crate::solver::Constraint {
            expr: $x - $y,
            positive: true,
        }
    };
    [($x:expr) == $y:expr] => {
        $crate::solver::Constraint {
            expr: $x - $y,
            positive: false,
        }
    };
}
