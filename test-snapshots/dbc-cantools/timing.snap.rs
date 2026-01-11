// Generated code!
//
// Message definitions from file `timing`
// Version: 3.0

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
    /// Bar
    Bar(Bar),
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
            Bar::MESSAGE_ID => Messages::Bar(Bar::try_from(payload)?),
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}

/// Foo
///
/// - Standard ID: 1 (0x1)
/// - Size: 8 bytes
/// - Transmitter: Sender
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
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x1)});
    
    pub const FOO_MIN: f32 = 229.53_f32;
    pub const FOO_MAX: f32 = 270.47_f32;
    
    /// Construct new Foo from values
    pub fn new(foo: f32) -> Result<Self, CanError> {
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
    /// - Min: 229.53
    /// - Max: 270.47
    /// - Unit: "degK"
    /// - Receivers: Receiver
    #[inline(always)]
    pub fn foo(&self) -> f32 {
        self.foo_raw()
    }
    
    /// Get raw value of Foo
    ///
    /// - Start bit: 0
    /// - Signal size: 12 bits
    /// - Factor: 0.01
    /// - Offset: 250
    /// - Byte order: BigEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn foo_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Msb0>()[7..19].load_be::<i16>();
        
        let factor = 0.01_f32;
        let offset = 250_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of Foo
    #[inline(always)]
    pub fn set_foo(&mut self, value: f32) -> Result<(), CanError> {
        if value < 229.53_f32 || 270.47_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Foo::MESSAGE_ID });
        }
        let factor = 0.01_f32;
        let offset = 250_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[7..19].store_be(value);
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

/// Bar
///
/// - Standard ID: 2 (0x2)
/// - Size: 8 bytes
/// - Transmitter: Sender
#[derive(Clone, Copy)]
pub struct Bar {
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
impl Bar {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x2)});
    
    pub const FOO_MIN: f32 = 229.53_f32;
    pub const FOO_MAX: f32 = 270.47_f32;
    
    /// Construct new Bar from values
    pub fn new(foo: f32) -> Result<Self, CanError> {
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
    /// - Min: 229.53
    /// - Max: 270.47
    /// - Unit: "degK"
    /// - Receivers: Receiver
    #[inline(always)]
    pub fn foo(&self) -> f32 {
        self.foo_raw()
    }
    
    /// Get raw value of Foo
    ///
    /// - Start bit: 0
    /// - Signal size: 12 bits
    /// - Factor: 0.01
    /// - Offset: 250
    /// - Byte order: BigEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn foo_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Msb0>()[7..19].load_be::<i16>();
        
        let factor = 0.01_f32;
        let offset = 250_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of Foo
    #[inline(always)]
    pub fn set_foo(&mut self, value: f32) -> Result<(), CanError> {
        if value < 229.53_f32 || 270.47_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bar::MESSAGE_ID });
        }
        let factor = 0.01_f32;
        let offset = 250_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[7..19].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Bar {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Bar {
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

