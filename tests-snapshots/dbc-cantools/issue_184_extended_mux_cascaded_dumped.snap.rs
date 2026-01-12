// Generated code!
//
// Message definitions from file `issue_184_extended_mux_cascaded_dumped`
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
    /// ext_MUX_cascaded
    ExtMuxCascaded(ExtMuxCascaded),
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
            ExtMuxCascaded::MESSAGE_ID => Messages::ExtMuxCascaded(ExtMuxCascaded::try_from(payload)?),
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}

/// ext_MUX_cascaded
///
/// - Extended ID: 1 (0x1)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct ExtMuxCascaded {
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
impl ExtMuxCascaded {
    pub const MESSAGE_ID: embedded_can::Id = Id::Extended(unsafe { ExtendedId::new_unchecked(0x1)});
    
    pub const MUXED_B_1_MIN: i8 = 0_i8;
    pub const MUXED_B_1_MAX: i8 = 0_i8;
    pub const MUXED_B_0_MIN: i8 = 0_i8;
    pub const MUXED_B_0_MAX: i8 = 0_i8;
    pub const MUXED_A_1_MIN: i8 = 0_i8;
    pub const MUXED_A_1_MAX: i8 = 0_i8;
    pub const MUXED_A_2_MUX_B_MIN: i8 = 0_i8;
    pub const MUXED_A_2_MUX_B_MAX: i8 = 0_i8;
    pub const MUX_A_MIN: i8 = 0_i8;
    pub const MUX_A_MAX: i8 = 0_i8;
    
    /// Construct new ext_MUX_cascaded from values
    pub fn new(muxed_a_2_mux_b: i8, mux_a: i8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_muxed_a_2_mux_b(muxed_a_2_mux_b)?;
        res.set_mux_a(mux_a)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// Get raw value of muxed_A_2_MUX_B
    ///
    /// - Start bit: 8
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn muxed_a_2_mux_b_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    pub fn muxed_a_2_mux_b(&mut self) -> Result<ExtMuxCascadedMuxedA2MuxBIndex, CanError> {
        match self.muxed_a_2_mux_b_raw() {
            0 => Ok(ExtMuxCascadedMuxedA2MuxBIndex::M0(ExtMuxCascadedMuxedA2MuxBM0{ raw: self.raw })),
            1 => Ok(ExtMuxCascadedMuxedA2MuxBIndex::M1(ExtMuxCascadedMuxedA2MuxBM1{ raw: self.raw })),
            multiplexor => Err(CanError::InvalidMultiplexor { message_id: ExtMuxCascaded::MESSAGE_ID, multiplexor: multiplexor.into() }),
        }
    }
    /// Set value of muxed_A_2_MUX_B
    #[inline(always)]
    fn set_muxed_a_2_mux_b(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: ExtMuxCascaded::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: ExtMuxCascaded::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
        Ok(())
    }
    
    /// Set value of muxed_A_2_MUX_B
    #[inline(always)]
    pub fn set_m0(&mut self, value: ExtMuxCascadedMuxedA2MuxBM0) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_muxed_a_2_mux_b(0)?;
        Ok(())
    }
    
    /// Set value of muxed_A_2_MUX_B
    #[inline(always)]
    pub fn set_m1(&mut self, value: ExtMuxCascadedMuxedA2MuxBM1) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_muxed_a_2_mux_b(1)?;
        Ok(())
    }
    
    /// Get raw value of MUX_A
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn mux_a_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    pub fn mux_a(&mut self) -> Result<ExtMuxCascadedMuxAIndex, CanError> {
        match self.mux_a_raw() {
            0 => Ok(ExtMuxCascadedMuxAIndex::M0(ExtMuxCascadedMuxAM0{ raw: self.raw })),
            1 => Ok(ExtMuxCascadedMuxAIndex::M1(ExtMuxCascadedMuxAM1{ raw: self.raw })),
            multiplexor => Err(CanError::InvalidMultiplexor { message_id: ExtMuxCascaded::MESSAGE_ID, multiplexor: multiplexor.into() }),
        }
    }
    /// Set value of MUX_A
    #[inline(always)]
    fn set_mux_a(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: ExtMuxCascaded::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: ExtMuxCascaded::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
    /// Set value of MUX_A
    #[inline(always)]
    pub fn set_m0(&mut self, value: ExtMuxCascadedMuxAM0) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mux_a(0)?;
        Ok(())
    }
    
    /// Set value of MUX_A
    #[inline(always)]
    pub fn set_m1(&mut self, value: ExtMuxCascadedMuxAM1) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mux_a(1)?;
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for ExtMuxCascaded {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for ExtMuxCascaded {
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
/// Defined values for multiplexed signal ext_MUX_cascaded
#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
pub enum ExtMuxCascadedMuxedA2MuxBIndex {
    M0(ExtMuxCascadedMuxedA2MuxBM0),
    M1(ExtMuxCascadedMuxedA2MuxBM1),
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
pub struct ExtMuxCascadedMuxedA2MuxBM0 { raw: [u8; 8] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl ExtMuxCascadedMuxedA2MuxBM0 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// muxed_B_0
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn muxed_b_0(&self) -> i8 {
    self.muxed_b_0_raw()
}

/// Get raw value of muxed_B_0
///
/// - Start bit: 16
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Signed
#[inline(always)]
pub fn muxed_b_0_raw(&self) -> i8 {
    let signal = self.raw.view_bits::<Lsb0>()[16..24].load_le::<i8>();
    
    let factor = 1;
    let signal = signal as i8;
    i8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of muxed_B_0
#[inline(always)]
pub fn set_muxed_b_0(&mut self, value: i8) -> Result<(), CanError> {
    if value < 0_i8 || 0_i8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: ExtMuxCascaded::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: ExtMuxCascaded::MESSAGE_ID })?;
    let value = (value / factor) as i8;
    
    let value = u8::from_ne_bytes(value.to_ne_bytes());
    self.raw.view_bits_mut::<Lsb0>()[16..24].store_le(value);
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
pub struct ExtMuxCascadedMuxedA2MuxBM1 { raw: [u8; 8] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl ExtMuxCascadedMuxedA2MuxBM1 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// muxed_B_1
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn muxed_b_1(&self) -> i8 {
    self.muxed_b_1_raw()
}

/// Get raw value of muxed_B_1
///
/// - Start bit: 24
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Signed
#[inline(always)]
pub fn muxed_b_1_raw(&self) -> i8 {
    let signal = self.raw.view_bits::<Lsb0>()[24..32].load_le::<i8>();
    
    let factor = 1;
    let signal = signal as i8;
    i8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of muxed_B_1
#[inline(always)]
pub fn set_muxed_b_1(&mut self, value: i8) -> Result<(), CanError> {
    if value < 0_i8 || 0_i8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: ExtMuxCascaded::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: ExtMuxCascaded::MESSAGE_ID })?;
    let value = (value / factor) as i8;
    
    let value = u8::from_ne_bytes(value.to_ne_bytes());
    self.raw.view_bits_mut::<Lsb0>()[24..32].store_le(value);
    Ok(())
}

/// muxed_A_1
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn muxed_a_1(&self) -> i8 {
    self.muxed_a_1_raw()
}

/// Get raw value of muxed_A_1
///
/// - Start bit: 8
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Signed
#[inline(always)]
pub fn muxed_a_1_raw(&self) -> i8 {
    let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<i8>();
    
    let factor = 1;
    let signal = signal as i8;
    i8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of muxed_A_1
#[inline(always)]
pub fn set_muxed_a_1(&mut self, value: i8) -> Result<(), CanError> {
    if value < 0_i8 || 0_i8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: ExtMuxCascaded::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: ExtMuxCascaded::MESSAGE_ID })?;
    let value = (value / factor) as i8;
    
    let value = u8::from_ne_bytes(value.to_ne_bytes());
    self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
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

