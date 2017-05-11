use std::any::Any;
use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use std::mem::swap;
use {Color, Vec2, Rect, Theme, Id};
use theme::{ColorId, ElementKind, ElementStyle, StyleVariant};

#[derive(PartialEq)]
pub struct Scene {
    theme: Theme,
    style_variant: StyleVariant,
    commands: Vec<Command>,
}

impl Scene {
    pub fn new(theme: Theme) -> Self {
        Scene {
            theme: theme,
            style_variant: StyleVariant::default(),
            commands: Vec::new(),
        }
    }

    pub fn color(&self, id: ColorId) -> Color {
        self.theme.color(self.style_variant, id).unwrap_or(Color::error())
    }

    pub fn element_style(&self, kind: ElementKind) -> Arc<ElementStyle> {
        self.theme.element_style(self.style_variant, kind).unwrap_or(Arc::new(ElementStyle::default()))
    }

    pub fn text(&mut self, text: Text) {
        self.commands.push(Command::Text(text));
    }

    pub fn mesh(&mut self, mesh: Mesh) {
        self.commands.push(Command::Mesh(mesh));
    }

    pub fn start_element(&mut self) {
        self.commands.push(Command::StartElement);
    }

    pub fn close_element(&mut self, element: Element) {
        self.commands.push(Command::CloseElement(element));
    }

    pub fn swap_theme(&mut self, mut theme: Theme) -> Theme {
        swap(&mut self.theme, &mut theme);
        theme
    }

    pub fn swap_style_variant(&mut self, mut style_variant: StyleVariant) -> StyleVariant {
        swap(&mut self.style_variant, &mut style_variant);
        style_variant
    }

    pub fn commands(&self) -> &[Command] {
        &self.commands
    }

    pub fn advance(&mut self) {
        self.commands.clear();
    }
}

#[derive(PartialEq)]
pub enum Command {
    StartElement,
    CloseElement(Element),
    Text(Text),
    Mesh(Mesh),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Element {
    pub id: Id,
    pub place: Rect<f64>,
    pub kind: ElementKind,
    pub style: Arc<ElementStyle>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Text {
    pub id: Id,
    pub glyphs: Rc<RefCell<Vec<Glyph>>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Glyph {
    /// Position is relative.
    pub position: Vec2<f64>,
    pub scale: Vec2<f64>,
    pub glyph_id: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Mesh {
    pub data: Rc<RefCell<MeshVertices>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MeshVertices {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

#[derive(Clone, Debug, PartialEq)]
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
