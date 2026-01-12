// Generated code!
//
// Message definitions from file `long_names_multiple_relations_dumped`
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
    /// Msg_Long_Name_56789_123456789_12
    MsgLongName5678912345678912(MsgLongName5678912345678912),
    /// TX_twice
    TxTwice(TxTwice),
    /// RX_TX_1
    RxTx1(RxTx1),
    /// MSG_CASE_TEST
    MsgCaseTest(MsgCaseTest),
    /// msg_case_test
    MsgCaseTest(MsgCaseTest),
    /// Msg_with_value_table_sigs
    MsgWithValueTableSigs(MsgWithValueTableSigs),
    /// Msg_Long_Name_56789_1234567_0000
    MsgLongName5678912345670000(MsgLongName5678912345670000),
    /// Msg_Long_Name_56789_1234567_0001
    MsgLongName5678912345670001(MsgLongName5678912345670001),
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
            MsgLongName5678912345678912::MESSAGE_ID => Messages::MsgLongName5678912345678912(MsgLongName5678912345678912::try_from(payload)?),
            TxTwice::MESSAGE_ID => Messages::TxTwice(TxTwice::try_from(payload)?),
            RxTx1::MESSAGE_ID => Messages::RxTx1(RxTx1::try_from(payload)?),
            MsgCaseTest::MESSAGE_ID => Messages::MsgCaseTest(MsgCaseTest::try_from(payload)?),
            MsgCaseTest::MESSAGE_ID => Messages::MsgCaseTest(MsgCaseTest::try_from(payload)?),
            MsgWithValueTableSigs::MESSAGE_ID => Messages::MsgWithValueTableSigs(MsgWithValueTableSigs::try_from(payload)?),
            MsgLongName5678912345670000::MESSAGE_ID => Messages::MsgLongName5678912345670000(MsgLongName5678912345670000::try_from(payload)?),
            MsgLongName5678912345670001::MESSAGE_ID => Messages::MsgLongName5678912345670001(MsgLongName5678912345670001::try_from(payload)?),
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}

/// Msg_Long_Name_56789_123456789_12
///
/// - Standard ID: 85 (0x55)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct MsgLongName5678912345678912 {
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
impl MsgLongName5678912345678912 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x55)});
    
    pub const RX_TWICE_SHORT_MIN: i32 = 0_i32;
    pub const RX_TWICE_SHORT_MAX: i32 = 0_i32;
    pub const RX_TWICE_11111111111111111111111_MIN: i8 = 0_i8;
    pub const RX_TWICE_11111111111111111111111_MAX: i8 = 0_i8;
    pub const SIG_USED_TWICE_EFGH_ABCDEFGHI_AB_MIN: i8 = 0_i8;
    pub const SIG_USED_TWICE_EFGH_ABCDEFGHI_AB_MAX: i8 = 0_i8;
    
    /// Construct new Msg_Long_Name_56789_123456789_12 from values
    pub fn new(rx_twice_short: i32, rx_twice_11111111111111111111111: i8, sig_used_twice_efgh_abcdefghi_ab: i8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_rx_twice_short(rx_twice_short)?;
        res.set_rx_twice_11111111111111111111111(rx_twice_11111111111111111111111)?;
        res.set_sig_used_twice_efgh_abcdefghi_ab(sig_used_twice_efgh_abcdefghi_ab)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// rx_twice_short
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn rx_twice_short(&self) -> i32 {
        self.rx_twice_short_raw()
    }
    
    /// Get raw value of rx_twice_short
    ///
    /// - Start bit: 16
    /// - Signal size: 18 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn rx_twice_short_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..34].load_le::<i32>();
        
        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of rx_twice_short
    #[inline(always)]
    pub fn set_rx_twice_short(&mut self, value: i32) -> Result<(), CanError> {
        if value < 0_i32 || 0_i32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: MsgLongName5678912345678912::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: MsgLongName5678912345678912::MESSAGE_ID })?;
        let value = (value / factor) as i32;
        
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[16..34].store_le(value);
        Ok(())
    }
    
    /// rx_twice_11111111111111111111111
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn rx_twice_11111111111111111111111(&self) -> i8 {
        self.rx_twice_11111111111111111111111_raw()
    }
    
    /// Get raw value of rx_twice_11111111111111111111111
    ///
    /// - Start bit: 8
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn rx_twice_11111111111111111111111_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of rx_twice_11111111111111111111111
    #[inline(always)]
    pub fn set_rx_twice_11111111111111111111111(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: MsgLongName5678912345678912::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: MsgLongName5678912345678912::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
        Ok(())
    }
    
    /// Sig_used_twice_efgh_abcdefghi_ab
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn sig_used_twice_efgh_abcdefghi_ab(&self) -> i8 {
        self.sig_used_twice_efgh_abcdefghi_ab_raw()
    }
    
    /// Get raw value of Sig_used_twice_efgh_abcdefghi_ab
    ///
    /// - Start bit: 0
    /// - Signal size: 6 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn sig_used_twice_efgh_abcdefghi_ab_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..6].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Sig_used_twice_efgh_abcdefghi_ab
    #[inline(always)]
    pub fn set_sig_used_twice_efgh_abcdefghi_ab(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: MsgLongName5678912345678912::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: MsgLongName5678912345678912::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..6].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for MsgLongName5678912345678912 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for MsgLongName5678912345678912 {
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

/// TX_twice
///
/// - Standard ID: 6 (0x6)
/// - Size: 2 bytes
/// - Transmitter: Node_6789_123456789_123456789_12
#[derive(Clone, Copy)]
pub struct TxTwice {
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
impl TxTwice {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x6)});
    
    pub const RX_TWICE_LONG_YYYYYYYYYYYYYYYYYY_MIN: i8 = 0_i8;
    pub const RX_TWICE_LONG_YYYYYYYYYYYYYYYYYY_MAX: i8 = 0_i8;
    pub const RX_TWICE_SHORT_MIN: i8 = 0_i8;
    pub const RX_TWICE_SHORT_MAX: i8 = 0_i8;
    
    /// Construct new TX_twice from values
    pub fn new(rx_twice_long_yyyyyyyyyyyyyyyyyy: i8, rx_twice_short: i8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 2] };
        res.set_rx_twice_long_yyyyyyyyyyyyyyyyyy(rx_twice_long_yyyyyyyyyyyyyyyyyy)?;
        res.set_rx_twice_short(rx_twice_short)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 2] {
        &self.raw
    }
    
    /// rx_twice_long_yyyyyyyyyyyyyyyyyy
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Receiver_1, Receiver_2_zzzzzzzzzzzzzzzzzzzzz
    #[inline(always)]
    pub fn rx_twice_long_yyyyyyyyyyyyyyyyyy(&self) -> i8 {
        self.rx_twice_long_yyyyyyyyyyyyyyyyyy_raw()
    }
    
    /// Get raw value of rx_twice_long_yyyyyyyyyyyyyyyyyy
    ///
    /// - Start bit: 8
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn rx_twice_long_yyyyyyyyyyyyyyyyyy_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of rx_twice_long_yyyyyyyyyyyyyyyyyy
    #[inline(always)]
    pub fn set_rx_twice_long_yyyyyyyyyyyyyyyyyy(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: TxTwice::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: TxTwice::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
        Ok(())
    }
    
    /// rx_twice_short
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Receiver_2_zzzzzzzzzzzzzzzzzzzzz, Receiver_1
    #[inline(always)]
    pub fn rx_twice_short(&self) -> i8 {
        self.rx_twice_short_raw()
    }
    
    /// Get raw value of rx_twice_short
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn rx_twice_short_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of rx_twice_short
    #[inline(always)]
    pub fn set_rx_twice_short(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: TxTwice::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: TxTwice::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for TxTwice {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 2 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 2];
        raw.copy_from_slice(&payload[..2]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for TxTwice {
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

/// RX_TX_1
///
/// - Standard ID: 5 (0x5)
/// - Size: 8 bytes
/// - Transmitter: Node_6789_123456789_123456789_12
#[derive(Clone, Copy)]
pub struct RxTx1 {
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
impl RxTx1 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x5)});
    
    pub const SIG_USED_TWICE_EFGH_ABCDEFG_0000_MIN: i16 = 0_i16;
    pub const SIG_USED_TWICE_EFGH_ABCDEFG_0000_MAX: i16 = 0_i16;
    
    /// Construct new RX_TX_1 from values
    pub fn new(sig_used_twice_efgh_abcdefg_0000: i16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_sig_used_twice_efgh_abcdefg_0000(sig_used_twice_efgh_abcdefg_0000)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// Sig_used_twice_efgh_abcdefg_0000
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn sig_used_twice_efgh_abcdefg_0000(&self) -> i16 {
        self.sig_used_twice_efgh_abcdefg_0000_raw()
    }
    
    /// Get raw value of Sig_used_twice_efgh_abcdefg_0000
    ///
    /// - Start bit: 0
    /// - Signal size: 9 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn sig_used_twice_efgh_abcdefg_0000_raw(&self) -> i16 {
        let signal = self.raw.view_bits::<Lsb0>()[0..9].load_le::<i16>();
        
        let factor = 1;
        let signal = signal as i16;
        i16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Sig_used_twice_efgh_abcdefg_0000
    #[inline(always)]
    pub fn set_sig_used_twice_efgh_abcdefg_0000(&mut self, value: i16) -> Result<(), CanError> {
        if value < 0_i16 || 0_i16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: RxTx1::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: RxTx1::MESSAGE_ID })?;
        let value = (value / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..9].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for RxTx1 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for RxTx1 {
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

/// MSG_CASE_TEST
///
/// - Standard ID: 4 (0x4)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct MsgCaseTest {
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
impl MsgCaseTest {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x4)});
    
    
    /// Construct new MSG_CASE_TEST from values
    pub fn new() -> Result<Self, CanError> {
        let res = Self { raw: [0u8; 8] };
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
}

impl core::convert::TryFrom<&[u8]> for MsgCaseTest {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for MsgCaseTest {
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

/// msg_case_test
///
/// - Standard ID: 3 (0x3)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct MsgCaseTest {
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
impl MsgCaseTest {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x3)});
    
    
    /// Construct new msg_case_test from values
    pub fn new() -> Result<Self, CanError> {
        let res = Self { raw: [0u8; 8] };
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
}

impl core::convert::TryFrom<&[u8]> for MsgCaseTest {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for MsgCaseTest {
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

/// Msg_with_value_table_sigs
///
/// - Standard ID: 2 (0x2)
/// - Size: 3 bytes
#[derive(Clone, Copy)]
pub struct MsgWithValueTableSigs {
    raw: [u8; 3],
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
impl MsgWithValueTableSigs {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x2)});
    
    pub const SIG_WITH_SHORT_VAL_TABLE_MIN: i8 = 0_i8;
    pub const SIG_WITH_SHORT_VAL_TABLE_MAX: i8 = 0_i8;
    pub const SIG_WITH_LONG_VAL_TABLE_2_MIN: i8 = 0_i8;
    pub const SIG_WITH_LONG_VAL_TABLE_2_MAX: i8 = 0_i8;
    pub const SIG_WITH_LONG_VAL_TABLE_1_MIN: i8 = 0_i8;
    pub const SIG_WITH_LONG_VAL_TABLE_1_MAX: i8 = 0_i8;
    
    /// Construct new Msg_with_value_table_sigs from values
    pub fn new(sig_with_short_val_table: i8, sig_with_long_val_table_2: i8, sig_with_long_val_table_1: i8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 3] };
        res.set_sig_with_short_val_table(sig_with_short_val_table)?;
        res.set_sig_with_long_val_table_2(sig_with_long_val_table_2)?;
        res.set_sig_with_long_val_table_1(sig_with_long_val_table_1)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 3] {
        &self.raw
    }
    
    /// Sig_with_short_val_table
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn sig_with_short_val_table(&self) -> MsgWithValueTableSigsSigWithShortValTable {
        let signal = self.raw.view_bits::<Lsb0>()[16..24].load_le::<u8>();
        
        match signal {
            1 => MsgWithValueTableSigsSigWithShortValTable::VeryLongLongLongDescriptionForTheValue0x1,
            0 => MsgWithValueTableSigsSigWithShortValTable::VeryLongLongLongDescriptionForTheValue0x0,
            _ => MsgWithValueTableSigsSigWithShortValTable::_Other(self.sig_with_short_val_table_raw()),
        }
    }
    
    /// Get raw value of Sig_with_short_val_table
    ///
    /// - Start bit: 16
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn sig_with_short_val_table_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[16..24].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Sig_with_short_val_table
    #[inline(always)]
    pub fn set_sig_with_short_val_table(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: MsgWithValueTableSigs::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: MsgWithValueTableSigs::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[16..24].store_le(value);
        Ok(())
    }
    
    /// Sig_with_long_val_table_2
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn sig_with_long_val_table_2(&self) -> MsgWithValueTableSigsSigWithLongValTable2 {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
        
        match signal {
            13 => MsgWithValueTableSigsSigWithLongValTable2::Value0xD,
            12 => MsgWithValueTableSigsSigWithLongValTable2::Dvalue0xC,
            11 => MsgWithValueTableSigsSigWithLongValTable2::Value0xB,
            10 => MsgWithValueTableSigsSigWithLongValTable2::Value0xA,
            9 => MsgWithValueTableSigsSigWithLongValTable2::Value0x9,
            8 => MsgWithValueTableSigsSigWithLongValTable2::Value0x8,
            7 => MsgWithValueTableSigsSigWithLongValTable2::Value0x7,
            6 => MsgWithValueTableSigsSigWithLongValTable2::Value0x6,
            5 => MsgWithValueTableSigsSigWithLongValTable2::Value0x5,
            4 => MsgWithValueTableSigsSigWithLongValTable2::Value0x4,
            3 => MsgWithValueTableSigsSigWithLongValTable2::Value0x3,
            2 => MsgWithValueTableSigsSigWithLongValTable2::Value0x2,
            1 => MsgWithValueTableSigsSigWithLongValTable2::Value0x1,
            0 => MsgWithValueTableSigsSigWithLongValTable2::Value0x0,
            _ => MsgWithValueTableSigsSigWithLongValTable2::_Other(self.sig_with_long_val_table_2_raw()),
        }
    }
    
    /// Get raw value of Sig_with_long_val_table_2
    ///
    /// - Start bit: 8
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn sig_with_long_val_table_2_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Sig_with_long_val_table_2
    #[inline(always)]
    pub fn set_sig_with_long_val_table_2(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: MsgWithValueTableSigs::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: MsgWithValueTableSigs::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
        Ok(())
    }
    
    /// Sig_with_long_val_table_1
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn sig_with_long_val_table_1(&self) -> MsgWithValueTableSigsSigWithLongValTable1 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        match signal {
            3 => MsgWithValueTableSigsSigWithLongValTable1::DescriptionForTheValue0x3,
            2 => MsgWithValueTableSigsSigWithLongValTable1::DescriptionForTheValue0x2,
            1 => MsgWithValueTableSigsSigWithLongValTable1::DescriptionForTheValue0x1,
            0 => MsgWithValueTableSigsSigWithLongValTable1::DescriptionForTheValue0x0,
            _ => MsgWithValueTableSigsSigWithLongValTable1::_Other(self.sig_with_long_val_table_1_raw()),
        }
    }
    
    /// Get raw value of Sig_with_long_val_table_1
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn sig_with_long_val_table_1_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Sig_with_long_val_table_1
    #[inline(always)]
    pub fn set_sig_with_long_val_table_1(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: MsgWithValueTableSigs::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: MsgWithValueTableSigs::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for MsgWithValueTableSigs {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 3 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 3];
        raw.copy_from_slice(&payload[..3]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for MsgWithValueTableSigs {
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
/// Defined values for Sig_with_short_val_table
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
pub enum MsgWithValueTableSigsSigWithShortValTable {
    VeryLongLongLongDescriptionForTheValue0x1,
    VeryLongLongLongDescriptionForTheValue0x0,
    _Other(i8),
}

impl From<MsgWithValueTableSigsSigWithShortValTable> for i8 {
    fn from(val: MsgWithValueTableSigsSigWithShortValTable) -> i8 {
        match val {
            MsgWithValueTableSigsSigWithShortValTable::VeryLongLongLongDescriptionForTheValue0x1 => 1,
            MsgWithValueTableSigsSigWithShortValTable::VeryLongLongLongDescriptionForTheValue0x0 => 0,
            MsgWithValueTableSigsSigWithShortValTable::_Other(x) => x,
        }
    }
}

/// Defined values for Sig_with_long_val_table_2
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
pub enum MsgWithValueTableSigsSigWithLongValTable2 {
    Value0xD,
    Dvalue0xC,
    Value0xB,
    Value0xA,
    Value0x9,
    Value0x8,
    Value0x7,
    Value0x6,
    Value0x5,
    Value0x4,
    Value0x3,
    Value0x2,
    Value0x1,
    Value0x0,
    _Other(i8),
}

impl From<MsgWithValueTableSigsSigWithLongValTable2> for i8 {
    fn from(val: MsgWithValueTableSigsSigWithLongValTable2) -> i8 {
        match val {
            MsgWithValueTableSigsSigWithLongValTable2::Value0xD => 13,
            MsgWithValueTableSigsSigWithLongValTable2::Dvalue0xC => 12,
            MsgWithValueTableSigsSigWithLongValTable2::Value0xB => 11,
            MsgWithValueTableSigsSigWithLongValTable2::Value0xA => 10,
            MsgWithValueTableSigsSigWithLongValTable2::Value0x9 => 9,
            MsgWithValueTableSigsSigWithLongValTable2::Value0x8 => 8,
            MsgWithValueTableSigsSigWithLongValTable2::Value0x7 => 7,
            MsgWithValueTableSigsSigWithLongValTable2::Value0x6 => 6,
            MsgWithValueTableSigsSigWithLongValTable2::Value0x5 => 5,
            MsgWithValueTableSigsSigWithLongValTable2::Value0x4 => 4,
            MsgWithValueTableSigsSigWithLongValTable2::Value0x3 => 3,
            MsgWithValueTableSigsSigWithLongValTable2::Value0x2 => 2,
            MsgWithValueTableSigsSigWithLongValTable2::Value0x1 => 1,
            MsgWithValueTableSigsSigWithLongValTable2::Value0x0 => 0,
            MsgWithValueTableSigsSigWithLongValTable2::_Other(x) => x,
        }
    }
}

/// Defined values for Sig_with_long_val_table_1
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
pub enum MsgWithValueTableSigsSigWithLongValTable1 {
    DescriptionForTheValue0x3,
    DescriptionForTheValue0x2,
    DescriptionForTheValue0x1,
    DescriptionForTheValue0x0,
    _Other(i8),
}

impl From<MsgWithValueTableSigsSigWithLongValTable1> for i8 {
    fn from(val: MsgWithValueTableSigsSigWithLongValTable1) -> i8 {
        match val {
            MsgWithValueTableSigsSigWithLongValTable1::DescriptionForTheValue0x3 => 3,
            MsgWithValueTableSigsSigWithLongValTable1::DescriptionForTheValue0x2 => 2,
            MsgWithValueTableSigsSigWithLongValTable1::DescriptionForTheValue0x1 => 1,
            MsgWithValueTableSigsSigWithLongValTable1::DescriptionForTheValue0x0 => 0,
            MsgWithValueTableSigsSigWithLongValTable1::_Other(x) => x,
        }
    }
}


/// Msg_Long_Name_56789_1234567_0000
///
/// - Standard ID: 1 (0x1)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct MsgLongName5678912345670000 {
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
impl MsgLongName5678912345670000 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x1)});
    
    pub const RX_TWICE_11111111111111111111111_MIN: i8 = 0_i8;
    pub const RX_TWICE_11111111111111111111111_MAX: i8 = 0_i8;
    pub const SIG_USED_TWICE_EFGH_ABCDEFG_0001_MIN: i8 = 0_i8;
    pub const SIG_USED_TWICE_EFGH_ABCDEFG_0001_MAX: i8 = 0_i8;
    
    /// Construct new Msg_Long_Name_56789_1234567_0000 from values
    pub fn new(rx_twice_11111111111111111111111: i8, sig_used_twice_efgh_abcdefg_0001: i8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_rx_twice_11111111111111111111111(rx_twice_11111111111111111111111)?;
        res.set_sig_used_twice_efgh_abcdefg_0001(sig_used_twice_efgh_abcdefg_0001)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// rx_twice_11111111111111111111111
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn rx_twice_11111111111111111111111(&self) -> i8 {
        self.rx_twice_11111111111111111111111_raw()
    }
    
    /// Get raw value of rx_twice_11111111111111111111111
    ///
    /// - Start bit: 8
    /// - Signal size: 2 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn rx_twice_11111111111111111111111_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..10].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of rx_twice_11111111111111111111111
    #[inline(always)]
    pub fn set_rx_twice_11111111111111111111111(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: MsgLongName5678912345670000::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: MsgLongName5678912345670000::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[8..10].store_le(value);
        Ok(())
    }
    
    /// Sig_used_twice_efgh_abcdefg_0001
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn sig_used_twice_efgh_abcdefg_0001(&self) -> i8 {
        self.sig_used_twice_efgh_abcdefg_0001_raw()
    }
    
    /// Get raw value of Sig_used_twice_efgh_abcdefg_0001
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn sig_used_twice_efgh_abcdefg_0001_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Sig_used_twice_efgh_abcdefg_0001
    #[inline(always)]
    pub fn set_sig_used_twice_efgh_abcdefg_0001(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: MsgLongName5678912345670000::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: MsgLongName5678912345670000::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for MsgLongName5678912345670000 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for MsgLongName5678912345670000 {
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

/// Msg_Long_Name_56789_1234567_0001
///
/// - Standard ID: 0 (0x0)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct MsgLongName5678912345670001 {
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
impl MsgLongName5678912345670001 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x0)});
    
    pub const RX_TWICE_SHORT_MIN: i8 = 0_i8;
    pub const RX_TWICE_SHORT_MAX: i8 = 0_i8;
    pub const SIG_USED_TWICE_EFGH_ABCDEFG_0002_MIN: i8 = 0_i8;
    pub const SIG_USED_TWICE_EFGH_ABCDEFG_0002_MAX: i8 = 0_i8;
    
    /// Construct new Msg_Long_Name_56789_1234567_0001 from values
    pub fn new(rx_twice_short: i8, sig_used_twice_efgh_abcdefg_0002: i8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_rx_twice_short(rx_twice_short)?;
        res.set_sig_used_twice_efgh_abcdefg_0002(sig_used_twice_efgh_abcdefg_0002)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// rx_twice_short
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn rx_twice_short(&self) -> i8 {
        self.rx_twice_short_raw()
    }
    
    /// Get raw value of rx_twice_short
    ///
    /// - Start bit: 8
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn rx_twice_short_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of rx_twice_short
    #[inline(always)]
    pub fn set_rx_twice_short(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: MsgLongName5678912345670001::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: MsgLongName5678912345670001::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
        Ok(())
    }
    
    /// Sig_used_twice_efgh_abcdefg_0002
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn sig_used_twice_efgh_abcdefg_0002(&self) -> i8 {
        self.sig_used_twice_efgh_abcdefg_0002_raw()
    }
    
    /// Get raw value of Sig_used_twice_efgh_abcdefg_0002
    ///
    /// - Start bit: 0
    /// - Signal size: 6 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn sig_used_twice_efgh_abcdefg_0002_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..6].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Sig_used_twice_efgh_abcdefg_0002
    #[inline(always)]
    pub fn set_sig_used_twice_efgh_abcdefg_0002(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: MsgLongName5678912345670001::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: MsgLongName5678912345670001::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..6].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for MsgLongName5678912345670001 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for MsgLongName5678912345670001 {
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

