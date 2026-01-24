use std::cmp::{max, min};

use can_dbc::ValueType::{Signed, Unsigned};
use can_dbc::{Signal, ValueType};

/// Determine the smallest rust integer that can fit the actual signal values,
/// i.e. accounting for factor and offset.
///
/// NOTE: Factor and offset must be whole integers.
pub(crate) fn ty_from_scaled_signal(signal: &Signal) -> String {
    assert!(
        signal.factor.fract().abs() <= f64::EPSILON,
        "Signal Factor ({}) should be an integer",
        signal.factor,
    );
    assert!(
        signal.offset.fract().abs() <= f64::EPSILON,
        "Signal Offset ({}) should be an integer",
        signal.offset,
    );

    ty_from_signal_range(signal).unwrap_or_else(|| {
        panic!(
            "Signal {} could not be represented as a Rust integer",
            signal.name,
        );
    })
}

/// Convert the relevant parameters of a [`Signal`] into a Rust type.
fn ty_from_signal_range(signal: &Signal) -> Option<String> {
    if signal.size > 64 {
        return None;
    }
    get_signal_value_range(signal).map(|(low, high)| ty_from_range(low, high))
}

/// Using the signal's parameters, find the range of values that it spans.
fn get_signal_value_range(signal: &Signal) -> Option<(i128, i128)> {
    let sign: ValueType = signal.value_type;
    let signal_size: u32 = signal.size as u32;
    let factor: i64 = signal.factor as i64;
    let offset: i64 = signal.offset as i64;

    if signal_size == 0 {
        return None;
    }
    let (low, high) = match sign {
        Signed => (
            1i128
                .checked_shl(signal_size.saturating_sub(1))
                .and_then(|n| n.checked_mul(-1)),
            1i128
                .checked_shl(signal_size.saturating_sub(1))
                .and_then(|n| n.checked_sub(1)),
        ),
        Unsigned => (
            Some(0),
            1i128
                .checked_shl(signal_size)
                .and_then(|n| n.checked_sub(1)),
        ),
    };
    let range1 = apply_factor_and_offset(low, factor, offset);
    let range2 = apply_factor_and_offset(high, factor, offset);
    match (range1, range2) {
        (Some(a), Some(b)) => Some((min(a, b), max(a, b))),
        _ => None,
    }
}

fn apply_factor_and_offset(input: Option<i128>, factor: i64, offset: i64) -> Option<i128> {
    input
        .and_then(|n| n.checked_mul(factor.into()))
        .and_then(|n| n.checked_add(offset.into()))
}

/// Determine the smallest Rust integer type that can fit the range of values
/// Only values derived from 64-bit integers are supported, i.e. the range [-2^64-1, 2^64-1]
fn ty_from_range(low: i128, high: i128) -> String {
    let lower_bound: u8;
    let upper_bound: u8;
    let sign: &str;

    if low < 0 {
        // signed case
        sign = "i";
        lower_bound = match low {
            n if n >= i8::MIN.into() => 8,
            n if n >= i16::MIN.into() => 16,
            n if n >= i32::MIN.into() => 32,
            n if n >= i64::MIN.into() => 64,
            _ => 128,
        };
        upper_bound = match high {
            n if n <= i8::MAX.into() => 8,
            n if n <= i16::MAX.into() => 16,
            n if n <= i32::MAX.into() => 32,
            n if n <= i64::MAX.into() => 64,
            _ => 128,
        };
    } else {
        sign = "u";
        lower_bound = 8;
        upper_bound = match high {
            n if n <= u8::MAX.into() => 8,
            n if n <= u16::MAX.into() => 16,
            n if n <= u32::MAX.into() => 32,
            n if n <= u64::MAX.into() => 64,
            _ => 128,
        };
    }

    let size = max(lower_bound, upper_bound);
    format!("{sign}{size}")
}

/// Bit-width suffix for Rust int types (8, 16, 32, 64).
fn signal_bit_size_suffix(size: u32) -> &'static str {
    match size {
        n if n <= 8 => "8",
        n if n <= 16 => "16",
        n if n <= 32 => "32",
        _ => "64",
    }
}

/// Determine the smallest rust integer that can fit the raw signal values.
pub(crate) fn ty_from_signal_int(signal: &Signal) -> String {
    let sign = match signal.value_type {
        Signed => "i",
        Unsigned => "u",
    };
    format!("{sign}{}", signal_bit_size_suffix(signal.size as u32))
}

/// Determine the smallest unsigned rust integer with no fewer bits than the signal.
pub(crate) fn ty_from_signal_uint(signal: &Signal) -> String {
    format!("u{}", signal_bit_size_suffix(signal.size as u32))
}

#[allow(clippy::float_cmp)]
pub(crate) fn is_float_signal(signal: &Signal) -> bool {
    signal.offset.fract() != 0.0 || signal.factor.fract() != 0.0
}

/// Get the Rust type for a signal
pub(crate) fn ty_from_signal(signal: &Signal) -> String {
    if signal.size == 1 {
        String::from("bool")
    } else if is_float_signal(signal) {
        // If there is any scaling needed, go for float
        String::from("f32")
    } else {
        ty_from_scaled_signal(signal)
    }
}

#[cfg(test)]
mod tests {
    use can_dbc::{Signal, ValueType};

    use super::*;

    fn signal(sign: ValueType, signal_size: u32, factor: i64, offset: i64) -> Signal {
        Signal {
            name: String::new(),
            start_bit: 0,
            size: u64::from(signal_size),
            byte_order: can_dbc::ByteOrder::LittleEndian,
            value_type: sign,
            factor: factor as f64,
            offset: offset as f64,
            min: 0.0,
            max: 0.0,
            unit: String::new(),
            receivers: vec![],
            multiplexer_indicator: can_dbc::MultiplexIndicator::Plain,
        }
    }

    #[test]
    fn test_get_signal_value_range() {
        assert_eq!(
            get_signal_value_range(&signal(Unsigned, 4, 1, 0)),
            Some((0, 15))
        );
        assert_eq!(
            get_signal_value_range(&signal(Unsigned, 32, -1, 0)),
            Some((-i128::from(u32::MAX), 0))
        );
        assert_eq!(
            get_signal_value_range(&signal(Unsigned, 12, 1, -1000)),
            Some((-1000, 3095))
        );
        assert_eq!(
            get_signal_value_range(&signal(Signed, 0, 1, 0)),
            None,
            "0 bit signal should be invalid",
        );
    }

    #[test]
    fn test_ty_from_range() {
        assert_eq!(ty_from_range(0, 255), "u8");
        assert_eq!(ty_from_range(-1, 127), "i8");
        assert_eq!(ty_from_range(-1, 128), "i16");
        assert_eq!(ty_from_range(-1, 255), "i16");
        assert_eq!(ty_from_range(-65535, 0), "i32");
        assert_eq!(ty_from_range(-129, -127), "i16");
        assert_eq!(ty_from_range(0, 1i128 << 65), "u128");
        assert_eq!(ty_from_range(-(1i128 << 65), 0), "i128");
    }

    #[test]
    fn test_ty_from_signal_range() {
        let typ = ty_from_signal_range(&signal(Signed, 8, 1, 0)).unwrap();
        assert_eq!(typ, "i8");
        let typ = ty_from_signal_range(&signal(Signed, 8, 2, 0)).unwrap();
        assert_eq!(typ, "i16");
        let typ = ty_from_signal_range(&signal(Signed, 63, 1, 0)).unwrap();
        assert_eq!(typ, "i64");
        let typ = ty_from_signal_range(&signal(Unsigned, 64, -1, 0)).unwrap();
        assert_eq!(typ, "i128");
        let typ = ty_from_signal_range(&signal(Unsigned, 65, 1, 0));
        assert_eq!(typ, None, "Not valid DBC, it's more than 64 bits");
    }
}
