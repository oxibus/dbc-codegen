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

/// Check whether `s` is usable as a Rust identifier (const or field name).
/// `syn` accepts exactly valid, non-keyword identifiers, so it also rejects
/// keywords, the bare wildcard `_`, and anything not identifier-shaped.
pub fn is_valid_ident(s: &str) -> bool {
    syn::parse_str::<syn::Ident>(s).is_ok()
}

/// Check whether `s` is valid with no lowercase letters.
pub fn is_screaming_snake_case(s: &str) -> bool {
    is_valid_ident(s) && !s.bytes().any(|b| b.is_ascii_lowercase())
}

/// Check whether `s` is a usable Rust type path, e.g. `crate::Foo`.
/// `syn` validates the whole path (including generics), while we
/// reject a qualified self-type (`<T as Trait>::X`) and a leading `::`,
/// since neither makes sense as an emitted constant's type.
pub fn is_valid_type_path(s: &str) -> bool {
    matches!(
        syn::parse_str::<syn::TypePath>(s),
        Ok(tp) if tp.qself.is_none() && tp.path.leading_colon.is_none()
    )
}

#[cfg(test)]
mod tests {
    use super::{is_screaming_snake_case, is_valid_ident, is_valid_type_path};

    #[test]
    fn idents() {
        assert!(is_valid_ident("data_id"));
        assert!(is_valid_ident("E2E"));
        assert!(!is_valid_ident("1bad"));
        assert!(!is_valid_ident("_"));
        assert!(!is_valid_ident("type")); // keyword
        assert!(!is_valid_ident("with-dash"));
    }

    #[test]
    fn screaming_snake_case() {
        assert!(is_screaming_snake_case("E2E_DATA_ID"));
        assert!(!is_screaming_snake_case("mixedCase"));
        assert!(!is_screaming_snake_case("new"));
    }

    #[test]
    fn type_paths() {
        assert!(is_valid_type_path("data_protection::E2EDataIdInfo"));
        assert!(is_valid_type_path("crate::Foo"));
        assert!(is_valid_type_path("my::Wrapper<u8>")); // generics now accepted
        assert!(!is_valid_type_path("1bad::Type")); // digit-leading segment
        assert!(!is_valid_type_path("")); // empty
        assert!(!is_valid_type_path("::foo")); // leading `::`
        assert!(!is_valid_type_path("<T as Trait>::X")); // qualified self-type
    }
}
