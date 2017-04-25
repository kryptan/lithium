use self::layout::Layout;
use self::input::Input;
use self::scene::{Scene, Element, ElementKind};
use {Id, Rect};

pub mod layout;
pub mod input;
pub mod scene;

pub struct Gui {
    pub layout: Layout,
    pub scene: Scene,
    pub input: Input,
}

impl Gui {
    pub fn element<F: FnOnce(&mut Gui)>(&mut self, id: Id, kind: ElementKind, f: F) {
        let style = self.scene.element_style(kind);
        let place = self.layout.prev_value_rect(Rect::from(id));

        self.scene.start_element(Element {
            id: id,
            style: style,
            place: place,
            kind: kind,
        });
        f(self);
        self.scene.close_element(); // FIXME: execute even in case of panic
    }
}
