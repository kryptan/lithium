#[macro_use]
mod syntax;
mod selector;
mod property;
mod value;
mod image;
mod border;
mod background;

use std::collections::HashMap;
use std::sync::Arc;
use cssparser::{Parser, ParserInput, ParseError, Token, Delimiter};
use {Theme, Color};
use theme::{ColorId, ElementKind, StyleVariant, ElementStyle};
use self::selector::{Selector, expect_hash_selector, selectors};

pub type CssError<'i> = ParseError<'i, &'static str>;
pub type CssResult<'i, T> = Result<T, CssError<'i>>;

fn error<'i, T>(err: &'static str) -> CssResult<'i, T> {
    Err(ParseError::Custom(err))
}

pub fn theme(input: &str) -> CssResult<Theme> {
    let mut parser_input = ParserInput::new(input);
    let mut parser = Parser::new(&mut parser_input);

    let mut styles: Vec<(Selector, &str)> = Vec::new();

    while !parser.is_exhausted() {
        if Ok(()) == parser.try(|parser| expect_hash_selector(parser, "colors")) {
            unimplemented!()
        } else {
            let selectors = parser.parse_until_before(Delimiter::CurlyBracketBlock, selectors)?;
            if parser.next() != Ok(Token::CurlyBracketBlock) {
                return error("expected curly bracket");
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

    let /*mut*/ _colors: HashMap<(StyleVariant, ColorId), Color> = HashMap::new();
    let mut element_styles: HashMap<(StyleVariant, ElementKind), ElementStyle> = HashMap::new();

    for &(selector, _) in &styles {
        if let Selector::Full(style_variant, element_kind) = selector {
            element_styles.entry((style_variant, element_kind)).or_insert(ElementStyle::default());
        }
    }

    // FIXME: quadratic loop
    for &(selector, style) in &styles {
        for (&(style_variant, element_kind), ref mut element_style_value) in &mut element_styles {
            if selector.matches((style_variant, element_kind)) {
                element_style(style, element_style_value)?;
            }
        }
    }

    let mut theme = Theme::empty();

    for (selector, element_style) in element_styles {
        theme.element_styles.insert(selector, Arc::new(element_style));
    }

    Ok(theme)
}

pub fn element_style<'i>(input: &'i str, element_style: &mut ElementStyle) -> CssResult<'i, ()> {
    let mut parser_input = ParserInput::new(input);
    let mut parser = Parser::new(&mut parser_input);

    while !parser.is_exhausted() {
        parser.parse_until_after(Delimiter::Semicolon, |parser| {
            let property = parser.expect_ident()?;
            parser.expect_colon()?;

            property::property(parser, &property, element_style)
        })?;
    }

    Ok(())
}

#[test]
fn test_parse_element_style() {
    let css = r#"
    border-width: 1px;
    border-color: #123456;
    "#;

    let mut element_style_value = ElementStyle::default();
    element_style(css, &mut element_style_value).unwrap();

    for i in 0..4 {
        assert_eq!(element_style_value.border[i].width, 1.0);
        assert_eq!(element_style_value.border[i].color, Color::from_css_hex(b"123456"));
    }
}

#[test]
fn test_parse_theme() {
    use theme::{style_variant, element_kind};

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

    let theme = theme(css).unwrap();

    assert_eq!(theme.element_styles.len(), 3);
    assert_eq!(theme.element_style(style_variant("error"),   element_kind("Button")).unwrap().border[0].width, 4.0);
    assert_eq!(theme.element_style(style_variant("default"), element_kind("Button")).unwrap().border[0].width, 1.0);
    assert_eq!(theme.element_style(style_variant("default"), element_kind("OtherWidget")).unwrap().border[0].width, 7.0);
    assert_eq!(theme.element_style(style_variant("error"),   element_kind("OtherWidget")).unwrap().border[0].width, 7.0);
}
