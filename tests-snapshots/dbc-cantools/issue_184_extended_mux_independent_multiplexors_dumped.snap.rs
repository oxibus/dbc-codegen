// Generated code!
//
// Message definitions from file `issue_184_extended_mux_independent_multiplexors_dumped`
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
    /// ext_MUX_indep_multiplexors
    ExtMuxIndepMultiplexors(ExtMuxIndepMultiplexors),
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
            ExtMuxIndepMultiplexors::MESSAGE_ID => Messages::ExtMuxIndepMultiplexors(ExtMuxIndepMultiplexors::try_from(payload)?),
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}

/// ext_MUX_indep_multiplexors
///
/// - Extended ID: 2 (0x2)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct ExtMuxIndepMultiplexors {
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
impl ExtMuxIndepMultiplexors {
    pub const MESSAGE_ID: embedded_can::Id = Id::Extended(unsafe { ExtendedId::new_unchecked(0x2)});
    
    pub const MUXED_B_1_MIN: i8 = 0_i8;
    pub const MUXED_B_1_MAX: i8 = 0_i8;
    pub const MUXED_B_2_MIN: i8 = 0_i8;
    pub const MUXED_B_2_MAX: i8 = 0_i8;
    pub const MUX_B_MIN: i8 = 0_i8;
    pub const MUX_B_MAX: i8 = 0_i8;
    pub const MUXED_A_0_MIN: i8 = 0_i8;
    pub const MUXED_A_0_MAX: i8 = 0_i8;
    pub const MUXED_A_1_MIN: i8 = 0_i8;
    pub const MUXED_A_1_MAX: i8 = 0_i8;
    pub const MUX_A_MIN: i8 = 0_i8;
    pub const MUX_A_MAX: i8 = 0_i8;
    
    /// Construct new ext_MUX_indep_multiplexors from values
    pub fn new(mux_b: i8, mux_a: i8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_mux_b(mux_b)?;
        res.set_mux_a(mux_a)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// Get raw value of MUX_B
    ///
    /// - Start bit: 16
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn mux_b_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[16..24].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    pub fn mux_b(&mut self) -> Result<ExtMuxIndepMultiplexorsMuxBIndex, CanError> {
        match self.mux_b_raw() {
            0 => Ok(ExtMuxIndepMultiplexorsMuxBIndex::M0(ExtMuxIndepMultiplexorsMuxBM0{ raw: self.raw })),
            1 => Ok(ExtMuxIndepMultiplexorsMuxBIndex::M1(ExtMuxIndepMultiplexorsMuxBM1{ raw: self.raw })),
            2 => Ok(ExtMuxIndepMultiplexorsMuxBIndex::M2(ExtMuxIndepMultiplexorsMuxBM2{ raw: self.raw })),
            multiplexor => Err(CanError::InvalidMultiplexor { message_id: ExtMuxIndepMultiplexors::MESSAGE_ID, multiplexor: multiplexor.into() }),
        }
    }
    /// Set value of MUX_B
    #[inline(always)]
    fn set_mux_b(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: ExtMuxIndepMultiplexors::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: ExtMuxIndepMultiplexors::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[16..24].store_le(value);
        Ok(())
    }
    
    /// Set value of MUX_B
    #[inline(always)]
    pub fn set_m0(&mut self, value: ExtMuxIndepMultiplexorsMuxBM0) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mux_b(0)?;
        Ok(())
    }
    
    /// Set value of MUX_B
    #[inline(always)]
    pub fn set_m1(&mut self, value: ExtMuxIndepMultiplexorsMuxBM1) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mux_b(1)?;
        Ok(())
    }
    
    /// Set value of MUX_B
    #[inline(always)]
    pub fn set_m2(&mut self, value: ExtMuxIndepMultiplexorsMuxBM2) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mux_b(2)?;
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
    
    pub fn mux_a(&mut self) -> Result<ExtMuxIndepMultiplexorsMuxAIndex, CanError> {
        match self.mux_a_raw() {
            0 => Ok(ExtMuxIndepMultiplexorsMuxAIndex::M0(ExtMuxIndepMultiplexorsMuxAM0{ raw: self.raw })),
            1 => Ok(ExtMuxIndepMultiplexorsMuxAIndex::M1(ExtMuxIndepMultiplexorsMuxAM1{ raw: self.raw })),
            2 => Ok(ExtMuxIndepMultiplexorsMuxAIndex::M2(ExtMuxIndepMultiplexorsMuxAM2{ raw: self.raw })),
            multiplexor => Err(CanError::InvalidMultiplexor { message_id: ExtMuxIndepMultiplexors::MESSAGE_ID, multiplexor: multiplexor.into() }),
        }
    }
    /// Set value of MUX_A
    #[inline(always)]
    fn set_mux_a(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: ExtMuxIndepMultiplexors::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: ExtMuxIndepMultiplexors::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
    /// Set value of MUX_A
    #[inline(always)]
    pub fn set_m0(&mut self, value: ExtMuxIndepMultiplexorsMuxAM0) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mux_a(0)?;
        Ok(())
    }
    
    /// Set value of MUX_A
    #[inline(always)]
    pub fn set_m1(&mut self, value: ExtMuxIndepMultiplexorsMuxAM1) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mux_a(1)?;
        Ok(())
    }
    
    /// Set value of MUX_A
    #[inline(always)]
    pub fn set_m2(&mut self, value: ExtMuxIndepMultiplexorsMuxAM2) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mux_a(2)?;
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for ExtMuxIndepMultiplexors {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for ExtMuxIndepMultiplexors {
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
/// Defined values for multiplexed signal ext_MUX_indep_multiplexors
#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
pub enum ExtMuxIndepMultiplexorsMuxBIndex {
    M0(ExtMuxIndepMultiplexorsMuxBM0),
    M1(ExtMuxIndepMultiplexorsMuxBM1),
    M2(ExtMuxIndepMultiplexorsMuxBM2),
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
pub struct ExtMuxIndepMultiplexorsMuxBM0 { raw: [u8; 8] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl ExtMuxIndepMultiplexorsMuxBM0 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// muxed_A_0
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn muxed_a_0(&self) -> i8 {
    self.muxed_a_0_raw()
}

/// Get raw value of muxed_A_0
///
/// - Start bit: 8
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Signed
#[inline(always)]
pub fn muxed_a_0_raw(&self) -> i8 {
    let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<i8>();
    
    let factor = 1;
    let signal = signal as i8;
    i8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of muxed_A_0
#[inline(always)]
pub fn set_muxed_a_0(&mut self, value: i8) -> Result<(), CanError> {
    if value < 0_i8 || 0_i8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: ExtMuxIndepMultiplexors::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: ExtMuxIndepMultiplexors::MESSAGE_ID })?;
    let value = (value / factor) as i8;
    
    let value = u8::from_ne_bytes(value.to_ne_bytes());
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
pub struct ExtMuxIndepMultiplexorsMuxBM1 { raw: [u8; 8] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl ExtMuxIndepMultiplexorsMuxBM1 {
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
        return Err(CanError::ParameterOutOfRange { message_id: ExtMuxIndepMultiplexors::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: ExtMuxIndepMultiplexors::MESSAGE_ID })?;
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
        return Err(CanError::ParameterOutOfRange { message_id: ExtMuxIndepMultiplexors::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: ExtMuxIndepMultiplexors::MESSAGE_ID })?;
    let value = (value / factor) as i8;
    
    let value = u8::from_ne_bytes(value.to_ne_bytes());
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
pub struct ExtMuxIndepMultiplexorsMuxBM2 { raw: [u8; 8] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl ExtMuxIndepMultiplexorsMuxBM2 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// muxed_B_2
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn muxed_b_2(&self) -> i8 {
    self.muxed_b_2_raw()
}

/// Get raw value of muxed_B_2
///
/// - Start bit: 24
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Signed
#[inline(always)]
pub fn muxed_b_2_raw(&self) -> i8 {
    let signal = self.raw.view_bits::<Lsb0>()[24..32].load_le::<i8>();
    
    let factor = 1;
    let signal = signal as i8;
    i8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of muxed_B_2
#[inline(always)]
pub fn set_muxed_b_2(&mut self, value: i8) -> Result<(), CanError> {
    if value < 0_i8 || 0_i8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: ExtMuxIndepMultiplexors::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: ExtMuxIndepMultiplexors::MESSAGE_ID })?;
    let value = (value / factor) as i8;
    
    let value = u8::from_ne_bytes(value.to_ne_bytes());
    self.raw.view_bits_mut::<Lsb0>()[24..32].store_le(value);
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

