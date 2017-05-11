
#[macro_export]
macro_rules! color_id {
    ($e:expr) => {
        // FIXME: hash string
        $crate::theme::ColorId(0)
    }
}

#[macro_export]
macro_rules! element_kind {
    ($e:expr) => {
        // FIXME: hash string
        $crate::theme::ElementKind(0)
    }
}

#[macro_export]
macro_rules! style_variant {
    ($e:expr) => {
        // FIXME: hash string
        $crate::theme::StyleVariant(0)
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
