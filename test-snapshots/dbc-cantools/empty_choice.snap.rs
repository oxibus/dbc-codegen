// Generated code!
//
// Message definitions from file `empty_choice`
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
    /// example_message
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

/// example_message
///
/// - Standard ID: 10 (0xa)
/// - Size: 3 bytes
/// - Transmitter: tx_node
#[derive(Clone, Copy)]
pub struct ExampleMessage {
    raw: [u8; 3],
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
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0xa)});
    
    pub const NO_CHOICE_MIN: i8 = 0_i8;
    pub const NO_CHOICE_MAX: i8 = 0_i8;
    pub const EMPTY_CHOICE_MIN: i8 = 0_i8;
    pub const EMPTY_CHOICE_MAX: i8 = 0_i8;
    pub const NON_EMPTY_CHOICE_MIN: i8 = 0_i8;
    pub const NON_EMPTY_CHOICE_MAX: i8 = 0_i8;
    
    /// Construct new example_message from values
    pub fn new(no_choice: i8, empty_choice: i8, non_empty_choice: i8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 3] };
        res.set_no_choice(no_choice)?;
        res.set_empty_choice(empty_choice)?;
        res.set_non_empty_choice(non_empty_choice)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 3] {
        &self.raw
    }
    
    /// no_choice
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn no_choice(&self) -> i8 {
        self.no_choice_raw()
    }
    
    /// Get raw value of no_choice
    ///
    /// - Start bit: 16
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn no_choice_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[16..24].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of no_choice
    #[inline(always)]
    pub fn set_no_choice(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: ExampleMessage::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: ExampleMessage::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[16..24].store_le(value);
        Ok(())
    }
    
    /// empty_choice
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn empty_choice(&self) -> ExampleMessageEmptyChoice {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
        
        match signal {
            _ => ExampleMessageEmptyChoice::_Other(self.empty_choice_raw()),
        }
    }
    
    /// Get raw value of empty_choice
    ///
    /// - Start bit: 8
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn empty_choice_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of empty_choice
    #[inline(always)]
    pub fn set_empty_choice(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: ExampleMessage::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: ExampleMessage::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
        Ok(())
    }
    
    /// non_empty_choice
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn non_empty_choice(&self) -> ExampleMessageNonEmptyChoice {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        match signal {
            255 => ExampleMessageNonEmptyChoice::NotAvailable,
            254 => ExampleMessageNonEmptyChoice::Error,
            _ => ExampleMessageNonEmptyChoice::_Other(self.non_empty_choice_raw()),
        }
    }
    
    /// Get raw value of non_empty_choice
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn non_empty_choice_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of non_empty_choice
    #[inline(always)]
    pub fn set_non_empty_choice(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: ExampleMessage::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: ExampleMessage::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for ExampleMessage {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 3 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 3];
        raw.copy_from_slice(&payload[..3]);
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
/// Defined values for empty_choice
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
pub enum ExampleMessageEmptyChoice {
    _Other(i8),
}

impl From<ExampleMessageEmptyChoice> for i8 {
    fn from(val: ExampleMessageEmptyChoice) -> i8 {
        match val {
            ExampleMessageEmptyChoice::_Other(x) => x,
        }
    }
}

/// Defined values for non_empty_choice
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
pub enum ExampleMessageNonEmptyChoice {
    NotAvailable,
    Error,
    _Other(i8),
}

impl From<ExampleMessageNonEmptyChoice> for i8 {
    fn from(val: ExampleMessageNonEmptyChoice) -> i8 {
        match val {
            ExampleMessageNonEmptyChoice::NotAvailable => 255,
            ExampleMessageNonEmptyChoice::Error => 254,
            ExampleMessageNonEmptyChoice::_Other(x) => x,
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

