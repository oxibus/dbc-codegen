// Generated code!
//
// Message definitions from file `variable_dlc`
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
    /// TestMessage1
    TestMessage1(TestMessage1),
    /// TestMessage2
    TestMessage2(TestMessage2),
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
            TestMessage1::MESSAGE_ID => Messages::TestMessage1(TestMessage1::try_from(payload)?),
            TestMessage2::MESSAGE_ID => Messages::TestMessage2(TestMessage2::try_from(payload)?),
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}

/// TestMessage1
///
/// - Extended ID: 16711936 (0xff0100)
/// - Size: 3 bytes
/// - Transmitter: Test
#[derive(Clone, Copy)]
pub struct TestMessage1 {
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
impl TestMessage1 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Extended(unsafe { ExtendedId::new_unchecked(0xff0100)});
    
    pub const SIGNAL1_MIN: u8 = 0_u8;
    pub const SIGNAL1_MAX: u8 = 0_u8;
    pub const SIGNAL2_MIN: u8 = 0_u8;
    pub const SIGNAL2_MAX: u8 = 0_u8;
    pub const SIGNAL3_MIN: u8 = 0_u8;
    pub const SIGNAL3_MAX: u8 = 0_u8;
    
    /// Construct new TestMessage1 from values
    pub fn new(signal1: u8, signal2: u8, signal3: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 3] };
        res.set_signal1(signal1)?;
        res.set_signal2(signal2)?;
        res.set_signal3(signal3)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 3] {
        &self.raw
    }
    
    /// Signal1
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Test
    #[inline(always)]
    pub fn signal1(&self) -> u8 {
        self.signal1_raw()
    }
    
    /// Get raw value of Signal1
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn signal1_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Signal1
    #[inline(always)]
    pub fn set_signal1(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: TestMessage1::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: TestMessage1::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
    /// Signal2
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Test
    #[inline(always)]
    pub fn signal2(&self) -> u8 {
        self.signal2_raw()
    }
    
    /// Get raw value of Signal2
    ///
    /// - Start bit: 8
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn signal2_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Signal2
    #[inline(always)]
    pub fn set_signal2(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: TestMessage1::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: TestMessage1::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
        Ok(())
    }
    
    /// Signal3
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Test
    #[inline(always)]
    pub fn signal3(&self) -> u8 {
        self.signal3_raw()
    }
    
    /// Get raw value of Signal3
    ///
    /// - Start bit: 16
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn signal3_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[16..24].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Signal3
    #[inline(always)]
    pub fn set_signal3(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: TestMessage1::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: TestMessage1::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[16..24].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for TestMessage1 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 3 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 3];
        raw.copy_from_slice(&payload[..3]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for TestMessage1 {
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

/// TestMessage2
///
/// - Extended ID: 16712192 (0xff0200)
/// - Size: 10 bytes
/// - Transmitter: Test
#[derive(Clone, Copy)]
pub struct TestMessage2 {
    raw: [u8; 10],
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
impl TestMessage2 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Extended(unsafe { ExtendedId::new_unchecked(0xff0200)});
    
    pub const SIGNAL4_MIN: u16 = 0_u16;
    pub const SIGNAL4_MAX: u16 = 0_u16;
    pub const SIGNAL5_MIN: u16 = 0_u16;
    pub const SIGNAL5_MAX: u16 = 0_u16;
    pub const SIGNAL6_MIN: u16 = 0_u16;
    pub const SIGNAL6_MAX: u16 = 0_u16;
    pub const SIGNAL7_MIN: u16 = 0_u16;
    pub const SIGNAL7_MAX: u16 = 0_u16;
    pub const SIGNAL8_MIN: u16 = 0_u16;
    pub const SIGNAL8_MAX: u16 = 0_u16;
    
    /// Construct new TestMessage2 from values
    pub fn new(signal4: u16, signal5: u16, signal6: u16, signal7: u16, signal8: u16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 10] };
        res.set_signal4(signal4)?;
        res.set_signal5(signal5)?;
        res.set_signal6(signal6)?;
        res.set_signal7(signal7)?;
        res.set_signal8(signal8)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 10] {
        &self.raw
    }
    
    /// Signal4
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Test
    #[inline(always)]
    pub fn signal4(&self) -> u16 {
        self.signal4_raw()
    }
    
    /// Get raw value of Signal4
    ///
    /// - Start bit: 0
    /// - Signal size: 15 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn signal4_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[0..15].load_le::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Signal4
    #[inline(always)]
    pub fn set_signal4(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: TestMessage2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: TestMessage2::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..15].store_le(value);
        Ok(())
    }
    
    /// Signal5
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Test
    #[inline(always)]
    pub fn signal5(&self) -> u16 {
        self.signal5_raw()
    }
    
    /// Get raw value of Signal5
    ///
    /// - Start bit: 15
    /// - Signal size: 15 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn signal5_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[15..30].load_le::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Signal5
    #[inline(always)]
    pub fn set_signal5(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: TestMessage2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: TestMessage2::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[15..30].store_le(value);
        Ok(())
    }
    
    /// Signal6
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Test
    #[inline(always)]
    pub fn signal6(&self) -> u16 {
        self.signal6_raw()
    }
    
    /// Get raw value of Signal6
    ///
    /// - Start bit: 30
    /// - Signal size: 15 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn signal6_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[30..45].load_le::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Signal6
    #[inline(always)]
    pub fn set_signal6(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: TestMessage2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: TestMessage2::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[30..45].store_le(value);
        Ok(())
    }
    
    /// Signal7
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Test
    #[inline(always)]
    pub fn signal7(&self) -> u16 {
        self.signal7_raw()
    }
    
    /// Get raw value of Signal7
    ///
    /// - Start bit: 45
    /// - Signal size: 15 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn signal7_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[45..60].load_le::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Signal7
    #[inline(always)]
    pub fn set_signal7(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: TestMessage2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: TestMessage2::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[45..60].store_le(value);
        Ok(())
    }
    
    /// Signal8
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Test
    #[inline(always)]
    pub fn signal8(&self) -> u16 {
        self.signal8_raw()
    }
    
    /// Get raw value of Signal8
    ///
    /// - Start bit: 60
    /// - Signal size: 15 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn signal8_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[60..75].load_le::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Signal8
    #[inline(always)]
    pub fn set_signal8(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: TestMessage2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: TestMessage2::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[60..75].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for TestMessage2 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 10 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 10];
        raw.copy_from_slice(&payload[..10]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for TestMessage2 {
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

