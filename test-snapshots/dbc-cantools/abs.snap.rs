// Generated code!
//
// Message definitions from file `abs`
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
    /// BREMSE_33
    Bremse33(Bremse33),
    /// BREMSE_10
    Bremse10(Bremse10),
    /// BREMSE_11
    Bremse11(Bremse11),
    /// BREMSE_12
    Bremse12(Bremse12),
    /// BREMSE_13
    Bremse13(Bremse13),
    /// DRS_RX_ID0
    DrsRxId0(DrsRxId0),
    /// MM5_10_TX1
    Mm510Tx1(Mm510Tx1),
    /// MM5_10_TX2
    Mm510Tx2(Mm510Tx2),
    /// MM5_10_TX3
    Mm510Tx3(Mm510Tx3),
    /// BREMSE_2
    Bremse2(Bremse2),
    /// ABS_Switch
    AbsSwitch(AbsSwitch),
    /// BREMSE_30
    Bremse30(Bremse30),
    /// BREMSE_31
    Bremse31(Bremse31),
    /// BREMSE_32
    Bremse32(Bremse32),
    /// BREMSE_51
    Bremse51(Bremse51),
    /// BREMSE_52
    Bremse52(Bremse52),
    /// BREMSE_50
    Bremse50(Bremse50),
    /// BREMSE_53
    Bremse53(Bremse53),
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
            Bremse33::MESSAGE_ID => Messages::Bremse33(Bremse33::try_from(payload)?),
            Bremse10::MESSAGE_ID => Messages::Bremse10(Bremse10::try_from(payload)?),
            Bremse11::MESSAGE_ID => Messages::Bremse11(Bremse11::try_from(payload)?),
            Bremse12::MESSAGE_ID => Messages::Bremse12(Bremse12::try_from(payload)?),
            Bremse13::MESSAGE_ID => Messages::Bremse13(Bremse13::try_from(payload)?),
            DrsRxId0::MESSAGE_ID => Messages::DrsRxId0(DrsRxId0::try_from(payload)?),
            Mm510Tx1::MESSAGE_ID => Messages::Mm510Tx1(Mm510Tx1::try_from(payload)?),
            Mm510Tx2::MESSAGE_ID => Messages::Mm510Tx2(Mm510Tx2::try_from(payload)?),
            Mm510Tx3::MESSAGE_ID => Messages::Mm510Tx3(Mm510Tx3::try_from(payload)?),
            Bremse2::MESSAGE_ID => Messages::Bremse2(Bremse2::try_from(payload)?),
            AbsSwitch::MESSAGE_ID => Messages::AbsSwitch(AbsSwitch::try_from(payload)?),
            Bremse30::MESSAGE_ID => Messages::Bremse30(Bremse30::try_from(payload)?),
            Bremse31::MESSAGE_ID => Messages::Bremse31(Bremse31::try_from(payload)?),
            Bremse32::MESSAGE_ID => Messages::Bremse32(Bremse32::try_from(payload)?),
            Bremse51::MESSAGE_ID => Messages::Bremse51(Bremse51::try_from(payload)?),
            Bremse52::MESSAGE_ID => Messages::Bremse52(Bremse52::try_from(payload)?),
            Bremse50::MESSAGE_ID => Messages::Bremse50(Bremse50::try_from(payload)?),
            Bremse53::MESSAGE_ID => Messages::Bremse53(Bremse53::try_from(payload)?),
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}

/// BREMSE_33
///
/// - Standard ID: 835 (0x343)
/// - Size: 8 bytes
/// - Transmitter: ABS
#[derive(Clone, Copy)]
pub struct Bremse33 {
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
impl Bremse33 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x343)});
    
    pub const WHLSPEED_FL_MIN: f32 = 0_f32;
    pub const WHLSPEED_FL_MAX: f32 = 100_f32;
    pub const WHLSPEED_FR_MIN: f32 = 0_f32;
    pub const WHLSPEED_FR_MAX: f32 = 100_f32;
    pub const WHLSPEED_RL_MIN: f32 = 0_f32;
    pub const WHLSPEED_RL_MAX: f32 = 100_f32;
    pub const WHLSPEED_RR_MIN: f32 = 0_f32;
    pub const WHLSPEED_RR_MAX: f32 = 100_f32;
    
    /// Construct new BREMSE_33 from values
    pub fn new(whlspeed_fl: f32, whlspeed_fr: f32, whlspeed_rl: f32, whlspeed_rr: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_whlspeed_fl(whlspeed_fl)?;
        res.set_whlspeed_fr(whlspeed_fr)?;
        res.set_whlspeed_rl(whlspeed_rl)?;
        res.set_whlspeed_rr(whlspeed_rr)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// whlspeed_FL
    ///
    /// Radgeschwindigkeit / wheel speed absCtrl FL
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "m/s"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn whlspeed_fl(&self) -> f32 {
        self.whlspeed_fl_raw()
    }
    
    /// Get raw value of whlspeed_FL
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.015625
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn whlspeed_fl_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 0.015625_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of whlspeed_FL
    #[inline(always)]
    pub fn set_whlspeed_fl(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 100_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse33::MESSAGE_ID });
        }
        let factor = 0.015625_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// whlspeed_FR
    ///
    /// Radgeschwindigkeit / wheel speed absCtrl FR
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "m/s"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn whlspeed_fr(&self) -> f32 {
        self.whlspeed_fr_raw()
    }
    
    /// Get raw value of whlspeed_FR
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.015625
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn whlspeed_fr_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let factor = 0.015625_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of whlspeed_FR
    #[inline(always)]
    pub fn set_whlspeed_fr(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 100_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse33::MESSAGE_ID });
        }
        let factor = 0.015625_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// whlspeed_RL
    ///
    /// Radgeschwindigkeit / wheel speed absCtrl RL
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "m/s"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn whlspeed_rl(&self) -> f32 {
        self.whlspeed_rl_raw()
    }
    
    /// Get raw value of whlspeed_RL
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 0.015625
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn whlspeed_rl_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let factor = 0.015625_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of whlspeed_RL
    #[inline(always)]
    pub fn set_whlspeed_rl(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 100_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse33::MESSAGE_ID });
        }
        let factor = 0.015625_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// whlspeed_RR
    ///
    /// Radgeschwindigkeit / wheel speed absCtrl RR
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "m/s"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn whlspeed_rr(&self) -> f32 {
        self.whlspeed_rr_raw()
    }
    
    /// Get raw value of whlspeed_RR
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 0.015625
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn whlspeed_rr_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        let factor = 0.015625_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of whlspeed_RR
    #[inline(always)]
    pub fn set_whlspeed_rr(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 100_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse33::MESSAGE_ID });
        }
        let factor = 0.015625_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Bremse33 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Bremse33 {
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

/// BREMSE_10
///
/// - Standard ID: 320 (0x140)
/// - Size: 8 bytes
/// - Transmitter: ABS
#[derive(Clone, Copy)]
pub struct Bremse10 {
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
impl Bremse10 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x140)});
    
    
    /// Construct new BREMSE_10 from values
    pub fn new() -> Result<Self, CanError> {
        let res = Self { raw: [0u8; 8] };
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
}

impl core::convert::TryFrom<&[u8]> for Bremse10 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Bremse10 {
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

/// BREMSE_11
///
/// - Standard ID: 321 (0x141)
/// - Size: 8 bytes
/// - Transmitter: ABS
#[derive(Clone, Copy)]
pub struct Bremse11 {
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
impl Bremse11 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x141)});
    
    
    /// Construct new BREMSE_11 from values
    pub fn new() -> Result<Self, CanError> {
        let res = Self { raw: [0u8; 8] };
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
}

impl core::convert::TryFrom<&[u8]> for Bremse11 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Bremse11 {
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

/// BREMSE_12
///
/// - Standard ID: 322 (0x142)
/// - Size: 8 bytes
/// - Transmitter: ABS
#[derive(Clone, Copy)]
pub struct Bremse12 {
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
impl Bremse12 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x142)});
    
    
    /// Construct new BREMSE_12 from values
    pub fn new() -> Result<Self, CanError> {
        let res = Self { raw: [0u8; 8] };
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
}

impl core::convert::TryFrom<&[u8]> for Bremse12 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Bremse12 {
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

/// BREMSE_13
///
/// - Standard ID: 323 (0x143)
/// - Size: 8 bytes
/// - Transmitter: ABS
#[derive(Clone, Copy)]
pub struct Bremse13 {
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
impl Bremse13 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x143)});
    
    
    /// Construct new BREMSE_13 from values
    pub fn new() -> Result<Self, CanError> {
        let res = Self { raw: [0u8; 8] };
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
}

impl core::convert::TryFrom<&[u8]> for Bremse13 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Bremse13 {
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

/// DRS_RX_ID0
///
/// - Standard ID: 117 (0x75)
/// - Size: 8 bytes
/// - Transmitter: ABS
#[derive(Clone, Copy)]
pub struct DrsRxId0 {
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
impl DrsRxId0 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x75)});
    
    
    /// Construct new DRS_RX_ID0 from values
    pub fn new() -> Result<Self, CanError> {
        let res = Self { raw: [0u8; 8] };
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
}

impl core::convert::TryFrom<&[u8]> for DrsRxId0 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for DrsRxId0 {
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

/// MM5_10_TX1
///
/// - Standard ID: 112 (0x70)
/// - Size: 8 bytes
/// - Transmitter: DRS_MM5_10
#[derive(Clone, Copy)]
pub struct Mm510Tx1 {
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
impl Mm510Tx1 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x70)});
    
    pub const YAW_RATE_MIN: f32 = -163.84_f32;
    pub const YAW_RATE_MAX: f32 = 163.83_f32;
    pub const AY1_MIN: f32 = -4.1768_f32;
    pub const AY1_MAX: f32 = 4.1765_f32;
    
    /// Construct new MM5_10_TX1 from values
    pub fn new(yaw_rate: f32, ay1: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_yaw_rate(yaw_rate)?;
        res.set_ay1(ay1)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// Yaw_Rate
    ///
    /// Measured yaw rate around the Z axle.
    ///
    /// - Min: -163.84
    /// - Max: 163.83
    /// - Unit: "°/s"
    /// - Receivers: ABS
    #[inline(always)]
    pub fn yaw_rate(&self) -> f32 {
        self.yaw_rate_raw()
    }
    
    /// Get raw value of Yaw_Rate
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.005
    /// - Offset: -163.84
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn yaw_rate_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 0.005_f32;
        let offset = -163.84_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of Yaw_Rate
    #[inline(always)]
    pub fn set_yaw_rate(&mut self, value: f32) -> Result<(), CanError> {
        if value < -163.84_f32 || 163.83_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Mm510Tx1::MESSAGE_ID });
        }
        let factor = 0.005_f32;
        let offset = -163.84_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// AY1
    ///
    /// Measured lateral acceleration.
    ///
    /// - Min: -4.1768
    /// - Max: 4.1765
    /// - Unit: "g"
    /// - Receivers: ABS
    #[inline(always)]
    pub fn ay1(&self) -> f32 {
        self.ay1_raw()
    }
    
    /// Get raw value of AY1
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 0.000127465
    /// - Offset: -4.1768
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn ay1_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let factor = 0.000127465_f32;
        let offset = -4.1768_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of AY1
    #[inline(always)]
    pub fn set_ay1(&mut self, value: f32) -> Result<(), CanError> {
        if value < -4.1768_f32 || 4.1765_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Mm510Tx1::MESSAGE_ID });
        }
        let factor = 0.000127465_f32;
        let offset = -4.1768_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Mm510Tx1 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Mm510Tx1 {
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

/// MM5_10_TX2
///
/// - Standard ID: 128 (0x80)
/// - Size: 8 bytes
/// - Transmitter: DRS_MM5_10
#[derive(Clone, Copy)]
pub struct Mm510Tx2 {
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
impl Mm510Tx2 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x80)});
    
    pub const ROLL_RATE_MIN: f32 = -163.84_f32;
    pub const ROLL_RATE_MAX: f32 = 163.835_f32;
    pub const AX1_MIN: f32 = -4.1768_f32;
    pub const AX1_MAX: f32 = 4.1765_f32;
    
    /// Construct new MM5_10_TX2 from values
    pub fn new(roll_rate: f32, ax1: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_roll_rate(roll_rate)?;
        res.set_ax1(ax1)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// Roll_Rate
    ///
    /// Measured roll rate around the X axle.
    ///
    /// - Min: -163.84
    /// - Max: 163.835
    /// - Unit: "°/s"
    /// - Receivers: ABS
    #[inline(always)]
    pub fn roll_rate(&self) -> f32 {
        self.roll_rate_raw()
    }
    
    /// Get raw value of Roll_Rate
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.005
    /// - Offset: -163.84
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn roll_rate_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 0.005_f32;
        let offset = -163.84_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of Roll_Rate
    #[inline(always)]
    pub fn set_roll_rate(&mut self, value: f32) -> Result<(), CanError> {
        if value < -163.84_f32 || 163.835_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Mm510Tx2::MESSAGE_ID });
        }
        let factor = 0.005_f32;
        let offset = -163.84_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// AX1
    ///
    /// Measured longitudional acceleration.
    ///
    /// - Min: -4.1768
    /// - Max: 4.1765
    /// - Unit: "g"
    /// - Receivers: ABS
    #[inline(always)]
    pub fn ax1(&self) -> f32 {
        self.ax1_raw()
    }
    
    /// Get raw value of AX1
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 0.000127465
    /// - Offset: -4.1768
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn ax1_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let factor = 0.000127465_f32;
        let offset = -4.1768_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of AX1
    #[inline(always)]
    pub fn set_ax1(&mut self, value: f32) -> Result<(), CanError> {
        if value < -4.1768_f32 || 4.1765_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Mm510Tx2::MESSAGE_ID });
        }
        let factor = 0.000127465_f32;
        let offset = -4.1768_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Mm510Tx2 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Mm510Tx2 {
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

/// MM5_10_TX3
///
/// - Standard ID: 1398 (0x576)
/// - Size: 8 bytes
/// - Transmitter: DRS_MM5_10
#[derive(Clone, Copy)]
pub struct Mm510Tx3 {
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
impl Mm510Tx3 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x576)});
    
    pub const AZ_MIN: f32 = -4.1768_f32;
    pub const AZ_MAX: f32 = 4.1765_f32;
    
    /// Construct new MM5_10_TX3 from values
    pub fn new(az: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_az(az)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// AZ
    ///
    /// Measured vertical acceleration.
    ///
    /// - Min: -4.1768
    /// - Max: 4.1765
    /// - Unit: "g"
    /// - Receivers: ABS
    #[inline(always)]
    pub fn az(&self) -> f32 {
        self.az_raw()
    }
    
    /// Get raw value of AZ
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 0.000127465
    /// - Offset: -4.1768
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn az_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let factor = 0.000127465_f32;
        let offset = -4.1768_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of AZ
    #[inline(always)]
    pub fn set_az(&mut self, value: f32) -> Result<(), CanError> {
        if value < -4.1768_f32 || 4.1765_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Mm510Tx3::MESSAGE_ID });
        }
        let factor = 0.000127465_f32;
        let offset = -4.1768_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Mm510Tx3 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Mm510Tx3 {
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

/// BREMSE_2
///
/// - Standard ID: 586 (0x24a)
/// - Size: 8 bytes
/// - Transmitter: ABS
#[derive(Clone, Copy)]
pub struct Bremse2 {
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
impl Bremse2 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x24a)});
    
    pub const WHLSPEED_FL_BREMSE2_MIN: f32 = 0_f32;
    pub const WHLSPEED_FL_BREMSE2_MAX: f32 = 100_f32;
    pub const WHLSPEED_FR_BREMSE2_MIN: f32 = 0_f32;
    pub const WHLSPEED_FR_BREMSE2_MAX: f32 = 100_f32;
    pub const WHLSPEED_RL_BREMSE2_MIN: f32 = 0_f32;
    pub const WHLSPEED_RL_BREMSE2_MAX: f32 = 100_f32;
    pub const WHLSPEED_RR_BREMSE2_MIN: f32 = 0_f32;
    pub const WHLSPEED_RR_BREMSE2_MAX: f32 = 100_f32;
    
    /// Construct new BREMSE_2 from values
    pub fn new(whlspeed_fl_bremse2: f32, whlspeed_fr_bremse2: f32, whlspeed_rl_bremse2: f32, whlspeed_rr_bremse2: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_whlspeed_fl_bremse2(whlspeed_fl_bremse2)?;
        res.set_whlspeed_fr_bremse2(whlspeed_fr_bremse2)?;
        res.set_whlspeed_rl_bremse2(whlspeed_rl_bremse2)?;
        res.set_whlspeed_rr_bremse2(whlspeed_rr_bremse2)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// whlspeed_FL_Bremse2
    ///
    /// Radgeschwindigkeit / wheel speed direct FL
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "m/s"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn whlspeed_fl_bremse2(&self) -> f32 {
        self.whlspeed_fl_bremse2_raw()
    }
    
    /// Get raw value of whlspeed_FL_Bremse2
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.015625
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn whlspeed_fl_bremse2_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 0.015625_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of whlspeed_FL_Bremse2
    #[inline(always)]
    pub fn set_whlspeed_fl_bremse2(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 100_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse2::MESSAGE_ID });
        }
        let factor = 0.015625_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// whlspeed_FR_Bremse2
    ///
    /// Radgeschwindigkeit / wheel speed direct FR
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "m/s"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn whlspeed_fr_bremse2(&self) -> f32 {
        self.whlspeed_fr_bremse2_raw()
    }
    
    /// Get raw value of whlspeed_FR_Bremse2
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.015625
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn whlspeed_fr_bremse2_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let factor = 0.015625_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of whlspeed_FR_Bremse2
    #[inline(always)]
    pub fn set_whlspeed_fr_bremse2(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 100_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse2::MESSAGE_ID });
        }
        let factor = 0.015625_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// whlspeed_RL_Bremse2
    ///
    /// Radgeschwindigkeit / wheel speed direct RL
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "m/s"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn whlspeed_rl_bremse2(&self) -> f32 {
        self.whlspeed_rl_bremse2_raw()
    }
    
    /// Get raw value of whlspeed_RL_Bremse2
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 0.015625
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn whlspeed_rl_bremse2_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let factor = 0.015625_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of whlspeed_RL_Bremse2
    #[inline(always)]
    pub fn set_whlspeed_rl_bremse2(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 100_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse2::MESSAGE_ID });
        }
        let factor = 0.015625_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// whlspeed_RR_Bremse2
    ///
    /// Radgeschwindigkeit / wheel speed direct RR
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "m/s"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn whlspeed_rr_bremse2(&self) -> f32 {
        self.whlspeed_rr_bremse2_raw()
    }
    
    /// Get raw value of whlspeed_RR_Bremse2
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 0.015625
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn whlspeed_rr_bremse2_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        let factor = 0.015625_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of whlspeed_RR_Bremse2
    #[inline(always)]
    pub fn set_whlspeed_rr_bremse2(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 100_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse2::MESSAGE_ID });
        }
        let factor = 0.015625_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Bremse2 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Bremse2 {
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

/// ABS_Switch
///
/// - Standard ID: 588 (0x24c)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct AbsSwitch {
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
impl AbsSwitch {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x24c)});
    
    pub const ABS_SWITCHPOSITION_MIN: u8 = 0_u8;
    pub const ABS_SWITCHPOSITION_MAX: u8 = 11_u8;
    
    /// Construct new ABS_Switch from values
    pub fn new(abs_switchposition: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_abs_switchposition(abs_switchposition)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// ABS_Switchposition
    ///
    /// Channel to send the swich position via CAN to the ABS.
    ///
    /// - Min: 0
    /// - Max: 11
    /// - Unit: ""
    /// - Receivers: ABS
    #[inline(always)]
    pub fn abs_switchposition(&self) -> u8 {
        self.abs_switchposition_raw()
    }
    
    /// Get raw value of ABS_Switchposition
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn abs_switchposition_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of ABS_Switchposition
    #[inline(always)]
    pub fn set_abs_switchposition(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 11_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: AbsSwitch::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: AbsSwitch::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for AbsSwitch {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for AbsSwitch {
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

/// BREMSE_30
///
/// - Standard ID: 832 (0x340)
/// - Size: 8 bytes
/// - Transmitter: ABS
#[derive(Clone, Copy)]
pub struct Bremse30 {
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
impl Bremse30 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x340)});
    
    
    /// Construct new BREMSE_30 from values
    pub fn new() -> Result<Self, CanError> {
        let res = Self { raw: [0u8; 8] };
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
}

impl core::convert::TryFrom<&[u8]> for Bremse30 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Bremse30 {
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

/// BREMSE_31
///
/// - Standard ID: 833 (0x341)
/// - Size: 8 bytes
/// - Transmitter: ABS
#[derive(Clone, Copy)]
pub struct Bremse31 {
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
impl Bremse31 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x341)});
    
    pub const IDLE_TIME_MIN: u16 = 0_u16;
    pub const IDLE_TIME_MAX: u16 = 0_u16;
    
    /// Construct new BREMSE_31 from values
    pub fn new(idle_time: u16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_idle_time(idle_time)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// Idle_Time
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "-"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn idle_time(&self) -> u16 {
        self.idle_time_raw()
    }
    
    /// Get raw value of Idle_Time
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn idle_time_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Idle_Time
    #[inline(always)]
    pub fn set_idle_time(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse31::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Bremse31::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Bremse31 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Bremse31 {
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

/// BREMSE_32
///
/// - Standard ID: 834 (0x342)
/// - Size: 8 bytes
/// - Transmitter: ABS
#[derive(Clone, Copy)]
pub struct Bremse32 {
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
impl Bremse32 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x342)});
    
    pub const ACC_FA_MIN: f32 = 0_f32;
    pub const ACC_FA_MAX: f32 = 10_f32;
    pub const ACC_RA_MIN: f32 = 0_f32;
    pub const ACC_RA_MAX: f32 = 10_f32;
    pub const WHEEL_QUALITY_FL_MIN: u8 = 0_u8;
    pub const WHEEL_QUALITY_FL_MAX: u8 = 32_u8;
    pub const WHEEL_QUALITY_FR_MIN: u8 = 0_u8;
    pub const WHEEL_QUALITY_FR_MAX: u8 = 32_u8;
    pub const WHEEL_QUALITY_RL_MIN: u8 = 0_u8;
    pub const WHEEL_QUALITY_RL_MAX: u8 = 32_u8;
    pub const WHEEL_QUALITY_RR_MIN: u8 = 0_u8;
    pub const WHEEL_QUALITY_RR_MAX: u8 = 32_u8;
    
    /// Construct new BREMSE_32 from values
    pub fn new(acc_fa: f32, acc_ra: f32, wheel_quality_fl: u8, wheel_quality_fr: u8, wheel_quality_rl: u8, wheel_quality_rr: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_acc_fa(acc_fa)?;
        res.set_acc_ra(acc_ra)?;
        res.set_wheel_quality_fl(wheel_quality_fl)?;
        res.set_wheel_quality_fr(wheel_quality_fr)?;
        res.set_wheel_quality_rl(wheel_quality_rl)?;
        res.set_wheel_quality_rr(wheel_quality_rr)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// acc_FA
    ///
    /// Fill level of the fluid reservoir of the front axle.
    ///
    /// - Min: 0
    /// - Max: 10
    /// - Unit: "cm3"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn acc_fa(&self) -> f32 {
        self.acc_fa_raw()
    }
    
    /// Get raw value of acc_FA
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 0.05
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn acc_fa_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        let factor = 0.05_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of acc_FA
    #[inline(always)]
    pub fn set_acc_fa(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 10_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse32::MESSAGE_ID });
        }
        let factor = 0.05_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
    /// acc_RA
    ///
    /// Fill level of the fluid reservoir of the rear axle.
    ///
    /// - Min: 0
    /// - Max: 10
    /// - Unit: "cm3"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn acc_ra(&self) -> f32 {
        self.acc_ra_raw()
    }
    
    /// Get raw value of acc_RA
    ///
    /// - Start bit: 8
    /// - Signal size: 8 bits
    /// - Factor: 0.05
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn acc_ra_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
        
        let factor = 0.05_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of acc_RA
    #[inline(always)]
    pub fn set_acc_ra(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 10_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse32::MESSAGE_ID });
        }
        let factor = 0.05_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
        Ok(())
    }
    
    /// WheelQuality_FL
    ///
    /// Bit matrix
    /// Bit0 ( 1) Signal Reduced Monitored
    /// Bit1 ( 2) Reduced Accuracy
    /// Bit2 ( 4) Interfered
    /// Bit3 ( 8) Suspicious Plausibility
    /// Bit4 (16) Suspicious Lost
    /// Bit5 (32) Not Initialized
    /// Bit6 (64) Invalid Generic
    /// Bit7 (128) Invalid Individual
    ///
    /// - Min: 0
    /// - Max: 32
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn wheel_quality_fl(&self) -> u8 {
        self.wheel_quality_fl_raw()
    }
    
    /// Get raw value of WheelQuality_FL
    ///
    /// - Start bit: 32
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn wheel_quality_fl_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[32..40].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of WheelQuality_FL
    #[inline(always)]
    pub fn set_wheel_quality_fl(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 32_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse32::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Bremse32::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[32..40].store_le(value);
        Ok(())
    }
    
    /// WheelQuality_FR
    ///
    /// Bit matrix
    /// Bit0 ( 1) Signal Reduced Monitored
    /// Bit1 ( 2) Reduced Accuracy
    /// Bit2 ( 4) Interfered
    /// Bit3 ( 8) Suspicious Plausibility
    /// Bit4 (16) Suspicious Lost
    /// Bit5 (32) Not Initialized
    /// Bit6 (64) Invalid Generic
    /// Bit7 (128) Invalid Individual
    ///
    /// - Min: 0
    /// - Max: 32
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn wheel_quality_fr(&self) -> u8 {
        self.wheel_quality_fr_raw()
    }
    
    /// Get raw value of WheelQuality_FR
    ///
    /// - Start bit: 40
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn wheel_quality_fr_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[40..48].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of WheelQuality_FR
    #[inline(always)]
    pub fn set_wheel_quality_fr(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 32_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse32::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Bremse32::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[40..48].store_le(value);
        Ok(())
    }
    
    /// WheelQuality_RL
    ///
    /// Bit matrix
    /// Bit0 ( 1) Signal Reduced Monitored
    /// Bit1 ( 2) Reduced Accuracy
    /// Bit2 ( 4) Interfered
    /// Bit3 ( 8) Suspicious Plausibility
    /// Bit4 (16) Suspicious Lost
    /// Bit5 (32) Not Initialized
    /// Bit6 (64) Invalid Generic
    /// Bit7 (128) Invalid Individual
    ///
    /// - Min: 0
    /// - Max: 32
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn wheel_quality_rl(&self) -> u8 {
        self.wheel_quality_rl_raw()
    }
    
    /// Get raw value of WheelQuality_RL
    ///
    /// - Start bit: 48
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn wheel_quality_rl_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[48..56].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of WheelQuality_RL
    #[inline(always)]
    pub fn set_wheel_quality_rl(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 32_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse32::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Bremse32::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[48..56].store_le(value);
        Ok(())
    }
    
    /// WheelQuality_RR
    ///
    /// Bit matrix
    /// Bit0 ( 1) Signal Reduced Monitored
    /// Bit1 ( 2) Reduced Accuracy
    /// Bit2 ( 4) Interfered
    /// Bit3 ( 8) Suspicious Plausibility
    /// Bit4 (16) Suspicious Lost
    /// Bit5 (32) Not Initialized
    /// Bit6 (64) Invalid Generic
    /// Bit7 (128) Invalid Individual
    ///
    /// - Min: 0
    /// - Max: 32
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn wheel_quality_rr(&self) -> u8 {
        self.wheel_quality_rr_raw()
    }
    
    /// Get raw value of WheelQuality_RR
    ///
    /// - Start bit: 56
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn wheel_quality_rr_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[56..64].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of WheelQuality_RR
    #[inline(always)]
    pub fn set_wheel_quality_rr(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 32_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse32::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Bremse32::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[56..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Bremse32 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Bremse32 {
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

/// BREMSE_51
///
/// - Standard ID: 1345 (0x541)
/// - Size: 8 bytes
/// - Transmitter: ABS
#[derive(Clone, Copy)]
pub struct Bremse51 {
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
impl Bremse51 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x541)});
    
    pub const AX1_ABS_INT_MIN: f32 = -4.1768_f32;
    pub const AX1_ABS_INT_MAX: f32 = 4.1736697_f32;
    pub const AY1_ABS_INT_MIN: f32 = -4.1768_f32;
    pub const AY1_ABS_INT_MAX: f32 = 4.1765_f32;
    pub const IF_VARIANT_MIN: u8 = 0_u8;
    pub const IF_VARIANT_MAX: u8 = 63_u8;
    pub const IF_REVISION_MIN: u8 = 0_u8;
    pub const IF_REVISION_MAX: u8 = 63_u8;
    pub const IF_CHKSUM_MIN: u8 = 0_u8;
    pub const IF_CHKSUM_MAX: u8 = 15_u8;
    
    /// Construct new BREMSE_51 from values
    pub fn new(ax1_abs_int: f32, ay1_abs_int: f32, if_variant: u8, if_revision: u8, if_chksum: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_ax1_abs_int(ax1_abs_int)?;
        res.set_ay1_abs_int(ay1_abs_int)?;
        res.set_if_variant(if_variant)?;
        res.set_if_revision(if_revision)?;
        res.set_if_chksum(if_chksum)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// AX1_ABS_int
    ///
    /// Used longitudional acceleration value in the ABS.
    ///
    /// - Min: -4.1768
    /// - Max: 4.1736697
    /// - Unit: "g"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn ax1_abs_int(&self) -> f32 {
        self.ax1_abs_int_raw()
    }
    
    /// Get raw value of AX1_ABS_int
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.00012742
    /// - Offset: -4.1768
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn ax1_abs_int_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 0.00012742_f32;
        let offset = -4.1768_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of AX1_ABS_int
    #[inline(always)]
    pub fn set_ax1_abs_int(&mut self, value: f32) -> Result<(), CanError> {
        if value < -4.1768_f32 || 4.1736697_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse51::MESSAGE_ID });
        }
        let factor = 0.00012742_f32;
        let offset = -4.1768_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// AY1_ABS_int
    ///
    /// Used lateral acceleration value in the ABS.
    ///
    /// - Min: -4.1768
    /// - Max: 4.1765
    /// - Unit: "g"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn ay1_abs_int(&self) -> f32 {
        self.ay1_abs_int_raw()
    }
    
    /// Get raw value of AY1_ABS_int
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.00012742
    /// - Offset: -4.1768
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn ay1_abs_int_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let factor = 0.00012742_f32;
        let offset = -4.1768_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of AY1_ABS_int
    #[inline(always)]
    pub fn set_ay1_abs_int(&mut self, value: f32) -> Result<(), CanError> {
        if value < -4.1768_f32 || 4.1765_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse51::MESSAGE_ID });
        }
        let factor = 0.00012742_f32;
        let offset = -4.1768_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// IF_variant
    ///
    /// external info to e.g. MS6 which dbc has to be used. This index increments on changes that make the MS6 interface incompatible to the predecessor CAN interface implementation
    ///
    /// - Min: 0
    /// - Max: 63
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn if_variant(&self) -> u8 {
        self.if_variant_raw()
    }
    
    /// Get raw value of IF_variant
    ///
    /// - Start bit: 48
    /// - Signal size: 6 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn if_variant_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[48..54].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of IF_variant
    #[inline(always)]
    pub fn set_if_variant(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 63_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse51::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Bremse51::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[48..54].store_le(value);
        Ok(())
    }
    
    /// IF_revision
    ///
    /// external info to e.g. MS6 which dbc has to be used. This index increments with added features (rest of MS6 interface stays intact.)
    ///
    /// - Min: 0
    /// - Max: 63
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn if_revision(&self) -> u8 {
        self.if_revision_raw()
    }
    
    /// Get raw value of IF_revision
    ///
    /// - Start bit: 54
    /// - Signal size: 6 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn if_revision_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[54..60].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of IF_revision
    #[inline(always)]
    pub fn set_if_revision(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 63_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse51::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Bremse51::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[54..60].store_le(value);
        Ok(())
    }
    
    /// IF_chksum
    ///
    /// external info to e.g. MS6 which dbc has to be used. Checksum
    ///
    /// - Min: 0
    /// - Max: 15
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn if_chksum(&self) -> u8 {
        self.if_chksum_raw()
    }
    
    /// Get raw value of IF_chksum
    ///
    /// - Start bit: 60
    /// - Signal size: 4 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn if_chksum_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[60..64].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of IF_chksum
    #[inline(always)]
    pub fn set_if_chksum(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 15_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse51::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Bremse51::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[60..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Bremse51 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Bremse51 {
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

/// BREMSE_52
///
/// - Standard ID: 1346 (0x542)
/// - Size: 8 bytes
/// - Transmitter: ABS
#[derive(Clone, Copy)]
pub struct Bremse52 {
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
impl Bremse52 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x542)});
    
    pub const MPLX_SW_INFO_MIN: u8 = 0_u8;
    pub const MPLX_SW_INFO_MAX: u8 = 255_u8;
    pub const SW_VERSION_HIGH_UPPER_MIN: u8 = 0_u8;
    pub const SW_VERSION_HIGH_UPPER_MAX: u8 = 255_u8;
    pub const SW_VERSION_HIGH_LOWER_MIN: u8 = 0_u8;
    pub const SW_VERSION_HIGH_LOWER_MAX: u8 = 255_u8;
    pub const SW_VERSION_MID_UPPER_MIN: u8 = 0_u8;
    pub const SW_VERSION_MID_UPPER_MAX: u8 = 255_u8;
    pub const SW_VERSION_MID_LOWER_MIN: u8 = 0_u8;
    pub const SW_VERSION_MID_LOWER_MAX: u8 = 255_u8;
    pub const SW_VERSION_LOW_UPPER_MIN: u8 = 0_u8;
    pub const SW_VERSION_LOW_UPPER_MAX: u8 = 255_u8;
    pub const SW_VERSION_LOW_LOWER_MIN: u8 = 0_u8;
    pub const SW_VERSION_LOW_LOWER_MAX: u8 = 255_u8;
    pub const BB_DIG1_MIN: u8 = 0_u8;
    pub const BB_DIG1_MAX: u8 = 255_u8;
    pub const BB_DIG2_MIN: u8 = 0_u8;
    pub const BB_DIG2_MAX: u8 = 255_u8;
    pub const BB_DIG3_MIN: u8 = 0_u8;
    pub const BB_DIG3_MAX: u8 = 255_u8;
    pub const BB_DIG4_MIN: u8 = 0_u8;
    pub const BB_DIG4_MAX: u8 = 255_u8;
    pub const BB_DIG5_MIN: u8 = 0_u8;
    pub const BB_DIG5_MAX: u8 = 255_u8;
    pub const BB_DIG6_MIN: u8 = 0_u8;
    pub const BB_DIG6_MAX: u8 = 255_u8;
    pub const BB_DIG7_MIN: u8 = 0_u8;
    pub const BB_DIG7_MAX: u8 = 255_u8;
    pub const APPL_ID_01_MIN: u8 = 0_u8;
    pub const APPL_ID_01_MAX: u8 = 255_u8;
    pub const APPL_ID_02_MIN: u8 = 0_u8;
    pub const APPL_ID_02_MAX: u8 = 255_u8;
    pub const APPL_ID_03_MIN: u8 = 0_u8;
    pub const APPL_ID_03_MAX: u8 = 255_u8;
    pub const APPL_ID_04_MIN: u8 = 0_u8;
    pub const APPL_ID_04_MAX: u8 = 255_u8;
    pub const APPL_ID_05_MIN: u8 = 0_u8;
    pub const APPL_ID_05_MAX: u8 = 255_u8;
    pub const APPL_ID_06_MIN: u8 = 0_u8;
    pub const APPL_ID_06_MAX: u8 = 255_u8;
    pub const APPL_ID_07_MIN: u8 = 0_u8;
    pub const APPL_ID_07_MAX: u8 = 255_u8;
    pub const APPL_ID_08_MIN: u8 = 0_u8;
    pub const APPL_ID_08_MAX: u8 = 255_u8;
    pub const APPL_ID_09_MIN: u8 = 0_u8;
    pub const APPL_ID_09_MAX: u8 = 255_u8;
    pub const APPL_ID_10_MIN: u8 = 0_u8;
    pub const APPL_ID_10_MAX: u8 = 255_u8;
    pub const APPL_ID_11_MIN: u8 = 0_u8;
    pub const APPL_ID_11_MAX: u8 = 255_u8;
    pub const APPL_ID_12_MIN: u8 = 0_u8;
    pub const APPL_ID_12_MAX: u8 = 255_u8;
    pub const APPL_ID_13_MIN: u8 = 0_u8;
    pub const APPL_ID_13_MAX: u8 = 255_u8;
    pub const APPL_ID_14_MIN: u8 = 0_u8;
    pub const APPL_ID_14_MAX: u8 = 255_u8;
    pub const APPL_DATE_01_MIN: u8 = 0_u8;
    pub const APPL_DATE_01_MAX: u8 = 99_u8;
    pub const APPL_DATE_02_MIN: u8 = 1_u8;
    pub const APPL_DATE_02_MAX: u8 = 12_u8;
    pub const APPL_DATE_03_MIN: u8 = 1_u8;
    pub const APPL_DATE_03_MAX: u8 = 31_u8;
    pub const APPL_DATE_04_MIN: u8 = 0_u8;
    pub const APPL_DATE_04_MAX: u8 = 24_u8;
    pub const APPL_DATE_05_MIN: u8 = 0_u8;
    pub const APPL_DATE_05_MAX: u8 = 59_u8;
    pub const APPL_DATE_06_MIN: u8 = 0_u8;
    pub const APPL_DATE_06_MAX: u8 = 59_u8;
    pub const SW_CAN_IDENT_MIN: u8 = 0_u8;
    pub const SW_CAN_IDENT_MAX: u8 = 255_u8;
    pub const HU_DATE_YEAR_MIN: u8 = 0_u8;
    pub const HU_DATE_YEAR_MAX: u8 = 99_u8;
    pub const HU_DATE_MONTH_MIN: u8 = 1_u8;
    pub const HU_DATE_MONTH_MAX: u8 = 12_u8;
    pub const HU_DATE_DAY_MIN: u8 = 1_u8;
    pub const HU_DATE_DAY_MAX: u8 = 31_u8;
    pub const ECU_SERIAL_MIN: u32 = 0_u32;
    pub const ECU_SERIAL_MAX: u32 = 99999_u32;
    
    /// Construct new BREMSE_52 from values
    pub fn new(mplx_sw_info: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_mplx_sw_info(mplx_sw_info)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// Get raw value of Mplx_SW_Info
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn mplx_sw_info_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    pub fn mplx_sw_info(&mut self) -> Result<Bremse52MplxSwInfoIndex, CanError> {
        match self.mplx_sw_info_raw() {
            1 => Ok(Bremse52MplxSwInfoIndex::M1(Bremse52MplxSwInfoM1{ raw: self.raw })),
            2 => Ok(Bremse52MplxSwInfoIndex::M2(Bremse52MplxSwInfoM2{ raw: self.raw })),
            3 => Ok(Bremse52MplxSwInfoIndex::M3(Bremse52MplxSwInfoM3{ raw: self.raw })),
            4 => Ok(Bremse52MplxSwInfoIndex::M4(Bremse52MplxSwInfoM4{ raw: self.raw })),
            5 => Ok(Bremse52MplxSwInfoIndex::M5(Bremse52MplxSwInfoM5{ raw: self.raw })),
            6 => Ok(Bremse52MplxSwInfoIndex::M6(Bremse52MplxSwInfoM6{ raw: self.raw })),
            7 => Ok(Bremse52MplxSwInfoIndex::M7(Bremse52MplxSwInfoM7{ raw: self.raw })),
            multiplexor => Err(CanError::InvalidMultiplexor { message_id: Bremse52::MESSAGE_ID, multiplexor: multiplexor.into() }),
        }
    }
    /// Set value of Mplx_SW_Info
    #[inline(always)]
    fn set_mplx_sw_info(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 255_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
    /// Set value of Mplx_SW_Info
    #[inline(always)]
    pub fn set_m1(&mut self, value: Bremse52MplxSwInfoM1) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mplx_sw_info(1)?;
        Ok(())
    }
    
    /// Set value of Mplx_SW_Info
    #[inline(always)]
    pub fn set_m2(&mut self, value: Bremse52MplxSwInfoM2) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mplx_sw_info(2)?;
        Ok(())
    }
    
    /// Set value of Mplx_SW_Info
    #[inline(always)]
    pub fn set_m3(&mut self, value: Bremse52MplxSwInfoM3) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mplx_sw_info(3)?;
        Ok(())
    }
    
    /// Set value of Mplx_SW_Info
    #[inline(always)]
    pub fn set_m4(&mut self, value: Bremse52MplxSwInfoM4) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mplx_sw_info(4)?;
        Ok(())
    }
    
    /// Set value of Mplx_SW_Info
    #[inline(always)]
    pub fn set_m5(&mut self, value: Bremse52MplxSwInfoM5) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mplx_sw_info(5)?;
        Ok(())
    }
    
    /// Set value of Mplx_SW_Info
    #[inline(always)]
    pub fn set_m6(&mut self, value: Bremse52MplxSwInfoM6) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mplx_sw_info(6)?;
        Ok(())
    }
    
    /// Set value of Mplx_SW_Info
    #[inline(always)]
    pub fn set_m7(&mut self, value: Bremse52MplxSwInfoM7) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mplx_sw_info(7)?;
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Bremse52 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Bremse52 {
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
/// Defined values for multiplexed signal BREMSE_52
#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
pub enum Bremse52MplxSwInfoIndex {
    M1(Bremse52MplxSwInfoM1),
    M2(Bremse52MplxSwInfoM2),
    M3(Bremse52MplxSwInfoM3),
    M4(Bremse52MplxSwInfoM4),
    M5(Bremse52MplxSwInfoM5),
    M6(Bremse52MplxSwInfoM6),
    M7(Bremse52MplxSwInfoM7),
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
pub struct Bremse52MplxSwInfoM1 { raw: [u8; 8] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl Bremse52MplxSwInfoM1 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// SW_version_High_upper
///
/// version 1.0 as 0x01(upper), version 100.20 as 0x64(upper)
///
/// - Min: 0
/// - Max: 255
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn sw_version_high_upper(&self) -> u8 {
    self.sw_version_high_upper_raw()
}

/// Get raw value of SW_version_High_upper
///
/// - Start bit: 8
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn sw_version_high_upper_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of SW_version_High_upper
#[inline(always)]
pub fn set_sw_version_high_upper(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 255_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
    Ok(())
}

/// SW_version_High_lower
///
/// version 1.0 as 0x00(lower), version 100.20 as 0x14(lower)
///
/// - Min: 0
/// - Max: 255
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn sw_version_high_lower(&self) -> u8 {
    self.sw_version_high_lower_raw()
}

/// Get raw value of SW_version_High_lower
///
/// - Start bit: 16
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn sw_version_high_lower_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[16..24].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of SW_version_High_lower
#[inline(always)]
pub fn set_sw_version_high_lower(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 255_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[16..24].store_le(value);
    Ok(())
}

/// SW_version_Mid_upper
///
/// version 1.0 as 0x01(upper), version 100.20 as 0x64(upper)
///
/// - Min: 0
/// - Max: 255
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn sw_version_mid_upper(&self) -> u8 {
    self.sw_version_mid_upper_raw()
}

/// Get raw value of SW_version_Mid_upper
///
/// - Start bit: 24
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn sw_version_mid_upper_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[24..32].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of SW_version_Mid_upper
#[inline(always)]
pub fn set_sw_version_mid_upper(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 255_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[24..32].store_le(value);
    Ok(())
}

/// SW_version_Mid_lower
///
/// version 1.0 as 0x00(lower), version 100.20 as 0x14(lower)
///
/// - Min: 0
/// - Max: 255
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn sw_version_mid_lower(&self) -> u8 {
    self.sw_version_mid_lower_raw()
}

/// Get raw value of SW_version_Mid_lower
///
/// - Start bit: 32
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn sw_version_mid_lower_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[32..40].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of SW_version_Mid_lower
#[inline(always)]
pub fn set_sw_version_mid_lower(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 255_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[32..40].store_le(value);
    Ok(())
}

/// SW_version_Low_upper
///
/// version 1.0 as 0x01(upper), version 100.20 as 0x64(upper)
///
/// - Min: 0
/// - Max: 255
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn sw_version_low_upper(&self) -> u8 {
    self.sw_version_low_upper_raw()
}

/// Get raw value of SW_version_Low_upper
///
/// - Start bit: 40
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn sw_version_low_upper_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[40..48].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of SW_version_Low_upper
#[inline(always)]
pub fn set_sw_version_low_upper(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 255_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[40..48].store_le(value);
    Ok(())
}

/// SW_version_Low_lower
///
/// version 1.0 as 0x00(lower), version 100.20 as 0x14(lower)
///
/// - Min: 0
/// - Max: 255
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn sw_version_low_lower(&self) -> u8 {
    self.sw_version_low_lower_raw()
}

/// Get raw value of SW_version_Low_lower
///
/// - Start bit: 48
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn sw_version_low_lower_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[48..56].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of SW_version_Low_lower
#[inline(always)]
pub fn set_sw_version_low_lower(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 255_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[48..56].store_le(value);
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
pub struct Bremse52MplxSwInfoM2 { raw: [u8; 8] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl Bremse52MplxSwInfoM2 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// BB_dig1
///
/// - Min: 0
/// - Max: 255
/// - Unit: "ASCII"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bb_dig1(&self) -> u8 {
    self.bb_dig1_raw()
}

/// Get raw value of BB_dig1
///
/// - Start bit: 8
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bb_dig1_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of BB_dig1
#[inline(always)]
pub fn set_bb_dig1(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 255_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
    Ok(())
}

/// BB_dig2
///
/// - Min: 0
/// - Max: 255
/// - Unit: "ASCII"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bb_dig2(&self) -> u8 {
    self.bb_dig2_raw()
}

/// Get raw value of BB_dig2
///
/// - Start bit: 16
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bb_dig2_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[16..24].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of BB_dig2
#[inline(always)]
pub fn set_bb_dig2(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 255_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[16..24].store_le(value);
    Ok(())
}

/// BB_dig3
///
/// - Min: 0
/// - Max: 255
/// - Unit: "ASCII"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bb_dig3(&self) -> u8 {
    self.bb_dig3_raw()
}

/// Get raw value of BB_dig3
///
/// - Start bit: 24
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bb_dig3_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[24..32].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of BB_dig3
#[inline(always)]
pub fn set_bb_dig3(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 255_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[24..32].store_le(value);
    Ok(())
}

/// BB_dig4
///
/// - Min: 0
/// - Max: 255
/// - Unit: "ASCII"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bb_dig4(&self) -> u8 {
    self.bb_dig4_raw()
}

/// Get raw value of BB_dig4
///
/// - Start bit: 32
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bb_dig4_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[32..40].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of BB_dig4
#[inline(always)]
pub fn set_bb_dig4(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 255_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[32..40].store_le(value);
    Ok(())
}

/// BB_dig5
///
/// - Min: 0
/// - Max: 255
/// - Unit: "ASCII"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bb_dig5(&self) -> u8 {
    self.bb_dig5_raw()
}

/// Get raw value of BB_dig5
///
/// - Start bit: 40
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bb_dig5_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[40..48].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of BB_dig5
#[inline(always)]
pub fn set_bb_dig5(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 255_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[40..48].store_le(value);
    Ok(())
}

/// BB_dig6
///
/// - Min: 0
/// - Max: 255
/// - Unit: "ASCII"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bb_dig6(&self) -> u8 {
    self.bb_dig6_raw()
}

/// Get raw value of BB_dig6
///
/// - Start bit: 48
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bb_dig6_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[48..56].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of BB_dig6
#[inline(always)]
pub fn set_bb_dig6(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 255_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[48..56].store_le(value);
    Ok(())
}

/// BB_dig7
///
/// - Min: 0
/// - Max: 255
/// - Unit: "ASCII"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn bb_dig7(&self) -> u8 {
    self.bb_dig7_raw()
}

/// Get raw value of BB_dig7
///
/// - Start bit: 56
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn bb_dig7_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[56..64].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of BB_dig7
#[inline(always)]
pub fn set_bb_dig7(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 255_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[56..64].store_le(value);
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
pub struct Bremse52MplxSwInfoM3 { raw: [u8; 8] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl Bremse52MplxSwInfoM3 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// Appl_Id_01
///
/// - Min: 0
/// - Max: 255
/// - Unit: "ASCII"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn appl_id_01(&self) -> u8 {
    self.appl_id_01_raw()
}

/// Get raw value of Appl_Id_01
///
/// - Start bit: 8
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn appl_id_01_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Appl_Id_01
#[inline(always)]
pub fn set_appl_id_01(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 255_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
    Ok(())
}

/// Appl_Id_02
///
/// - Min: 0
/// - Max: 255
/// - Unit: "ASCII"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn appl_id_02(&self) -> u8 {
    self.appl_id_02_raw()
}

/// Get raw value of Appl_Id_02
///
/// - Start bit: 16
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn appl_id_02_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[16..24].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Appl_Id_02
#[inline(always)]
pub fn set_appl_id_02(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 255_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[16..24].store_le(value);
    Ok(())
}

/// Appl_Id_03
///
/// - Min: 0
/// - Max: 255
/// - Unit: "ASCII"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn appl_id_03(&self) -> u8 {
    self.appl_id_03_raw()
}

/// Get raw value of Appl_Id_03
///
/// - Start bit: 24
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn appl_id_03_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[24..32].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Appl_Id_03
#[inline(always)]
pub fn set_appl_id_03(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 255_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[24..32].store_le(value);
    Ok(())
}

/// Appl_Id_04
///
/// - Min: 0
/// - Max: 255
/// - Unit: "ASCII"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn appl_id_04(&self) -> u8 {
    self.appl_id_04_raw()
}

/// Get raw value of Appl_Id_04
///
/// - Start bit: 32
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn appl_id_04_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[32..40].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Appl_Id_04
#[inline(always)]
pub fn set_appl_id_04(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 255_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[32..40].store_le(value);
    Ok(())
}

/// Appl_Id_05
///
/// - Min: 0
/// - Max: 255
/// - Unit: "ASCII"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn appl_id_05(&self) -> u8 {
    self.appl_id_05_raw()
}

/// Get raw value of Appl_Id_05
///
/// - Start bit: 40
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn appl_id_05_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[40..48].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Appl_Id_05
#[inline(always)]
pub fn set_appl_id_05(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 255_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[40..48].store_le(value);
    Ok(())
}

/// Appl_Id_06
///
/// - Min: 0
/// - Max: 255
/// - Unit: "ASCII"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn appl_id_06(&self) -> u8 {
    self.appl_id_06_raw()
}

/// Get raw value of Appl_Id_06
///
/// - Start bit: 48
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn appl_id_06_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[48..56].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Appl_Id_06
#[inline(always)]
pub fn set_appl_id_06(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 255_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[48..56].store_le(value);
    Ok(())
}

/// Appl_Id_07
///
/// - Min: 0
/// - Max: 255
/// - Unit: "ASCII"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn appl_id_07(&self) -> u8 {
    self.appl_id_07_raw()
}

/// Get raw value of Appl_Id_07
///
/// - Start bit: 56
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn appl_id_07_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[56..64].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Appl_Id_07
#[inline(always)]
pub fn set_appl_id_07(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 255_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[56..64].store_le(value);
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
pub struct Bremse52MplxSwInfoM4 { raw: [u8; 8] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl Bremse52MplxSwInfoM4 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// Appl_Id_08
///
/// - Min: 0
/// - Max: 255
/// - Unit: "ASCII"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn appl_id_08(&self) -> u8 {
    self.appl_id_08_raw()
}

/// Get raw value of Appl_Id_08
///
/// - Start bit: 8
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn appl_id_08_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Appl_Id_08
#[inline(always)]
pub fn set_appl_id_08(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 255_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
    Ok(())
}

/// Appl_Id_09
///
/// - Min: 0
/// - Max: 255
/// - Unit: "ASCII"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn appl_id_09(&self) -> u8 {
    self.appl_id_09_raw()
}

/// Get raw value of Appl_Id_09
///
/// - Start bit: 16
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn appl_id_09_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[16..24].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Appl_Id_09
#[inline(always)]
pub fn set_appl_id_09(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 255_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[16..24].store_le(value);
    Ok(())
}

/// Appl_Id_10
///
/// - Min: 0
/// - Max: 255
/// - Unit: "ASCII"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn appl_id_10(&self) -> u8 {
    self.appl_id_10_raw()
}

/// Get raw value of Appl_Id_10
///
/// - Start bit: 24
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn appl_id_10_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[24..32].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Appl_Id_10
#[inline(always)]
pub fn set_appl_id_10(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 255_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[24..32].store_le(value);
    Ok(())
}

/// Appl_Id_11
///
/// - Min: 0
/// - Max: 255
/// - Unit: "ASCII"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn appl_id_11(&self) -> u8 {
    self.appl_id_11_raw()
}

/// Get raw value of Appl_Id_11
///
/// - Start bit: 32
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn appl_id_11_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[32..40].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Appl_Id_11
#[inline(always)]
pub fn set_appl_id_11(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 255_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[32..40].store_le(value);
    Ok(())
}

/// Appl_Id_12
///
/// - Min: 0
/// - Max: 255
/// - Unit: "ASCII"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn appl_id_12(&self) -> u8 {
    self.appl_id_12_raw()
}

/// Get raw value of Appl_Id_12
///
/// - Start bit: 40
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn appl_id_12_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[40..48].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Appl_Id_12
#[inline(always)]
pub fn set_appl_id_12(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 255_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[40..48].store_le(value);
    Ok(())
}

/// Appl_Id_13
///
/// - Min: 0
/// - Max: 255
/// - Unit: "ASCII"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn appl_id_13(&self) -> u8 {
    self.appl_id_13_raw()
}

/// Get raw value of Appl_Id_13
///
/// - Start bit: 48
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn appl_id_13_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[48..56].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Appl_Id_13
#[inline(always)]
pub fn set_appl_id_13(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 255_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[48..56].store_le(value);
    Ok(())
}

/// Appl_Id_14
///
/// - Min: 0
/// - Max: 255
/// - Unit: "ASCII"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn appl_id_14(&self) -> u8 {
    self.appl_id_14_raw()
}

/// Get raw value of Appl_Id_14
///
/// - Start bit: 56
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn appl_id_14_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[56..64].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Appl_Id_14
#[inline(always)]
pub fn set_appl_id_14(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 255_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[56..64].store_le(value);
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
pub struct Bremse52MplxSwInfoM5 { raw: [u8; 8] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl Bremse52MplxSwInfoM5 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// Appl_date_01
///
/// year
///
/// - Min: 0
/// - Max: 99
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn appl_date_01(&self) -> u8 {
    self.appl_date_01_raw()
}

/// Get raw value of Appl_date_01
///
/// - Start bit: 8
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn appl_date_01_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Appl_date_01
#[inline(always)]
pub fn set_appl_date_01(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 99_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
    Ok(())
}

/// Appl_date_02
///
/// month
///
/// - Min: 1
/// - Max: 12
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn appl_date_02(&self) -> u8 {
    self.appl_date_02_raw()
}

/// Get raw value of Appl_date_02
///
/// - Start bit: 16
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn appl_date_02_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[16..24].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Appl_date_02
#[inline(always)]
pub fn set_appl_date_02(&mut self, value: u8) -> Result<(), CanError> {
    if value < 1_u8 || 12_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[16..24].store_le(value);
    Ok(())
}

/// Appl_date_03
///
/// day
///
/// - Min: 1
/// - Max: 31
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn appl_date_03(&self) -> u8 {
    self.appl_date_03_raw()
}

/// Get raw value of Appl_date_03
///
/// - Start bit: 24
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn appl_date_03_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[24..32].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Appl_date_03
#[inline(always)]
pub fn set_appl_date_03(&mut self, value: u8) -> Result<(), CanError> {
    if value < 1_u8 || 31_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[24..32].store_le(value);
    Ok(())
}

/// Appl_date_04
///
/// hour
///
/// - Min: 0
/// - Max: 24
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn appl_date_04(&self) -> u8 {
    self.appl_date_04_raw()
}

/// Get raw value of Appl_date_04
///
/// - Start bit: 32
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn appl_date_04_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[32..40].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Appl_date_04
#[inline(always)]
pub fn set_appl_date_04(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 24_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[32..40].store_le(value);
    Ok(())
}

/// Appl_date_05
///
/// minute
///
/// - Min: 0
/// - Max: 59
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn appl_date_05(&self) -> u8 {
    self.appl_date_05_raw()
}

/// Get raw value of Appl_date_05
///
/// - Start bit: 40
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn appl_date_05_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[40..48].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Appl_date_05
#[inline(always)]
pub fn set_appl_date_05(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 59_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[40..48].store_le(value);
    Ok(())
}

/// Appl_date_06
///
/// seconds
///
/// - Min: 0
/// - Max: 59
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn appl_date_06(&self) -> u8 {
    self.appl_date_06_raw()
}

/// Get raw value of Appl_date_06
///
/// - Start bit: 48
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn appl_date_06_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[48..56].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Appl_date_06
#[inline(always)]
pub fn set_appl_date_06(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 59_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[48..56].store_le(value);
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
pub struct Bremse52MplxSwInfoM6 { raw: [u8; 8] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl Bremse52MplxSwInfoM6 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// SW_CAN_ident
///
/// - Min: 0
/// - Max: 255
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn sw_can_ident(&self) -> u8 {
    self.sw_can_ident_raw()
}

/// Get raw value of SW_CAN_ident
///
/// - Start bit: 8
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn sw_can_ident_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of SW_CAN_ident
#[inline(always)]
pub fn set_sw_can_ident(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 255_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
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
pub struct Bremse52MplxSwInfoM7 { raw: [u8; 8] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl Bremse52MplxSwInfoM7 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// HU_date_year
///
/// - Min: 0
/// - Max: 99
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn hu_date_year(&self) -> u8 {
    self.hu_date_year_raw()
}

/// Get raw value of HU_date_year
///
/// - Start bit: 8
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn hu_date_year_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of HU_date_year
#[inline(always)]
pub fn set_hu_date_year(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 99_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
    Ok(())
}

/// HU_date_month
///
/// - Min: 1
/// - Max: 12
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn hu_date_month(&self) -> u8 {
    self.hu_date_month_raw()
}

/// Get raw value of HU_date_month
///
/// - Start bit: 16
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn hu_date_month_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[16..24].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of HU_date_month
#[inline(always)]
pub fn set_hu_date_month(&mut self, value: u8) -> Result<(), CanError> {
    if value < 1_u8 || 12_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[16..24].store_le(value);
    Ok(())
}

/// HU_date_day
///
/// - Min: 1
/// - Max: 31
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn hu_date_day(&self) -> u8 {
    self.hu_date_day_raw()
}

/// Get raw value of HU_date_day
///
/// - Start bit: 24
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn hu_date_day_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[24..32].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of HU_date_day
#[inline(always)]
pub fn set_hu_date_day(&mut self, value: u8) -> Result<(), CanError> {
    if value < 1_u8 || 31_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[24..32].store_le(value);
    Ok(())
}

/// Ecu_serial
///
/// - Min: 0
/// - Max: 99999
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn ecu_serial(&self) -> u32 {
    self.ecu_serial_raw()
}

/// Get raw value of Ecu_serial
///
/// - Start bit: 32
/// - Signal size: 32 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn ecu_serial_raw(&self) -> u32 {
    let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<u32>();
    
    let factor = 1;
    u32::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Ecu_serial
#[inline(always)]
pub fn set_ecu_serial(&mut self, value: u32) -> Result<(), CanError> {
    if value < 0_u32 || 99999_u32 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Bremse52::MESSAGE_ID })?;
    let value = (value / factor) as u32;
    
    self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
    Ok(())
}

}


/// BREMSE_50
///
/// - Standard ID: 1376 (0x560)
/// - Size: 8 bytes
/// - Transmitter: ABS
#[derive(Clone, Copy)]
pub struct Bremse50 {
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
impl Bremse50 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x560)});
    
    pub const BRAKE_BAL_AT50_MIN: f32 = 0_f32;
    pub const BRAKE_BAL_AT50_MAX: f32 = 100_f32;
    pub const BRAKE_BAL_AT50_ADVICE_MIN: u8 = 0_u8;
    pub const BRAKE_BAL_AT50_ADVICE_MAX: u8 = 100_u8;
    pub const BRAKE_BAL_PCT_MIN: f32 = 0_f32;
    pub const BRAKE_BAL_PCT_MAX: f32 = 100_f32;
    pub const BRAKE_BAL_PCT_ADVICE_MIN: u8 = 0_u8;
    pub const BRAKE_BAL_PCT_ADVICE_MAX: u8 = 100_u8;
    
    /// Construct new BREMSE_50 from values
    pub fn new(brake_bal_at50: f32, brake_bal_at50_advice: u8, brake_bal_pct: f32, brake_bal_pct_advice: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_brake_bal_at50(brake_bal_at50)?;
        res.set_brake_bal_at50_advice(brake_bal_at50_advice)?;
        res.set_brake_bal_pct(brake_bal_pct)?;
        res.set_brake_bal_pct_advice(brake_bal_pct_advice)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// Brake_bal_at50
    ///
    /// Calculated rear axle brake pressure if the front pressure is at 50 bar.
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "Bar"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn brake_bal_at50(&self) -> f32 {
        self.brake_bal_at50_raw()
    }
    
    /// Get raw value of Brake_bal_at50
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn brake_bal_at50_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of Brake_bal_at50
    #[inline(always)]
    pub fn set_brake_bal_at50(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 100_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse50::MESSAGE_ID });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// Brake_bal_at50_advice
    ///
    /// Recommended rear axle brake pressure if the front pressure is at 50 bar.
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "Bar"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn brake_bal_at50_advice(&self) -> u8 {
        self.brake_bal_at50_advice_raw()
    }
    
    /// Get raw value of Brake_bal_at50_advice
    ///
    /// - Start bit: 32
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn brake_bal_at50_advice_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[32..40].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Brake_bal_at50_advice
    #[inline(always)]
    pub fn set_brake_bal_at50_advice(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 100_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse50::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Bremse50::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[32..40].store_le(value);
        Ok(())
    }
    
    /// Brake_bal_pct
    ///
    /// Percental brake balance on the front axle.
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "%"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn brake_bal_pct(&self) -> f32 {
        self.brake_bal_pct_raw()
    }
    
    /// Get raw value of Brake_bal_pct
    ///
    /// - Start bit: 40
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn brake_bal_pct_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[40..56].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of Brake_bal_pct
    #[inline(always)]
    pub fn set_brake_bal_pct(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 100_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse50::MESSAGE_ID });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[40..56].store_le(value);
        Ok(())
    }
    
    /// Brake_bal_pct_advice
    ///
    /// Recommended percental brake balance on the front axle.
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "%"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn brake_bal_pct_advice(&self) -> u8 {
        self.brake_bal_pct_advice_raw()
    }
    
    /// Get raw value of Brake_bal_pct_advice
    ///
    /// - Start bit: 56
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn brake_bal_pct_advice_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[56..64].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Brake_bal_pct_advice
    #[inline(always)]
    pub fn set_brake_bal_pct_advice(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 100_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse50::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Bremse50::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[56..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Bremse50 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Bremse50 {
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

/// BREMSE_53
///
/// - Standard ID: 1472 (0x5c0)
/// - Size: 8 bytes
/// - Transmitter: ABS
#[derive(Clone, Copy)]
pub struct Bremse53 {
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
impl Bremse53 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x5c0)});
    
    pub const SWITCH_POSITION_MIN: u8 = 1_u8;
    pub const SWITCH_POSITION_MAX: u8 = 12_u8;
    pub const P_FA_MIN: f32 = -42.5_f32;
    pub const P_FA_MAX: f32 = 425_f32;
    pub const BREMSE_53_CNT_MIN: u8 = 0_u8;
    pub const BREMSE_53_CNT_MAX: u8 = 3_u8;
    pub const DIAG_FL_MIN: u8 = 0_u8;
    pub const DIAG_FL_MAX: u8 = 3_u8;
    pub const DIAG_FR_MIN: u8 = 0_u8;
    pub const DIAG_FR_MAX: u8 = 3_u8;
    pub const DIAG_RL_MIN: u8 = 0_u8;
    pub const DIAG_RL_MAX: u8 = 3_u8;
    pub const DIAG_RR_MIN: u8 = 0_u8;
    pub const DIAG_RR_MAX: u8 = 3_u8;
    pub const ABS_FAULT_INFO_MIN: u8 = 0_u8;
    pub const ABS_FAULT_INFO_MAX: u8 = 3_u8;
    pub const P_RA_MIN: f32 = -42.5_f32;
    pub const P_RA_MAX: f32 = 425_f32;
    
    /// Construct new BREMSE_53 from values
    pub fn new(switch_position: u8, p_fa: f32, bls: bool, bremse_53_cnt: u8, abs_malfunction: bool, abs_active: bool, ebd_lamp: bool, abs_lamp: bool, diag_fl: u8, diag_fr: u8, diag_rl: u8, diag_rr: u8, diag_abs_unit: bool, diag_fuse_valve: bool, diag_fuse_pump: bool, diag_p_fa: bool, diag_p_ra: bool, diag_yrs: bool, abs_fault_info: u8, p_ra: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_switch_position(switch_position)?;
        res.set_p_fa(p_fa)?;
        res.set_bls(bls)?;
        res.set_bremse_53_cnt(bremse_53_cnt)?;
        res.set_abs_malfunction(abs_malfunction)?;
        res.set_abs_active(abs_active)?;
        res.set_ebd_lamp(ebd_lamp)?;
        res.set_abs_lamp(abs_lamp)?;
        res.set_diag_fl(diag_fl)?;
        res.set_diag_fr(diag_fr)?;
        res.set_diag_rl(diag_rl)?;
        res.set_diag_rr(diag_rr)?;
        res.set_diag_abs_unit(diag_abs_unit)?;
        res.set_diag_fuse_valve(diag_fuse_valve)?;
        res.set_diag_fuse_pump(diag_fuse_pump)?;
        res.set_diag_p_fa(diag_p_fa)?;
        res.set_diag_p_ra(diag_p_ra)?;
        res.set_diag_yrs(diag_yrs)?;
        res.set_abs_fault_info(abs_fault_info)?;
        res.set_p_ra(p_ra)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// SwitchPosition
    ///
    /// Used switch position of the ABS.
    ///
    /// - Min: 1
    /// - Max: 12
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn switch_position(&self) -> u8 {
        self.switch_position_raw()
    }
    
    /// Get raw value of SwitchPosition
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn switch_position_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of SwitchPosition
    #[inline(always)]
    pub fn set_switch_position(&mut self, value: u8) -> Result<(), CanError> {
        if value < 1_u8 || 12_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse53::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Bremse53::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
    /// P_FA
    ///
    /// Brake pressure on the front axle.
    ///
    /// - Min: -42.5
    /// - Max: 425
    /// - Unit: "bar"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn p_fa(&self) -> f32 {
        self.p_fa_raw()
    }
    
    /// Get raw value of P_FA
    ///
    /// - Start bit: 8
    /// - Signal size: 16 bits
    /// - Factor: 0.01526
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn p_fa_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[8..24].load_le::<i16>();
        
        let factor = 0.01526_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of P_FA
    #[inline(always)]
    pub fn set_p_fa(&mut self, value: f32) -> Result<(), CanError> {
        if value < -42.5_f32 || 425_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse53::MESSAGE_ID });
        }
        let factor = 0.01526_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[8..24].store_le(value);
        Ok(())
    }
    
    /// BLS
    ///
    /// Bit for the brake light switch.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn bls(&self) -> bool {
        self.bls_raw()
    }
    
    /// Get raw value of BLS
    ///
    /// - Start bit: 24
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn bls_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[24..25].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of BLS
    #[inline(always)]
    pub fn set_bls(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[24..25].store_le(value);
        Ok(())
    }
    
    /// Bremse_53_cnt
    ///
    /// - Min: 0
    /// - Max: 3
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn bremse_53_cnt(&self) -> u8 {
        self.bremse_53_cnt_raw()
    }
    
    /// Get raw value of Bremse_53_cnt
    ///
    /// - Start bit: 26
    /// - Signal size: 2 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn bremse_53_cnt_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[26..28].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Bremse_53_cnt
    #[inline(always)]
    pub fn set_bremse_53_cnt(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 3_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse53::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Bremse53::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[26..28].store_le(value);
        Ok(())
    }
    
    /// ABS_Malfunction
    ///
    /// Bit will jump to 1, if the ABS control is deactivated by a fault.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn abs_malfunction(&self) -> bool {
        self.abs_malfunction_raw()
    }
    
    /// Get raw value of ABS_Malfunction
    ///
    /// - Start bit: 28
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn abs_malfunction_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[28..29].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of ABS_Malfunction
    #[inline(always)]
    pub fn set_abs_malfunction(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[28..29].store_le(value);
        Ok(())
    }
    
    /// ABS_Active
    ///
    /// Bit will jump to 1, when the ABS control is active.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn abs_active(&self) -> bool {
        self.abs_active_raw()
    }
    
    /// Get raw value of ABS_Active
    ///
    /// - Start bit: 29
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn abs_active_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[29..30].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of ABS_Active
    #[inline(always)]
    pub fn set_abs_active(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[29..30].store_le(value);
        Ok(())
    }
    
    /// EBD_Lamp
    ///
    /// Bit will jump to 1, when the EBD is deactivated due to a fault.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn ebd_lamp(&self) -> bool {
        self.ebd_lamp_raw()
    }
    
    /// Get raw value of EBD_Lamp
    ///
    /// - Start bit: 30
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn ebd_lamp_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[30..31].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of EBD_Lamp
    #[inline(always)]
    pub fn set_ebd_lamp(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[30..31].store_le(value);
        Ok(())
    }
    
    /// ABS_Lamp
    ///
    /// Bit will jump to 1, when the ABS control is deactivated due to a fault, switch to the off position or while working with RaceABS.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn abs_lamp(&self) -> bool {
        self.abs_lamp_raw()
    }
    
    /// Get raw value of ABS_Lamp
    ///
    /// - Start bit: 31
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn abs_lamp_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[31..32].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of ABS_Lamp
    #[inline(always)]
    pub fn set_abs_lamp(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[31..32].store_le(value);
        Ok(())
    }
    
    /// Diag_FL
    ///
    /// Value to show faults related to the wheel speed sensor. 
    /// 0 - Signal ok, 1 - Wiring related fault, 2 - Signal related fault
    ///
    /// - Min: 0
    /// - Max: 3
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn diag_fl(&self) -> Bremse53DiagFl {
        let signal = self.raw.view_bits::<Lsb0>()[32..34].load_le::<u8>();
        
        match signal {
            2 => Bremse53DiagFl::SignalError,
            1 => Bremse53DiagFl::LineError,
            0 => Bremse53DiagFl::SignalOk,
            _ => Bremse53DiagFl::_Other(self.diag_fl_raw()),
        }
    }
    
    /// Get raw value of Diag_FL
    ///
    /// - Start bit: 32
    /// - Signal size: 2 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn diag_fl_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[32..34].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Diag_FL
    #[inline(always)]
    pub fn set_diag_fl(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 3_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse53::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Bremse53::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[32..34].store_le(value);
        Ok(())
    }
    
    /// Diag_FR
    ///
    /// Value to show faults related to the wheel speed sensor. 
    /// 0 - Signal ok, 1 - Wiring related fault, 2 - Signal related fault
    ///
    /// - Min: 0
    /// - Max: 3
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn diag_fr(&self) -> Bremse53DiagFr {
        let signal = self.raw.view_bits::<Lsb0>()[34..36].load_le::<u8>();
        
        match signal {
            2 => Bremse53DiagFr::SignalError,
            1 => Bremse53DiagFr::LineError,
            0 => Bremse53DiagFr::SignalOk,
            _ => Bremse53DiagFr::_Other(self.diag_fr_raw()),
        }
    }
    
    /// Get raw value of Diag_FR
    ///
    /// - Start bit: 34
    /// - Signal size: 2 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn diag_fr_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[34..36].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Diag_FR
    #[inline(always)]
    pub fn set_diag_fr(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 3_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse53::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Bremse53::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[34..36].store_le(value);
        Ok(())
    }
    
    /// Diag_RL
    ///
    /// Value to show faults related to the wheel speed sensor. 
    /// 0 - Signal ok, 1 - Wiring related fault, 2 - Signal related fault
    ///
    /// - Min: 0
    /// - Max: 3
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn diag_rl(&self) -> Bremse53DiagRl {
        let signal = self.raw.view_bits::<Lsb0>()[36..38].load_le::<u8>();
        
        match signal {
            2 => Bremse53DiagRl::SignalError,
            1 => Bremse53DiagRl::LineError,
            0 => Bremse53DiagRl::SignalOk,
            _ => Bremse53DiagRl::_Other(self.diag_rl_raw()),
        }
    }
    
    /// Get raw value of Diag_RL
    ///
    /// - Start bit: 36
    /// - Signal size: 2 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn diag_rl_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[36..38].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Diag_RL
    #[inline(always)]
    pub fn set_diag_rl(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 3_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse53::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Bremse53::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[36..38].store_le(value);
        Ok(())
    }
    
    /// Diag_RR
    ///
    /// Value to show faults related to the wheel speed sensor. 
    /// 0 - Signal ok, 1 - Wiring related fault, 2 - Signal related fault
    ///
    /// - Min: 0
    /// - Max: 3
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn diag_rr(&self) -> Bremse53DiagRr {
        let signal = self.raw.view_bits::<Lsb0>()[38..40].load_le::<u8>();
        
        match signal {
            2 => Bremse53DiagRr::SignalError,
            1 => Bremse53DiagRr::LineError,
            0 => Bremse53DiagRr::SignalOk,
            _ => Bremse53DiagRr::_Other(self.diag_rr_raw()),
        }
    }
    
    /// Get raw value of Diag_RR
    ///
    /// - Start bit: 38
    /// - Signal size: 2 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn diag_rr_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[38..40].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Diag_RR
    #[inline(always)]
    pub fn set_diag_rr(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 3_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse53::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Bremse53::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[38..40].store_le(value);
        Ok(())
    }
    
    /// Diag_ABSUnit
    ///
    /// Bit to show, if a ABS error related to the hydraulic unit is present
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn diag_abs_unit(&self) -> bool {
        self.diag_abs_unit_raw()
    }
    
    /// Get raw value of Diag_ABSUnit
    ///
    /// - Start bit: 40
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn diag_abs_unit_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[40..41].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of Diag_ABSUnit
    #[inline(always)]
    pub fn set_diag_abs_unit(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[40..41].store_le(value);
        Ok(())
    }
    
    /// Diag_FuseValve
    ///
    /// Bit to show, if a ABS error related to the fuse or power supply of the ABS valves is present.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn diag_fuse_valve(&self) -> bool {
        self.diag_fuse_valve_raw()
    }
    
    /// Get raw value of Diag_FuseValve
    ///
    /// - Start bit: 41
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn diag_fuse_valve_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[41..42].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of Diag_FuseValve
    #[inline(always)]
    pub fn set_diag_fuse_valve(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[41..42].store_le(value);
        Ok(())
    }
    
    /// Diag_FusePump
    ///
    /// Bit to show, if a ABS error related to the fuse or power supply of the ABS pump is present.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn diag_fuse_pump(&self) -> bool {
        self.diag_fuse_pump_raw()
    }
    
    /// Get raw value of Diag_FusePump
    ///
    /// - Start bit: 42
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn diag_fuse_pump_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[42..43].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of Diag_FusePump
    #[inline(always)]
    pub fn set_diag_fuse_pump(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[42..43].store_le(value);
        Ok(())
    }
    
    /// Diag_P_FA
    ///
    /// Bit to show, if the pressure sensor FA is working properly. An error is pressent, if the bit is 1.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn diag_p_fa(&self) -> bool {
        self.diag_p_fa_raw()
    }
    
    /// Get raw value of Diag_P_FA
    ///
    /// - Start bit: 43
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn diag_p_fa_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[43..44].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of Diag_P_FA
    #[inline(always)]
    pub fn set_diag_p_fa(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[43..44].store_le(value);
        Ok(())
    }
    
    /// Diag_P_RA
    ///
    /// Bit to show, if the pressure sensor RA is working properly. An error is pressent, if the bit is 1.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn diag_p_ra(&self) -> bool {
        self.diag_p_ra_raw()
    }
    
    /// Get raw value of Diag_P_RA
    ///
    /// - Start bit: 44
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn diag_p_ra_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[44..45].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of Diag_P_RA
    #[inline(always)]
    pub fn set_diag_p_ra(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[44..45].store_le(value);
        Ok(())
    }
    
    /// Diag_YRS
    ///
    /// Bit to show, if the yaw rate sensor is working properly. An error is pressent, if the bit is 1.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn diag_yrs(&self) -> bool {
        self.diag_yrs_raw()
    }
    
    /// Get raw value of Diag_YRS
    ///
    /// - Start bit: 45
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn diag_yrs_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[45..46].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of Diag_YRS
    #[inline(always)]
    pub fn set_diag_yrs(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[45..46].store_le(value);
        Ok(())
    }
    
    /// ABS_fault_info
    ///
    /// Bit matrix to show if a fault or a active fault is stored in the ABS. Bit will also show minor errors which do  not shut down the ABS controller.
    ///
    /// - Min: 0
    /// - Max: 3
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn abs_fault_info(&self) -> Bremse53AbsFaultInfo {
        let signal = self.raw.view_bits::<Lsb0>()[46..48].load_le::<u8>();
        
        match signal {
            2 => Bremse53AbsFaultInfo::ActiveFaultsStored,
            1 => Bremse53AbsFaultInfo::InactiveFaultsStored,
            0 => Bremse53AbsFaultInfo::NoFaultsStored,
            _ => Bremse53AbsFaultInfo::_Other(self.abs_fault_info_raw()),
        }
    }
    
    /// Get raw value of ABS_fault_info
    ///
    /// - Start bit: 46
    /// - Signal size: 2 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn abs_fault_info_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[46..48].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of ABS_fault_info
    #[inline(always)]
    pub fn set_abs_fault_info(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 3_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse53::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Bremse53::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[46..48].store_le(value);
        Ok(())
    }
    
    /// P_RA
    ///
    /// Brake pressure on the rear axle.
    ///
    /// - Min: -42.5
    /// - Max: 425
    /// - Unit: "bar"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn p_ra(&self) -> f32 {
        self.p_ra_raw()
    }
    
    /// Get raw value of P_RA
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 0.01526
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn p_ra_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<i16>();
        
        let factor = 0.01526_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of P_RA
    #[inline(always)]
    pub fn set_p_ra(&mut self, value: f32) -> Result<(), CanError> {
        if value < -42.5_f32 || 425_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Bremse53::MESSAGE_ID });
        }
        let factor = 0.01526_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Bremse53 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Bremse53 {
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
/// Defined values for Diag_FL
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
pub enum Bremse53DiagFl {
    SignalError,
    LineError,
    SignalOk,
    _Other(u8),
}

impl From<Bremse53DiagFl> for u8 {
    fn from(val: Bremse53DiagFl) -> u8 {
        match val {
            Bremse53DiagFl::SignalError => 2,
            Bremse53DiagFl::LineError => 1,
            Bremse53DiagFl::SignalOk => 0,
            Bremse53DiagFl::_Other(x) => x,
        }
    }
}

/// Defined values for Diag_FR
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
pub enum Bremse53DiagFr {
    SignalError,
    LineError,
    SignalOk,
    _Other(u8),
}

impl From<Bremse53DiagFr> for u8 {
    fn from(val: Bremse53DiagFr) -> u8 {
        match val {
            Bremse53DiagFr::SignalError => 2,
            Bremse53DiagFr::LineError => 1,
            Bremse53DiagFr::SignalOk => 0,
            Bremse53DiagFr::_Other(x) => x,
        }
    }
}

/// Defined values for Diag_RL
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
pub enum Bremse53DiagRl {
    SignalError,
    LineError,
    SignalOk,
    _Other(u8),
}

impl From<Bremse53DiagRl> for u8 {
    fn from(val: Bremse53DiagRl) -> u8 {
        match val {
            Bremse53DiagRl::SignalError => 2,
            Bremse53DiagRl::LineError => 1,
            Bremse53DiagRl::SignalOk => 0,
            Bremse53DiagRl::_Other(x) => x,
        }
    }
}

/// Defined values for Diag_RR
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
pub enum Bremse53DiagRr {
    SignalError,
    LineError,
    SignalOk,
    _Other(u8),
}

impl From<Bremse53DiagRr> for u8 {
    fn from(val: Bremse53DiagRr) -> u8 {
        match val {
            Bremse53DiagRr::SignalError => 2,
            Bremse53DiagRr::LineError => 1,
            Bremse53DiagRr::SignalOk => 0,
            Bremse53DiagRr::_Other(x) => x,
        }
    }
}

/// Defined values for ABS_fault_info
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
pub enum Bremse53AbsFaultInfo {
    ActiveFaultsStored,
    InactiveFaultsStored,
    NoFaultsStored,
    _Other(u8),
}

impl From<Bremse53AbsFaultInfo> for u8 {
    fn from(val: Bremse53AbsFaultInfo) -> u8 {
        match val {
            Bremse53AbsFaultInfo::ActiveFaultsStored => 2,
            Bremse53AbsFaultInfo::InactiveFaultsStored => 1,
            Bremse53AbsFaultInfo::NoFaultsStored => 0,
            Bremse53AbsFaultInfo::_Other(x) => x,
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

