// Generated code!
//
// Message definitions from file `issue_168`
// Version: 2.0

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
    /// Foo
    Foo(Foo),
    /// Fum
    Fum(Fum),
    /// Bar
    Bar(Bar),
    /// CanFd
    CanFd(CanFd),
    /// FOOBAR
    Foobar(Foobar),
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
            Foo::MESSAGE_ID => Messages::Foo(Foo::try_from(payload)?),
            Fum::MESSAGE_ID => Messages::Fum(Fum::try_from(payload)?),
            Bar::MESSAGE_ID => Messages::Bar(Bar::try_from(payload)?),
            CanFd::MESSAGE_ID => Messages::CanFd(CanFd::try_from(payload)?),
            Foobar::MESSAGE_ID => Messages::Foobar(Foobar::try_from(payload)?),
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}

/// Foo
///
/// - Extended ID: 74544 (0x12330)
/// - Size: 8 bytes
/// - Transmitter: FOO
///
/// Foo.
#[derive(Clone, Copy)]
pub struct Foo {
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
impl Foo {
    pub const MESSAGE_ID: embedded_can::Id = Id::Extended(unsafe { ExtendedId::new_unchecked(0x12330)});
    
    pub const FOO_MIN: f32 = 229.53_f32;
    pub const FOO_MAX: f32 = 270.47_f32;
    pub const BAR_MIN: f32 = 0_f32;
    pub const BAR_MAX: f32 = 5_f32;
    
    /// Construct new Foo from values
    pub fn new(foo: f32, bar: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_foo(foo)?;
        res.set_bar(bar)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// Foo
    ///
    /// - Min: 229.53
    /// - Max: 270.47
    /// - Unit: "degK"
    /// - Receivers: BAR
    #[inline(always)]
    pub fn foo(&self) -> f32 {
        self.foo_raw()
    }
    
    /// Get raw value of Foo
    ///
    /// - Start bit: 0
    /// - Signal size: 12 bits
    /// - Factor: 0.01
    /// - Offset: 250
    /// - Byte order: BigEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn foo_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Msb0>()[7..19].load_be::<i16>();
        
        let factor = 0.01_f32;
        let offset = 250_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of Foo
    #[inline(always)]
    pub fn set_foo(&mut self, value: f32) -> Result<(), CanError> {
        if value < 229.53_f32 || 270.47_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Foo::MESSAGE_ID });
        }
        let factor = 0.01_f32;
        let offset = 250_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[7..19].store_be(value);
        Ok(())
    }
    
    /// Bar
    ///
    /// Bar.
    ///
    /// - Min: 0
    /// - Max: 5
    /// - Unit: "m"
    /// - Receivers: FOO
    #[inline(always)]
    pub fn bar(&self) -> f32 {
        self.bar_raw()
    }
    
    /// Get raw value of Bar
    ///
    /// - Start bit: 24
    /// - Signal size: 32 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn bar_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Msb0>()[31..63].load_be::<i32>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of Bar
    #[inline(always)]
    pub fn set_bar(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 5_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Foo::MESSAGE_ID });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i32;
        
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[31..63].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Foo {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Foo {
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

/// Fum
///
/// - Extended ID: 74545 (0x12331)
/// - Size: 5 bytes
/// - Transmitter: FOO
#[derive(Clone, Copy)]
pub struct Fum {
    raw: [u8; 5],
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
impl Fum {
    pub const MESSAGE_ID: embedded_can::Id = Id::Extended(unsafe { ExtendedId::new_unchecked(0x12331)});
    
    pub const FUM_MIN: i16 = 0_i16;
    pub const FUM_MAX: i16 = 10_i16;
    pub const FAM_MIN: i16 = 0_i16;
    pub const FAM_MAX: i16 = 8_i16;
    
    /// Construct new Fum from values
    pub fn new(fum: i16, fam: i16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 5] };
        res.set_fum(fum)?;
        res.set_fam(fam)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 5] {
        &self.raw
    }
    
    /// Fum
    ///
    /// - Min: 0
    /// - Max: 10
    /// - Unit: ""
    /// - Receivers: BAR
    #[inline(always)]
    pub fn fum(&self) -> i16 {
        self.fum_raw()
    }
    
    /// Get raw value of Fum
    ///
    /// - Start bit: 0
    /// - Signal size: 12 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn fum_raw(&self) -> i16 {
        let signal = self.raw.view_bits::<Lsb0>()[0..12].load_le::<i16>();
        
        let factor = 1;
        let signal = signal as i16;
        i16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Fum
    #[inline(always)]
    pub fn set_fum(&mut self, value: i16) -> Result<(), CanError> {
        if value < 0_i16 || 10_i16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Fum::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Fum::MESSAGE_ID })?;
        let value = (value / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..12].store_le(value);
        Ok(())
    }
    
    /// Fam
    ///
    /// - Min: 0
    /// - Max: 8
    /// - Unit: ""
    /// - Receivers: BAR
    #[inline(always)]
    pub fn fam(&self) -> FumFam {
        let signal = self.raw.view_bits::<Lsb0>()[12..24].load_le::<u16>();
        
        match signal {
            1 => FumFam::Enabled,
            0 => FumFam::Disabled,
            _ => FumFam::_Other(self.fam_raw()),
        }
    }
    
    /// Get raw value of Fam
    ///
    /// - Start bit: 12
    /// - Signal size: 12 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn fam_raw(&self) -> i16 {
        let signal = self.raw.view_bits::<Lsb0>()[12..24].load_le::<i16>();
        
        let factor = 1;
        let signal = signal as i16;
        i16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Fam
    #[inline(always)]
    pub fn set_fam(&mut self, value: i16) -> Result<(), CanError> {
        if value < 0_i16 || 8_i16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Fum::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Fum::MESSAGE_ID })?;
        let value = (value / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[12..24].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Fum {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 5 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 5];
        raw.copy_from_slice(&payload[..5]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Fum {
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
/// Defined values for Fam
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
pub enum FumFam {
    Enabled,
    Disabled,
    _Other(i16),
}

impl From<FumFam> for i16 {
    fn from(val: FumFam) -> i16 {
        match val {
            FumFam::Enabled => 1,
            FumFam::Disabled => 0,
            FumFam::_Other(x) => x,
        }
    }
}


/// Bar
///
/// - Extended ID: 74546 (0x12332)
/// - Size: 4 bytes
/// - Transmitter: FOO
#[derive(Clone, Copy)]
pub struct Bar {
    raw: [u8; 4],
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
impl Bar {
    pub const MESSAGE_ID: embedded_can::Id = Id::Extended(unsafe { ExtendedId::new_unchecked(0x12332)});
    
    pub const BINARY32_MIN: i32 = 0_i32;
    pub const BINARY32_MAX: i32 = 0_i32;
    
    /// Construct new Bar from values
    pub fn new(binary32: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 4] };
        res.set_binary32(binary32)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 4] {
        &self.raw
    }
    
    /// Binary32
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: FUM
    #[inline(always)]
    pub fn binary32(&self) -> i32 {
        self.binary32_raw()
    }
    
    /// Get raw value of Binary32
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn binary32_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<i32>();
        
        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Binary32
    #[inline(always)]
    pub fn set_binary32(&mut self, value: i32) -> Result<(), CanError> {
        if value < 0_i32 || 0_i32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bar::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Bar::MESSAGE_ID })?;
        let value = (value / factor) as i32;
        
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Bar {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 4 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 4];
        raw.copy_from_slice(&payload[..4]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Bar {
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

/// CanFd
///
/// - Extended ID: 74547 (0x12333)
/// - Size: 64 bytes
/// - Transmitter: FOO
#[derive(Clone, Copy)]
pub struct CanFd {
    raw: [u8; 64],
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
impl CanFd {
    pub const MESSAGE_ID: embedded_can::Id = Id::Extended(unsafe { ExtendedId::new_unchecked(0x12333)});
    
    pub const FIE_MIN: u64 = 0_u64;
    pub const FIE_MAX: u64 = 0_u64;
    pub const FAS_MIN: u64 = 0_u64;
    pub const FAS_MAX: u64 = 0_u64;
    
    /// Construct new CanFd from values
    pub fn new(fie: u64, fas: u64) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 64] };
        res.set_fie(fie)?;
        res.set_fas(fas)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 64] {
        &self.raw
    }
    
    /// Fie
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: FUM
    #[inline(always)]
    pub fn fie(&self) -> u64 {
        self.fie_raw()
    }
    
    /// Get raw value of Fie
    ///
    /// - Start bit: 0
    /// - Signal size: 64 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn fie_raw(&self) -> u64 {
        let signal = self.raw.view_bits::<Lsb0>()[0..64].load_le::<u64>();
        
        let factor = 1;
        u64::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Fie
    #[inline(always)]
    pub fn set_fie(&mut self, value: u64) -> Result<(), CanError> {
        if value < 0_u64 || 0_u64 < value {
            return Err(CanError::ParameterOutOfRange { message_id: CanFd::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: CanFd::MESSAGE_ID })?;
        let value = (value / factor) as u64;
        
        self.raw.view_bits_mut::<Lsb0>()[0..64].store_le(value);
        Ok(())
    }
    
    /// Fas
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn fas(&self) -> u64 {
        self.fas_raw()
    }
    
    /// Get raw value of Fas
    ///
    /// - Start bit: 64
    /// - Signal size: 64 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn fas_raw(&self) -> u64 {
        let signal = self.raw.view_bits::<Lsb0>()[64..128].load_le::<u64>();
        
        let factor = 1;
        u64::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Fas
    #[inline(always)]
    pub fn set_fas(&mut self, value: u64) -> Result<(), CanError> {
        if value < 0_u64 || 0_u64 < value {
            return Err(CanError::ParameterOutOfRange { message_id: CanFd::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: CanFd::MESSAGE_ID })?;
        let value = (value / factor) as u64;
        
        self.raw.view_bits_mut::<Lsb0>()[64..128].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for CanFd {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 64 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 64];
        raw.copy_from_slice(&payload[..64]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for CanFd {
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

/// FOOBAR
///
/// - Standard ID: 780 (0x30c)
/// - Size: 8 bytes
/// - Transmitter: FIE
#[derive(Clone, Copy)]
pub struct Foobar {
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
impl Foobar {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x30c)});
    
    pub const ACC_02_CRC_MIN: i16 = 0_i16;
    pub const ACC_02_CRC_MAX: i16 = 1_i16;
    
    /// Construct new FOOBAR from values
    pub fn new(acc_02_crc: i16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_acc_02_crc(acc_02_crc)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// ACC_02_CRC
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: BAR
    #[inline(always)]
    pub fn acc_02_crc(&self) -> i16 {
        self.acc_02_crc_raw()
    }
    
    /// Get raw value of ACC_02_CRC
    ///
    /// - Start bit: 0
    /// - Signal size: 12 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn acc_02_crc_raw(&self) -> i16 {
        let signal = self.raw.view_bits::<Lsb0>()[0..12].load_le::<i16>();
        
        let factor = 1;
        let signal = signal as i16;
        i16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of ACC_02_CRC
    #[inline(always)]
    pub fn set_acc_02_crc(&mut self, value: i16) -> Result<(), CanError> {
        if value < 0_i16 || 1_i16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Foobar::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Foobar::MESSAGE_ID })?;
        let value = (value / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..12].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Foobar {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Foobar {
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

