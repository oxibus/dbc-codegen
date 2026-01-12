// Generated code!
//
// Message definitions from file `dump_signal_choices`
// Version: HIPBNYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY/4/%%%/4/'%**4YYY///

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
    /// Message0
    Message0(Message0),
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
            Message0::MESSAGE_ID => Messages::Message0(Message0::try_from(payload)?),
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}

/// Message0
///
/// - Standard ID: 1024 (0x400)
/// - Size: 8 bytes
/// - Transmitter: Node0
#[derive(Clone, Copy)]
pub struct Message0 {
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
impl Message0 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x400)});
    
    pub const FOO_SIGNAL_MIN: u8 = 0_u8;
    pub const FOO_SIGNAL_MAX: u8 = 0_u8;
    pub const BAR_SIGNAL_MIN: u8 = 0_u8;
    pub const BAR_SIGNAL_MAX: u8 = 0_u8;
    
    /// Construct new Message0 from values
    pub fn new(foo_signal: u8, bar_signal: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_foo_signal(foo_signal)?;
        res.set_bar_signal(bar_signal)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// FooSignal
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn foo_signal(&self) -> Message0FooSignal {
        let signal = self.raw.view_bits::<Lsb0>()[0..2].load_le::<u8>();
        
        match signal {
            0 => Message0FooSignal::FooA,
            1 => Message0FooSignal::FooB,
            2 => Message0FooSignal::FooC,
            3 => Message0FooSignal::FooD,
            _ => Message0FooSignal::_Other(self.foo_signal_raw()),
        }
    }
    
    /// Get raw value of FooSignal
    ///
    /// - Start bit: 0
    /// - Signal size: 2 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn foo_signal_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..2].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of FooSignal
    #[inline(always)]
    pub fn set_foo_signal(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Message0::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Message0::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..2].store_le(value);
        Ok(())
    }
    
    /// BarSignal
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn bar_signal(&self) -> Message0BarSignal {
        let signal = self.raw.view_bits::<Lsb0>()[2..5].load_le::<u8>();
        
        match signal {
            0 => Message0BarSignal::BarA,
            1 => Message0BarSignal::BarB,
            2 => Message0BarSignal::BarC,
            3 => Message0BarSignal::BarD,
            4 => Message0BarSignal::BarE,
            5 => Message0BarSignal::BarF,
            6 => Message0BarSignal::BarG,
            7 => Message0BarSignal::BarH,
            _ => Message0BarSignal::_Other(self.bar_signal_raw()),
        }
    }
    
    /// Get raw value of BarSignal
    ///
    /// - Start bit: 2
    /// - Signal size: 3 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn bar_signal_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[2..5].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of BarSignal
    #[inline(always)]
    pub fn set_bar_signal(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Message0::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Message0::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[2..5].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Message0 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Message0 {
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
/// Defined values for FooSignal
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
pub enum Message0FooSignal {
    FooA,
    FooB,
    FooC,
    FooD,
    _Other(u8),
}

impl From<Message0FooSignal> for u8 {
    fn from(val: Message0FooSignal) -> u8 {
        match val {
            Message0FooSignal::FooA => 0,
            Message0FooSignal::FooB => 1,
            Message0FooSignal::FooC => 2,
            Message0FooSignal::FooD => 3,
            Message0FooSignal::_Other(x) => x,
        }
    }
}

/// Defined values for BarSignal
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
pub enum Message0BarSignal {
    BarA,
    BarB,
    BarC,
    BarD,
    BarE,
    BarF,
    BarG,
    BarH,
    _Other(u8),
}

impl From<Message0BarSignal> for u8 {
    fn from(val: Message0BarSignal) -> u8 {
        match val {
            Message0BarSignal::BarA => 0,
            Message0BarSignal::BarB => 1,
            Message0BarSignal::BarC => 2,
            Message0BarSignal::BarD => 3,
            Message0BarSignal::BarE => 4,
            Message0BarSignal::BarF => 5,
            Message0BarSignal::BarG => 6,
            Message0BarSignal::BarH => 7,
            Message0BarSignal::_Other(x) => x,
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

