// Generated code!
//
// Message definitions from file `issue_636_negative_scaling`
// Version: 1.0

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
    /// ExampleMessage
    ExampleMessage(ExampleMessage),
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
            ExampleMessage::MESSAGE_ID => Messages::ExampleMessage(ExampleMessage::try_from(payload)?),
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}

/// ExampleMessage
///
/// - Standard ID: 496 (0x1f0)
/// - Size: 2 bytes
/// - Transmitter: PCM1
///
/// Example message
#[derive(Clone, Copy)]
pub struct ExampleMessage {
    raw: [u8; 2],
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
impl ExampleMessage {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x1f0)});
    
    pub const TEMPERATURE_MIN: f32 = 4070_f32;
    pub const TEMPERATURE_MAX: f32 = 4100_f32;
    
    /// Construct new ExampleMessage from values
    pub fn new(temperature: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 2] };
        res.set_temperature(temperature)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 2] {
        &self.raw
    }
    
    /// Temperature
    ///
    /// - Min: 4070
    /// - Max: 4100
    /// - Unit: "degK"
    /// - Receivers: PCM1, FOO
    #[inline(always)]
    pub fn temperature(&self) -> ExampleMessageTemperature {
        let signal = self.raw.view_bits::<Msb0>()[4..16].load_be::<u16>();
        
        match signal {
            4095 => ExampleMessageTemperature::Error,
            4094 => ExampleMessageTemperature::Init,
            _ => ExampleMessageTemperature::_Other(self.temperature_raw()),
        }
    }
    
    /// Get raw value of Temperature
    ///
    /// - Start bit: 3
    /// - Signal size: 12 bits
    /// - Factor: -0.01
    /// - Offset: 4100
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn temperature_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Msb0>()[4..16].load_be::<u16>();
        
        let factor = -0.01_f32;
        let offset = 4100_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of Temperature
    #[inline(always)]
    pub fn set_temperature(&mut self, value: f32) -> Result<(), CanError> {
        if value < 4070_f32 || 4100_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: ExampleMessage::MESSAGE_ID });
        }
        let factor = -0.01_f32;
        let offset = 4100_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Msb0>()[4..16].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for ExampleMessage {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 2 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 2];
        raw.copy_from_slice(&payload[..2]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for ExampleMessage {
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
/// Defined values for Temperature
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
pub enum ExampleMessageTemperature {
    Error,
    Init,
    _Other(f32),
}

impl From<ExampleMessageTemperature> for f32 {
    fn from(val: ExampleMessageTemperature) -> f32 {
        match val {
            ExampleMessageTemperature::Error => 4095_f32,
            ExampleMessageTemperature::Init => 4094_f32,
            ExampleMessageTemperature::_Other(x) => x,
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

