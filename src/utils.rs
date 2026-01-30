use std::fmt::Display;

use anyhow::{anyhow, Result};
use heck::ToPascalCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, IdentFragment};
use syn::{LitFloat, LitInt};

use crate::keywords;

pub fn multiplexed_enum_variant_wrapper_name(switch_index: u64) -> Ident {
    format_ident!("M{switch_index}")
}

pub fn multiplexed_enum_variant_wrapper_setter_name(switch_index: u64) -> Ident {
    format_ident!("set_m{switch_index}")
}

pub fn sanitize_name(x: &str, to_case: fn(&str) -> String) -> String {
    if keywords::is_keyword(x) || !x.starts_with(|c: char| c.is_ascii_alphabetic()) {
        format!("{}{}", to_case("x"), to_case(x))
    } else {
        to_case(x)
    }
}

pub fn enum_variant_name(x: &str) -> String {
    sanitize_name(x, ToPascalCase::to_pascal_case)
}

/// Check if a floating point value is an integer
pub fn is_integer(val: f64) -> bool {
    val.fract().abs() < f64::EPSILON
}

/// A trait to convert a type to a proc-macro Ident
pub trait ToIdent {
    fn ident(&self) -> Ident;
}

impl<T: Display + IdentFragment> ToIdent for T {
    fn ident(&self) -> Ident {
        format_ident!("{self}")
    }
}

/// A trait to convert a type to a proc-macro Ident
pub trait Tokens {
    fn tokens(&self) -> Result<TokenStream>;
}

impl Tokens for &str {
    fn tokens(&self) -> Result<TokenStream> {
        self.parse()
            .map_err(|e| anyhow!("Unable to parse {self}\n{e}"))
    }
}

impl Tokens for String {
    fn tokens(&self) -> Result<TokenStream> {
        self.as_str().tokens()
    }
}

pub fn lit_int<T: Display>(value: T) -> LitInt {
    LitInt::new(&value.to_string(), Span::call_site())
}

pub fn lit_float<T: Display>(value: T) -> LitFloat {
    LitFloat::new(&value.to_string(), Span::call_site())
}
