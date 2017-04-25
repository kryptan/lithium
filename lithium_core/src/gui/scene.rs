use std::any::Any;
use std::rc::Rc;
use std::collections::HashMap;
use std::cell::RefCell;
use std::mem::swap;
use {font, Color, Vec2, Rect, Font, Id};
use util::IdIdentityHasherBuilder;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ColorId(u64);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ElementKind(pub u64);

pub struct Scene {
    themes: Vec<Theme>,
    commands: Vec<Command>,
}

impl Scene {
    pub fn new(&mut self, default_theme: Theme) -> Self {
        Scene {
            themes: vec![default_theme],
            commands: Vec::new(),
        }
    }

    pub fn color(&self, id: ColorId) -> Color {
        self.themes.iter().rev().flat_map(|theme| theme.colors.get(&id)).cloned().next().unwrap_or(Color::error())
    }

    pub fn element_style(&self, kind: ElementKind) -> ElementStyle {
        self.themes.iter().rev().flat_map(|theme| theme.element_styles.get(&kind)).cloned().next().unwrap_or(ElementStyle::error())
    }

    pub fn text(&mut self, text: Text) {
        self.commands.push(Command::Text(text));
    }

    pub fn mesh(&mut self, mesh: Mesh) {
        self.commands.push(Command::Mesh(mesh));
    }

    pub fn start_element(&mut self, element: Element) {
        self.commands.push(Command::StartElement(element));
    }

    pub fn close_element(&mut self) {
        self.commands.push(Command::CloseElement);
    }

    pub fn themed<F: FnOnce(&mut Scene)>(&mut self, theme: Theme, f: F) -> Theme {
        self.themes.push(theme);
        f(self);
        self.themes.pop().unwrap() // FIXME: execute even in case of panic
    }

    pub fn swap_default_theme(&mut self, mut theme: Theme) -> Theme {
        swap(&mut self.themes[0], &mut theme); // zeroth theme is always added in `new`.
        theme
    }

    pub fn commands(&self) -> &[Command] {
        &self.commands
    }

    pub fn advance(&mut self) {
        self.commands.clear();
    }
}

pub enum Command {
    StartElement(Element),
    CloseElement,
    Text(Text),
    Mesh(Mesh),
    Other(Box<Render>),
}

#[derive(Clone, Default)]
pub struct Theme {
    pub colors: HashMap<ColorId, Color, IdIdentityHasherBuilder>,
    pub element_styles: HashMap<ElementKind, ElementStyle, IdIdentityHasherBuilder>,
}

#[derive(Clone, Debug)]
pub struct ElementStyle {
    pub background: Color,
    pub color: Color,
    pub font: Rc<Font>,
    pub blur_radius: f32,
}

impl ElementStyle {
    pub fn error() -> Self {
        ElementStyle {
            color: Color::error(),
            font: Rc::new(font::ErrorFont),
            blur_radius: 0.0,
            background: Color::error(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Element {
    pub id: Id,
    pub place: Rect<f64>,
    pub kind: ElementKind,
    pub style: ElementStyle,
}

#[derive(Clone)]
pub struct Text {
    pub id: Id,
    pub glyphs: Rc<RefCell<Vec<Glyph>>>,
}

#[derive(Clone, Debug)]
pub struct Glyph {
    /// Position is relative.
    pub position: Vec2<f64>,
    pub scale: Vec2<f64>,
    pub glyph_id: u32,
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub data: Rc<RefCell<MeshVertices>>,
}

#[derive(Clone, Debug)]
pub struct MeshVertices {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

#[derive(Clone, Debug)]
pub struct Vertex {
    pub position: Vec2<f64>,
    pub color: Color,
}

/// Anything that can be rendered.
///
/// There are no methods because renderer should downcast it to the concrete type.
///
/// This trait is mostly to prevent accidentally trying to draw things which cannot be rendered.
pub trait Render: 'static + Any {}
