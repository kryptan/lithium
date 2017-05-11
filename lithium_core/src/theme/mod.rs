use std::collections::HashMap;
use std::sync::Arc;
use util::IdIdentityHasherBuilder;
use Color;

pub use self::element_style::ElementStyle;

pub mod element_style;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ColorId(pub u64);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ElementKind(pub u64);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct StyleVariant(pub u64);

#[derive(Clone, Default, PartialEq)]
pub struct Theme {
    pub colors: HashMap<(StyleVariant, ColorId), Color, IdIdentityHasherBuilder>,
    pub element_styles: HashMap<(StyleVariant, ElementKind), Arc<ElementStyle>, IdIdentityHasherBuilder>,
}

impl Theme {
    pub fn empty() -> Self {
        Theme {
            colors: HashMap::with_hasher(IdIdentityHasherBuilder),
            element_styles: HashMap::with_hasher(IdIdentityHasherBuilder),
        }
    }

    pub fn color(&self, style_variant: StyleVariant, id: ColorId) -> Option<Color> {
        if let Some(&color) = self.colors.get(&(style_variant, id)) {
            Some(color)
        } else if let Some(&color) = self.colors.get(&(StyleVariant::default(), id)) {
            Some(color)
        } else {
            None
        }
    }

    pub fn element_style(&self, style_variant: StyleVariant, kind: ElementKind) -> Option<Arc<ElementStyle>> {
        if let Some(&ref element_style) = self.element_styles.get(&(style_variant, kind)) {
            Some(element_style.clone())
        } else if let Some(&ref element_style) = self.element_styles.get(&(StyleVariant::default(), kind)) {
            Some(element_style.clone())
        } else {
            None
        }
    }
}

impl Default for StyleVariant {
    fn default() -> Self {
        style_variant!("default")
    }
}