/// The name of the DBC file this code was generated from
#[allow(dead_code)]
pub const DBC_FILE_NAME: &str = "foobar";
/// The version of the DBC file this code was generated from
#[allow(dead_code)]
pub const DBC_FILE_VERSION: &str = "2.0";
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
    pub const MESSAGE_ID: embedded_can::Id = Id::Extended(
        ExtendedId::new(0x12330).unwrap(),
    );
    pub const MESSAGE_SIZE: usize = 8;
    pub const FOO_MIN: f32 = 229.53_f32;
    pub const FOO_MAX: f32 = 270.47_f32;
    pub const BAR_MIN: f32 = 0_f32;
    pub const BAR_MAX: f32 = 5_f32;
    /// Constructs a new `Foo` message from values.
    pub fn new(foo: f32, bar: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0x00; 8] };
        res.set_foo(foo)?;
        res.set_bar(bar)?;
        Ok(res)
    }
    /// Returns the raw `Foo` message payload.
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    /// Returns the value of `Foo`.
    ///
    /// - Min: 229.53
    /// - Max: 270.47
    /// - Unit: "degK"
    /// - Receivers: BAR
    /// - Factor: 0.01
    /// - Offset: 250
    #[inline(always)]
    pub fn foo(&self) -> f32 {
        let signal = self.raw.view_bits::<Msb0>()[7..19].load_be::<i16>();
        let factor = 0.01_f32;
        let offset = 250_f32;
        (signal as f32) * factor + offset
    }
    /// Returns the raw value of `Foo`.
    ///
    /// - Start bit: 0
    /// - Signal size: 12 bits
    /// - Byte order: BigEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn foo_raw_val(&self) -> i16 {
        self.raw.view_bits::<Msb0>()[7..19].load_be::<i16>()
    }
    /// Sets the raw value of `Foo`.
    #[inline(always)]
    pub fn set_foo_raw_val(&mut self, value: i16) {
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[7..19].store_be(value);
    }
    /// Sets the value of `Foo`.
    #[inline(always)]
    pub fn set_foo(&mut self, value: f32) -> Result<(), CanError> {
        if value < 229.53_f32 || 270.47_f32 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Foo::MESSAGE_ID,
            });
        }
        let factor = 0.01_f32;
        let offset = 250_f32;
        let value = ((value - offset) / factor) as i16;
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[7..19].store_be(value);
        Ok(())
    }
    /// Returns the value of `Bar`.
    ///
    /// Bar.
    ///
    /// - Min: 0
    /// - Max: 5
    /// - Unit: "m"
    /// - Receivers: FOO
    /// - Factor: 0.1
    /// - Offset: 0
    #[inline(always)]
    pub fn bar(&self) -> f32 {
        let signal = self.raw.view_bits::<Msb0>()[31..63].load_be::<i32>();
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    /// Returns the raw value of `Bar`.
    ///
    /// - Start bit: 24
    /// - Signal size: 32 bits
    /// - Byte order: BigEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn bar_raw_val(&self) -> i32 {
        self.raw.view_bits::<Msb0>()[31..63].load_be::<i32>()
    }
    /// Sets the raw value of `Bar`.
    #[inline(always)]
    pub fn set_bar_raw_val(&mut self, value: i32) {
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[31..63].store_be(value);
    }
    /// Sets the value of `Bar`.
    #[inline(always)]
    pub fn set_bar(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 5_f32 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Foo::MESSAGE_ID,
            });
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
        if payload.len() != 8 {
            return Err(CanError::InvalidPayloadSize);
        }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}
impl embedded_can::Frame for Foo {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID { None } else { data.try_into().ok() }
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
    pub const MESSAGE_ID: embedded_can::Id = Id::Extended(
        ExtendedId::new(0x12331).unwrap(),
    );
    pub const MESSAGE_SIZE: usize = 5;
    pub const MESSAGE_CYCLE_TIME_MS: u32 = 1;
    pub const FUM_MIN: i16 = 0_i16;
    pub const FUM_MAX: i16 = 10_i16;
    pub const FAM_MIN: i16 = 0_i16;
    pub const FAM_MAX: i16 = 8_i16;
    /// Constructs a new `Fum` message from values.
    pub fn new(fum: i16, fam: FumFam) -> Result<Self, CanError> {
        let mut res = Self { raw: [0x00; 5] };
        res.set_fum(fum)?;
        res.set_fam(fam)?;
        Ok(res)
    }
    /// Returns the raw `Fum` message payload.
    pub fn raw(&self) -> &[u8; 5] {
        &self.raw
    }
    /// Returns the value of `Fum`.
    ///
    /// - Min: 0
    /// - Max: 10
    /// - Unit: Not specified
    /// - Receivers: BAR
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn fum(&self) -> i16 {
        self.fum_raw_val()
    }
    /// Returns the raw value of `Fum`.
    ///
    /// - Start bit: 0
    /// - Signal size: 12 bits
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn fum_raw_val(&self) -> i16 {
        self.raw.view_bits::<Lsb0>()[0..12].load_le::<i16>()
    }
    /// Sets the raw value of `Fum`.
    #[inline(always)]
    pub fn set_fum_raw_val(&mut self, value: i16) {
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..12].store_le(value);
    }
    /// Sets the value of `Fum`.
    #[inline(always)]
    pub fn set_fum(&mut self, value: i16) -> Result<(), CanError> {
        if value < 0_i16 || 10_i16 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Fum::MESSAGE_ID,
            });
        }
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..12].store_le(value);
        Ok(())
    }
    /// Returns the value of `Fam`.
    ///
    /// - Min: 0
    /// - Max: 8
    /// - Unit: Not specified
    /// - Receivers: BAR
    #[inline(always)]
    pub fn fam(&self) -> FumFam {
        let signal = self.raw.view_bits::<Lsb0>()[12..24].load_le::<u16>();
        match signal {
            1 => FumFam::Enabled,
            0 => FumFam::Disabled,
            _ => FumFam::_Other(self.fam_phys_val()),
        }
    }
    #[inline(always)]
    fn fam_phys_val(&self) -> i16 {
        self.fam_raw_val()
    }
    /// Returns the raw value of `Fam`.
    ///
    /// - Start bit: 12
    /// - Signal size: 12 bits
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn fam_raw_val(&self) -> i16 {
        self.raw.view_bits::<Lsb0>()[12..24].load_le::<i16>()
    }
    /// Sets the raw value of `Fam`.
    #[inline(always)]
    pub fn set_fam_raw_val(&mut self, value: i16) {
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[12..24].store_le(value);
    }
    /// Sets the value of `Fam`.
    #[inline(always)]
    pub fn set_fam(&mut self, value: FumFam) -> Result<(), CanError> {
        let value = i16::from(value);
        if value < 0_i16 || 8_i16 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Fum::MESSAGE_ID,
            });
        }
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[12..24].store_le(value);
        Ok(())
    }
}
impl core::convert::TryFrom<&[u8]> for Fum {
    type Error = CanError;
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 5 {
            return Err(CanError::InvalidPayloadSize);
        }
        let mut raw = [0u8; 5];
        raw.copy_from_slice(&payload[..5]);
        Ok(Self { raw })
    }
}
impl embedded_can::Frame for Fum {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID { None } else { data.try_into().ok() }
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
    pub const MESSAGE_ID: embedded_can::Id = Id::Extended(
        ExtendedId::new(0x12332).unwrap(),
    );
    pub const MESSAGE_SIZE: usize = 4;
    pub const BINARY32_MIN: i32 = 0_i32;
    pub const BINARY32_MAX: i32 = 0_i32;
    /// Constructs a new `Bar` message from values.
    pub fn new(binary32: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0x00; 4] };
        res.set_binary32(binary32)?;
        Ok(res)
    }
    /// Returns the raw `Bar` message payload.
    pub fn raw(&self) -> &[u8; 4] {
        &self.raw
    }
    /// Returns the value of `Binary32`.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: Not specified
    /// - Receivers: FUM
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn binary32(&self) -> i32 {
        self.binary32_raw_val()
    }
    /// Returns the raw value of `Binary32`.
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn binary32_raw_val(&self) -> i32 {
        self.raw.view_bits::<Lsb0>()[0..32].load_le::<i32>()
    }
    /// Sets the raw value of `Binary32`.
    #[inline(always)]
    pub fn set_binary32_raw_val(&mut self, value: i32) {
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
    }
    /// Sets the value of `Binary32`.
    #[inline(always)]
    pub fn set_binary32(&mut self, value: i32) -> Result<(), CanError> {
        if value < 0_i32 || 0_i32 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Bar::MESSAGE_ID,
            });
        }
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }
}
impl core::convert::TryFrom<&[u8]> for Bar {
    type Error = CanError;
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 4 {
            return Err(CanError::InvalidPayloadSize);
        }
        let mut raw = [0u8; 4];
        raw.copy_from_slice(&payload[..4]);
        Ok(Self { raw })
    }
}
impl embedded_can::Frame for Bar {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID { None } else { data.try_into().ok() }
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
    pub const MESSAGE_ID: embedded_can::Id = Id::Extended(
        ExtendedId::new(0x12333).unwrap(),
    );
    pub const MESSAGE_SIZE: usize = 64;
    pub const FIE_MIN: u64 = 0_u64;
    pub const FIE_MAX: u64 = 0_u64;
    pub const FAS_MIN: u64 = 0_u64;
    pub const FAS_MAX: u64 = 0_u64;
    /// Constructs a new `CanFd` message from values.
    pub fn new(fie: u64, fas: u64) -> Result<Self, CanError> {
        let mut res = Self { raw: [0x00; 64] };
        res.set_fie(fie)?;
        res.set_fas(fas)?;
        Ok(res)
    }
    /// Returns the raw `CanFd` message payload.
    pub fn raw(&self) -> &[u8; 64] {
        &self.raw
    }
    /// Returns the value of `Fie`.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: Not specified
    /// - Receivers: FUM
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn fie(&self) -> u64 {
        self.fie_raw_val()
    }
    /// Returns the raw value of `Fie`.
    ///
    /// - Start bit: 0
    /// - Signal size: 64 bits
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn fie_raw_val(&self) -> u64 {
        self.raw.view_bits::<Lsb0>()[0..64].load_le::<u64>()
    }
    /// Sets the raw value of `Fie`.
    #[inline(always)]
    pub fn set_fie_raw_val(&mut self, value: u64) {
        self.raw.view_bits_mut::<Lsb0>()[0..64].store_le(value);
    }
    /// Sets the value of `Fie`.
    #[inline(always)]
    pub fn set_fie(&mut self, value: u64) -> Result<(), CanError> {
        if value < 0_u64 || 0_u64 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: CanFd::MESSAGE_ID,
            });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..64].store_le(value);
        Ok(())
    }
    /// Returns the value of `Fas`.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: Not specified
    /// - Receivers: Vector__XXX
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn fas(&self) -> u64 {
        self.fas_raw_val()
    }
    /// Returns the raw value of `Fas`.
    ///
    /// - Start bit: 64
    /// - Signal size: 64 bits
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn fas_raw_val(&self) -> u64 {
        self.raw.view_bits::<Lsb0>()[64..128].load_le::<u64>()
    }
    /// Sets the raw value of `Fas`.
    #[inline(always)]
    pub fn set_fas_raw_val(&mut self, value: u64) {
        self.raw.view_bits_mut::<Lsb0>()[64..128].store_le(value);
    }
    /// Sets the value of `Fas`.
    #[inline(always)]
    pub fn set_fas(&mut self, value: u64) -> Result<(), CanError> {
        if value < 0_u64 || 0_u64 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: CanFd::MESSAGE_ID,
            });
        }
        self.raw.view_bits_mut::<Lsb0>()[64..128].store_le(value);
        Ok(())
    }
}
impl core::convert::TryFrom<&[u8]> for CanFd {
    type Error = CanError;
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 64 {
            return Err(CanError::InvalidPayloadSize);
        }
        let mut raw = [0u8; 64];
        raw.copy_from_slice(&payload[..64]);
        Ok(Self { raw })
    }
}
impl embedded_can::Frame for CanFd {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID { None } else { data.try_into().ok() }
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
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(
        StandardId::new(0x30c).unwrap(),
    );
    pub const MESSAGE_SIZE: usize = 8;
    pub const ACC_02_CRC_MIN: i16 = 0_i16;
    pub const ACC_02_CRC_MAX: i16 = 1_i16;
    /// Constructs a new `FOOBAR` message from values.
    pub fn new(acc_02_crc: i16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0x00; 8] };
        res.set_acc_02_crc(acc_02_crc)?;
        Ok(res)
    }
    /// Returns the raw `FOOBAR` message payload.
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    /// Returns the value of `ACC_02_CRC`.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: Not specified
    /// - Receivers: BAR
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn acc_02_crc(&self) -> i16 {
        self.acc_02_crc_raw_val()
    }
    /// Returns the raw value of `ACC_02_CRC`.
    ///
    /// - Start bit: 0
    /// - Signal size: 12 bits
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn acc_02_crc_raw_val(&self) -> i16 {
        self.raw.view_bits::<Lsb0>()[0..12].load_le::<i16>()
    }
    /// Sets the raw value of `ACC_02_CRC`.
    #[inline(always)]
    pub fn set_acc_02_crc_raw_val(&mut self, value: i16) {
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..12].store_le(value);
    }
    /// Sets the value of `ACC_02_CRC`.
    #[inline(always)]
    pub fn set_acc_02_crc(&mut self, value: i16) -> Result<(), CanError> {
        if value < 0_i16 || 1_i16 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Foobar::MESSAGE_ID,
            });
        }
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..12].store_le(value);
        Ok(())
    }
}
impl core::convert::TryFrom<&[u8]> for Foobar {
    type Error = CanError;
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 {
            return Err(CanError::InvalidPayloadSize);
        }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}
impl embedded_can::Frame for Foobar {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID { None } else { data.try_into().ok() }
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


#[allow(dead_code)]
fn main() {}
