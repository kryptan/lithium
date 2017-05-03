use {Rect, Var, Gui};

pub fn center_vertical(gui: &mut Gui, one: Rect<Var>, other: Rect<Var>) {
	add_constraints!(gui.layout, [
		(one.left - other.left) == one.right - other.right,
	]);
}

pub fn center_horizontal(gui: &mut Gui, one: Rect<Var>, other: Rect<Var>) {
	add_constraints!(gui.layout, [
        (one.top - other.top) == one.bottom - other.bottom,
	]);
}

pub fn center(gui: &mut Gui, one: Rect<Var>, other: Rect<Var>) {
	center_horizontal(gui, one, other);
	center_vertical(gui, one, other);
}

pub fn equal_vertical(gui: &mut Gui, one: Rect<Var>, other: Rect<Var>) {
	add_constraints!(gui.layout, [
		(one.left) == other.left,
		(one.right) == other.right,
	]);
}

pub fn equal_horizontal(gui: &mut Gui, one: Rect<Var>, other: Rect<Var>) {
	add_constraints!(gui.layout, [
		(one.top) == other.top,
		(one.bottom) == other.bottom,
	]);
}

pub fn equal(gui: &mut Gui, one: Rect<Var>, other: Rect<Var>) {
	equal_horizontal(gui, one, other);
	equal_vertical(gui, one, other);
}
