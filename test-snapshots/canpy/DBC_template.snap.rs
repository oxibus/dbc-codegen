// Generated code!
//
// Message definitions from file `DBC_template`
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
    /// CANMultiplexed
    CanMultiplexed(CanMultiplexed),
    /// CANMessage
    CanMessage(CanMessage),
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
            CanMultiplexed::MESSAGE_ID => Messages::CanMultiplexed(CanMultiplexed::try_from(payload)?),
            CanMessage::MESSAGE_ID => Messages::CanMessage(CanMessage::try_from(payload)?),
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}

/// CANMultiplexed
///
/// - Extended ID: 4321 (0x10e1)
/// - Size: 2 bytes
/// - Transmitter: Node0
///
/// Multiplexed CAN-Message
#[derive(Clone, Copy)]
pub struct CanMultiplexed {
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
impl CanMultiplexed {
    pub const MESSAGE_ID: embedded_can::Id = Id::Extended(unsafe { ExtendedId::new_unchecked(0x10e1)});
    
    pub const VALUE1_MIN: u8 = 0_u8;
    pub const VALUE1_MAX: u8 = 0_u8;
    pub const VALUE0_MIN: u8 = 0_u8;
    pub const VALUE0_MAX: u8 = 0_u8;
    pub const MULTIPLEXER_MIN: u8 = 0_u8;
    pub const MULTIPLEXER_MAX: u8 = 0_u8;
    
    /// Construct new CANMultiplexed from values
    pub fn new(multiplexer: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 2] };
        res.set_multiplexer(multiplexer)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 2] {
        &self.raw
    }
    
    /// Get raw value of Multiplexer
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn multiplexer_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    pub fn multiplexer(&mut self) -> Result<CanMultiplexedMultiplexerIndex, CanError> {
        match self.multiplexer_raw() {
            0 => Ok(CanMultiplexedMultiplexerIndex::M0(CanMultiplexedMultiplexerM0{ raw: self.raw })),
            1 => Ok(CanMultiplexedMultiplexerIndex::M1(CanMultiplexedMultiplexerM1{ raw: self.raw })),
            multiplexor => Err(CanError::InvalidMultiplexor { message_id: CanMultiplexed::MESSAGE_ID, multiplexor: multiplexor.into() }),
        }
    }
    /// Set value of Multiplexer
    #[inline(always)]
    fn set_multiplexer(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: CanMultiplexed::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: CanMultiplexed::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
    /// Set value of Multiplexer
    #[inline(always)]
    pub fn set_m0(&mut self, value: CanMultiplexedMultiplexerM0) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_multiplexer(0)?;
        Ok(())
    }
    
    /// Set value of Multiplexer
    #[inline(always)]
    pub fn set_m1(&mut self, value: CanMultiplexedMultiplexerM1) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_multiplexer(1)?;
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for CanMultiplexed {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 2 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 2];
        raw.copy_from_slice(&payload[..2]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for CanMultiplexed {
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
/// Defined values for Value1
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
pub enum CanMultiplexedValue1 {
    Three,
    Two,
    One,
    Zero,
    _Other(u8),
}

impl From<CanMultiplexedValue1> for u8 {
    fn from(val: CanMultiplexedValue1) -> u8 {
        match val {
            CanMultiplexedValue1::Three => 3,
            CanMultiplexedValue1::Two => 2,
            CanMultiplexedValue1::One => 1,
            CanMultiplexedValue1::Zero => 0,
            CanMultiplexedValue1::_Other(x) => x,
        }
    }
}

/// Defined values for Value0
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
pub enum CanMultiplexedValue0 {
    Value2,
    Value1,
    Value0,
    _Other(u8),
}

impl From<CanMultiplexedValue0> for u8 {
    fn from(val: CanMultiplexedValue0) -> u8 {
        match val {
            CanMultiplexedValue0::Value2 => 2,
            CanMultiplexedValue0::Value1 => 1,
            CanMultiplexedValue0::Value0 => 0,
            CanMultiplexedValue0::_Other(x) => x,
        }
    }
}

/// Defined values for multiplexed signal CANMultiplexed
#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
pub enum CanMultiplexedMultiplexerIndex {
    M0(CanMultiplexedMultiplexerM0),
    M1(CanMultiplexedMultiplexerM1),
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
#[derive(Default)]
pub struct CanMultiplexedMultiplexerM0 { raw: [u8; 2] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl CanMultiplexedMultiplexerM0 {
pub fn new() -> Self { Self { raw: [0u8; 2] } }
/// Value0
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Node0
#[inline(always)]
pub fn value0(&self) -> CanMultiplexedValue0 {
    let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
    
    match signal {
        2 => CanMultiplexedValue0::Value2,
        1 => CanMultiplexedValue0::Value1,
        0 => CanMultiplexedValue0::Value0,
        _ => CanMultiplexedValue0::_Other(self.value0_raw()),
    }
}

/// Get raw value of Value0
///
/// - Start bit: 8
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn value0_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Value0
#[inline(always)]
pub fn set_value0(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 0_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CanMultiplexed::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CanMultiplexed::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
    Ok(())
}

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
#[derive(Default)]
pub struct CanMultiplexedMultiplexerM1 { raw: [u8; 2] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl CanMultiplexedMultiplexerM1 {
pub fn new() -> Self { Self { raw: [0u8; 2] } }
/// Value1
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Node1
#[inline(always)]
pub fn value1(&self) -> CanMultiplexedValue1 {
    let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
    
    match signal {
        3 => CanMultiplexedValue1::Three,
        2 => CanMultiplexedValue1::Two,
        1 => CanMultiplexedValue1::One,
        0 => CanMultiplexedValue1::Zero,
        _ => CanMultiplexedValue1::_Other(self.value1_raw()),
    }
}

/// Get raw value of Value1
///
/// - Start bit: 8
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn value1_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Value1
#[inline(always)]
pub fn set_value1(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 0_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CanMultiplexed::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CanMultiplexed::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
    Ok(())
}

}


/// CANMessage
///
/// - Standard ID: 1234 (0x4d2)
/// - Size: 8 bytes
/// - Transmitter: Node0
#[derive(Clone, Copy)]
pub struct CanMessage {
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
impl CanMessage {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x4d2)});
    
    pub const SIGNAL1_MIN: u64 = 0_u64;
    pub const SIGNAL1_MAX: u64 = 100_u64;
    pub const SIGNAL0_MIN: i32 = 0_i32;
    pub const SIGNAL0_MAX: i32 = 0_i32;
    
    /// Construct new CANMessage from values
    pub fn new(signal1: u64, signal0: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_signal1(signal1)?;
        res.set_signal0(signal0)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// Signal1
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "%"
    /// - Receivers: Node1, Node2
    #[inline(always)]
    pub fn signal1(&self) -> u64 {
        self.signal1_raw()
    }
    
    /// Get raw value of Signal1
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 100
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn signal1_raw(&self) -> u64 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<u32>();
        
        let factor = 100;
        u64::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Signal1
    #[inline(always)]
    pub fn set_signal1(&mut self, value: u64) -> Result<(), CanError> {
        if value < 0_u64 || 100_u64 < value {
            return Err(CanError::ParameterOutOfRange { message_id: CanMessage::MESSAGE_ID });
        }
        let factor = 100;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: CanMessage::MESSAGE_ID })?;
        let value = (value / factor) as u32;
        
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
    
    /// Signal0
    ///
    /// First signal in this message
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Node1, Node2
    #[inline(always)]
    pub fn signal0(&self) -> i32 {
        self.signal0_raw()
    }
    
    /// Get raw value of Signal0
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn signal0_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<i32>();
        
        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Signal0
    #[inline(always)]
    pub fn set_signal0(&mut self, value: i32) -> Result<(), CanError> {
        if value < 0_i32 || 0_i32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: CanMessage::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: CanMessage::MESSAGE_ID })?;
        let value = (value / factor) as i32;
        
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for CanMessage {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for CanMessage {
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

