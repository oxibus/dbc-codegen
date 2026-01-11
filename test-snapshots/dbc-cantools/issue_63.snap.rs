// Generated code!
//
// Message definitions from file `issue_63`
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
    /// AFT1PSI2
    Aft1psi2(Aft1psi2),
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
            Aft1psi2::MESSAGE_ID => Messages::Aft1psi2(Aft1psi2::try_from(payload)?),
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}

/// AFT1PSI2
///
/// - Extended ID: 419204094 (0x18fc8bfe)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Aft1psi2 {
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
impl Aft1psi2 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Extended(unsafe { ExtendedId::new_unchecked(0x18fc8bfe)});
    
    pub const HTR_RES_MIN: f32 = 0_f32;
    pub const HTR_RES_MAX: f32 = 6425.5_f32;
    pub const MAX_RES_MIN: u32 = 0_u32;
    pub const MAX_RES_MAX: u32 = 62500_u32;
    pub const TEMP_MIN: f32 = -273_f32;
    pub const TEMP_MAX: f32 = 1734.96875_f32;
    pub const REGEN_FAILED_COUNT_MIN: u8 = 0_u8;
    pub const REGEN_FAILED_COUNT_MAX: u8 = 250_u8;
    pub const PWR_SUPPLY_MIN: u8 = 0_u8;
    pub const PWR_SUPPLY_MAX: u8 = 3_u8;
    pub const DETECTION_STATUS_MIN: u8 = 0_u8;
    pub const DETECTION_STATUS_MAX: u8 = 15_u8;
    
    /// Construct new AFT1PSI2 from values
    pub fn new(htr_res: f32, max_res: u32, temp: f32, regen_failed_count: u8, pwr_supply: u8, detection_status: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_htr_res(htr_res)?;
        res.set_max_res(max_res)?;
        res.set_temp(temp)?;
        res.set_regen_failed_count(regen_failed_count)?;
        res.set_pwr_supply(pwr_supply)?;
        res.set_detection_status(detection_status)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// HtrRes
    ///
    /// - Min: 0
    /// - Max: 6425.5
    /// - Unit: "ohm"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn htr_res(&self) -> f32 {
        self.htr_res_raw()
    }
    
    /// Get raw value of HtrRes
    ///
    /// - Start bit: 40
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn htr_res_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[40..56].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of HtrRes
    #[inline(always)]
    pub fn set_htr_res(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 6425.5_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Aft1psi2::MESSAGE_ID });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[40..56].store_le(value);
        Ok(())
    }
    
    /// MaxRes
    ///
    /// - Min: 0
    /// - Max: 62500
    /// - Unit: "kohm"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn max_res(&self) -> u32 {
        self.max_res_raw()
    }
    
    /// Get raw value of MaxRes
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 250
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn max_res_raw(&self) -> u32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let factor = 250;
        u32::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of MaxRes
    #[inline(always)]
    pub fn set_max_res(&mut self, value: u32) -> Result<(), CanError> {
        if value < 0_u32 || 62500_u32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Aft1psi2::MESSAGE_ID });
        }
        let factor = 250;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Aft1psi2::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// Temp
    ///
    /// - Min: -273
    /// - Max: 1734.96875
    /// - Unit: "ï¿½C"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn temp(&self) -> f32 {
        self.temp_raw()
    }
    
    /// Get raw value of Temp
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.03125
    /// - Offset: -273
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn temp_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let factor = 0.03125_f32;
        let offset = -273_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of Temp
    #[inline(always)]
    pub fn set_temp(&mut self, value: f32) -> Result<(), CanError> {
        if value < -273_f32 || 1734.96875_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Aft1psi2::MESSAGE_ID });
        }
        let factor = 0.03125_f32;
        let offset = -273_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// RegenFailedCount
    ///
    /// - Min: 0
    /// - Max: 250
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn regen_failed_count(&self) -> u8 {
        self.regen_failed_count_raw()
    }
    
    /// Get raw value of RegenFailedCount
    ///
    /// - Start bit: 8
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn regen_failed_count_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of RegenFailedCount
    #[inline(always)]
    pub fn set_regen_failed_count(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 250_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Aft1psi2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Aft1psi2::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
        Ok(())
    }
    
    /// PwrSupply
    ///
    /// - Min: 0
    /// - Max: 3
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn pwr_supply(&self) -> u8 {
        self.pwr_supply_raw()
    }
    
    /// Get raw value of PwrSupply
    ///
    /// - Start bit: 4
    /// - Signal size: 2 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn pwr_supply_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[4..6].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of PwrSupply
    #[inline(always)]
    pub fn set_pwr_supply(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 3_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Aft1psi2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Aft1psi2::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[4..6].store_le(value);
        Ok(())
    }
    
    /// DetectionStatus
    ///
    /// - Min: 0
    /// - Max: 15
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn detection_status(&self) -> u8 {
        self.detection_status_raw()
    }
    
    /// Get raw value of DetectionStatus
    ///
    /// - Start bit: 0
    /// - Signal size: 4 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn detection_status_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..4].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of DetectionStatus
    #[inline(always)]
    pub fn set_detection_status(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 15_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Aft1psi2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Aft1psi2::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..4].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Aft1psi2 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Aft1psi2 {
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

