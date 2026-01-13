// Generated code!
//
// Message definitions from file `long_names`
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
    /// SS123456789012345678901234587890
    Ss123456789012345678901234587890(Ss123456789012345678901234587890),
    /// SS1234567890123456789012345_0000
    Ss12345678901234567890123450000(Ss12345678901234567890123450000),
    /// SS1234567890123456789012345_0001
    Ss12345678901234567890123450001(Ss12345678901234567890123450001),
    /// SS123456789012345678901234577890
    Ss123456789012345678901234577890(Ss123456789012345678901234577890),
    /// SS123456789012345678901234567890
    Ss123456789012345678901234567890(Ss123456789012345678901234567890),
    /// S1234567890123456789012345678901
    S1234567890123456789012345678901(S1234567890123456789012345678901),
    /// M12345678901234567890123456_0000
    M123456789012345678901234560000(M123456789012345678901234560000),
    /// M1234567890123456789012345678901
    M1234567890123456789012345678901(M1234567890123456789012345678901),
    /// M12345678901234567890123456_0001
    M123456789012345678901234560001(M123456789012345678901234560001),
    /// MM123456789012345678901234567890
    Mm123456789012345678901234567890(Mm123456789012345678901234567890),
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
            Ss123456789012345678901234587890::MESSAGE_ID => Messages::Ss123456789012345678901234587890(Ss123456789012345678901234587890::try_from(payload)?),
            Ss12345678901234567890123450000::MESSAGE_ID => Messages::Ss12345678901234567890123450000(Ss12345678901234567890123450000::try_from(payload)?),
            Ss12345678901234567890123450001::MESSAGE_ID => Messages::Ss12345678901234567890123450001(Ss12345678901234567890123450001::try_from(payload)?),
            Ss123456789012345678901234577890::MESSAGE_ID => Messages::Ss123456789012345678901234577890(Ss123456789012345678901234577890::try_from(payload)?),
            Ss123456789012345678901234567890::MESSAGE_ID => Messages::Ss123456789012345678901234567890(Ss123456789012345678901234567890::try_from(payload)?),
            S1234567890123456789012345678901::MESSAGE_ID => Messages::S1234567890123456789012345678901(S1234567890123456789012345678901::try_from(payload)?),
            M123456789012345678901234560000::MESSAGE_ID => Messages::M123456789012345678901234560000(M123456789012345678901234560000::try_from(payload)?),
            M1234567890123456789012345678901::MESSAGE_ID => Messages::M1234567890123456789012345678901(M1234567890123456789012345678901::try_from(payload)?),
            M123456789012345678901234560001::MESSAGE_ID => Messages::M123456789012345678901234560001(M123456789012345678901234560001::try_from(payload)?),
            Mm123456789012345678901234567890::MESSAGE_ID => Messages::Mm123456789012345678901234567890(Mm123456789012345678901234567890::try_from(payload)?),
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}

/// SS123456789012345678901234587890
///
/// - Standard ID: 9 (0x9)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Ss123456789012345678901234587890 {
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
impl Ss123456789012345678901234587890 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x9)});
    
    
    /// Construct new SS123456789012345678901234587890 from values
    pub fn new() -> Result<Self, CanError> {
        let res = Self { raw: [0u8; 8] };
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
}

impl core::convert::TryFrom<&[u8]> for Ss123456789012345678901234587890 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Ss123456789012345678901234587890 {
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

/// SS1234567890123456789012345_0000
///
/// - Standard ID: 8 (0x8)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Ss12345678901234567890123450000 {
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
impl Ss12345678901234567890123450000 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x8)});
    
    
    /// Construct new SS1234567890123456789012345_0000 from values
    pub fn new() -> Result<Self, CanError> {
        let res = Self { raw: [0u8; 8] };
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
}

impl core::convert::TryFrom<&[u8]> for Ss12345678901234567890123450000 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Ss12345678901234567890123450000 {
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

/// SS1234567890123456789012345_0001
///
/// - Standard ID: 7 (0x7)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Ss12345678901234567890123450001 {
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
impl Ss12345678901234567890123450001 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x7)});
    
    
    /// Construct new SS1234567890123456789012345_0001 from values
    pub fn new() -> Result<Self, CanError> {
        let res = Self { raw: [0u8; 8] };
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
}

impl core::convert::TryFrom<&[u8]> for Ss12345678901234567890123450001 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Ss12345678901234567890123450001 {
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

/// SS123456789012345678901234577890
///
/// - Standard ID: 6 (0x6)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Ss123456789012345678901234577890 {
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
impl Ss123456789012345678901234577890 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x6)});
    
    
    /// Construct new SS123456789012345678901234577890 from values
    pub fn new() -> Result<Self, CanError> {
        let res = Self { raw: [0u8; 8] };
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
}

impl core::convert::TryFrom<&[u8]> for Ss123456789012345678901234577890 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Ss123456789012345678901234577890 {
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

/// SS123456789012345678901234567890
///
/// - Standard ID: 5 (0x5)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Ss123456789012345678901234567890 {
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
impl Ss123456789012345678901234567890 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x5)});
    
    
    /// Construct new SS123456789012345678901234567890 from values
    pub fn new() -> Result<Self, CanError> {
        let res = Self { raw: [0u8; 8] };
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
}

impl core::convert::TryFrom<&[u8]> for Ss123456789012345678901234567890 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Ss123456789012345678901234567890 {
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

/// S1234567890123456789012345678901
///
/// - Standard ID: 4 (0x4)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct S1234567890123456789012345678901 {
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
impl S1234567890123456789012345678901 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x4)});
    
    pub const SS123456789012345678901234567890_MIN: i8 = 0_i8;
    pub const SS123456789012345678901234567890_MAX: i8 = 0_i8;
    
    /// Construct new S1234567890123456789012345678901 from values
    pub fn new(ss123456789012345678901234567890: i8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_ss123456789012345678901234567890(ss123456789012345678901234567890)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// SS123456789012345678901234567890
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn ss123456789012345678901234567890(&self) -> i8 {
        self.ss123456789012345678901234567890_raw()
    }
    
    /// Get raw value of SS123456789012345678901234567890
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn ss123456789012345678901234567890_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of SS123456789012345678901234567890
    #[inline(always)]
    pub fn set_ss123456789012345678901234567890(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: S1234567890123456789012345678901::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: S1234567890123456789012345678901::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for S1234567890123456789012345678901 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for S1234567890123456789012345678901 {
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

/// M12345678901234567890123456_0000
///
/// - Standard ID: 0 (0x0)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct M123456789012345678901234560000 {
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
impl M123456789012345678901234560000 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x0)});
    
    pub const SSS12345678901234567890123456789_MIN: i8 = 0_i8;
    pub const SSS12345678901234567890123456789_MAX: i8 = 0_i8;
    
    /// Construct new M12345678901234567890123456_0000 from values
    pub fn new(sss12345678901234567890123456789: i8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_sss12345678901234567890123456789(sss12345678901234567890123456789)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// SSS12345678901234567890123456789
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn sss12345678901234567890123456789(&self) -> i8 {
        self.sss12345678901234567890123456789_raw()
    }
    
    /// Get raw value of SSS12345678901234567890123456789
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn sss12345678901234567890123456789_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of SSS12345678901234567890123456789
    #[inline(always)]
    pub fn set_sss12345678901234567890123456789(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: M123456789012345678901234560000::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: M123456789012345678901234560000::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for M123456789012345678901234560000 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for M123456789012345678901234560000 {
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

/// M1234567890123456789012345678901
///
/// - Standard ID: 1 (0x1)
/// - Size: 8 bytes
/// - Transmitter: N1234567890123456789012345678901
#[derive(Clone, Copy)]
pub struct M1234567890123456789012345678901 {
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
impl M1234567890123456789012345678901 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x1)});
    
    pub const SS1234567890123456789012345_0000_MIN: i8 = 0_i8;
    pub const SS1234567890123456789012345_0000_MAX: i8 = 0_i8;
    pub const SS1234567890123456789012345_0001_MIN: i8 = 0_i8;
    pub const SS1234567890123456789012345_0001_MAX: i8 = 0_i8;
    pub const SS1234567890123456789012345_0002_MIN: i8 = 0_i8;
    pub const SS1234567890123456789012345_0002_MAX: i8 = 0_i8;
    pub const S12345678901234567890123456_0000_MIN: i8 = 0_i8;
    pub const S12345678901234567890123456_0000_MAX: i8 = 0_i8;
    pub const S1234567890123456789012345678901_MIN: i8 = 0_i8;
    pub const S1234567890123456789012345678901_MAX: i8 = 0_i8;
    
    /// Construct new M1234567890123456789012345678901 from values
    pub fn new(ss1234567890123456789012345_0000: i8, ss1234567890123456789012345_0001: i8, ss1234567890123456789012345_0002: i8, s12345678901234567890123456_0000: i8, s1234567890123456789012345678901: i8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_ss1234567890123456789012345_0000(ss1234567890123456789012345_0000)?;
        res.set_ss1234567890123456789012345_0001(ss1234567890123456789012345_0001)?;
        res.set_ss1234567890123456789012345_0002(ss1234567890123456789012345_0002)?;
        res.set_s12345678901234567890123456_0000(s12345678901234567890123456_0000)?;
        res.set_s1234567890123456789012345678901(s1234567890123456789012345678901)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// SS1234567890123456789012345_0000
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn ss1234567890123456789012345_0000(&self) -> i8 {
        self.ss1234567890123456789012345_0000_raw()
    }
    
    /// Get raw value of SS1234567890123456789012345_0000
    ///
    /// - Start bit: 32
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn ss1234567890123456789012345_0000_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[32..40].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of SS1234567890123456789012345_0000
    #[inline(always)]
    pub fn set_ss1234567890123456789012345_0000(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: M1234567890123456789012345678901::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: M1234567890123456789012345678901::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..40].store_le(value);
        Ok(())
    }
    
    /// SS1234567890123456789012345_0001
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn ss1234567890123456789012345_0001(&self) -> i8 {
        self.ss1234567890123456789012345_0001_raw()
    }
    
    /// Get raw value of SS1234567890123456789012345_0001
    ///
    /// - Start bit: 24
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn ss1234567890123456789012345_0001_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[24..32].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of SS1234567890123456789012345_0001
    #[inline(always)]
    pub fn set_ss1234567890123456789012345_0001(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: M1234567890123456789012345678901::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: M1234567890123456789012345678901::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[24..32].store_le(value);
        Ok(())
    }
    
    /// SS1234567890123456789012345_0002
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: N12345678901234567890123456_0000
    #[inline(always)]
    pub fn ss1234567890123456789012345_0002(&self) -> i8 {
        self.ss1234567890123456789012345_0002_raw()
    }
    
    /// Get raw value of SS1234567890123456789012345_0002
    ///
    /// - Start bit: 16
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn ss1234567890123456789012345_0002_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[16..24].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of SS1234567890123456789012345_0002
    #[inline(always)]
    pub fn set_ss1234567890123456789012345_0002(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: M1234567890123456789012345678901::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: M1234567890123456789012345678901::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[16..24].store_le(value);
        Ok(())
    }
    
    /// S12345678901234567890123456_0000
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn s12345678901234567890123456_0000(&self) -> i8 {
        self.s12345678901234567890123456_0000_raw()
    }
    
    /// Get raw value of S12345678901234567890123456_0000
    ///
    /// - Start bit: 8
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn s12345678901234567890123456_0000_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of S12345678901234567890123456_0000
    #[inline(always)]
    pub fn set_s12345678901234567890123456_0000(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: M1234567890123456789012345678901::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: M1234567890123456789012345678901::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
        Ok(())
    }
    
    /// S1234567890123456789012345678901
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn s1234567890123456789012345678901(&self) -> i8 {
        self.s1234567890123456789012345678901_raw()
    }
    
    /// Get raw value of S1234567890123456789012345678901
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn s1234567890123456789012345678901_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of S1234567890123456789012345678901
    #[inline(always)]
    pub fn set_s1234567890123456789012345678901(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: M1234567890123456789012345678901::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: M1234567890123456789012345678901::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for M1234567890123456789012345678901 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for M1234567890123456789012345678901 {
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

/// M12345678901234567890123456_0001
///
/// - Standard ID: 2 (0x2)
/// - Size: 8 bytes
/// - Transmitter: N12345678901234567890123456_0001
#[derive(Clone, Copy)]
pub struct M123456789012345678901234560001 {
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
impl M123456789012345678901234560001 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x2)});
    
    pub const SS1234567890123456789012345_0003_MIN: i8 = 0_i8;
    pub const SS1234567890123456789012345_0003_MAX: i8 = 0_i8;
    pub const SS1234567890123456789012345_0004_MIN: i8 = 0_i8;
    pub const SS1234567890123456789012345_0004_MAX: i8 = 0_i8;
    pub const S12345678901234567890123456_0001_MIN: i8 = 0_i8;
    pub const S12345678901234567890123456_0001_MAX: i8 = 0_i8;
    pub const S12345678901234567890123456_0002_MIN: i8 = 0_i8;
    pub const S12345678901234567890123456_0002_MAX: i8 = 0_i8;
    
    /// Construct new M12345678901234567890123456_0001 from values
    pub fn new(ss1234567890123456789012345_0003: i8, ss1234567890123456789012345_0004: i8, s12345678901234567890123456_0001: i8, s12345678901234567890123456_0002: i8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_ss1234567890123456789012345_0003(ss1234567890123456789012345_0003)?;
        res.set_ss1234567890123456789012345_0004(ss1234567890123456789012345_0004)?;
        res.set_s12345678901234567890123456_0001(s12345678901234567890123456_0001)?;
        res.set_s12345678901234567890123456_0002(s12345678901234567890123456_0002)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// SS1234567890123456789012345_0003
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn ss1234567890123456789012345_0003(&self) -> i8 {
        self.ss1234567890123456789012345_0003_raw()
    }
    
    /// Get raw value of SS1234567890123456789012345_0003
    ///
    /// - Start bit: 24
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn ss1234567890123456789012345_0003_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[24..32].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of SS1234567890123456789012345_0003
    #[inline(always)]
    pub fn set_ss1234567890123456789012345_0003(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: M123456789012345678901234560001::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: M123456789012345678901234560001::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[24..32].store_le(value);
        Ok(())
    }
    
    /// SS1234567890123456789012345_0004
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn ss1234567890123456789012345_0004(&self) -> i8 {
        self.ss1234567890123456789012345_0004_raw()
    }
    
    /// Get raw value of SS1234567890123456789012345_0004
    ///
    /// - Start bit: 16
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn ss1234567890123456789012345_0004_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[16..24].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of SS1234567890123456789012345_0004
    #[inline(always)]
    pub fn set_ss1234567890123456789012345_0004(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: M123456789012345678901234560001::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: M123456789012345678901234560001::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[16..24].store_le(value);
        Ok(())
    }
    
    /// S12345678901234567890123456_0001
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn s12345678901234567890123456_0001(&self) -> i8 {
        self.s12345678901234567890123456_0001_raw()
    }
    
    /// Get raw value of S12345678901234567890123456_0001
    ///
    /// - Start bit: 8
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn s12345678901234567890123456_0001_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of S12345678901234567890123456_0001
    #[inline(always)]
    pub fn set_s12345678901234567890123456_0001(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: M123456789012345678901234560001::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: M123456789012345678901234560001::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
        Ok(())
    }
    
    /// S12345678901234567890123456_0002
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn s12345678901234567890123456_0002(&self) -> i8 {
        self.s12345678901234567890123456_0002_raw()
    }
    
    /// Get raw value of S12345678901234567890123456_0002
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn s12345678901234567890123456_0002_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of S12345678901234567890123456_0002
    #[inline(always)]
    pub fn set_s12345678901234567890123456_0002(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: M123456789012345678901234560001::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: M123456789012345678901234560001::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for M123456789012345678901234560001 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for M123456789012345678901234560001 {
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

/// MM123456789012345678901234567890
///
/// - Standard ID: 3 (0x3)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Mm123456789012345678901234567890 {
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
impl Mm123456789012345678901234567890 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x3)});
    
    pub const SSS123456789012345678901234_0000_MIN: i8 = 0_i8;
    pub const SSS123456789012345678901234_0000_MAX: i8 = 0_i8;
    pub const SS1234567890123456789012345_0005_MIN: i8 = 0_i8;
    pub const SS1234567890123456789012345_0005_MAX: i8 = 0_i8;
    
    /// Construct new MM123456789012345678901234567890 from values
    pub fn new(sss123456789012345678901234_0000: i8, ss1234567890123456789012345_0005: i8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_sss123456789012345678901234_0000(sss123456789012345678901234_0000)?;
        res.set_ss1234567890123456789012345_0005(ss1234567890123456789012345_0005)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// SSS123456789012345678901234_0000
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn sss123456789012345678901234_0000(&self) -> i8 {
        self.sss123456789012345678901234_0000_raw()
    }
    
    /// Get raw value of SSS123456789012345678901234_0000
    ///
    /// - Start bit: 8
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn sss123456789012345678901234_0000_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of SSS123456789012345678901234_0000
    #[inline(always)]
    pub fn set_sss123456789012345678901234_0000(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Mm123456789012345678901234567890::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Mm123456789012345678901234567890::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
        Ok(())
    }
    
    /// SS1234567890123456789012345_0005
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn ss1234567890123456789012345_0005(&self) -> i8 {
        self.ss1234567890123456789012345_0005_raw()
    }
    
    /// Get raw value of SS1234567890123456789012345_0005
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn ss1234567890123456789012345_0005_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of SS1234567890123456789012345_0005
    #[inline(always)]
    pub fn set_ss1234567890123456789012345_0005(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Mm123456789012345678901234567890::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Mm123456789012345678901234567890::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Mm123456789012345678901234567890 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Mm123456789012345678901234567890 {
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

