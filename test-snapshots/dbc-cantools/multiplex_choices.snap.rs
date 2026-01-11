// Generated code!
//
// Message definitions from file `multiplex_choices`
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
/// - Extended ID: 1193046 (0x123456)
/// - Size: 8 bytes
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
    pub const MESSAGE_ID: embedded_can::Id = Id::Extended(unsafe { ExtendedId::new_unchecked(0x123456)});
    
    pub const MULTIPLEXOR_MIN: u8 = 0_u8;
    pub const MULTIPLEXOR_MAX: u8 = 0_u8;
    
    /// Construct new Message1 from values
    pub fn new(multiplexor: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_multiplexor(multiplexor)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// Get raw value of Multiplexor
    ///
    /// - Start bit: 2
    /// - Signal size: 6 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn multiplexor_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[2..8].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    pub fn multiplexor(&mut self) -> Result<Message1MultiplexorIndex, CanError> {
        match self.multiplexor_raw() {
            8 => Ok(Message1MultiplexorIndex::M8(Message1MultiplexorM8{ raw: self.raw })),
            24 => Ok(Message1MultiplexorIndex::M24(Message1MultiplexorM24{ raw: self.raw })),
            multiplexor => Err(CanError::InvalidMultiplexor { message_id: Message1::MESSAGE_ID, multiplexor: multiplexor.into() }),
        }
    }
    /// Set value of Multiplexor
    #[inline(always)]
    fn set_multiplexor(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Message1::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Message1::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[2..8].store_le(value);
        Ok(())
    }
    
    /// Set value of Multiplexor
    #[inline(always)]
    pub fn set_m8(&mut self, value: Message1MultiplexorM8) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_multiplexor(8)?;
        Ok(())
    }
    
    /// Set value of Multiplexor
    #[inline(always)]
    pub fn set_m24(&mut self, value: Message1MultiplexorM24) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_multiplexor(24)?;
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
/// Defined values for BIT_L
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
pub enum Message1BitL {
    Off,
    On,
    _Other(bool),
}

impl From<Message1BitL> for bool {
    fn from(val: Message1BitL) -> bool {
        match val {
            Message1BitL::Off => false,
            Message1BitL::On => true,
            Message1BitL::_Other(x) => x,
        }
    }
}

/// Defined values for Multiplexor
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
pub enum Message1Multiplexor {
    Multiplexor8,
    Multiplexor16,
    Multiplexor24,
    _Other(u8),
}

impl From<Message1Multiplexor> for u8 {
    fn from(val: Message1Multiplexor) -> u8 {
        match val {
            Message1Multiplexor::Multiplexor8 => 8,
            Message1Multiplexor::Multiplexor16 => 16,
            Message1Multiplexor::Multiplexor24 => 24,
            Message1Multiplexor::_Other(x) => x,
        }
    }
}

/// Defined values for multiplexed signal Message1
#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
pub enum Message1MultiplexorIndex {
    M8(Message1MultiplexorM8),
    M24(Message1MultiplexorM24),
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
pub struct Message1MultiplexorM8 { raw: [u8; 8] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl Message1MultiplexorM8 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// BIT_L
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bit_l(&self) -> Message1BitL {
    let signal = self.raw.view_bits::<Lsb0>()[24..25].load_le::<u8>();
    
    match signal {
        0 => Message1BitL::Off,
        1 => Message1BitL::On,
        _ => Message1BitL::_Other(self.bit_l_raw()),
    }
}

/// Get raw value of BIT_L
///
/// - Start bit: 24
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bit_l_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[24..25].load_le::<u8>();
    
    signal == 1
}

/// Set value of BIT_L
#[inline(always)]
pub fn set_bit_l(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[24..25].store_le(value);
    Ok(())
}

/// BIT_G
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bit_g(&self) -> bool {
    self.bit_g_raw()
}

/// Get raw value of BIT_G
///
/// - Start bit: 23
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bit_g_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[23..24].load_le::<u8>();
    
    signal == 1
}

/// Set value of BIT_G
#[inline(always)]
pub fn set_bit_g(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[23..24].store_le(value);
    Ok(())
}

/// BIT_C
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bit_c(&self) -> bool {
    self.bit_c_raw()
}

/// Get raw value of BIT_C
///
/// - Start bit: 19
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bit_c_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[19..20].load_le::<u8>();
    
    signal == 1
}

/// Set value of BIT_C
#[inline(always)]
pub fn set_bit_c(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[19..20].store_le(value);
    Ok(())
}

/// BIT_J
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bit_j(&self) -> bool {
    self.bit_j_raw()
}

/// Get raw value of BIT_J
///
/// - Start bit: 18
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bit_j_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[18..19].load_le::<u8>();
    
    signal == 1
}

/// Set value of BIT_J
#[inline(always)]
pub fn set_bit_j(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[18..19].store_le(value);
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
pub struct Message1MultiplexorM24 { raw: [u8; 8] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl Message1MultiplexorM24 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// BIT_K
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bit_k(&self) -> bool {
    self.bit_k_raw()
}

/// Get raw value of BIT_K
///
/// - Start bit: 28
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bit_k_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[28..29].load_le::<u8>();
    
    signal == 1
}

/// Set value of BIT_K
#[inline(always)]
pub fn set_bit_k(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[28..29].store_le(value);
    Ok(())
}

/// BIT_D
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bit_d(&self) -> bool {
    self.bit_d_raw()
}

/// Get raw value of BIT_D
///
/// - Start bit: 32
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bit_d_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[32..33].load_le::<u8>();
    
    signal == 1
}

/// Set value of BIT_D
#[inline(always)]
pub fn set_bit_d(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[32..33].store_le(value);
    Ok(())
}

/// BIT_B
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bit_b(&self) -> bool {
    self.bit_b_raw()
}

/// Get raw value of BIT_B
///
/// - Start bit: 33
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bit_b_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[33..34].load_le::<u8>();
    
    signal == 1
}

/// Set value of BIT_B
#[inline(always)]
pub fn set_bit_b(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[33..34].store_le(value);
    Ok(())
}

/// BIT_F
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bit_f(&self) -> bool {
    self.bit_f_raw()
}

/// Get raw value of BIT_F
///
/// - Start bit: 39
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bit_f_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[39..40].load_le::<u8>();
    
    signal == 1
}

/// Set value of BIT_F
#[inline(always)]
pub fn set_bit_f(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[39..40].store_le(value);
    Ok(())
}

/// BIT_H
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bit_h(&self) -> bool {
    self.bit_h_raw()
}

/// Get raw value of BIT_H
///
/// - Start bit: 38
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bit_h_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[38..39].load_le::<u8>();
    
    signal == 1
}

/// Set value of BIT_H
#[inline(always)]
pub fn set_bit_h(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[38..39].store_le(value);
    Ok(())
}

/// BIT_E
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bit_e(&self) -> bool {
    self.bit_e_raw()
}

/// Get raw value of BIT_E
///
/// - Start bit: 29
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bit_e_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[29..30].load_le::<u8>();
    
    signal == 1
}

/// Set value of BIT_E
#[inline(always)]
pub fn set_bit_e(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[29..30].store_le(value);
    Ok(())
}

/// BIT_A
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bit_a(&self) -> bool {
    self.bit_a_raw()
}

/// Get raw value of BIT_A
///
/// - Start bit: 26
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bit_a_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[26..27].load_le::<u8>();
    
    signal == 1
}

/// Set value of BIT_A
#[inline(always)]
pub fn set_bit_a(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[26..27].store_le(value);
    Ok(())
}

}


/// Message2
///
/// - Extended ID: 1193047 (0x123457)
/// - Size: 8 bytes
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
    pub const MESSAGE_ID: embedded_can::Id = Id::Extended(unsafe { ExtendedId::new_unchecked(0x123457)});
    
    pub const MULTIPLEXOR_MIN: u8 = 0_u8;
    pub const MULTIPLEXOR_MAX: u8 = 0_u8;
    
    /// Construct new Message2 from values
    pub fn new(multiplexor: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_multiplexor(multiplexor)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// Get raw value of Multiplexor
    ///
    /// - Start bit: 2
    /// - Signal size: 6 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn multiplexor_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[2..8].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    pub fn multiplexor(&mut self) -> Result<Message2MultiplexorIndex, CanError> {
        match self.multiplexor_raw() {
            8 => Ok(Message2MultiplexorIndex::M8(Message2MultiplexorM8{ raw: self.raw })),
            24 => Ok(Message2MultiplexorIndex::M24(Message2MultiplexorM24{ raw: self.raw })),
            multiplexor => Err(CanError::InvalidMultiplexor { message_id: Message2::MESSAGE_ID, multiplexor: multiplexor.into() }),
        }
    }
    /// Set value of Multiplexor
    #[inline(always)]
    fn set_multiplexor(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Message2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Message2::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[2..8].store_le(value);
        Ok(())
    }
    
    /// Set value of Multiplexor
    #[inline(always)]
    pub fn set_m8(&mut self, value: Message2MultiplexorM8) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_multiplexor(8)?;
        Ok(())
    }
    
    /// Set value of Multiplexor
    #[inline(always)]
    pub fn set_m24(&mut self, value: Message2MultiplexorM24) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_multiplexor(24)?;
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
/// Defined values for Multiplexor
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
pub enum Message2Multiplexor {
    Multiplexor4NoSignals,
    Multiplexor8,
    Multiplexor16,
    Multiplexor24,
    _Other(u8),
}

impl From<Message2Multiplexor> for u8 {
    fn from(val: Message2Multiplexor) -> u8 {
        match val {
            Message2Multiplexor::Multiplexor4NoSignals => 4,
            Message2Multiplexor::Multiplexor8 => 8,
            Message2Multiplexor::Multiplexor16 => 16,
            Message2Multiplexor::Multiplexor24 => 24,
            Message2Multiplexor::_Other(x) => x,
        }
    }
}

/// Defined values for multiplexed signal Message2
#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
pub enum Message2MultiplexorIndex {
    M8(Message2MultiplexorM8),
    M24(Message2MultiplexorM24),
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
pub struct Message2MultiplexorM8 { raw: [u8; 8] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl Message2MultiplexorM8 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// BIT_L
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bit_l(&self) -> bool {
    self.bit_l_raw()
}

/// Get raw value of BIT_L
///
/// - Start bit: 24
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bit_l_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[24..25].load_le::<u8>();
    
    signal == 1
}

/// Set value of BIT_L
#[inline(always)]
pub fn set_bit_l(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[24..25].store_le(value);
    Ok(())
}

/// BIT_G
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bit_g(&self) -> bool {
    self.bit_g_raw()
}

/// Get raw value of BIT_G
///
/// - Start bit: 23
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bit_g_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[23..24].load_le::<u8>();
    
    signal == 1
}

/// Set value of BIT_G
#[inline(always)]
pub fn set_bit_g(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[23..24].store_le(value);
    Ok(())
}

/// BIT_C
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bit_c(&self) -> bool {
    self.bit_c_raw()
}

/// Get raw value of BIT_C
///
/// - Start bit: 19
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bit_c_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[19..20].load_le::<u8>();
    
    signal == 1
}

/// Set value of BIT_C
#[inline(always)]
pub fn set_bit_c(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[19..20].store_le(value);
    Ok(())
}

/// BIT_J
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bit_j(&self) -> bool {
    self.bit_j_raw()
}

/// Get raw value of BIT_J
///
/// - Start bit: 18
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bit_j_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[18..19].load_le::<u8>();
    
    signal == 1
}

/// Set value of BIT_J
#[inline(always)]
pub fn set_bit_j(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[18..19].store_le(value);
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
pub struct Message2MultiplexorM24 { raw: [u8; 8] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl Message2MultiplexorM24 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// BIT_K
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bit_k(&self) -> bool {
    self.bit_k_raw()
}

/// Get raw value of BIT_K
///
/// - Start bit: 28
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bit_k_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[28..29].load_le::<u8>();
    
    signal == 1
}

/// Set value of BIT_K
#[inline(always)]
pub fn set_bit_k(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[28..29].store_le(value);
    Ok(())
}

/// BIT_D
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bit_d(&self) -> bool {
    self.bit_d_raw()
}

/// Get raw value of BIT_D
///
/// - Start bit: 32
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bit_d_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[32..33].load_le::<u8>();
    
    signal == 1
}

/// Set value of BIT_D
#[inline(always)]
pub fn set_bit_d(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[32..33].store_le(value);
    Ok(())
}

/// BIT_B
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bit_b(&self) -> bool {
    self.bit_b_raw()
}

/// Get raw value of BIT_B
///
/// - Start bit: 33
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bit_b_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[33..34].load_le::<u8>();
    
    signal == 1
}

/// Set value of BIT_B
#[inline(always)]
pub fn set_bit_b(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[33..34].store_le(value);
    Ok(())
}

/// BIT_F
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bit_f(&self) -> bool {
    self.bit_f_raw()
}

/// Get raw value of BIT_F
///
/// - Start bit: 39
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bit_f_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[39..40].load_le::<u8>();
    
    signal == 1
}

/// Set value of BIT_F
#[inline(always)]
pub fn set_bit_f(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[39..40].store_le(value);
    Ok(())
}

/// BIT_H
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bit_h(&self) -> bool {
    self.bit_h_raw()
}

/// Get raw value of BIT_H
///
/// - Start bit: 38
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bit_h_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[38..39].load_le::<u8>();
    
    signal == 1
}

/// Set value of BIT_H
#[inline(always)]
pub fn set_bit_h(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[38..39].store_le(value);
    Ok(())
}

/// BIT_E
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bit_e(&self) -> bool {
    self.bit_e_raw()
}

/// Get raw value of BIT_E
///
/// - Start bit: 29
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bit_e_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[29..30].load_le::<u8>();
    
    signal == 1
}

/// Set value of BIT_E
#[inline(always)]
pub fn set_bit_e(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[29..30].store_le(value);
    Ok(())
}

/// BIT_A
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bit_a(&self) -> bool {
    self.bit_a_raw()
}

/// Get raw value of BIT_A
///
/// - Start bit: 26
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bit_a_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[26..27].load_le::<u8>();
    
    signal == 1
}

/// Set value of BIT_A
#[inline(always)]
pub fn set_bit_a(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[26..27].store_le(value);
    Ok(())
}

}


/// Message3
///
/// - Extended ID: 1193048 (0x123458)
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
    pub const MESSAGE_ID: embedded_can::Id = Id::Extended(unsafe { ExtendedId::new_unchecked(0x123458)});
    
    pub const MULTIPLEXOR_MIN: u8 = 0_u8;
    pub const MULTIPLEXOR_MAX: u8 = 0_u8;
    
    /// Construct new Message3 from values
    pub fn new(multiplexor: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_multiplexor(multiplexor)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// Get raw value of Multiplexor
    ///
    /// - Start bit: 2
    /// - Signal size: 6 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn multiplexor_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[2..8].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    pub fn multiplexor(&mut self) -> Result<Message3MultiplexorIndex, CanError> {
        match self.multiplexor_raw() {
            8 => Ok(Message3MultiplexorIndex::M8(Message3MultiplexorM8{ raw: self.raw })),
            multiplexor => Err(CanError::InvalidMultiplexor { message_id: Message3::MESSAGE_ID, multiplexor: multiplexor.into() }),
        }
    }
    /// Set value of Multiplexor
    #[inline(always)]
    fn set_multiplexor(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Message3::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Message3::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[2..8].store_le(value);
        Ok(())
    }
    
    /// Set value of Multiplexor
    #[inline(always)]
    pub fn set_m8(&mut self, value: Message3MultiplexorM8) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_multiplexor(8)?;
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
/// Defined values for multiplexed signal Message3
#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
pub enum Message3MultiplexorIndex {
    M8(Message3MultiplexorM8),
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
pub struct Message3MultiplexorM8 { raw: [u8; 8] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl Message3MultiplexorM8 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// BIT_L
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bit_l(&self) -> bool {
    self.bit_l_raw()
}

/// Get raw value of BIT_L
///
/// - Start bit: 24
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bit_l_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[24..25].load_le::<u8>();
    
    signal == 1
}

/// Set value of BIT_L
#[inline(always)]
pub fn set_bit_l(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[24..25].store_le(value);
    Ok(())
}

/// BIT_G
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bit_g(&self) -> bool {
    self.bit_g_raw()
}

/// Get raw value of BIT_G
///
/// - Start bit: 23
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bit_g_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[23..24].load_le::<u8>();
    
    signal == 1
}

/// Set value of BIT_G
#[inline(always)]
pub fn set_bit_g(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[23..24].store_le(value);
    Ok(())
}

/// BIT_C
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bit_c(&self) -> bool {
    self.bit_c_raw()
}

/// Get raw value of BIT_C
///
/// - Start bit: 19
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bit_c_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[19..20].load_le::<u8>();
    
    signal == 1
}

/// Set value of BIT_C
#[inline(always)]
pub fn set_bit_c(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[19..20].store_le(value);
    Ok(())
}

/// BIT_J
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bit_j(&self) -> bool {
    self.bit_j_raw()
}

/// Get raw value of BIT_J
///
/// - Start bit: 18
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bit_j_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[18..19].load_le::<u8>();
    
    signal == 1
}

/// Set value of BIT_J
#[inline(always)]
pub fn set_bit_j(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[18..19].store_le(value);
    Ok(())
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

