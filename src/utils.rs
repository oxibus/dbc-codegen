use can_dbc::{Message, Signal};
use heck::{ToPascalCase, ToSnakeCase};

use crate::keywords;

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
