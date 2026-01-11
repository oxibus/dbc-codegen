// Generated code!
//
// Message definitions from file `motohawk_fd`
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
/// - Extended ID: 496 (0x1f0)
/// - Size: 8 bytes
/// - Transmitter: PCM1
///
/// Example message used as template in MotoHawk models.
#[derive(Clone, Copy)]
pub struct ExampleMessage {
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
impl ExampleMessage {
    pub const MESSAGE_ID: embedded_can::Id = Id::Extended(unsafe { ExtendedId::new_unchecked(0x1f0)});
    
    pub const TEMPERATURE_MIN: f32 = 229.52_f32;
    pub const TEMPERATURE_MAX: f32 = 270.47_f32;
    pub const AVERAGE_RADIUS_MIN: f32 = 0_f32;
    pub const AVERAGE_RADIUS_MAX: f32 = 5_f32;
    
    /// Construct new ExampleMessage from values
    pub fn new(temperature: f32, average_radius: f32, enable: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_temperature(temperature)?;
        res.set_average_radius(average_radius)?;
        res.set_enable(enable)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// Temperature
    ///
    /// - Min: 229.52
    /// - Max: 270.47
    /// - Unit: "degK"
    /// - Receivers: PCM1, FOO
    #[inline(always)]
    pub fn temperature(&self) -> f32 {
        self.temperature_raw()
    }
    
    /// Get raw value of Temperature
    ///
    /// - Start bit: 0
    /// - Signal size: 12 bits
    /// - Factor: 0.01
    /// - Offset: 250
    /// - Byte order: BigEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn temperature_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Msb0>()[7..19].load_be::<i16>();
        
        let factor = 0.01_f32;
        let offset = 250_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of Temperature
    #[inline(always)]
    pub fn set_temperature(&mut self, value: f32) -> Result<(), CanError> {
        if value < 229.52_f32 || 270.47_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: ExampleMessage::MESSAGE_ID });
        }
        let factor = 0.01_f32;
        let offset = 250_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[7..19].store_be(value);
        Ok(())
    }
    
    /// AverageRadius
    ///
    /// - Min: 0
    /// - Max: 5
    /// - Unit: "m"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn average_radius(&self) -> f32 {
        self.average_radius_raw()
    }
    
    /// Get raw value of AverageRadius
    ///
    /// - Start bit: 6
    /// - Signal size: 6 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn average_radius_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Msb0>()[1..7].load_be::<u8>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of AverageRadius
    #[inline(always)]
    pub fn set_average_radius(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 5_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: ExampleMessage::MESSAGE_ID });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u8;
        
        self.raw.view_bits_mut::<Msb0>()[1..7].store_be(value);
        Ok(())
    }
    
    /// Enable
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "-"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn enable(&self) -> ExampleMessageEnable {
        let signal = self.raw.view_bits::<Msb0>()[0..1].load_be::<u8>();
        
        match signal {
            0 => ExampleMessageEnable::Disabled,
            1 => ExampleMessageEnable::Enabled,
            _ => ExampleMessageEnable::_Other(self.enable_raw()),
        }
    }
    
    /// Get raw value of Enable
    ///
    /// - Start bit: 7
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn enable_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[0..1].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of Enable
    #[inline(always)]
    pub fn set_enable(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[0..1].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for ExampleMessage {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
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
/// Defined values for Enable
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
pub enum ExampleMessageEnable {
    Disabled,
    Enabled,
    _Other(bool),
}

impl From<ExampleMessageEnable> for bool {
    fn from(val: ExampleMessageEnable) -> bool {
        match val {
            ExampleMessageEnable::Disabled => false,
            ExampleMessageEnable::Enabled => true,
            ExampleMessageEnable::_Other(x) => x,
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

