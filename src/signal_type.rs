use std::cmp::{max, min};
use std::fmt::Display;

use can_dbc::Signal;
use can_dbc::ValueType::{Signed, Unsigned};
use IntSize::{Size128, Size16, Size32, Size64, Size8};
use ValType::{Bool, SignedInt, UnsignedInt, F32};

use crate::utils::is_integer;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum IntSize {
    Size8,
    Size16,
    Size32,
    Size64,
    Size128,
}

impl IntSize {
    fn bits(self) -> u8 {
        match self {
            Size8 => 8,
            Size16 => 16,
            Size32 => 32,
            Size64 => 64,
            Size128 => 128,
        }
    }

    fn from_size(size: u64) -> Self {
        match size {
            n if n <= 8 => Size8,
            n if n <= 16 => Size16,
            n if n <= 32 => Size32,
            _ => Size64,
        }
    }

    fn union(self, other: Self) -> Self {
        if self.bits() >= other.bits() {
            self
        } else {
            other
        }
    }
}

/// Types available in DBC signals
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ValType {
    Bool,
    F32,
    UnsignedInt(IntSize),
    SignedInt(IntSize),
}

impl Display for ValType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bool => write!(f, "bool"),
            UnsignedInt(v) => write!(f, "u{}", v.bits()),
            SignedInt(v) => write!(f, "i{}", v.bits()),
            F32 => write!(f, "f32"),
        }
    }
}

impl ValType {
    pub(crate) fn unsigned_to_signed(self) -> Option<Self> {
        if let UnsignedInt(size) = self {
            Some(SignedInt(size))
        } else {
            None
        }
    }

    /// Convert the relevant parameters of a [`Signal`] into a Rust type.
    fn from_signal_range(signal: &Signal) -> Option<Self> {
        if signal.size > 64 {
            return None;
        }
        get_signal_value_range(signal).map(|(low, high)| Self::from_range(low, high))
    }
}

/// Using the signal's parameters, find the range of values that it spans.
fn get_signal_value_range(signal: &Signal) -> Option<(i128, i128)> {
    let signal_size: u32 = signal.size.try_into().ok()?;
    if signal_size == 0 {
        return None;
    }
    let (low, high) = match signal.value_type {
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

    let factor = signal.factor as i64;
    let offset = signal.offset as i64;
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

impl ValType {
    /// Determine the smallest Rust integer type that can fit the range of values
    /// Only values derived from 64-bit integers are supported, i.e. the range [-2^64-1, 2^64-1]
    fn from_range(low: i128, high: i128) -> Self {
        if low < 0 {
            let lower_bound = match low {
                n if n >= i8::MIN.into() => Size8,
                n if n >= i16::MIN.into() => Size16,
                n if n >= i32::MIN.into() => Size32,
                n if n >= i64::MIN.into() => Size64,
                _ => Size128,
            };
            let upper_bound = match high {
                n if n <= i8::MAX.into() => Size8,
                n if n <= i16::MAX.into() => Size16,
                n if n <= i32::MAX.into() => Size32,
                n if n <= i64::MAX.into() => Size64,
                _ => Size128,
            };
            SignedInt(lower_bound.union(upper_bound))
        } else {
            UnsignedInt(match high {
                n if n <= u8::MAX.into() => Size8,
                n if n <= u16::MAX.into() => Size16,
                n if n <= u32::MAX.into() => Size32,
                n if n <= u64::MAX.into() => Size64,
                _ => Size128,
            })
        }
    }

    /// Determine the smallest rust integer that can fit the raw signal values.
    pub(crate) fn from_signal_int(signal: &Signal) -> Self {
        let size = IntSize::from_size(signal.size);
        if signal.value_type == Signed {
            SignedInt(size)
        } else {
            UnsignedInt(size)
        }
    }

    /// Determine the smallest unsigned rust integer with no fewer bits than the signal.
    pub(crate) fn from_signal_uint(signal: &Signal) -> Self {
        UnsignedInt(IntSize::from_size(signal.size))
    }
}

fn is_float_signal(signal: &Signal) -> bool {
    !is_integer(signal.offset) || !is_integer(signal.factor)
}

impl ValType {
    /// Get the Rust type for a signal
    pub(crate) fn from_signal(signal: &Signal) -> Self {
        if signal.size == 1 {
            Bool
        } else if is_float_signal(signal) {
            // If there is any scaling needed, go for float
            F32
        } else {
            // FIXME: don't panic here
            Self::from_signal_range(signal).unwrap_or_else(|| {
                panic!(
                    "Signal {} could not be represented as a Rust integer",
                    signal.name,
                );
            })
        }
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
    fn test_from_range() {
        assert_eq!(ValType::from_range(0, 255), UnsignedInt(Size8));
        assert_eq!(ValType::from_range(-1, 127), SignedInt(Size8));
        assert_eq!(ValType::from_range(-1, 128), SignedInt(Size16));
        assert_eq!(ValType::from_range(-1, 255), SignedInt(Size16));
        assert_eq!(ValType::from_range(-65535, 0), SignedInt(Size32));
        assert_eq!(ValType::from_range(-129, -127), SignedInt(Size16));
        assert_eq!(ValType::from_range(0, 1i128 << 65), UnsignedInt(Size128));
        assert_eq!(ValType::from_range(-(1i128 << 65), 0), SignedInt(Size128));
    }

    #[test]
    fn test_from_signal_range() {
        let typ = ValType::from_signal_range(&signal(Signed, 8, 1, 0));
        assert_eq!(typ, Some(SignedInt(Size8)));
        let typ = ValType::from_signal_range(&signal(Signed, 8, 2, 0));
        assert_eq!(typ, Some(SignedInt(Size16)));
        let typ = ValType::from_signal_range(&signal(Signed, 63, 1, 0));
        assert_eq!(typ, Some(SignedInt(Size64)));
        let typ = ValType::from_signal_range(&signal(Unsigned, 64, -1, 0));
        assert_eq!(typ, Some(SignedInt(Size128)));
        let typ = ValType::from_signal_range(&signal(Unsigned, 65, 1, 0));
        assert_eq!(typ, None, "Not valid DBC, it's more than 64 bits");
    }
}
