use cssparser::{Parser, Token};
use theme::{ElementKind, StyleVariant, style_variant, element_kind};
use super::{CssResult, error};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Selector {
    ElementKind(ElementKind),
    StyleVariant(StyleVariant),
    Full(StyleVariant, ElementKind),
}

impl Selector {
    pub fn specificity(self) -> u32 {
        match self {
            Selector::ElementKind(_) => 0,
            Selector::StyleVariant(_) => 1,
            Selector::Full(..) => 2,
        }
    }

    pub fn matches(self, (style_variant, element_kind): (StyleVariant, ElementKind)) -> bool {
        match self {
            Selector::ElementKind(self_element_kind) => self_element_kind == element_kind,
            Selector::StyleVariant(self_style_variant) => self_style_variant == style_variant,
            Selector::Full(self_style_variant, self_element_kind) =>
                self_style_variant == style_variant &&
                self_element_kind == element_kind,
        }
    }
}

pub fn expect_hash_selector<'i, 'tt>(parser: &mut Parser<'i, 'tt>, expected_id: &str) -> CssResult<'i, ()> {
    match parser.next()? {
        Token::IDHash(ref actual_id) if actual_id.as_ref() == expected_id => Ok(()),
        _ => error("expected hash selector")
    }
}

pub fn selectors<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> CssResult<'i, Vec<Selector>> {
    parser.parse_comma_separated(selector)
}

fn selector<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> CssResult<'i, Selector> {
    let kind = if let Ok(id) = parser.try(|parser| parser.expect_ident()) {
        Some(element_kind(&id))
    } else {
        None
    };

    let style_variant = if Ok(()) == parser.try(|parser| parser.expect_delim('.')) {
        let id = parser.expect_ident()?;
        Some(style_variant(&id))
    } else {
        None
    };

    match (kind, style_variant) {
        (Some(kind), Some(style_variant)) => Ok(Selector::Full(style_variant, kind)),
        (Some(kind), None               ) => Ok(Selector::ElementKind(kind)),
        (None,       Some(style_variant)) => Ok(Selector::StyleVariant(style_variant)),

        (None, None) => error("invalid selector"),
    }
}

#[test]
fn test_parse_selectors() {
    use cssparser::ParserInput;

    assert_eq!(selector(&mut Parser::new(&mut ParserInput::new("Test"))).unwrap(), Selector::ElementKind(element_kind("Test")));
    assert_eq!(selector(&mut Parser::new(&mut ParserInput::new("Test.style"))).unwrap(), Selector::Full(style_variant("style"), element_kind("Test")));
    assert_eq!(selector(&mut Parser::new(&mut ParserInput::new(" Test . style "))).unwrap(), Selector::Full(style_variant("style"), element_kind("Test")));
    assert_eq!(selector(&mut Parser::new(&mut ParserInput::new(".style"))).unwrap(), Selector::StyleVariant(style_variant("style")));

    assert_eq!(selectors(
        &mut Parser::new(&mut ParserInput::new("Test.style, Chunga.changa, Button"))).unwrap(),
        vec![
            Selector::Full(style_variant("style"), element_kind("Test")),
            Selector::Full(style_variant("changa"), element_kind("Chunga")),
            Selector::ElementKind(element_kind("Button"))
        ]
    );

    assert_eq!(expect_hash_selector(&mut Parser::new(&mut ParserInput::new("#theme")), "theme"), Ok(()));
    assert!(expect_hash_selector(&mut Parser::new(&mut ParserInput::new("#theme")), "theme2").is_err());
}
