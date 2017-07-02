// CSS value definition syntax, a formal grammar, is used for defining the set of valid values for a CSS property or function.
//
// https://developer.mozilla.org/en/docs/Web/CSS/Value_definition_syntax

use std;
use cssparser::Parser;
use super::{CssResult, error};

// <value>?
// 0 or 1 time (that is optional).
pub fn maybe<'i, 'tt, F, R>(parser: &mut Parser<'i, 'tt>, f: F) -> CssResult<'i, Option<R>>
    where F: for<'tt2> Fn(&mut Parser<'i, 'tt2>) -> CssResult<'i, R>
{
    if let Ok(value) = parser.try(f) {
        Ok(Some(value))
    } else {
        Ok(None)
    }
}

// <value>*
// 0 or more times.
/*pub fn zero_or_more<'i, 'tt, F, R>(parser: &mut Parser<'i, 'tt>, f: F) -> CssResult<'i, Vec<R>>
    where F: for<'tt2> Fn(&mut Parser<'i, 'tt2>) -> CssResult<'i, R>
{
    between(parser, 0, std::u32::MAX, f)
}*/

// <value>+
// 1 or more times.
pub fn one_or_more<'i, 'tt, F, R>(parser: &mut Parser<'i, 'tt>, f: F) -> CssResult<'i, Vec<R>>
    where F: for<'tt2> Fn(&mut Parser<'i, 'tt2>) -> CssResult<'i, R>
{
    between(parser, 1, std::u32::MAX, f)
}

// <value>{A,B}
// At least A times, at most B times.
pub fn between<'i, 'tt, F, R>(parser: &mut Parser<'i, 'tt>, min: u32, max: u32, f: F) -> CssResult<'i, Vec<R>>
    where F: for<'tt2> Fn(&mut Parser<'i, 'tt2>) -> CssResult<'i, R>
{
    let mut values = Vec::new();
    while let Ok(value) = parser.try(&f) {
        if values.len() >= max as usize {
            return Ok(values);
        }
        values.push(value);
    }

    if values.len() < min as usize {
        error("not enough values")
    } else {
        Ok(values)
    }
}

// <value> && <value> && ...
// Components are mandatory but may appear in any order.
macro_rules! impl_double_ampersand {
    ($name:ident, $n:expr, $(($F:ident, $R:ident)),*) => {
        #[allow(non_snake_case)]
        pub fn $name<'i, 'tt, $($F, $R,)*>(parser: &mut Parser<'i, 'tt>, $($F: $F,)*) -> CssResult<'i, ($($R,)*)>
          where
            $($F: for<'tt2> Fn(&mut Parser<'i, 'tt2>) -> CssResult<'i, $R>,)*
        {
            $(let mut $R = None;)*

            for _ in 0..$n {
                $(if $R.is_none() {
                    if let Ok(r0) = parser.try(&$F) {
                        $R = Some(r0);
                        continue;
                    }
                })*

                return error("unexpected token");
            }

            Ok(($($R.unwrap(),)*))
        }
    }
}

impl_double_ampersand!(all_of_2, 2, (FA, RA), (FB, RB));
impl_double_ampersand!(all_of_3, 3, (FA, RA), (FB, RB), (FC, RC));
//impl_double_ampersand!(all_of_4, 4, (FA, RA), (FB, RB), (FC, RC), (FD, RD));
//impl_double_ampersand!(all_of_5, 5, (FA, RA), (FB, RB), (FC, RC), (FD, RD), (FE, RE));

// <value> || <value> || ...
// At least one of the components must be present, and they may appear in any order.
macro_rules! impl_double_bar {
    ($name:ident, $n:expr, $(($F:ident, $R:ident)),*) => {
        #[allow(non_snake_case)]
        pub fn $name<'i, 'tt, $($F, $R,)*>(parser: &mut Parser<'i, 'tt>, $($F: $F,)*) -> CssResult<'i, ($(Option<$R>,)*)>
          where
            $($F: for<'tt2> Fn(&mut Parser<'i, 'tt2>) -> CssResult<'i, $R>,)*
        {
            $(let mut $R = None;)*

            for _ in 0..$n {
                $(if $R.is_none() {
                    if let Ok(value) = parser.try(&$F) {
                        $R = Some(value);
                        continue;
                    }
                })*

                break;
            }
            
            if $($R.is_none()) &&* {
                error("")
            } else {
                Ok(($($R,)*))
            }
        }
    }
}

impl_double_bar!(one_or_more_of_2, 2, (FA, RA), (FB, RB));
impl_double_bar!(one_or_more_of_3, 3, (FA, RA), (FB, RB), (FC, RC));
//impl_double_bar!(one_or_more_of_4, 4, (FA, RA), (FB, RB), (FC, RC), (FD, RD));
//impl_double_bar!(one_or_more_of_5, 5, (FA, RA), (FB, RB), (FC, RC), (FD, RD), (FE, RE));
impl_double_bar!(one_or_more_of_6, 6, (FA, RA), (FB, RB), (FC, RC), (FD, RD), (FE, RE), (FF, RF));

// <value> | <value> | ...
// Exactly one of the components must be present.
macro_rules! impl_single_bar {
    ($name:ident, $n:expr, ($($F:ident),*)) => {
        #[allow(non_snake_case)]
        pub fn $name<'i, 'tt, R, $($F),*>(parser: &mut Parser<'i, 'tt>, $($F: $F,)*) -> CssResult<'i, R>
          where
            $($F: for<'tt2> Fn(&mut Parser<'i, 'tt2>) -> CssResult<'i, R>,)*
        {
            $(if let Ok(value) = parser.try($F) {
                return Ok(value);
            })*

            error("")
        }
    }
}

impl_single_bar!(one_of_2, 2, (FA, FB));
impl_single_bar!(one_of_3, 3, (FA, FB, FC));
//impl_single_bar!(one_of_4, 4, (FA, FB, FC, FD));
impl_single_bar!(one_of_5, 5, (FA, FB, FC, FD, FE));

#[macro_export]
macro_rules! css_enum {
    ($name:ident, $($rust_name:ident = $css_name:expr),*) => {
        #[derive(Copy, Clone, Eq, PartialEq)]
        enum $name {
            $($rust_name),*
        }

        impl $name {
            pub fn parse<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> CssResult<'i, $name> {
                let ident = parser.expect_ident()?;
                Ok(match_ignore_ascii_case! { ident.as_ref(),
                    $($css_name => $name::$rust_name,)*
                    _ => return error(""),
                })
            }
        }
    }
}
