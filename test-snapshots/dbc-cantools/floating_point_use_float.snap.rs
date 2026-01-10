// Generated code!
//
// Message definitions from file `floating_point_use_float`
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
    /// Message1
    Message1(Message1),
    /// Message2
    Message2(Message2),
    /// Message3
    Message3(Message3),
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
            Message1::MESSAGE_ID => Messages::Message1(Message1::try_from(payload)?),
            Message2::MESSAGE_ID => Messages::Message2(Message2::try_from(payload)?),
            Message3::MESSAGE_ID => Messages::Message3(Message3::try_from(payload)?),
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}

/// Message1
///
/// - Standard ID: 1024 (0x400)
/// - Size: 8 bytes
/// - Transmitter: TestNode
#[derive(Clone, Copy)]
pub struct Message1 {
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
impl Message1 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x400)});
    
    pub const SIGNAL1_MIN: i64 = 0_i64;
    pub const SIGNAL1_MAX: i64 = 0_i64;
    
    /// Construct new Message1 from values
    pub fn new(signal1: i64) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_signal1(signal1)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// Signal1
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn signal1(&self) -> i64 {
        self.signal1_raw()
    }
    
    /// Get raw value of Signal1
    ///
    /// - Start bit: 0
    /// - Signal size: 64 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn signal1_raw(&self) -> i64 {
        let signal = self.raw.view_bits::<Lsb0>()[0..64].load_le::<i64>();
        
        let factor = 1;
        let signal = signal as i64;
        i64::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Signal1
    #[inline(always)]
    pub fn set_signal1(&mut self, value: i64) -> Result<(), CanError> {
        if value < 0_i64 || 0_i64 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Message1::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Message1::MESSAGE_ID })?;
        let value = (value / factor) as i64;
        
        let value = u64::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Message1 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Message1 {
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

/// Message2
///
/// - Standard ID: 1025 (0x401)
/// - Size: 8 bytes
/// - Transmitter: TestNode
#[derive(Clone, Copy)]
pub struct Message2 {
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
impl Message2 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x401)});
    
    pub const SIGNAL2_MIN: i32 = 0_i32;
    pub const SIGNAL2_MAX: i32 = 0_i32;
    pub const SIGNAL1_MIN: i32 = 0_i32;
    pub const SIGNAL1_MAX: i32 = 0_i32;
    
    /// Construct new Message2 from values
    pub fn new(signal2: i32, signal1: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_signal2(signal2)?;
        res.set_signal1(signal1)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// Signal2
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn signal2(&self) -> i32 {
        self.signal2_raw()
    }
    
    /// Get raw value of Signal2
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn signal2_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<i32>();
        
        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Signal2
    #[inline(always)]
    pub fn set_signal2(&mut self, value: i32) -> Result<(), CanError> {
        if value < 0_i32 || 0_i32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Message2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Message2::MESSAGE_ID })?;
        let value = (value / factor) as i32;
        
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
    
    /// Signal1
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn signal1(&self) -> i32 {
        self.signal1_raw()
    }
    
    /// Get raw value of Signal1
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn signal1_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<i32>();
        
        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Signal1
    #[inline(always)]
    pub fn set_signal1(&mut self, value: i32) -> Result<(), CanError> {
        if value < 0_i32 || 0_i32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Message2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Message2::MESSAGE_ID })?;
        let value = (value / factor) as i32;
        
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Message2 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Message2 {
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

/// Message3
///
/// - Standard ID: 1026 (0x402)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Message3 {
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
impl Message3 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x402)});
    
    pub const SIGNAL3_MIN: f32 = 0_f32;
    pub const SIGNAL3_MAX: f32 = 0_f32;
    
    /// Construct new Message3 from values
    pub fn new(signal3: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_signal3(signal3)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// Signal3
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn signal3(&self) -> f32 {
        self.signal3_raw()
    }
    
    /// Get raw value of Signal3
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 0.001
    /// - Offset: -0.125
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn signal3_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        let factor = 0.001_f32;
        let offset = -0.125_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of Signal3
    #[inline(always)]
    pub fn set_signal3(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Message3::MESSAGE_ID });
        }
        let factor = 0.001_f32;
        let offset = -0.125_f32;
        let value = ((value - offset) / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Message3 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Message3 {
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

