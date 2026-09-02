/// The name of the DBC file this code was generated from
#[allow(dead_code)]
pub const DBC_FILE_NAME: &str = "padding_bit_order";
/// The version of the DBC file this code was generated from
#[allow(dead_code)]
pub const DBC_FILE_VERSION: &str = "VER";
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
    /// MSG0
    Msg0(Msg0),
    /// MSG1
    Msg1(Msg1),
    /// MSG2
    Msg2(Msg2),
    /// MSG3
    Msg3(Msg3),
    /// MSG4
    Msg4(Msg4),
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
            Msg0::MESSAGE_ID => Messages::Msg0(Msg0::try_from(payload)?),
            Msg1::MESSAGE_ID => Messages::Msg1(Msg1::try_from(payload)?),
            Msg2::MESSAGE_ID => Messages::Msg2(Msg2::try_from(payload)?),
            Msg3::MESSAGE_ID => Messages::Msg3(Msg3::try_from(payload)?),
            Msg4::MESSAGE_ID => Messages::Msg4(Msg4::try_from(payload)?),
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}
/// MSG0
///
/// - Standard ID: 1 (0x1)
/// - Size: 8 bytes
/// - Transmitter: E1
#[derive(Clone, Copy)]
pub struct Msg0 {
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
impl Msg0 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(StandardId::new(0x1).unwrap());
    pub const MESSAGE_SIZE: usize = 8;
    pub const A_MIN: u16 = 0_u16;
    pub const A_MAX: u16 = 32767_u16;
    pub const C_MIN: u16 = 0_u16;
    pub const C_MAX: u16 = 32767_u16;
    /// Constructs a new `MSG0` message from values.
    pub fn new(a: u16, b: bool, c: u16, d: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0x00; 8] };
        res.set_a(a)?;
        res.set_b(b)?;
        res.set_c(c)?;
        res.set_d(d)?;
        Ok(res)
    }
    /// Returns the raw `MSG0` message payload.
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    /// Returns the value of `A`.
    ///
    /// - Min: 0
    /// - Max: 32767
    /// - Unit: Not specified
    /// - Receivers: E1
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn a(&self) -> u16 {
        self.a_raw_val()
    }
    /// Returns the raw value of `A`.
    ///
    /// - Start bit: 6
    /// - Signal size: 15 bits
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn a_raw_val(&self) -> u16 {
        self.raw.view_bits::<Msb0>()[1..16].load_be::<u16>()
    }
    /// Sets the raw value of `A`.
    #[inline(always)]
    pub fn set_a_raw_val(&mut self, value: u16) {
        self.raw.view_bits_mut::<Msb0>()[1..16].store_be(value);
    }
    /// Sets the value of `A`.
    #[inline(always)]
    pub fn set_a(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 32767_u16 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Msg0::MESSAGE_ID,
            });
        }
        self.raw.view_bits_mut::<Msb0>()[1..16].store_be(value);
        Ok(())
    }
    /// Returns the value of `B`.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: Not specified
    /// - Receivers: E1
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn b(&self) -> bool {
        self.b_raw_val() == 1
    }
    /// Returns the raw value of `B`.
    ///
    /// - Start bit: 7
    /// - Signal size: 1 bits
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn b_raw_val(&self) -> u8 {
        self.raw.view_bits::<Msb0>()[0..1].load_be::<u8>()
    }
    /// Sets the raw value of `B`.
    #[inline(always)]
    pub fn set_b_raw_val(&mut self, value: u8) {
        self.raw.view_bits_mut::<Msb0>()[0..1].store_be(value);
    }
    /// Sets the value of `B`.
    #[inline(always)]
    pub fn set_b(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[0..1].store_be(value);
        Ok(())
    }
    /// Returns the value of `C`.
    ///
    /// - Min: 0
    /// - Max: 32767
    /// - Unit: Not specified
    /// - Receivers: E1
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn c(&self) -> u16 {
        self.c_raw_val()
    }
    /// Returns the raw value of `C`.
    ///
    /// - Start bit: 38
    /// - Signal size: 15 bits
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn c_raw_val(&self) -> u16 {
        self.raw.view_bits::<Msb0>()[33..48].load_be::<u16>()
    }
    /// Sets the raw value of `C`.
    #[inline(always)]
    pub fn set_c_raw_val(&mut self, value: u16) {
        self.raw.view_bits_mut::<Msb0>()[33..48].store_be(value);
    }
    /// Sets the value of `C`.
    #[inline(always)]
    pub fn set_c(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 32767_u16 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Msg0::MESSAGE_ID,
            });
        }
        self.raw.view_bits_mut::<Msb0>()[33..48].store_be(value);
        Ok(())
    }
    /// Returns the value of `D`.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: Not specified
    /// - Receivers: E1
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn d(&self) -> bool {
        self.d_raw_val() == 1
    }
    /// Returns the raw value of `D`.
    ///
    /// - Start bit: 39
    /// - Signal size: 1 bits
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn d_raw_val(&self) -> u8 {
        self.raw.view_bits::<Msb0>()[32..33].load_be::<u8>()
    }
    /// Sets the raw value of `D`.
    #[inline(always)]
    pub fn set_d_raw_val(&mut self, value: u8) {
        self.raw.view_bits_mut::<Msb0>()[32..33].store_be(value);
    }
    /// Sets the value of `D`.
    #[inline(always)]
    pub fn set_d(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[32..33].store_be(value);
        Ok(())
    }
}
impl core::convert::TryFrom<&[u8]> for Msg0 {
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
impl embedded_can::Frame for Msg0 {
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
/// MSG1
///
/// - Standard ID: 2 (0x2)
/// - Size: 8 bytes
/// - Transmitter: E1
#[derive(Clone, Copy)]
pub struct Msg1 {
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
impl Msg1 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(StandardId::new(0x2).unwrap());
    pub const MESSAGE_SIZE: usize = 8;
    pub const F_MIN: u16 = 0_u16;
    pub const F_MAX: u16 = 32767_u16;
    pub const H_MIN: u16 = 0_u16;
    pub const H_MAX: u16 = 32767_u16;
    /// Constructs a new `MSG1` message from values.
    pub fn new(e: bool, f: u16, g: bool, h: u16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0x00; 8] };
        res.set_e(e)?;
        res.set_f(f)?;
        res.set_g(g)?;
        res.set_h(h)?;
        Ok(res)
    }
    /// Returns the raw `MSG1` message payload.
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    /// Returns the value of `E`.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: Not specified
    /// - Receivers: E1
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn e(&self) -> bool {
        self.e_raw_val() == 1
    }
    /// Returns the raw value of `E`.
    ///
    /// - Start bit: 0
    /// - Signal size: 1 bits
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn e_raw_val(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[0..1].load_le::<u8>()
    }
    /// Sets the raw value of `E`.
    #[inline(always)]
    pub fn set_e_raw_val(&mut self, value: u8) {
        self.raw.view_bits_mut::<Lsb0>()[0..1].store_le(value);
    }
    /// Sets the value of `E`.
    #[inline(always)]
    pub fn set_e(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[0..1].store_le(value);
        Ok(())
    }
    /// Returns the value of `F`.
    ///
    /// - Min: 0
    /// - Max: 32767
    /// - Unit: Not specified
    /// - Receivers: E1
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn f(&self) -> u16 {
        self.f_raw_val()
    }
    /// Returns the raw value of `F`.
    ///
    /// - Start bit: 1
    /// - Signal size: 15 bits
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn f_raw_val(&self) -> u16 {
        self.raw.view_bits::<Lsb0>()[1..16].load_le::<u16>()
    }
    /// Sets the raw value of `F`.
    #[inline(always)]
    pub fn set_f_raw_val(&mut self, value: u16) {
        self.raw.view_bits_mut::<Lsb0>()[1..16].store_le(value);
    }
    /// Sets the value of `F`.
    #[inline(always)]
    pub fn set_f(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 32767_u16 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Msg1::MESSAGE_ID,
            });
        }
        self.raw.view_bits_mut::<Lsb0>()[1..16].store_le(value);
        Ok(())
    }
    /// Returns the value of `G`.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: Not specified
    /// - Receivers: E1
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn g(&self) -> bool {
        self.g_raw_val() == 1
    }
    /// Returns the raw value of `G`.
    ///
    /// - Start bit: 32
    /// - Signal size: 1 bits
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn g_raw_val(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[32..33].load_le::<u8>()
    }
    /// Sets the raw value of `G`.
    #[inline(always)]
    pub fn set_g_raw_val(&mut self, value: u8) {
        self.raw.view_bits_mut::<Lsb0>()[32..33].store_le(value);
    }
    /// Sets the value of `G`.
    #[inline(always)]
    pub fn set_g(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[32..33].store_le(value);
        Ok(())
    }
    /// Returns the value of `H`.
    ///
    /// - Min: 0
    /// - Max: 32767
    /// - Unit: Not specified
    /// - Receivers: E1
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn h(&self) -> u16 {
        self.h_raw_val()
    }
    /// Returns the raw value of `H`.
    ///
    /// - Start bit: 33
    /// - Signal size: 15 bits
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn h_raw_val(&self) -> u16 {
        self.raw.view_bits::<Lsb0>()[33..48].load_le::<u16>()
    }
    /// Sets the raw value of `H`.
    #[inline(always)]
    pub fn set_h_raw_val(&mut self, value: u16) {
        self.raw.view_bits_mut::<Lsb0>()[33..48].store_le(value);
    }
    /// Sets the value of `H`.
    #[inline(always)]
    pub fn set_h(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 32767_u16 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Msg1::MESSAGE_ID,
            });
        }
        self.raw.view_bits_mut::<Lsb0>()[33..48].store_le(value);
        Ok(())
    }
}
impl core::convert::TryFrom<&[u8]> for Msg1 {
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
impl embedded_can::Frame for Msg1 {
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
/// MSG2
///
/// - Standard ID: 3 (0x3)
/// - Size: 8 bytes
/// - Transmitter: E1
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
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(StandardId::new(0x3).unwrap());
    pub const MESSAGE_SIZE: usize = 8;
    pub const I_MIN: u8 = 0_u8;
    pub const I_MAX: u8 = 15_u8;
    pub const J_MIN: u8 = 0_u8;
    pub const J_MAX: u8 = 15_u8;
    pub const K_MIN: u8 = 0_u8;
    pub const K_MAX: u8 = 15_u8;
    /// Constructs a new `MSG2` message from values.
    pub fn new(i: u8, j: u8, k: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0x00; 8] };
        res.set_i(i)?;
        res.set_j(j)?;
        res.set_k(k)?;
        Ok(res)
    }
    /// Returns the raw `MSG2` message payload.
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    /// Returns the value of `I`.
    ///
    /// - Min: 0
    /// - Max: 15
    /// - Unit: Not specified
    /// - Receivers: E1
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn i(&self) -> u8 {
        self.i_raw_val()
    }
    /// Returns the raw value of `I`.
    ///
    /// - Start bit: 0
    /// - Signal size: 4 bits
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn i_raw_val(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[0..4].load_le::<u8>()
    }
    /// Sets the raw value of `I`.
    #[inline(always)]
    pub fn set_i_raw_val(&mut self, value: u8) {
        self.raw.view_bits_mut::<Lsb0>()[0..4].store_le(value);
    }
    /// Sets the value of `I`.
    #[inline(always)]
    pub fn set_i(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 15_u8 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Msg2::MESSAGE_ID,
            });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..4].store_le(value);
        Ok(())
    }
    /// Returns the value of `J`.
    ///
    /// - Min: 0
    /// - Max: 15
    /// - Unit: Not specified
    /// - Receivers: E1
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn j(&self) -> u8 {
        self.j_raw_val()
    }
    /// Returns the raw value of `J`.
    ///
    /// - Start bit: 4
    /// - Signal size: 4 bits
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn j_raw_val(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[4..8].load_le::<u8>()
    }
    /// Sets the raw value of `J`.
    #[inline(always)]
    pub fn set_j_raw_val(&mut self, value: u8) {
        self.raw.view_bits_mut::<Lsb0>()[4..8].store_le(value);
    }
    /// Sets the value of `J`.
    #[inline(always)]
    pub fn set_j(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 15_u8 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Msg2::MESSAGE_ID,
            });
        }
        self.raw.view_bits_mut::<Lsb0>()[4..8].store_le(value);
        Ok(())
    }
    /// Returns the value of `K`.
    ///
    /// - Min: 0
    /// - Max: 15
    /// - Unit: Not specified
    /// - Receivers: E1
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn k(&self) -> u8 {
        self.k_raw_val()
    }
    /// Returns the raw value of `K`.
    ///
    /// - Start bit: 8
    /// - Signal size: 4 bits
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn k_raw_val(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[8..12].load_le::<u8>()
    }
    /// Sets the raw value of `K`.
    #[inline(always)]
    pub fn set_k_raw_val(&mut self, value: u8) {
        self.raw.view_bits_mut::<Lsb0>()[8..12].store_le(value);
    }
    /// Sets the value of `K`.
    #[inline(always)]
    pub fn set_k(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 15_u8 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Msg2::MESSAGE_ID,
            });
        }
        self.raw.view_bits_mut::<Lsb0>()[8..12].store_le(value);
        Ok(())
    }
}
impl core::convert::TryFrom<&[u8]> for Msg2 {
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
impl embedded_can::Frame for Msg2 {
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
/// MSG3
///
/// - Standard ID: 4 (0x4)
/// - Size: 8 bytes
/// - Transmitter: E1
#[derive(Clone, Copy)]
pub struct Msg3 {
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
impl Msg3 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(StandardId::new(0x4).unwrap());
    pub const MESSAGE_SIZE: usize = 8;
    pub const L_MIN: u64 = 0_u64;
    pub const L_MAX: u64 = 18446744073709551615_u64;
    /// Constructs a new `MSG3` message from values.
    pub fn new(l: u64) -> Result<Self, CanError> {
        let mut res = Self { raw: [0x00; 8] };
        res.set_l(l)?;
        Ok(res)
    }
    /// Returns the raw `MSG3` message payload.
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    /// Returns the value of `L`.
    ///
    /// - Min: 0
    /// - Max: 18446744073709551615
    /// - Unit: Not specified
    /// - Receivers: E1
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn l(&self) -> u64 {
        self.l_raw_val()
    }
    /// Returns the raw value of `L`.
    ///
    /// - Start bit: 7
    /// - Signal size: 64 bits
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn l_raw_val(&self) -> u64 {
        self.raw.view_bits::<Msb0>()[0..64].load_be::<u64>()
    }
    /// Sets the raw value of `L`.
    #[inline(always)]
    pub fn set_l_raw_val(&mut self, value: u64) {
        self.raw.view_bits_mut::<Msb0>()[0..64].store_be(value);
    }
    /// Sets the value of `L`.
    #[inline(always)]
    pub fn set_l(&mut self, value: u64) -> Result<(), CanError> {
        if value < 0_u64 || 18446744073709551615_u64 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Msg3::MESSAGE_ID,
            });
        }
        self.raw.view_bits_mut::<Msb0>()[0..64].store_be(value);
        Ok(())
    }
}
impl core::convert::TryFrom<&[u8]> for Msg3 {
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
impl embedded_can::Frame for Msg3 {
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
/// MSG4
///
/// - Standard ID: 5 (0x5)
/// - Size: 8 bytes
/// - Transmitter: E1
#[derive(Clone, Copy)]
pub struct Msg4 {
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
impl Msg4 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(StandardId::new(0x5).unwrap());
    pub const MESSAGE_SIZE: usize = 8;
    pub const M_MIN: u64 = 0_u64;
    pub const M_MAX: u64 = 18446744073709551615_u64;
    /// Constructs a new `MSG4` message from values.
    pub fn new(m: u64) -> Result<Self, CanError> {
        let mut res = Self { raw: [0x00; 8] };
        res.set_m(m)?;
        Ok(res)
    }
    /// Returns the raw `MSG4` message payload.
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    /// Returns the value of `M`.
    ///
    /// - Min: 0
    /// - Max: 18446744073709551615
    /// - Unit: Not specified
    /// - Receivers: E1
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn m(&self) -> u64 {
        self.m_raw_val()
    }
    /// Returns the raw value of `M`.
    ///
    /// - Start bit: 0
    /// - Signal size: 64 bits
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn m_raw_val(&self) -> u64 {
        self.raw.view_bits::<Lsb0>()[0..64].load_le::<u64>()
    }
    /// Sets the raw value of `M`.
    #[inline(always)]
    pub fn set_m_raw_val(&mut self, value: u64) {
        self.raw.view_bits_mut::<Lsb0>()[0..64].store_le(value);
    }
    /// Sets the value of `M`.
    #[inline(always)]
    pub fn set_m(&mut self, value: u64) -> Result<(), CanError> {
        if value < 0_u64 || 18446744073709551615_u64 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: Msg4::MESSAGE_ID,
            });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..64].store_le(value);
        Ok(())
    }
}
impl core::convert::TryFrom<&[u8]> for Msg4 {
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
impl embedded_can::Frame for Msg4 {
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
