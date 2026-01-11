// Generated code!
//
// Message definitions from file `comments_hex_and_motorola_converted_from_sym`
// Version: 6.0

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
    /// Msg1
    Msg1(Msg1),
    /// Msg2
    Msg2(Msg2),
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
            Msg1::MESSAGE_ID => Messages::Msg1(Msg1::try_from(payload)?),
            Msg2::MESSAGE_ID => Messages::Msg2(Msg2::try_from(payload)?),
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}

/// Msg1
///
/// - Standard ID: 1568 (0x620)
/// - Size: 2 bytes
/// - Transmitter: Peripherals
#[derive(Clone, Copy)]
pub struct Msg1 {
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
impl Msg1 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x620)});
    
    pub const SIG22_MIN: u8 = 0_u8;
    pub const SIG22_MAX: u8 = 1_u8;
    pub const SIG12_MIN: u8 = 0_u8;
    pub const SIG12_MAX: u8 = 1_u8;
    pub const SIG1_MIN: u8 = 0_u8;
    pub const SIG1_MAX: u8 = 0_u8;
    
    /// Construct new Msg1 from values
    pub fn new(sig1: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 2] };
        res.set_sig1(sig1)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 2] {
        &self.raw
    }
    
    /// Get raw value of sig1
    ///
    /// - Start bit: 7
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn sig1_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[0..8].load_be::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    pub fn sig1(&mut self) -> Result<Msg1Sig1Index, CanError> {
        match self.sig1_raw() {
            1 => Ok(Msg1Sig1Index::M1(Msg1Sig1M1{ raw: self.raw })),
            2 => Ok(Msg1Sig1Index::M2(Msg1Sig1M2{ raw: self.raw })),
            multiplexor => Err(CanError::InvalidMultiplexor { message_id: Msg1::MESSAGE_ID, multiplexor: multiplexor.into() }),
        }
    }
    /// Set value of sig1
    #[inline(always)]
    fn set_sig1(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Msg1::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Msg1::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Msb0>()[0..8].store_be(value);
        Ok(())
    }
    
    /// Set value of sig1
    #[inline(always)]
    pub fn set_m1(&mut self, value: Msg1Sig1M1) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_sig1(1)?;
        Ok(())
    }
    
    /// Set value of sig1
    #[inline(always)]
    pub fn set_m2(&mut self, value: Msg1Sig1M2) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_sig1(2)?;
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Msg1 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 2 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 2];
        raw.copy_from_slice(&payload[..2]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Msg1 {
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
/// Defined values for multiplexed signal Msg1
#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
pub enum Msg1Sig1Index {
    M1(Msg1Sig1M1),
    M2(Msg1Sig1M2),
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
pub struct Msg1Sig1M1 { raw: [u8; 2] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl Msg1Sig1M1 {
pub fn new() -> Self { Self { raw: [0u8; 2] } }
/// sig12
///
/// another comment for sig1=1
///
/// - Min: 0
/// - Max: 1
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn sig12(&self) -> u8 {
    self.sig12_raw()
}

/// Get raw value of sig12
///
/// - Start bit: 15
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: BigEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn sig12_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Msb0>()[8..16].load_be::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of sig12
#[inline(always)]
pub fn set_sig12(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 1_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Msg1::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Msg1::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Msb0>()[8..16].store_be(value);
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
pub struct Msg1Sig1M2 { raw: [u8; 2] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl Msg1Sig1M2 {
pub fn new() -> Self { Self { raw: [0u8; 2] } }
/// sig22
///
/// another comment for sig1=2
///
/// - Min: 0
/// - Max: 1
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn sig22(&self) -> u8 {
    self.sig22_raw()
}

/// Get raw value of sig22
///
/// - Start bit: 15
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: BigEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn sig22_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Msb0>()[8..16].load_be::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of sig22
#[inline(always)]
pub fn set_sig22(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 1_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Msg1::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Msg1::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Msb0>()[8..16].store_be(value);
    Ok(())
}

}


/// Msg2
///
/// - Standard ID: 1365 (0x555)
/// - Size: 8 bytes
/// - Transmitter: ECU
///
/// test
#[derive(Clone, Copy)]
pub struct Msg2 {
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
impl Msg2 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x555)});
    
    pub const TEST7_MIN: u8 = 0_u8;
    pub const TEST7_MAX: u8 = 0_u8;
    pub const TEST6_MIN: u8 = 0_u8;
    pub const TEST6_MAX: u8 = 0_u8;
    pub const TEST5_MIN: u8 = 0_u8;
    pub const TEST5_MAX: u8 = 0_u8;
    pub const TEST4_MIN: u8 = 0_u8;
    pub const TEST4_MAX: u8 = 0_u8;
    pub const TEST3_MIN: u8 = 0_u8;
    pub const TEST3_MAX: u8 = 0_u8;
    pub const TEST2_MIN: u8 = 0_u8;
    pub const TEST2_MAX: u8 = 0_u8;
    pub const TEST1_MIN: u8 = 0_u8;
    pub const TEST1_MAX: u8 = 0_u8;
    pub const TEST0_MIN: u8 = 0_u8;
    pub const TEST0_MAX: u8 = 0_u8;
    
    /// Construct new Msg2 from values
    pub fn new(test7: u8, test6: u8, test5: u8, test4: u8, test3: u8, test2: u8, test1: u8, test0: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_test7(test7)?;
        res.set_test6(test6)?;
        res.set_test5(test5)?;
        res.set_test4(test4)?;
        res.set_test3(test3)?;
        res.set_test2(test2)?;
        res.set_test1(test1)?;
        res.set_test0(test0)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// Test7
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn test7(&self) -> Msg2Test7 {
        let signal = self.raw.view_bits::<Msb0>()[56..64].load_be::<u8>();
        
        match signal {
            1 => Msg2Test7::A,
            2 => Msg2Test7::B,
            3 => Msg2Test7::C,
            4 => Msg2Test7::D,
            _ => Msg2Test7::_Other(self.test7_raw()),
        }
    }
    
    /// Get raw value of Test7
    ///
    /// - Start bit: 63
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn test7_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[56..64].load_be::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Test7
    #[inline(always)]
    pub fn set_test7(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Msg2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Msg2::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Msb0>()[56..64].store_be(value);
        Ok(())
    }
    
    /// Test6
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn test6(&self) -> u8 {
        self.test6_raw()
    }
    
    /// Get raw value of Test6
    ///
    /// - Start bit: 55
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn test6_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[48..56].load_be::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Test6
    #[inline(always)]
    pub fn set_test6(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Msg2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Msg2::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Msb0>()[48..56].store_be(value);
        Ok(())
    }
    
    /// Test5
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn test5(&self) -> u8 {
        self.test5_raw()
    }
    
    /// Get raw value of Test5
    ///
    /// - Start bit: 47
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn test5_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[40..48].load_be::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Test5
    #[inline(always)]
    pub fn set_test5(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Msg2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Msg2::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Msb0>()[40..48].store_be(value);
        Ok(())
    }
    
    /// Test4
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn test4(&self) -> u8 {
        self.test4_raw()
    }
    
    /// Get raw value of Test4
    ///
    /// - Start bit: 39
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn test4_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[32..40].load_be::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Test4
    #[inline(always)]
    pub fn set_test4(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Msg2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Msg2::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Msb0>()[32..40].store_be(value);
        Ok(())
    }
    
    /// Test3
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn test3(&self) -> u8 {
        self.test3_raw()
    }
    
    /// Get raw value of Test3
    ///
    /// - Start bit: 31
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn test3_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[24..32].load_be::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Test3
    #[inline(always)]
    pub fn set_test3(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Msg2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Msg2::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Msb0>()[24..32].store_be(value);
        Ok(())
    }
    
    /// Test2
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn test2(&self) -> u8 {
        self.test2_raw()
    }
    
    /// Get raw value of Test2
    ///
    /// - Start bit: 23
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn test2_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[16..24].load_be::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Test2
    #[inline(always)]
    pub fn set_test2(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Msg2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Msg2::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Msb0>()[16..24].store_be(value);
        Ok(())
    }
    
    /// Test1
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn test1(&self) -> u8 {
        self.test1_raw()
    }
    
    /// Get raw value of Test1
    ///
    /// - Start bit: 15
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn test1_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[8..16].load_be::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Test1
    #[inline(always)]
    pub fn set_test1(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Msg2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Msg2::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Msb0>()[8..16].store_be(value);
        Ok(())
    }
    
    /// Test0
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn test0(&self) -> u8 {
        self.test0_raw()
    }
    
    /// Get raw value of Test0
    ///
    /// - Start bit: 7
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn test0_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[0..8].load_be::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Test0
    #[inline(always)]
    pub fn set_test0(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Msg2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Msg2::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Msb0>()[0..8].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Msg2 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Msg2 {
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
/// Defined values for Test7
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
pub enum Msg2Test7 {
    A,
    B,
    C,
    D,
    _Other(u8),
}

impl From<Msg2Test7> for u8 {
    fn from(val: Msg2Test7) -> u8 {
        match val {
            Msg2Test7::A => 1,
            Msg2Test7::B => 2,
            Msg2Test7::C => 3,
            Msg2Test7::D => 4,
            Msg2Test7::_Other(x) => x,
        }
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

