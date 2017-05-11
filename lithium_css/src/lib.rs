extern crate lithium_core;
#[macro_use] extern crate cssparser;
extern crate blake2_rfc;

mod identifiers;
mod properties;

use std::collections::HashMap;
use std::sync::Arc;
use cssparser::{Parser, Token, Delimiter};
use lithium_core::{Theme, Color};
use lithium_core::theme::{ColorId, ElementKind, StyleVariant, ElementStyle};

pub use identifiers::{color_id, element_kind, style_variant};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Selector {
    ElementKind(ElementKind),
    StyleVariant(StyleVariant),
    Full(StyleVariant, ElementKind),
}

impl Selector {
    fn specificity(self) -> u32 {
        match self {
            Selector::ElementKind(_) => 0,
            Selector::StyleVariant(_) => 1,
            Selector::Full(..) => 2,
        }
    }

    fn matches(self, (style_variant, element_kind): (StyleVariant, ElementKind)) -> bool {
        match self {
            Selector::ElementKind(self_element_kind) => self_element_kind == element_kind,
            Selector::StyleVariant(self_style_variant) => self_style_variant == style_variant,
            Selector::Full(self_style_variant, self_element_kind) => self_style_variant == style_variant && self_element_kind == element_kind,
        }
    }
}

pub fn parse_theme(input: &str) -> Result<Theme, ()> {
    let mut parser = Parser::new(input);

    let mut styles: Vec<(Selector, &str)> = Vec::new();

    while !parser.is_exhausted() {
        if Ok(()) == parser.try(|parser| parse_hash_selector(parser, "colors")) {
            unimplemented!()
        } else {
            let selectors = parser.parse_until_before(Delimiter::CurlyBracketBlock, parse_selectors)?;
            if parser.next() != Ok(Token::CurlyBracketBlock) {
                return Err(());
            }

            let start_position = parser.position();
            let end_position = parser.parse_nested_block(|parser| {
                while !parser.is_exhausted() {
                    parser.next()?;
                }

                Ok(parser.position())
            })?;
            
            let slice = parser.slice(start_position..end_position);

            for &selector in &selectors {
                styles.push((selector, slice));
            }
        }
    }
    
    // Stable sort by increasing specificity.
    styles.sort_by_key(|&(selector, _)| selector.specificity());

    let mut colors: HashMap<(StyleVariant, ColorId), Color> = HashMap::new();
    let mut element_styles: HashMap<(StyleVariant, ElementKind), ElementStyle> = HashMap::new();

    for &(selector, _) in &styles {
        if let Selector::Full(style_variant, element_kind) = selector {
            element_styles.entry((style_variant, element_kind)).or_insert(ElementStyle::default());
        }
    }

    // FIXME: quadratic loop
    for &(selector, style) in &styles {
        for (&(style_variant, element_kind), ref mut element_style) in &mut element_styles {
            if selector.matches((style_variant, element_kind)) {
                parse_element_style(style, element_style)?;
            }
        }
    }

    let mut theme = Theme::empty();

    for (selector, element_style) in element_styles.into_iter() {
        theme.element_styles.insert(selector, Arc::new(element_style));
    }

    Ok(theme)
}

pub fn parse_element_style(input: &str, element_style: &mut ElementStyle) -> Result<(), ()> {
    let mut parser = Parser::new(input);

    while !parser.is_exhausted() {
        parser.parse_until_after(Delimiter::Semicolon, |parser| {
            let property = parser.expect_ident()?;
            parser.expect_colon()?;

            properties::parse_property(parser, &property, element_style)
        })?;
    }

    Ok(())
}

fn parse_hash_selector(parser: &mut Parser, expected_id: &str) -> Result<(), ()>{
    match parser.next()? {
        Token::IDHash(ref actual_id) if actual_id == expected_id => Ok(()),
        _ => Err(())
    }
}

fn parse_selectors(parser: &mut Parser) -> Result<Vec<Selector>, ()> {
    parser.parse_comma_separated(parse_selector)
}

fn parse_selector(parser: &mut Parser) -> Result<Selector, ()> {
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

        (None, None) => Err(()),
    }
}

#[test]
fn test_parse_selectors() {
    assert_eq!(parse_selector(&mut Parser::new("Test")).unwrap(), Selector::ElementKind(element_kind("Test")));
    assert_eq!(parse_selector(&mut Parser::new("Test.style")).unwrap(), Selector::Full(style_variant("style"), element_kind("Test")));
    assert_eq!(parse_selector(&mut Parser::new(" Test . style ")).unwrap(), Selector::Full(style_variant("style"), element_kind("Test")));
    assert_eq!(parse_selector(&mut Parser::new(".style")).unwrap(), Selector::StyleVariant(style_variant("style")));

    assert_eq!(parse_selectors(
        &mut Parser::new("Test.style, Chunga.changa, Button")).unwrap(),
        vec![
            Selector::Full(style_variant("style"), element_kind("Test")),
            Selector::Full(style_variant("changa"), element_kind("Chunga")),
            Selector::ElementKind(element_kind("Button"))
        ]
    );

    assert_eq!(parse_hash_selector(&mut Parser::new("#theme"), "theme"), Ok(()));
    assert_eq!(parse_hash_selector(&mut Parser::new("#theme"), "theme2"), Err(()));
}

#[test]
fn test_parse_element_style() {
    let css = r#"
    border-width: 1px;
    border-color: #123456;
    "#;

    let mut element_style = ElementStyle::default();
    parse_element_style(css, &mut element_style).unwrap();

    for i in 0..4 {
        assert_eq!(element_style.border[i].width, 1.0);
        assert_eq!(element_style.border[i].color, Color::from_css_hex(b"123456"));
    }
}

#[test]
fn test_parse_theme() {
    let css = r#"
    Button {
        border-width: 1px;
        border-color: #123456;
    }

    OtherWidget {
        border-width: 7px;
        border-color: #654321;
    }
    
    Button.error {
        border-width: 4px;
        border-color: #412578;
    }

    Button.default, OtherWidget.default, Button.error {}
    "#;

    let theme = parse_theme(css).unwrap();

    assert_eq!(theme.element_styles.len(), 3);
    assert_eq!(theme.element_style(style_variant("error"),   element_kind("Button")).unwrap().border[0].width, 4.0);
    assert_eq!(theme.element_style(style_variant("default"), element_kind("Button")).unwrap().border[0].width, 1.0);
    assert_eq!(theme.element_style(style_variant("default"), element_kind("OtherWidget")).unwrap().border[0].width, 7.0);
    assert_eq!(theme.element_style(style_variant("error"),   element_kind("OtherWidget")).unwrap().border[0].width, 7.0);

}
