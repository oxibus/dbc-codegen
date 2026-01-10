// Generated code!
//
// Message definitions from file `multiplex_2`
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
    /// Shared
    Shared(Shared),
    /// Normal
    Normal(Normal),
    /// Extended
    Extended(Extended),
    /// ExtendedTypes
    ExtendedTypes(ExtendedTypes),
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
            Shared::MESSAGE_ID => Messages::Shared(Shared::try_from(payload)?),
            Normal::MESSAGE_ID => Messages::Normal(Normal::try_from(payload)?),
            Extended::MESSAGE_ID => Messages::Extended(Extended::try_from(payload)?),
            ExtendedTypes::MESSAGE_ID => Messages::ExtendedTypes(ExtendedTypes::try_from(payload)?),
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}

/// Shared
///
/// - Extended ID: 201522942 (0xc02fefe)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Shared {
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
impl Shared {
    pub const MESSAGE_ID: embedded_can::Id = Id::Extended(unsafe { ExtendedId::new_unchecked(0xc02fefe)});
    
    pub const S2_MIN: i8 = 0_i8;
    pub const S2_MAX: i8 = 0_i8;
    pub const S1_MIN: i8 = 0_i8;
    pub const S1_MAX: i8 = 0_i8;
    pub const S0_MIN: i8 = 0_i8;
    pub const S0_MAX: i8 = 0_i8;
    
    /// Construct new Shared from values
    pub fn new(s0: i8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_s0(s0)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// Get raw value of S0
    ///
    /// - Start bit: 0
    /// - Signal size: 4 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn s0_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..4].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    pub fn s0(&mut self) -> Result<SharedS0Index, CanError> {
        match self.s0_raw() {
            1 => Ok(SharedS0Index::M1(SharedS0M1{ raw: self.raw })),
            2 => Ok(SharedS0Index::M2(SharedS0M2{ raw: self.raw })),
            multiplexor => Err(CanError::InvalidMultiplexor { message_id: Shared::MESSAGE_ID, multiplexor: multiplexor.into() }),
        }
    }
    /// Set value of S0
    #[inline(always)]
    fn set_s0(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Shared::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Shared::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..4].store_le(value);
        Ok(())
    }
    
    /// Set value of S0
    #[inline(always)]
    pub fn set_m1(&mut self, value: SharedS0M1) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_s0(1)?;
        Ok(())
    }
    
    /// Set value of S0
    #[inline(always)]
    pub fn set_m2(&mut self, value: SharedS0M2) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_s0(2)?;
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Shared {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Shared {
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
/// Defined values for multiplexed signal Shared
#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
pub enum SharedS0Index {
    M1(SharedS0M1),
    M2(SharedS0M2),
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
pub struct SharedS0M1 { raw: [u8; 8] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl SharedS0M1 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// S1
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn s1(&self) -> i8 {
    self.s1_raw()
}

/// Get raw value of S1
///
/// - Start bit: 4
/// - Signal size: 4 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Signed
#[inline(always)]
pub fn s1_raw(&self) -> i8 {
    let signal = self.raw.view_bits::<Lsb0>()[4..8].load_le::<i8>();
    
    let factor = 1;
    let signal = signal as i8;
    i8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of S1
#[inline(always)]
pub fn set_s1(&mut self, value: i8) -> Result<(), CanError> {
    if value < 0_i8 || 0_i8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Shared::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Shared::MESSAGE_ID })?;
    let value = (value / factor) as i8;
    
    let value = u8::from_ne_bytes(value.to_ne_bytes());
    self.raw.view_bits_mut::<Lsb0>()[4..8].store_le(value);
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
pub struct SharedS0M2 { raw: [u8; 8] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl SharedS0M2 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// S2
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn s2(&self) -> i8 {
    self.s2_raw()
}

/// Get raw value of S2
///
/// - Start bit: 8
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Signed
#[inline(always)]
pub fn s2_raw(&self) -> i8 {
    let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<i8>();
    
    let factor = 1;
    let signal = signal as i8;
    i8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of S2
#[inline(always)]
pub fn set_s2(&mut self, value: i8) -> Result<(), CanError> {
    if value < 0_i8 || 0_i8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Shared::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Shared::MESSAGE_ID })?;
    let value = (value / factor) as i8;
    
    let value = u8::from_ne_bytes(value.to_ne_bytes());
    self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
    Ok(())
}

}


/// Normal
///
/// - Extended ID: 201457406 (0xc01fefe)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Normal {
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
impl Normal {
    pub const MESSAGE_ID: embedded_can::Id = Id::Extended(unsafe { ExtendedId::new_unchecked(0xc01fefe)});
    
    pub const S2_MIN: i8 = 0_i8;
    pub const S2_MAX: i8 = 0_i8;
    pub const S1_MIN: i8 = 0_i8;
    pub const S1_MAX: i8 = 0_i8;
    pub const S0_MIN: i8 = 0_i8;
    pub const S0_MAX: i8 = 0_i8;
    
    /// Construct new Normal from values
    pub fn new(s0: i8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_s0(s0)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// Get raw value of S0
    ///
    /// - Start bit: 0
    /// - Signal size: 4 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn s0_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..4].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    pub fn s0(&mut self) -> Result<NormalS0Index, CanError> {
        match self.s0_raw() {
            0 => Ok(NormalS0Index::M0(NormalS0M0{ raw: self.raw })),
            1 => Ok(NormalS0Index::M1(NormalS0M1{ raw: self.raw })),
            multiplexor => Err(CanError::InvalidMultiplexor { message_id: Normal::MESSAGE_ID, multiplexor: multiplexor.into() }),
        }
    }
    /// Set value of S0
    #[inline(always)]
    fn set_s0(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Normal::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Normal::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..4].store_le(value);
        Ok(())
    }
    
    /// Set value of S0
    #[inline(always)]
    pub fn set_m0(&mut self, value: NormalS0M0) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_s0(0)?;
        Ok(())
    }
    
    /// Set value of S0
    #[inline(always)]
    pub fn set_m1(&mut self, value: NormalS0M1) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_s0(1)?;
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Normal {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Normal {
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
/// Defined values for multiplexed signal Normal
#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
pub enum NormalS0Index {
    M0(NormalS0M0),
    M1(NormalS0M1),
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
pub struct NormalS0M0 { raw: [u8; 8] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl NormalS0M0 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// S1
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn s1(&self) -> i8 {
    self.s1_raw()
}

/// Get raw value of S1
///
/// - Start bit: 4
/// - Signal size: 4 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Signed
#[inline(always)]
pub fn s1_raw(&self) -> i8 {
    let signal = self.raw.view_bits::<Lsb0>()[4..8].load_le::<i8>();
    
    let factor = 1;
    let signal = signal as i8;
    i8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of S1
#[inline(always)]
pub fn set_s1(&mut self, value: i8) -> Result<(), CanError> {
    if value < 0_i8 || 0_i8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Normal::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Normal::MESSAGE_ID })?;
    let value = (value / factor) as i8;
    
    let value = u8::from_ne_bytes(value.to_ne_bytes());
    self.raw.view_bits_mut::<Lsb0>()[4..8].store_le(value);
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
pub struct NormalS0M1 { raw: [u8; 8] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl NormalS0M1 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// S2
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn s2(&self) -> i8 {
    self.s2_raw()
}

/// Get raw value of S2
///
/// - Start bit: 8
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Signed
#[inline(always)]
pub fn s2_raw(&self) -> i8 {
    let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<i8>();
    
    let factor = 1;
    let signal = signal as i8;
    i8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of S2
#[inline(always)]
pub fn set_s2(&mut self, value: i8) -> Result<(), CanError> {
    if value < 0_i8 || 0_i8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Normal::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Normal::MESSAGE_ID })?;
    let value = (value / factor) as i8;
    
    let value = u8::from_ne_bytes(value.to_ne_bytes());
    self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
    Ok(())
}

}


/// Extended
///
/// - Extended ID: 201391870 (0xc00fefe)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Extended {
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
impl Extended {
    pub const MESSAGE_ID: embedded_can::Id = Id::Extended(unsafe { ExtendedId::new_unchecked(0xc00fefe)});
    
    pub const S8_MIN: i8 = 0_i8;
    pub const S8_MAX: i8 = 0_i8;
    pub const S7_MIN: i32 = 0_i32;
    pub const S7_MAX: i32 = 0_i32;
    pub const S6_MIN: i8 = 0_i8;
    pub const S6_MAX: i8 = 0_i8;
    pub const S5_MIN: i32 = 0_i32;
    pub const S5_MAX: i32 = 0_i32;
    pub const S4_MIN: i32 = 0_i32;
    pub const S4_MAX: i32 = 0_i32;
    pub const S3_MIN: i16 = 0_i16;
    pub const S3_MAX: i16 = 0_i16;
    pub const S2_MIN: i8 = 0_i8;
    pub const S2_MAX: i8 = 0_i8;
    pub const S1_MIN: i8 = 0_i8;
    pub const S1_MAX: i8 = 0_i8;
    pub const S0_MIN: i8 = 0_i8;
    pub const S0_MAX: i8 = 0_i8;
    
    /// Construct new Extended from values
    pub fn new(s6: i8, s0: i8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_s6(s6)?;
        res.set_s0(s0)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// Get raw value of S6
    ///
    /// - Start bit: 32
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn s6_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[32..40].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    pub fn s6(&mut self) -> Result<ExtendedS6Index, CanError> {
        match self.s6_raw() {
            0 => Ok(ExtendedS6Index::M0(ExtendedS6M0{ raw: self.raw })),
            1 => Ok(ExtendedS6Index::M1(ExtendedS6M1{ raw: self.raw })),
            2 => Ok(ExtendedS6Index::M2(ExtendedS6M2{ raw: self.raw })),
            multiplexor => Err(CanError::InvalidMultiplexor { message_id: Extended::MESSAGE_ID, multiplexor: multiplexor.into() }),
        }
    }
    /// Set value of S6
    #[inline(always)]
    fn set_s6(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Extended::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Extended::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..40].store_le(value);
        Ok(())
    }
    
    /// Set value of S6
    #[inline(always)]
    pub fn set_m0(&mut self, value: ExtendedS6M0) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_s6(0)?;
        Ok(())
    }
    
    /// Set value of S6
    #[inline(always)]
    pub fn set_m1(&mut self, value: ExtendedS6M1) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_s6(1)?;
        Ok(())
    }
    
    /// Set value of S6
    #[inline(always)]
    pub fn set_m2(&mut self, value: ExtendedS6M2) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_s6(2)?;
        Ok(())
    }
    
    /// Get raw value of S0
    ///
    /// - Start bit: 0
    /// - Signal size: 4 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn s0_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..4].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    pub fn s0(&mut self) -> Result<ExtendedS0Index, CanError> {
        match self.s0_raw() {
            0 => Ok(ExtendedS0Index::M0(ExtendedS0M0{ raw: self.raw })),
            1 => Ok(ExtendedS0Index::M1(ExtendedS0M1{ raw: self.raw })),
            2 => Ok(ExtendedS0Index::M2(ExtendedS0M2{ raw: self.raw })),
            multiplexor => Err(CanError::InvalidMultiplexor { message_id: Extended::MESSAGE_ID, multiplexor: multiplexor.into() }),
        }
    }
    /// Set value of S0
    #[inline(always)]
    fn set_s0(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Extended::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Extended::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..4].store_le(value);
        Ok(())
    }
    
    /// Set value of S0
    #[inline(always)]
    pub fn set_m0(&mut self, value: ExtendedS0M0) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_s0(0)?;
        Ok(())
    }
    
    /// Set value of S0
    #[inline(always)]
    pub fn set_m1(&mut self, value: ExtendedS0M1) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_s0(1)?;
        Ok(())
    }
    
    /// Set value of S0
    #[inline(always)]
    pub fn set_m2(&mut self, value: ExtendedS0M2) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_s0(2)?;
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Extended {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Extended {
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
/// Defined values for multiplexed signal Extended
#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
pub enum ExtendedS6Index {
    M0(ExtendedS6M0),
    M1(ExtendedS6M1),
    M2(ExtendedS6M2),
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
pub struct ExtendedS6M0 { raw: [u8; 8] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl ExtendedS6M0 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// S3
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn s3(&self) -> i16 {
    self.s3_raw()
}

/// Get raw value of S3
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Signed
#[inline(always)]
pub fn s3_raw(&self) -> i16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<i16>();
    
    let factor = 1;
    let signal = signal as i16;
    i16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of S3
#[inline(always)]
pub fn set_s3(&mut self, value: i16) -> Result<(), CanError> {
    if value < 0_i16 || 0_i16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Extended::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Extended::MESSAGE_ID })?;
    let value = (value / factor) as i16;
    
    let value = u16::from_ne_bytes(value.to_ne_bytes());
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
    Ok(())
}

/// S2
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn s2(&self) -> i8 {
    self.s2_raw()
}

/// Get raw value of S2
///
/// - Start bit: 8
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Signed
#[inline(always)]
pub fn s2_raw(&self) -> i8 {
    let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<i8>();
    
    let factor = 1;
    let signal = signal as i8;
    i8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of S2
#[inline(always)]
pub fn set_s2(&mut self, value: i8) -> Result<(), CanError> {
    if value < 0_i8 || 0_i8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Extended::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Extended::MESSAGE_ID })?;
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
pub struct ExtendedS6M1 { raw: [u8; 8] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl ExtendedS6M1 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// S7
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn s7(&self) -> i32 {
    self.s7_raw()
}

/// Get raw value of S7
///
/// - Start bit: 40
/// - Signal size: 24 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Signed
#[inline(always)]
pub fn s7_raw(&self) -> i32 {
    let signal = self.raw.view_bits::<Lsb0>()[40..64].load_le::<i32>();
    
    let factor = 1;
    let signal = signal as i32;
    i32::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of S7
#[inline(always)]
pub fn set_s7(&mut self, value: i32) -> Result<(), CanError> {
    if value < 0_i32 || 0_i32 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Extended::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Extended::MESSAGE_ID })?;
    let value = (value / factor) as i32;
    
    let value = u32::from_ne_bytes(value.to_ne_bytes());
    self.raw.view_bits_mut::<Lsb0>()[40..64].store_le(value);
    Ok(())
}

/// S5
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn s5(&self) -> i32 {
    self.s5_raw()
}

/// Get raw value of S5
///
/// - Start bit: 4
/// - Signal size: 28 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Signed
#[inline(always)]
pub fn s5_raw(&self) -> i32 {
    let signal = self.raw.view_bits::<Lsb0>()[4..32].load_le::<i32>();
    
    let factor = 1;
    let signal = signal as i32;
    i32::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of S5
#[inline(always)]
pub fn set_s5(&mut self, value: i32) -> Result<(), CanError> {
    if value < 0_i32 || 0_i32 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Extended::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Extended::MESSAGE_ID })?;
    let value = (value / factor) as i32;
    
    let value = u32::from_ne_bytes(value.to_ne_bytes());
    self.raw.view_bits_mut::<Lsb0>()[4..32].store_le(value);
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
pub struct ExtendedS6M2 { raw: [u8; 8] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl ExtendedS6M2 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// S8
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn s8(&self) -> i8 {
    self.s8_raw()
}

/// Get raw value of S8
///
/// - Start bit: 40
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Signed
#[inline(always)]
pub fn s8_raw(&self) -> i8 {
    let signal = self.raw.view_bits::<Lsb0>()[40..48].load_le::<i8>();
    
    let factor = 1;
    let signal = signal as i8;
    i8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of S8
#[inline(always)]
pub fn set_s8(&mut self, value: i8) -> Result<(), CanError> {
    if value < 0_i8 || 0_i8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Extended::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Extended::MESSAGE_ID })?;
    let value = (value / factor) as i8;
    
    let value = u8::from_ne_bytes(value.to_ne_bytes());
    self.raw.view_bits_mut::<Lsb0>()[40..48].store_le(value);
    Ok(())
}

/// S4
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn s4(&self) -> i32 {
    self.s4_raw()
}

/// Get raw value of S4
///
/// - Start bit: 8
/// - Signal size: 24 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Signed
#[inline(always)]
pub fn s4_raw(&self) -> i32 {
    let signal = self.raw.view_bits::<Lsb0>()[8..32].load_le::<i32>();
    
    let factor = 1;
    let signal = signal as i32;
    i32::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of S4
#[inline(always)]
pub fn set_s4(&mut self, value: i32) -> Result<(), CanError> {
    if value < 0_i32 || 0_i32 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Extended::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Extended::MESSAGE_ID })?;
    let value = (value / factor) as i32;
    
    let value = u32::from_ne_bytes(value.to_ne_bytes());
    self.raw.view_bits_mut::<Lsb0>()[8..32].store_le(value);
    Ok(())
}

}


/// ExtendedTypes
///
/// - Extended ID: 201588478 (0xc03fefe)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct ExtendedTypes {
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
impl ExtendedTypes {
    pub const MESSAGE_ID: embedded_can::Id = Id::Extended(unsafe { ExtendedId::new_unchecked(0xc03fefe)});
    
    pub const S9_MIN: i32 = -1.34_i32;
    pub const S9_MAX: i32 = 1235_i32;
    pub const S10_MIN: i32 = -340000000000000000000000000000000000000_i32;
    pub const S10_MAX: i32 = 340000000000000000000000000000000000000_i32;
    pub const S0_MIN: i8 = 0_i8;
    pub const S0_MAX: i8 = 0_i8;
    pub const S11_MIN: u8 = 2_u8;
    pub const S11_MAX: u8 = 6_u8;
    
    /// Construct new ExtendedTypes from values
    pub fn new(s11: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_s11(s11)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// Get raw value of S11
    ///
    /// - Start bit: 0
    /// - Signal size: 5 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn s11_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..5].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    pub fn s11(&mut self) -> Result<ExtendedTypesS11Index, CanError> {
        match self.s11_raw() {
            0 => Ok(ExtendedTypesS11Index::M0(ExtendedTypesS11M0{ raw: self.raw })),
            5 => Ok(ExtendedTypesS11Index::M5(ExtendedTypesS11M5{ raw: self.raw })),
            multiplexor => Err(CanError::InvalidMultiplexor { message_id: ExtendedTypes::MESSAGE_ID, multiplexor: multiplexor.into() }),
        }
    }
    /// Set value of S11
    #[inline(always)]
    fn set_s11(&mut self, value: u8) -> Result<(), CanError> {
        if value < 2_u8 || 6_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: ExtendedTypes::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: ExtendedTypes::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..5].store_le(value);
        Ok(())
    }
    
    /// Set value of S11
    #[inline(always)]
    pub fn set_m0(&mut self, value: ExtendedTypesS11M0) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_s11(0)?;
        Ok(())
    }
    
    /// Set value of S11
    #[inline(always)]
    pub fn set_m5(&mut self, value: ExtendedTypesS11M5) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_s11(5)?;
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for ExtendedTypes {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for ExtendedTypes {
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
/// Defined values for multiplexed signal ExtendedTypes
#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
pub enum ExtendedTypesS11Index {
    M0(ExtendedTypesS11M0),
    M5(ExtendedTypesS11M5),
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
pub struct ExtendedTypesS11M0 { raw: [u8; 8] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl ExtendedTypesS11M0 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// S10
///
/// - Min: -340000000000000000000000000000000000000
/// - Max: 340000000000000000000000000000000000000
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn s10(&self) -> i32 {
    self.s10_raw()
}

/// Get raw value of S10
///
/// - Start bit: 16
/// - Signal size: 32 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Signed
#[inline(always)]
pub fn s10_raw(&self) -> i32 {
    let signal = self.raw.view_bits::<Lsb0>()[16..48].load_le::<i32>();
    
    let factor = 1;
    let signal = signal as i32;
    i32::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of S10
#[inline(always)]
pub fn set_s10(&mut self, value: i32) -> Result<(), CanError> {
    if value < -340000000000000000000000000000000000000_i32 || 340000000000000000000000000000000000000_i32 < value {
        return Err(CanError::ParameterOutOfRange { message_id: ExtendedTypes::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: ExtendedTypes::MESSAGE_ID })?;
    let value = (value / factor) as i32;
    
    let value = u32::from_ne_bytes(value.to_ne_bytes());
    self.raw.view_bits_mut::<Lsb0>()[16..48].store_le(value);
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
pub struct ExtendedTypesS11M5 { raw: [u8; 8] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl ExtendedTypesS11M5 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// S9
///
/// - Min: -1.34
/// - Max: 1235
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn s9(&self) -> i32 {
    self.s9_raw()
}

/// Get raw value of S9
///
/// - Start bit: 24
/// - Signal size: 32 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Signed
#[inline(always)]
pub fn s9_raw(&self) -> i32 {
    let signal = self.raw.view_bits::<Lsb0>()[24..56].load_le::<i32>();
    
    let factor = 1;
    let signal = signal as i32;
    i32::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of S9
#[inline(always)]
pub fn set_s9(&mut self, value: i32) -> Result<(), CanError> {
    if value < -1.34_i32 || 1235_i32 < value {
        return Err(CanError::ParameterOutOfRange { message_id: ExtendedTypes::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: ExtendedTypes::MESSAGE_ID })?;
    let value = (value / factor) as i32;
    
    let value = u32::from_ne_bytes(value.to_ne_bytes());
    self.raw.view_bits_mut::<Lsb0>()[24..56].store_le(value);
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

