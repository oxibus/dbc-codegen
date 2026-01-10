// Generated code!
//
// Message definitions from file `emc32`
// Version: HINBNYYYYYYYYYYYYYYYYYYYYYYYYYYYNNNNNNNNNN/4/%%%/4/'%**4NNN///

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
    /// EMV_Stati
    EmvStati(EmvStati),
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
            EmvStati::MESSAGE_ID => Messages::EmvStati(EmvStati::try_from(payload)?),
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}

/// EMV_Stati
///
/// - Standard ID: 546 (0x222)
/// - Size: 8 bytes
/// - Transmitter: EMV_Statusmeldungen
#[derive(Clone, Copy)]
pub struct EmvStati {
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
impl EmvStati {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x222)});
    
    
    /// Construct new EMV_Stati from values
    pub fn new(emv_aktion_status_5: bool, emv_aktion_status_4: bool, emv_aktion_status_3: bool, emv_aktion_status_2: bool, emv_aktion_status_1: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_emv_aktion_status_5(emv_aktion_status_5)?;
        res.set_emv_aktion_status_4(emv_aktion_status_4)?;
        res.set_emv_aktion_status_3(emv_aktion_status_3)?;
        res.set_emv_aktion_status_2(emv_aktion_status_2)?;
        res.set_emv_aktion_status_1(emv_aktion_status_1)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// EMV_Aktion_Status_5
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn emv_aktion_status_5(&self) -> bool {
        self.emv_aktion_status_5_raw()
    }
    
    /// Get raw value of EMV_Aktion_Status_5
    ///
    /// - Start bit: 4
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn emv_aktion_status_5_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[4..5].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of EMV_Aktion_Status_5
    #[inline(always)]
    pub fn set_emv_aktion_status_5(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[4..5].store_le(value);
        Ok(())
    }
    
    /// EMV_Aktion_Status_4
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn emv_aktion_status_4(&self) -> bool {
        self.emv_aktion_status_4_raw()
    }
    
    /// Get raw value of EMV_Aktion_Status_4
    ///
    /// - Start bit: 3
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn emv_aktion_status_4_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[3..4].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of EMV_Aktion_Status_4
    #[inline(always)]
    pub fn set_emv_aktion_status_4(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[3..4].store_le(value);
        Ok(())
    }
    
    /// EMV_Aktion_Status_3
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: EMV_Statusmeldungen
    #[inline(always)]
    pub fn emv_aktion_status_3(&self) -> bool {
        self.emv_aktion_status_3_raw()
    }
    
    /// Get raw value of EMV_Aktion_Status_3
    ///
    /// - Start bit: 2
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn emv_aktion_status_3_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[2..3].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of EMV_Aktion_Status_3
    #[inline(always)]
    pub fn set_emv_aktion_status_3(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[2..3].store_le(value);
        Ok(())
    }
    
    /// EMV_Aktion_Status_2
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn emv_aktion_status_2(&self) -> bool {
        self.emv_aktion_status_2_raw()
    }
    
    /// Get raw value of EMV_Aktion_Status_2
    ///
    /// - Start bit: 6
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn emv_aktion_status_2_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[6..7].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of EMV_Aktion_Status_2
    #[inline(always)]
    pub fn set_emv_aktion_status_2(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[6..7].store_le(value);
        Ok(())
    }
    
    /// EMV_Aktion_Status_1
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn emv_aktion_status_1(&self) -> bool {
        self.emv_aktion_status_1_raw()
    }
    
    /// Get raw value of EMV_Aktion_Status_1
    ///
    /// - Start bit: 5
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn emv_aktion_status_1_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[5..6].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of EMV_Aktion_Status_1
    #[inline(always)]
    pub fn set_emv_aktion_status_1(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[5..6].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for EmvStati {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for EmvStati {
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

