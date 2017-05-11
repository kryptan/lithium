use self::layout::Layout;
use self::input::Input;
use self::scene::{Scene, Element};
use {Id, Rect, Theme};
use theme::{ElementKind, StyleVariant};

pub mod layout;
pub mod input;
pub mod scene;

pub struct Gui {
    pub layout: Layout,
    pub scene: Scene,
    pub input: Input,
}

impl Gui {
    pub fn new(default_theme: Theme) -> Self {
        Gui {
            layout: Layout::default(),
            scene: Scene::new(default_theme),
            input: Input::default(),
        }
    }

    pub fn element<F, R>(&mut self, id: Id, kind: ElementKind, f: F) -> R
        where F: FnOnce(&mut Gui) -> R
    {
        let style = self.scene.element_style(kind);
        let place = self.layout.prev_value_rect(Rect::from(id));

        self.scene.start_element();
        let result = f(self);
        self.scene.close_element(Element {
            id: id,
            style: style,
            place: place,
            kind: kind,
        }); // FIXME: execute even in case of panic

        result
    }
    
    pub fn themed<F: FnOnce(&mut Gui)>(&mut self, theme: Theme, f: F) -> Theme {
        let old_theme = self.scene.swap_theme(theme);
        f(self);
        self.scene.swap_theme(old_theme) // FIXME: execute even in case of panic
    }

    pub fn styled<F: FnOnce(&mut Gui)>(&mut self, style_variant: StyleVariant, f: F) -> StyleVariant {
        let old_style = self.scene.swap_style_variant(style_variant);
        f(self);
        self.scene.swap_style_variant(old_style) // FIXME: execute even in case of panic
    }

    pub fn advance(&mut self) {
        self.input.advance();
        self.scene.advance();
        self.layout.advance();
    }
}
