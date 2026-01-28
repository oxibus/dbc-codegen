use anyhow::{ensure, Result};
use can_dbc::MultiplexIndicator::Multiplexor;
use can_dbc::{Message, Signal};
use heck::{ToPascalCase, ToSnakeCase};

use crate::keywords;

pub fn enum_name(msg: &Message, signal: &Signal) -> String {
    // this turns signal `_4DRIVE` into `4drive`
    let signal_name = signal
        .name
        .trim_start_matches(|c: char| c.is_ascii_punctuation())
        .to_pascal_case();
    let msg_name = enum_variant_name(&msg.name);

    format!("{msg_name}{signal_name}")
}

pub fn multiplexed_enum_variant_wrapper_name(switch_index: u64) -> String {
    format!("M{switch_index}")
}

pub fn multiplex_enum_name(msg: &Message, multiplexor: &Signal) -> Result<String> {
    ensure!(
        matches!(multiplexor.multiplexer_indicator, Multiplexor),
        "signal {multiplexor:?} is not the multiplexor",
    );
    Ok(format!(
        "{}{}Index",
        msg.name.to_pascal_case(),
        multiplexor.name.to_pascal_case(),
    ))
}

pub fn multiplexed_enum_variant_name(
    msg: &Message,
    multiplexor: &Signal,
    switch_index: u64,
) -> Result<String> {
    ensure!(
        matches!(multiplexor.multiplexer_indicator, Multiplexor),
        "signal {multiplexor:?} is not the multiplexor",
    );

    Ok(format!(
        "{}{}M{switch_index}",
        msg.name.to_pascal_case(),
        multiplexor.name.to_pascal_case(),
    ))
}

pub trait SignalExt {
    fn field_name(&self) -> String;
}

impl SignalExt for Signal {
    fn field_name(&self) -> String {
        sanitize_name(&self.name, "x", ToSnakeCase::to_snake_case)
    }
}

pub trait MessageExt {
    fn type_name(&self) -> String;
}

impl MessageExt for Message {
    fn type_name(&self) -> String {
        sanitize_name(&self.name, "X", ToPascalCase::to_pascal_case)
    }
}

pub fn sanitize_name(x: &str, prefix: &str, to_case: fn(&str) -> String) -> String {
    if keywords::is_keyword(x) || !x.starts_with(|c: char| c.is_ascii_alphabetic()) {
        format!("{prefix}{}", to_case(x))
    } else {
        to_case(x)
    }
}

pub fn type_name(x: &str) -> String {
    sanitize_name(x, "X", ToPascalCase::to_pascal_case)
}

pub fn enum_variant_name(x: &str) -> String {
    type_name(x) // enum variant and type encoding are identical
}

/// Check if a floating point value is an integer
pub fn is_integer(val: f64) -> bool {
    val.fract().abs() < f64::EPSILON
}
