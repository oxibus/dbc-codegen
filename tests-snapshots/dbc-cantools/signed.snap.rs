/// The name of the DBC file this code was generated from
#[allow(dead_code)]
pub const DBC_FILE_NAME: &str = "signed";
/// The version of the DBC file this code was generated from
#[allow(dead_code)]
pub const DBC_FILE_VERSION: &str = "";
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
    /// Message378910
    Message378910(Message378910),
    /// Message63big_1
    Message63big1(Message63big1),
    /// Message63_1
    Message631(Message631),
    /// Message63big
    Message63big(Message63big),
    /// Message63
    Message63(Message63),
    /// Message32big
    Message32big(Message32big),
    /// Message33big
    Message33big(Message33big),
    /// Message64big
    Message64big(Message64big),
    /// Message64
    Message64(Message64),
    /// Message33
    Message33(Message33),
    /// Message32
    Message32(Message32),
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
            Message378910::MESSAGE_ID => {
                Messages::Message378910(Message378910::try_from(payload)?)
            }
            Message63big1::MESSAGE_ID => {
                Messages::Message63big1(Message63big1::try_from(payload)?)
            }
            Message631::MESSAGE_ID => {
                Messages::Message631(Message631::try_from(payload)?)
            }
            Message63big::MESSAGE_ID => {
                Messages::Message63big(Message63big::try_from(payload)?)
            }
            Message63::MESSAGE_ID => Messages::Message63(Message63::try_from(payload)?),
            Message32big::MESSAGE_ID => {
                Messages::Message32big(Message32big::try_from(payload)?)
            }
            Message33big::MESSAGE_ID => {
                Messages::Message33big(Message33big::try_from(payload)?)
            }
            Message64big::MESSAGE_ID => {
                Messages::Message64big(Message64big::try_from(payload)?)
            }
            Message64::MESSAGE_ID => Messages::Message64(Message64::try_from(payload)?),
            Message33::MESSAGE_ID => Messages::Message33(Message33::try_from(payload)?),
            Message32::MESSAGE_ID => Messages::Message32(Message32::try_from(payload)?),
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}
/// Message378910
///
/// - Standard ID: 10 (0xa)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Message378910 {
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
impl Message378910 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(StandardId::new(0xa).unwrap());
    pub const MESSAGE_SIZE: usize = 8;
    pub const S3BIG_MIN: i8 = 0_i8;
    pub const S3BIG_MAX: i8 = 0_i8;
    pub const S3_MIN: i8 = 0_i8;
    pub const S3_MAX: i8 = 0_i8;
    pub const S10BIG_MIN: i16 = 0_i16;
    pub const S10BIG_MAX: i16 = 0_i16;
    pub const S8BIG_MIN: i8 = 0_i8;
    pub const S8BIG_MAX: i8 = 0_i8;
    pub const S7BIG_MIN: i8 = 0_i8;
    pub const S7BIG_MAX: i8 = 0_i8;
    pub const S9_MIN: i16 = 0_i16;
    pub const S9_MAX: i16 = 0_i16;
    pub const S8_MIN: i8 = 0_i8;
    pub const S8_MAX: i8 = 0_i8;
    pub const S7_MIN: i8 = 0_i8;
    pub const S7_MAX: i8 = 0_i8;
    /// Constructs a new `Message378910` message from values.
    pub fn new(
        s3big: i8,
        s3: i8,
        s10big: i16,
        s8big: i8,
        s7big: i8,
        s9: i16,
        s8: i8,
        s7: i8,
    ) -> Result<Self, CanError> {
        let mut res = Self { raw: [0x00; 8] };
        res.set_s3big(s3big)?;
        res.set_s3(s3)?;
        res.set_s10big(s10big)?;
        res.set_s8big(s8big)?;
        res.set_s7big(s7big)?;
        res.set_s9(s9)?;
        res.set_s8(s8)?;
        res.set_s7(s7)?;
        Ok(res)
    }
    /// Returns the raw `Message378910` message payload.
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    /// Returns the value of `s3big`.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: Not specified
    /// - Receivers: Vector__XXX
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn s3big(&self) -> i8 {
        self.s3big_raw_val()
    }
    /// Returns the raw value of `s3big`.
    ///
    /// - Start bit: 39
    /// - Signal size: 3 bits
    /// - Byte order: BigEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn s3big_raw_val(&self) -> i8 {
        self.raw.view_bits::<Msb0>()[32..35].load_be::<i8>()
    }
    /// Sets the raw value of `s3big`.
    #[inline(always)]
    pub fn set_s3big_raw_val(&mut self, value: i8) {
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[32..35].store_be(value);
    }
    /// Sets the value of `s3big`.
    #[inline(always)]
    pub fn set_s3big(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Message378910::MESSAGE_ID,
            });
        }
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[32..35].store_be(value);
        Ok(())
    }
    /// Returns the value of `s3`.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: Not specified
    /// - Receivers: Vector__XXX
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn s3(&self) -> i8 {
        self.s3_raw_val()
    }
    /// Returns the raw value of `s3`.
    ///
    /// - Start bit: 34
    /// - Signal size: 3 bits
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn s3_raw_val(&self) -> i8 {
        self.raw.view_bits::<Lsb0>()[34..37].load_le::<i8>()
    }
    /// Sets the raw value of `s3`.
    #[inline(always)]
    pub fn set_s3_raw_val(&mut self, value: i8) {
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[34..37].store_le(value);
    }
    /// Sets the value of `s3`.
    #[inline(always)]
    pub fn set_s3(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Message378910::MESSAGE_ID,
            });
        }
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[34..37].store_le(value);
        Ok(())
    }
    /// Returns the value of `s10big`.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: Not specified
    /// - Receivers: Vector__XXX
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn s10big(&self) -> i16 {
        self.s10big_raw_val()
    }
    /// Returns the raw value of `s10big`.
    ///
    /// - Start bit: 40
    /// - Signal size: 10 bits
    /// - Byte order: BigEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn s10big_raw_val(&self) -> i16 {
        self.raw.view_bits::<Msb0>()[47..57].load_be::<i16>()
    }
    /// Sets the raw value of `s10big`.
    #[inline(always)]
    pub fn set_s10big_raw_val(&mut self, value: i16) {
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[47..57].store_be(value);
    }
    /// Sets the value of `s10big`.
    #[inline(always)]
    pub fn set_s10big(&mut self, value: i16) -> Result<(), CanError> {
        if value < 0_i16 || 0_i16 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Message378910::MESSAGE_ID,
            });
        }
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[47..57].store_be(value);
        Ok(())
    }
    /// Returns the value of `s8big`.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: Not specified
    /// - Receivers: Vector__XXX
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn s8big(&self) -> i8 {
        self.s8big_raw_val()
    }
    /// Returns the raw value of `s8big`.
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Byte order: BigEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn s8big_raw_val(&self) -> i8 {
        self.raw.view_bits::<Msb0>()[7..15].load_be::<i8>()
    }
    /// Sets the raw value of `s8big`.
    #[inline(always)]
    pub fn set_s8big_raw_val(&mut self, value: i8) {
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[7..15].store_be(value);
    }
    /// Sets the value of `s8big`.
    #[inline(always)]
    pub fn set_s8big(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Message378910::MESSAGE_ID,
            });
        }
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[7..15].store_be(value);
        Ok(())
    }
    /// Returns the value of `s7big`.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: Not specified
    /// - Receivers: Vector__XXX
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn s7big(&self) -> i8 {
        self.s7big_raw_val()
    }
    /// Returns the raw value of `s7big`.
    ///
    /// - Start bit: 62
    /// - Signal size: 7 bits
    /// - Byte order: BigEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn s7big_raw_val(&self) -> i8 {
        self.raw.view_bits::<Msb0>()[57..64].load_be::<i8>()
    }
    /// Sets the raw value of `s7big`.
    #[inline(always)]
    pub fn set_s7big_raw_val(&mut self, value: i8) {
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[57..64].store_be(value);
    }
    /// Sets the value of `s7big`.
    #[inline(always)]
    pub fn set_s7big(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Message378910::MESSAGE_ID,
            });
        }
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[57..64].store_be(value);
        Ok(())
    }
    /// Returns the value of `s9`.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: Not specified
    /// - Receivers: Vector__XXX
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn s9(&self) -> i16 {
        self.s9_raw_val()
    }
    /// Returns the raw value of `s9`.
    ///
    /// - Start bit: 17
    /// - Signal size: 9 bits
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn s9_raw_val(&self) -> i16 {
        self.raw.view_bits::<Lsb0>()[17..26].load_le::<i16>()
    }
    /// Sets the raw value of `s9`.
    #[inline(always)]
    pub fn set_s9_raw_val(&mut self, value: i16) {
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[17..26].store_le(value);
    }
    /// Sets the value of `s9`.
    #[inline(always)]
    pub fn set_s9(&mut self, value: i16) -> Result<(), CanError> {
        if value < 0_i16 || 0_i16 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Message378910::MESSAGE_ID,
            });
        }
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[17..26].store_le(value);
        Ok(())
    }
    /// Returns the value of `s8`.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: Not specified
    /// - Receivers: Vector__XXX
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn s8(&self) -> i8 {
        self.s8_raw_val()
    }
    /// Returns the raw value of `s8`.
    ///
    /// - Start bit: 26
    /// - Signal size: 8 bits
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn s8_raw_val(&self) -> i8 {
        self.raw.view_bits::<Lsb0>()[26..34].load_le::<i8>()
    }
    /// Sets the raw value of `s8`.
    #[inline(always)]
    pub fn set_s8_raw_val(&mut self, value: i8) {
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[26..34].store_le(value);
    }
    /// Sets the value of `s8`.
    #[inline(always)]
    pub fn set_s8(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Message378910::MESSAGE_ID,
            });
        }
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[26..34].store_le(value);
        Ok(())
    }
    /// Returns the value of `s7`.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: Not specified
    /// - Receivers: Vector__XXX
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn s7(&self) -> i8 {
        self.s7_raw_val()
    }
    /// Returns the raw value of `s7`.
    ///
    /// - Start bit: 1
    /// - Signal size: 7 bits
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn s7_raw_val(&self) -> i8 {
        self.raw.view_bits::<Lsb0>()[1..8].load_le::<i8>()
    }
    /// Sets the raw value of `s7`.
    #[inline(always)]
    pub fn set_s7_raw_val(&mut self, value: i8) {
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[1..8].store_le(value);
    }
    /// Sets the value of `s7`.
    #[inline(always)]
    pub fn set_s7(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Message378910::MESSAGE_ID,
            });
        }
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[1..8].store_le(value);
        Ok(())
    }
}
impl core::convert::TryFrom<&[u8]> for Message378910 {
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
impl embedded_can::Frame for Message378910 {
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
/// Message63big_1
///
/// - Standard ID: 9 (0x9)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Message63big1 {
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
impl Message63big1 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(StandardId::new(0x9).unwrap());
    pub const MESSAGE_SIZE: usize = 8;
    pub const S63BIG_MIN: i64 = 0_i64;
    pub const S63BIG_MAX: i64 = 0_i64;
    /// Constructs a new `Message63big_1` message from values.
    pub fn new(s63big: i64) -> Result<Self, CanError> {
        let mut res = Self { raw: [0x00; 8] };
        res.set_s63big(s63big)?;
        Ok(res)
    }
    /// Returns the raw `Message63big_1` message payload.
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    /// Returns the value of `s63big`.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: Not specified
    /// - Receivers: Vector__XXX
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn s63big(&self) -> i64 {
        self.s63big_raw_val()
    }
    /// Returns the raw value of `s63big`.
    ///
    /// - Start bit: 6
    /// - Signal size: 63 bits
    /// - Byte order: BigEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn s63big_raw_val(&self) -> i64 {
        self.raw.view_bits::<Msb0>()[1..64].load_be::<i64>()
    }
    /// Sets the raw value of `s63big`.
    #[inline(always)]
    pub fn set_s63big_raw_val(&mut self, value: i64) {
        let value = u64::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[1..64].store_be(value);
    }
    /// Sets the value of `s63big`.
    #[inline(always)]
    pub fn set_s63big(&mut self, value: i64) -> Result<(), CanError> {
        if value < 0_i64 || 0_i64 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Message63big1::MESSAGE_ID,
            });
        }
        let value = u64::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[1..64].store_be(value);
        Ok(())
    }
}
impl core::convert::TryFrom<&[u8]> for Message63big1 {
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
impl embedded_can::Frame for Message63big1 {
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
/// Message63_1
///
/// - Standard ID: 8 (0x8)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Message631 {
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
impl Message631 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(StandardId::new(0x8).unwrap());
    pub const MESSAGE_SIZE: usize = 8;
    pub const S63_MIN: i64 = 0_i64;
    pub const S63_MAX: i64 = 0_i64;
    /// Constructs a new `Message63_1` message from values.
    pub fn new(s63: i64) -> Result<Self, CanError> {
        let mut res = Self { raw: [0x00; 8] };
        res.set_s63(s63)?;
        Ok(res)
    }
    /// Returns the raw `Message63_1` message payload.
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    /// Returns the value of `s63`.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: Not specified
    /// - Receivers: Vector__XXX
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn s63(&self) -> i64 {
        self.s63_raw_val()
    }
    /// Returns the raw value of `s63`.
    ///
    /// - Start bit: 1
    /// - Signal size: 63 bits
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn s63_raw_val(&self) -> i64 {
        self.raw.view_bits::<Lsb0>()[1..64].load_le::<i64>()
    }
    /// Sets the raw value of `s63`.
    #[inline(always)]
    pub fn set_s63_raw_val(&mut self, value: i64) {
        let value = u64::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[1..64].store_le(value);
    }
    /// Sets the value of `s63`.
    #[inline(always)]
    pub fn set_s63(&mut self, value: i64) -> Result<(), CanError> {
        if value < 0_i64 || 0_i64 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Message631::MESSAGE_ID,
            });
        }
        let value = u64::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[1..64].store_le(value);
        Ok(())
    }
}
impl core::convert::TryFrom<&[u8]> for Message631 {
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
impl embedded_can::Frame for Message631 {
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
/// Message63big
///
/// - Standard ID: 7 (0x7)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Message63big {
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
impl Message63big {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(StandardId::new(0x7).unwrap());
    pub const MESSAGE_SIZE: usize = 8;
    pub const S63BIG_MIN: i64 = 0_i64;
    pub const S63BIG_MAX: i64 = 0_i64;
    /// Constructs a new `Message63big` message from values.
    pub fn new(s63big: i64) -> Result<Self, CanError> {
        let mut res = Self { raw: [0x00; 8] };
        res.set_s63big(s63big)?;
        Ok(res)
    }
    /// Returns the raw `Message63big` message payload.
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    /// Returns the value of `s63big`.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: Not specified
    /// - Receivers: Vector__XXX
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn s63big(&self) -> i64 {
        self.s63big_raw_val()
    }
    /// Returns the raw value of `s63big`.
    ///
    /// - Start bit: 7
    /// - Signal size: 63 bits
    /// - Byte order: BigEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn s63big_raw_val(&self) -> i64 {
        self.raw.view_bits::<Msb0>()[0..63].load_be::<i64>()
    }
    /// Sets the raw value of `s63big`.
    #[inline(always)]
    pub fn set_s63big_raw_val(&mut self, value: i64) {
        let value = u64::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[0..63].store_be(value);
    }
    /// Sets the value of `s63big`.
    #[inline(always)]
    pub fn set_s63big(&mut self, value: i64) -> Result<(), CanError> {
        if value < 0_i64 || 0_i64 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Message63big::MESSAGE_ID,
            });
        }
        let value = u64::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[0..63].store_be(value);
        Ok(())
    }
}
impl core::convert::TryFrom<&[u8]> for Message63big {
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
impl embedded_can::Frame for Message63big {
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
/// Message63
///
/// - Standard ID: 6 (0x6)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Message63 {
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
impl Message63 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(StandardId::new(0x6).unwrap());
    pub const MESSAGE_SIZE: usize = 8;
    pub const S63_MIN: i64 = 0_i64;
    pub const S63_MAX: i64 = 0_i64;
    /// Constructs a new `Message63` message from values.
    pub fn new(s63: i64) -> Result<Self, CanError> {
        let mut res = Self { raw: [0x00; 8] };
        res.set_s63(s63)?;
        Ok(res)
    }
    /// Returns the raw `Message63` message payload.
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    /// Returns the value of `s63`.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: Not specified
    /// - Receivers: Vector__XXX
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn s63(&self) -> i64 {
        self.s63_raw_val()
    }
    /// Returns the raw value of `s63`.
    ///
    /// - Start bit: 0
    /// - Signal size: 63 bits
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn s63_raw_val(&self) -> i64 {
        self.raw.view_bits::<Lsb0>()[0..63].load_le::<i64>()
    }
    /// Sets the raw value of `s63`.
    #[inline(always)]
    pub fn set_s63_raw_val(&mut self, value: i64) {
        let value = u64::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..63].store_le(value);
    }
    /// Sets the value of `s63`.
    #[inline(always)]
    pub fn set_s63(&mut self, value: i64) -> Result<(), CanError> {
        if value < 0_i64 || 0_i64 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Message63::MESSAGE_ID,
            });
        }
        let value = u64::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..63].store_le(value);
        Ok(())
    }
}
impl core::convert::TryFrom<&[u8]> for Message63 {
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
impl embedded_can::Frame for Message63 {
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
/// Message32big
///
/// - Standard ID: 5 (0x5)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Message32big {
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
impl Message32big {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(StandardId::new(0x5).unwrap());
    pub const MESSAGE_SIZE: usize = 8;
    pub const S32BIG_MIN: i32 = 0_i32;
    pub const S32BIG_MAX: i32 = 0_i32;
    /// Constructs a new `Message32big` message from values.
    pub fn new(s32big: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0x00; 8] };
        res.set_s32big(s32big)?;
        Ok(res)
    }
    /// Returns the raw `Message32big` message payload.
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    /// Returns the value of `s32big`.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: Not specified
    /// - Receivers: Vector__XXX
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn s32big(&self) -> i32 {
        self.s32big_raw_val()
    }
    /// Returns the raw value of `s32big`.
    ///
    /// - Start bit: 7
    /// - Signal size: 32 bits
    /// - Byte order: BigEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn s32big_raw_val(&self) -> i32 {
        self.raw.view_bits::<Msb0>()[0..32].load_be::<i32>()
    }
    /// Sets the raw value of `s32big`.
    #[inline(always)]
    pub fn set_s32big_raw_val(&mut self, value: i32) {
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[0..32].store_be(value);
    }
    /// Sets the value of `s32big`.
    #[inline(always)]
    pub fn set_s32big(&mut self, value: i32) -> Result<(), CanError> {
        if value < 0_i32 || 0_i32 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Message32big::MESSAGE_ID,
            });
        }
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[0..32].store_be(value);
        Ok(())
    }
}
impl core::convert::TryFrom<&[u8]> for Message32big {
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
impl embedded_can::Frame for Message32big {
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
/// Message33big
///
/// - Standard ID: 4 (0x4)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Message33big {
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
impl Message33big {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(StandardId::new(0x4).unwrap());
    pub const MESSAGE_SIZE: usize = 8;
    pub const S33BIG_MIN: i64 = 0_i64;
    pub const S33BIG_MAX: i64 = 0_i64;
    /// Constructs a new `Message33big` message from values.
    pub fn new(s33big: i64) -> Result<Self, CanError> {
        let mut res = Self { raw: [0x00; 8] };
        res.set_s33big(s33big)?;
        Ok(res)
    }
    /// Returns the raw `Message33big` message payload.
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    /// Returns the value of `s33big`.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: Not specified
    /// - Receivers: Vector__XXX
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn s33big(&self) -> i64 {
        self.s33big_raw_val()
    }
    /// Returns the raw value of `s33big`.
    ///
    /// - Start bit: 7
    /// - Signal size: 33 bits
    /// - Byte order: BigEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn s33big_raw_val(&self) -> i64 {
        self.raw.view_bits::<Msb0>()[0..33].load_be::<i64>()
    }
    /// Sets the raw value of `s33big`.
    #[inline(always)]
    pub fn set_s33big_raw_val(&mut self, value: i64) {
        let value = u64::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[0..33].store_be(value);
    }
    /// Sets the value of `s33big`.
    #[inline(always)]
    pub fn set_s33big(&mut self, value: i64) -> Result<(), CanError> {
        if value < 0_i64 || 0_i64 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Message33big::MESSAGE_ID,
            });
        }
        let value = u64::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[0..33].store_be(value);
        Ok(())
    }
}
impl core::convert::TryFrom<&[u8]> for Message33big {
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
impl embedded_can::Frame for Message33big {
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
/// Message64big
///
/// - Standard ID: 3 (0x3)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Message64big {
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
impl Message64big {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(StandardId::new(0x3).unwrap());
    pub const MESSAGE_SIZE: usize = 8;
    pub const S64BIG_MIN: i64 = 0_i64;
    pub const S64BIG_MAX: i64 = 0_i64;
    /// Constructs a new `Message64big` message from values.
    pub fn new(s64big: i64) -> Result<Self, CanError> {
        let mut res = Self { raw: [0x00; 8] };
        res.set_s64big(s64big)?;
        Ok(res)
    }
    /// Returns the raw `Message64big` message payload.
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    /// Returns the value of `s64big`.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: Not specified
    /// - Receivers: Vector__XXX
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn s64big(&self) -> i64 {
        self.s64big_raw_val()
    }
    /// Returns the raw value of `s64big`.
    ///
    /// - Start bit: 7
    /// - Signal size: 64 bits
    /// - Byte order: BigEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn s64big_raw_val(&self) -> i64 {
        self.raw.view_bits::<Msb0>()[0..64].load_be::<i64>()
    }
    /// Sets the raw value of `s64big`.
    #[inline(always)]
    pub fn set_s64big_raw_val(&mut self, value: i64) {
        let value = u64::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[0..64].store_be(value);
    }
    /// Sets the value of `s64big`.
    #[inline(always)]
    pub fn set_s64big(&mut self, value: i64) -> Result<(), CanError> {
        if value < 0_i64 || 0_i64 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Message64big::MESSAGE_ID,
            });
        }
        let value = u64::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[0..64].store_be(value);
        Ok(())
    }
}
impl core::convert::TryFrom<&[u8]> for Message64big {
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
impl embedded_can::Frame for Message64big {
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
/// Message64
///
/// - Standard ID: 2 (0x2)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Message64 {
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
impl Message64 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(StandardId::new(0x2).unwrap());
    pub const MESSAGE_SIZE: usize = 8;
    pub const S64_MIN: i64 = -9223372036854780000_i64;
    pub const S64_MAX: i64 = 9223372036854780000_i64;
    /// Constructs a new `Message64` message from values.
    pub fn new(s64: i64) -> Result<Self, CanError> {
        let mut res = Self { raw: [0x00; 8] };
        res.set_s64(s64)?;
        Ok(res)
    }
    /// Returns the raw `Message64` message payload.
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    /// Returns the value of `s64`.
    ///
    /// - Min: -9223372036854780000
    /// - Max: 9223372036854780000
    /// - Unit: Not specified
    /// - Receivers: Vector__XXX
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn s64(&self) -> i64 {
        self.s64_raw_val()
    }
    /// Returns the raw value of `s64`.
    ///
    /// - Start bit: 0
    /// - Signal size: 64 bits
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn s64_raw_val(&self) -> i64 {
        self.raw.view_bits::<Lsb0>()[0..64].load_le::<i64>()
    }
    /// Sets the raw value of `s64`.
    #[inline(always)]
    pub fn set_s64_raw_val(&mut self, value: i64) {
        let value = u64::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..64].store_le(value);
    }
    /// Sets the value of `s64`.
    #[inline(always)]
    pub fn set_s64(&mut self, value: i64) -> Result<(), CanError> {
        if value < -9223372036854780000_i64 || 9223372036854780000_i64 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Message64::MESSAGE_ID,
            });
        }
        let value = u64::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..64].store_le(value);
        Ok(())
    }
}
impl core::convert::TryFrom<&[u8]> for Message64 {
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
impl embedded_can::Frame for Message64 {
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
/// Message33
///
/// - Standard ID: 1 (0x1)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Message33 {
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
impl Message33 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(StandardId::new(0x1).unwrap());
    pub const MESSAGE_SIZE: usize = 8;
    pub const S33_MIN: i64 = -4294967296_i64;
    pub const S33_MAX: i64 = 4294967295_i64;
    /// Constructs a new `Message33` message from values.
    pub fn new(s33: i64) -> Result<Self, CanError> {
        let mut res = Self { raw: [0x00; 8] };
        res.set_s33(s33)?;
        Ok(res)
    }
    /// Returns the raw `Message33` message payload.
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    /// Returns the value of `s33`.
    ///
    /// - Min: -4294967296
    /// - Max: 4294967295
    /// - Unit: Not specified
    /// - Receivers: Vector__XXX
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn s33(&self) -> i64 {
        self.s33_raw_val()
    }
    /// Returns the raw value of `s33`.
    ///
    /// - Start bit: 0
    /// - Signal size: 33 bits
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn s33_raw_val(&self) -> i64 {
        self.raw.view_bits::<Lsb0>()[0..33].load_le::<i64>()
    }
    /// Sets the raw value of `s33`.
    #[inline(always)]
    pub fn set_s33_raw_val(&mut self, value: i64) {
        let value = u64::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..33].store_le(value);
    }
    /// Sets the value of `s33`.
    #[inline(always)]
    pub fn set_s33(&mut self, value: i64) -> Result<(), CanError> {
        if value < -4294967296_i64 || 4294967295_i64 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Message33::MESSAGE_ID,
            });
        }
        let value = u64::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..33].store_le(value);
        Ok(())
    }
}
impl core::convert::TryFrom<&[u8]> for Message33 {
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
impl embedded_can::Frame for Message33 {
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
/// Message32
///
/// - Standard ID: 0 (0x0)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Message32 {
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
impl Message32 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(StandardId::new(0x0).unwrap());
    pub const MESSAGE_SIZE: usize = 8;
    pub const S32_MIN: i32 = 0_i32;
    pub const S32_MAX: i32 = 0_i32;
    /// Constructs a new `Message32` message from values.
    pub fn new(s32: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0x00; 8] };
        res.set_s32(s32)?;
        Ok(res)
    }
    /// Returns the raw `Message32` message payload.
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    /// Returns the value of `s32`.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: Not specified
    /// - Receivers: Vector__XXX
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn s32(&self) -> i32 {
        self.s32_raw_val()
    }
    /// Returns the raw value of `s32`.
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn s32_raw_val(&self) -> i32 {
        self.raw.view_bits::<Lsb0>()[0..32].load_le::<i32>()
    }
    /// Sets the raw value of `s32`.
    #[inline(always)]
    pub fn set_s32_raw_val(&mut self, value: i32) {
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
    }
    /// Sets the value of `s32`.
    #[inline(always)]
    pub fn set_s32(&mut self, value: i32) -> Result<(), CanError> {
        if value < 0_i32 || 0_i32 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Message32::MESSAGE_ID,
            });
        }
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }
}
impl core::convert::TryFrom<&[u8]> for Message32 {
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
impl embedded_can::Frame for Message32 {
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
