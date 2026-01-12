// Generated code!
//
// Message definitions from file `issue_207_sig_plus`
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
    /// myMsg
    MyMsg(MyMsg),
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
            MyMsg::MESSAGE_ID => Messages::MyMsg(MyMsg::try_from(payload)?),
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}

/// myMsg
///
/// - Standard ID: 1 (0x1)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct MyMsg {
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
impl MyMsg {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x1)});
    
    pub const MY_EXTRA_SIG_WITH_PLUS_MIN: i16 = -128_i16;
    pub const MY_EXTRA_SIG_WITH_PLUS_MAX: i16 = 127_i16;
    pub const MY_NORMAL_SIG_MIN: i16 = -128_i16;
    pub const MY_NORMAL_SIG_MAX: i16 = 127_i16;
    
    /// Construct new myMsg from values
    pub fn new(my_extra_sig_with_plus: i16, my_normal_sig: i16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_my_extra_sig_with_plus(my_extra_sig_with_plus)?;
        res.set_my_normal_sig(my_normal_sig)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// myExtraSigWithPlus
    ///
    /// - Min: -128
    /// - Max: 127
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn my_extra_sig_with_plus(&self) -> i16 {
        self.my_extra_sig_with_plus_raw()
    }
    
    /// Get raw value of myExtraSigWithPlus
    ///
    /// - Start bit: 8
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: -128
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn my_extra_sig_with_plus_raw(&self) -> i16 {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<i8>();
        
        let factor = 1;
        i16::from(signal).saturating_mul(factor).saturating_sub(128)
    }
    
    /// Set value of myExtraSigWithPlus
    #[inline(always)]
    pub fn set_my_extra_sig_with_plus(&mut self, value: i16) -> Result<(), CanError> {
        if value < -128_i16 || 127_i16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: MyMsg::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_add(128)
            .ok_or(CanError::ParameterOutOfRange { message_id: MyMsg::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
        Ok(())
    }
    
    /// myNormalSig
    ///
    /// - Min: -128
    /// - Max: 127
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn my_normal_sig(&self) -> i16 {
        self.my_normal_sig_raw()
    }
    
    /// Get raw value of myNormalSig
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: -128
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn my_normal_sig_raw(&self) -> i16 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<i8>();
        
        let factor = 1;
        i16::from(signal).saturating_mul(factor).saturating_sub(128)
    }
    
    /// Set value of myNormalSig
    #[inline(always)]
    pub fn set_my_normal_sig(&mut self, value: i16) -> Result<(), CanError> {
        if value < -128_i16 || 127_i16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: MyMsg::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_add(128)
            .ok_or(CanError::ParameterOutOfRange { message_id: MyMsg::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for MyMsg {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for MyMsg {
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

