use lithium_core::theme::{ColorId, ElementKind, StyleVariant, ElementStyle};
use blake2_rfc::blake2b::blake2b;

/// Generate color identifier from the string.
pub fn color_id(text: &str) -> ColorId {
    ColorId(hash("color_id", text))
}

/// Generate element kind identifier from the string.
pub fn element_kind(text: &str) -> ElementKind {
    ElementKind(hash("element_kind", text))
}

/// Generate style variant identifier from the string.
pub fn style_variant(text: &str) -> StyleVariant {
    StyleVariant(hash("style_variant", text))
}

fn hash(key: &str, text: &str) -> u64 {
    let result = blake2b(8, key.as_bytes(), text.as_bytes());
    let bytes = result.as_bytes();

    bytes[0] as u64 |
    ((bytes[1] as u64) << 8) |
    ((bytes[2] as u64) << 16) |
    ((bytes[3] as u64) << 24) |
    ((bytes[4] as u64) << 32) |
    ((bytes[5] as u64) << 40) |
    ((bytes[6] as u64) << 48) |
    ((bytes[7] as u64) << 56)
}

// Check that identifiers are always generated in the same way.
#[test]
fn ids_stay_the_same() {
    assert_eq!(color_id("test"), ColorId(15669914510866457799));
    assert_eq!(element_kind("test"), ElementKind(14929189791165124317));
    assert_eq!(style_variant("test"), StyleVariant(15111611029265304875));
}