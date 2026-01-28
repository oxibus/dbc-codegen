use std::fmt::Display;

use anyhow::{anyhow, ensure, Result};
use can_dbc::MultiplexIndicator::Multiplexor;
use can_dbc::{Message, Signal};
use heck::{ToPascalCase, ToShoutySnakeCase, ToSnakeCase};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, IdentFragment};

use crate::keywords;

pub fn enum_name(msg: &Message, signal: &Signal) -> Ident {
    // this turns signal `_4DRIVE` into `4drive`
    let signal_name = signal
        .name
        .trim_start_matches(|c: char| c.is_ascii_punctuation())
        .to_pascal_case();
    let msg_name = enum_variant_name(&msg.name);

    format_ident!("{msg_name}{signal_name}")
}

pub fn multiplexed_enum_variant_wrapper_name(switch_index: u64) -> Ident {
    format_ident!("M{switch_index}")
}

pub fn multiplexed_enum_variant_wrapper_setter_name(switch_index: u64) -> Ident {
    format_ident!("set_m{switch_index}")
}

pub fn multiplex_enum_name(msg: &Message, multiplexor: &Signal) -> Result<Ident> {
    ensure!(
        matches!(multiplexor.multiplexer_indicator, Multiplexor),
        "signal {multiplexor:?} is not the multiplexor",
    );
    Ok(format!(
        "{}{}Index",
        msg.name.to_pascal_case(),
        multiplexor.name.to_pascal_case(),
    )
    .ident())
}

pub fn multiplexed_enum_variant_name(
    msg: &Message,
    multiplexor: &Signal,
    switch_index: u64,
) -> Result<Ident> {
    ensure!(
        matches!(multiplexor.multiplexer_indicator, Multiplexor),
        "signal {multiplexor:?} is not the multiplexor",
    );

    Ok(format!(
        "{}{}M{switch_index}",
        msg.name.to_pascal_case(),
        multiplexor.name.to_pascal_case(),
    )
    .ident())
}

pub trait SignalExt {
    fn get_name(&self) -> &str;
    fn field_name(&self) -> Ident {
        sanitize_name(self.get_name(), ToSnakeCase::to_snake_case).ident()
    }
    fn field_name2(&self, prefix: &str, suffix: &str) -> Ident {
        format!(
            "{prefix}{}{suffix}",
            sanitize_name(self.get_name(), ToSnakeCase::to_snake_case)
        )
        .ident()
    }
    fn const_name(&self, suffix: &str) -> Ident {
        let tmp: String;
        sanitize_name(
            if suffix.is_empty() {
                self.get_name()
            } else {
                tmp = format!("{}{suffix}", self.get_name());
                &tmp
            },
            ToShoutySnakeCase::to_shouty_snake_case,
        )
        .ident()
    }
}

impl SignalExt for Signal {
    fn get_name(&self) -> &str {
        &self.name
    }
}

pub trait MessageExt {
    fn type_name(&self) -> Ident;
}

impl MessageExt for Message {
    fn type_name(&self) -> Ident {
        sanitize_name(&self.name, ToPascalCase::to_pascal_case).ident()
    }
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
