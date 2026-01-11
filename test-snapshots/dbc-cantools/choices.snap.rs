// Generated code!
//
// Message definitions from file `choices`
// Version: 

#[allow(unused_imports)]
use core::ops::BitOr;
#[allow(unused_imports)]
use bitvec::prelude::*;
#[allow(unused_imports)]
use embedded_can::{Id, StandardId, ExtendedId};

/// All messages
#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
#[derive(Clone)]
pub enum Messages {
    /// Foo
    Foo(Foo),
}

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl Messages {
    /// Read message from CAN frame
    #[inline(never)]
    pub fn from_can_message(id: Id, payload: &[u8]) -> Result<Self, CanError> {
        
        let res = match id {
            Foo::MESSAGE_ID => Messages::Foo(Foo::try_from(payload)?),
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}

/// Foo
///
/// - Standard ID: 0 (0x0)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Foo {
    raw: [u8; 8],
}

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl Foo {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x0)});
    
    pub const FOO_MIN: i8 = -128_i8;
    pub const FOO_MAX: i8 = 127_i8;
    
    /// Construct new Foo from values
    pub fn new(foo: i8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_foo(foo)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// Foo
    ///
    /// - Min: -128
    /// - Max: 127
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn foo(&self) -> FooFoo {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        match signal {
            6 => FooFoo::Reserved,
            5 => FooFoo::Reserved,
            4 => FooFoo::Unused2,
            3 => FooFoo::Unused,
            2 => FooFoo::Unused,
            1 => FooFoo::XÄ,
            0 => FooFoo::WithSpace,
            -5 => FooFoo::ANegativeValue,
            _ => FooFoo::_Other(self.foo_raw()),
        }
    }
    
    /// Get raw value of Foo
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn foo_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Foo
    #[inline(always)]
    pub fn set_foo(&mut self, value: i8) -> Result<(), CanError> {
        if value < -128_i8 || 127_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Foo::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Foo::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Foo {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Foo {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}
/// Defined values for Foo
#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
#[derive(Clone, Copy, PartialEq)]
pub enum FooFoo {
    Reserved,
    Reserved,
    Unused2,
    Unused,
    Unused,
    XÄ,
    WithSpace,
    ANegativeValue,
    _Other(i8),
}

impl From<FooFoo> for i8 {
    fn from(val: FooFoo) -> i8 {
        match val {
            FooFoo::Reserved => 6,
            FooFoo::Reserved => 5,
            FooFoo::Unused2 => 4,
            FooFoo::Unused => 3,
            FooFoo::Unused => 2,
            FooFoo::XÄ => 1,
            FooFoo::WithSpace => 0,
            FooFoo::ANegativeValue => -5,
            FooFoo::_Other(x) => x,
        }
    }
}



/// This is just to make testing easier
#[allow(dead_code)]
fn main() {}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CanError {
    UnknownMessageId(embedded_can::Id),
    /// Signal parameter is not within the range
    /// defined in the dbc
    ParameterOutOfRange {
        /// dbc message id
        message_id: embedded_can::Id,
    },
    InvalidPayloadSize,
    /// Multiplexor value not defined in the dbc
    InvalidMultiplexor {
        /// dbc message id
        message_id: embedded_can::Id,
        /// Multiplexor value not defined in the dbc
        multiplexor: u16,
    },
}

impl core::fmt::Display for CanError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

