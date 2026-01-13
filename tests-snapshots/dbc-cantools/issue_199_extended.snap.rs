// Generated code!
//
// Message definitions from file `issue_199_extended`
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
    /// DriverDoorStatus
    DriverDoorStatus(DriverDoorStatus),
    /// Chime
    Chime(Chime),
    /// BlinkerStatus
    BlinkerStatus(BlinkerStatus),
    /// SteeringWheelAngle
    SteeringWheelAngle(SteeringWheelAngle),
    /// GearShifter
    GearShifter(GearShifter),
    /// GasPedalRegenCruise
    GasPedalRegenCruise(GasPedalRegenCruise),
    /// BrakePedal
    BrakePedal(BrakePedal),
    /// WheelSpeed
    WheelSpeed(WheelSpeed),
    /// VehicleSpeed
    VehicleSpeed(VehicleSpeed),
    /// CruiseButtons
    CruiseButtons(CruiseButtons),
    /// CruiseButtons2
    CruiseButtons2(CruiseButtons2),
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
            DriverDoorStatus::MESSAGE_ID => Messages::DriverDoorStatus(DriverDoorStatus::try_from(payload)?),
            Chime::MESSAGE_ID => Messages::Chime(Chime::try_from(payload)?),
            BlinkerStatus::MESSAGE_ID => Messages::BlinkerStatus(BlinkerStatus::try_from(payload)?),
            SteeringWheelAngle::MESSAGE_ID => Messages::SteeringWheelAngle(SteeringWheelAngle::try_from(payload)?),
            GearShifter::MESSAGE_ID => Messages::GearShifter(GearShifter::try_from(payload)?),
            GasPedalRegenCruise::MESSAGE_ID => Messages::GasPedalRegenCruise(GasPedalRegenCruise::try_from(payload)?),
            BrakePedal::MESSAGE_ID => Messages::BrakePedal(BrakePedal::try_from(payload)?),
            WheelSpeed::MESSAGE_ID => Messages::WheelSpeed(WheelSpeed::try_from(payload)?),
            VehicleSpeed::MESSAGE_ID => Messages::VehicleSpeed(VehicleSpeed::try_from(payload)?),
            CruiseButtons::MESSAGE_ID => Messages::CruiseButtons(CruiseButtons::try_from(payload)?),
            CruiseButtons2::MESSAGE_ID => Messages::CruiseButtons2(CruiseButtons2::try_from(payload)?),
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}

/// DriverDoorStatus
///
/// - Extended ID: 536870911 (0x1fffffff)
/// - Size: 1 bytes
/// - Transmitter: GMLAN
#[derive(Clone, Copy)]
pub struct DriverDoorStatus {
    raw: [u8; 1],
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
impl DriverDoorStatus {
    pub const MESSAGE_ID: embedded_can::Id = Id::Extended(unsafe { ExtendedId::new_unchecked(0x1fffffff)});
    
    
    /// Construct new DriverDoorStatus from values
    pub fn new(driver_door_opened: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_driver_door_opened(driver_door_opened)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// DriverDoorOpened
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: NEO
    #[inline(always)]
    pub fn driver_door_opened(&self) -> DriverDoorStatusDriverDoorOpened {
        let signal = self.raw.view_bits::<Msb0>()[7..8].load_be::<u8>();
        
        match signal {
            1 => DriverDoorStatusDriverDoorOpened::Opened,
            0 => DriverDoorStatusDriverDoorOpened::Closed,
            _ => DriverDoorStatusDriverDoorOpened::_Other(self.driver_door_opened_raw()),
        }
    }
    
    /// Get raw value of DriverDoorOpened
    ///
    /// - Start bit: 0
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn driver_door_opened_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[7..8].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of DriverDoorOpened
    #[inline(always)]
    pub fn set_driver_door_opened(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[7..8].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for DriverDoorStatus {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for DriverDoorStatus {
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
/// Defined values for DriverDoorOpened
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
pub enum DriverDoorStatusDriverDoorOpened {
    Opened,
    Closed,
    _Other(bool),
}

impl From<DriverDoorStatusDriverDoorOpened> for bool {
    fn from(val: DriverDoorStatusDriverDoorOpened) -> bool {
        match val {
            DriverDoorStatusDriverDoorOpened::Opened => true,
            DriverDoorStatusDriverDoorOpened::Closed => false,
            DriverDoorStatusDriverDoorOpened::_Other(x) => x,
        }
    }
}


/// Chime
///
/// - Standard ID: 0 (0x0)
/// - Size: 5 bytes
/// - Transmitter: NEO
#[derive(Clone, Copy)]
pub struct Chime {
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
impl Chime {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x0)});
    
    pub const CHIME_TYPE_MIN: u8 = 0_u8;
    pub const CHIME_TYPE_MAX: u8 = 0_u8;
    pub const CHIME_REPEAT_MIN: u8 = 0_u8;
    pub const CHIME_REPEAT_MAX: u8 = 0_u8;
    pub const CHIME_DURATION_MIN: u8 = 0_u8;
    pub const CHIME_DURATION_MAX: u8 = 0_u8;
    pub const CHIME_BYTE5_MIN: u8 = 0_u8;
    pub const CHIME_BYTE5_MAX: u8 = 0_u8;
    pub const CHIME_BYTE4_MIN: u8 = 0_u8;
    pub const CHIME_BYTE4_MAX: u8 = 0_u8;
    
    /// Construct new Chime from values
    pub fn new(chime_type: u8, chime_repeat: u8, chime_duration: u8, chime_byte5: u8, chime_byte4: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 5] };
        res.set_chime_type(chime_type)?;
        res.set_chime_repeat(chime_repeat)?;
        res.set_chime_duration(chime_duration)?;
        res.set_chime_byte5(chime_byte5)?;
        res.set_chime_byte4(chime_byte4)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 5] {
        &self.raw
    }
    
    /// ChimeType
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: GMLAN
    #[inline(always)]
    pub fn chime_type(&self) -> u8 {
        self.chime_type_raw()
    }
    
    /// Get raw value of ChimeType
    ///
    /// - Start bit: 7
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn chime_type_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[0..8].load_be::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of ChimeType
    #[inline(always)]
    pub fn set_chime_type(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Chime::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Chime::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Msb0>()[0..8].store_be(value);
        Ok(())
    }
    
    /// ChimeRepeat
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: GMLAN
    #[inline(always)]
    pub fn chime_repeat(&self) -> u8 {
        self.chime_repeat_raw()
    }
    
    /// Get raw value of ChimeRepeat
    ///
    /// - Start bit: 23
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn chime_repeat_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[16..24].load_be::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of ChimeRepeat
    #[inline(always)]
    pub fn set_chime_repeat(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Chime::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Chime::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Msb0>()[16..24].store_be(value);
        Ok(())
    }
    
    /// ChimeDuration
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: GMLAN
    #[inline(always)]
    pub fn chime_duration(&self) -> u8 {
        self.chime_duration_raw()
    }
    
    /// Get raw value of ChimeDuration
    ///
    /// - Start bit: 15
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn chime_duration_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[8..16].load_be::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of ChimeDuration
    #[inline(always)]
    pub fn set_chime_duration(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Chime::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Chime::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Msb0>()[8..16].store_be(value);
        Ok(())
    }
    
    /// ChimeByte5
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: GMLAN
    #[inline(always)]
    pub fn chime_byte5(&self) -> u8 {
        self.chime_byte5_raw()
    }
    
    /// Get raw value of ChimeByte5
    ///
    /// - Start bit: 39
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn chime_byte5_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[32..40].load_be::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of ChimeByte5
    #[inline(always)]
    pub fn set_chime_byte5(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Chime::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Chime::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Msb0>()[32..40].store_be(value);
        Ok(())
    }
    
    /// ChimeByte4
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: GMLAN
    #[inline(always)]
    pub fn chime_byte4(&self) -> u8 {
        self.chime_byte4_raw()
    }
    
    /// Get raw value of ChimeByte4
    ///
    /// - Start bit: 31
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn chime_byte4_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[24..32].load_be::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of ChimeByte4
    #[inline(always)]
    pub fn set_chime_byte4(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Chime::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Chime::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Msb0>()[24..32].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Chime {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 5 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 5];
        raw.copy_from_slice(&payload[..5]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Chime {
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

/// BlinkerStatus
///
/// - Standard ID: 49152 (0xc000)
/// - Size: 5 bytes
/// - Transmitter: GMLAN
#[derive(Clone, Copy)]
pub struct BlinkerStatus {
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
impl BlinkerStatus {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0xc000)});
    
    
    /// Construct new BlinkerStatus from values
    pub fn new(right_blinker: bool, left_blinker: bool, blinker_light: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 5] };
        res.set_right_blinker(right_blinker)?;
        res.set_left_blinker(left_blinker)?;
        res.set_blinker_light(blinker_light)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 5] {
        &self.raw
    }
    
    /// RightBlinker
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: NEO
    #[inline(always)]
    pub fn right_blinker(&self) -> BlinkerStatusRightBlinker {
        let signal = self.raw.view_bits::<Msb0>()[1..2].load_be::<u8>();
        
        match signal {
            1 => BlinkerStatusRightBlinker::Active,
            0 => BlinkerStatusRightBlinker::Inactive,
            _ => BlinkerStatusRightBlinker::_Other(self.right_blinker_raw()),
        }
    }
    
    /// Get raw value of RightBlinker
    ///
    /// - Start bit: 6
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn right_blinker_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[1..2].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of RightBlinker
    #[inline(always)]
    pub fn set_right_blinker(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[1..2].store_be(value);
        Ok(())
    }
    
    /// LeftBlinker
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: NEO
    #[inline(always)]
    pub fn left_blinker(&self) -> BlinkerStatusLeftBlinker {
        let signal = self.raw.view_bits::<Msb0>()[0..1].load_be::<u8>();
        
        match signal {
            1 => BlinkerStatusLeftBlinker::Active,
            0 => BlinkerStatusLeftBlinker::Inactive,
            _ => BlinkerStatusLeftBlinker::_Other(self.left_blinker_raw()),
        }
    }
    
    /// Get raw value of LeftBlinker
    ///
    /// - Start bit: 7
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn left_blinker_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[0..1].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of LeftBlinker
    #[inline(always)]
    pub fn set_left_blinker(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[0..1].store_be(value);
        Ok(())
    }
    
    /// BlinkerLight
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: NEO
    #[inline(always)]
    pub fn blinker_light(&self) -> BlinkerStatusBlinkerLight {
        let signal = self.raw.view_bits::<Msb0>()[30..31].load_be::<u8>();
        
        match signal {
            1 => BlinkerStatusBlinkerLight::Active,
            0 => BlinkerStatusBlinkerLight::Inactive,
            _ => BlinkerStatusBlinkerLight::_Other(self.blinker_light_raw()),
        }
    }
    
    /// Get raw value of BlinkerLight
    ///
    /// - Start bit: 25
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn blinker_light_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[30..31].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of BlinkerLight
    #[inline(always)]
    pub fn set_blinker_light(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[30..31].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for BlinkerStatus {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 5 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 5];
        raw.copy_from_slice(&payload[..5]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for BlinkerStatus {
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
/// Defined values for RightBlinker
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
pub enum BlinkerStatusRightBlinker {
    Active,
    Inactive,
    _Other(bool),
}

impl From<BlinkerStatusRightBlinker> for bool {
    fn from(val: BlinkerStatusRightBlinker) -> bool {
        match val {
            BlinkerStatusRightBlinker::Active => true,
            BlinkerStatusRightBlinker::Inactive => false,
            BlinkerStatusRightBlinker::_Other(x) => x,
        }
    }
}

/// Defined values for LeftBlinker
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
pub enum BlinkerStatusLeftBlinker {
    Active,
    Inactive,
    _Other(bool),
}

impl From<BlinkerStatusLeftBlinker> for bool {
    fn from(val: BlinkerStatusLeftBlinker) -> bool {
        match val {
            BlinkerStatusLeftBlinker::Active => true,
            BlinkerStatusLeftBlinker::Inactive => false,
            BlinkerStatusLeftBlinker::_Other(x) => x,
        }
    }
}

/// Defined values for BlinkerLight
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
pub enum BlinkerStatusBlinkerLight {
    Active,
    Inactive,
    _Other(bool),
}

impl From<BlinkerStatusBlinkerLight> for bool {
    fn from(val: BlinkerStatusBlinkerLight) -> bool {
        match val {
            BlinkerStatusBlinkerLight::Active => true,
            BlinkerStatusBlinkerLight::Inactive => false,
            BlinkerStatusBlinkerLight::_Other(x) => x,
        }
    }
}


/// SteeringWheelAngle
///
/// - Standard ID: 0 (0x0)
/// - Size: 8 bytes
/// - Transmitter: GMLAN
#[derive(Clone, Copy)]
pub struct SteeringWheelAngle {
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
impl SteeringWheelAngle {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x0)});
    
    pub const STEERING_WHEEL_ANGLE_MIN: f32 = -540_f32;
    pub const STEERING_WHEEL_ANGLE_MAX: f32 = 540_f32;
    
    /// Construct new SteeringWheelAngle from values
    pub fn new(steering_wheel_angle: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_steering_wheel_angle(steering_wheel_angle)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// SteeringWheelAngle
    ///
    /// - Min: -540
    /// - Max: 540
    /// - Unit: "deg"
    /// - Receivers: NEO
    #[inline(always)]
    pub fn steering_wheel_angle(&self) -> f32 {
        self.steering_wheel_angle_raw()
    }
    
    /// Get raw value of SteeringWheelAngle
    ///
    /// - Start bit: 39
    /// - Signal size: 16 bits
    /// - Factor: 0.0625
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn steering_wheel_angle_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Msb0>()[32..48].load_be::<i16>();
        
        let factor = 0.0625_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of SteeringWheelAngle
    #[inline(always)]
    pub fn set_steering_wheel_angle(&mut self, value: f32) -> Result<(), CanError> {
        if value < -540_f32 || 540_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: SteeringWheelAngle::MESSAGE_ID });
        }
        let factor = 0.0625_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[32..48].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for SteeringWheelAngle {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for SteeringWheelAngle {
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

/// GearShifter
///
/// - Standard ID: 49152 (0xc000)
/// - Size: 8 bytes
/// - Transmitter: GMLAN
#[derive(Clone, Copy)]
pub struct GearShifter {
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
impl GearShifter {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0xc000)});
    
    pub const GEAR_SHIFTER_MIN: u8 = 0_u8;
    pub const GEAR_SHIFTER_MAX: u8 = 3_u8;
    
    /// Construct new GearShifter from values
    pub fn new(gear_shifter: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_gear_shifter(gear_shifter)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// GearShifter
    ///
    /// - Min: 0
    /// - Max: 3
    /// - Unit: ""
    /// - Receivers: NEO
    #[inline(always)]
    pub fn gear_shifter(&self) -> GearShifterGearShifter {
        let signal = self.raw.view_bits::<Msb0>()[22..24].load_be::<u8>();
        
        match signal {
            3 => GearShifterGearShifter::Park,
            0 => GearShifterGearShifter::DriveLow,
            _ => GearShifterGearShifter::_Other(self.gear_shifter_raw()),
        }
    }
    
    /// Get raw value of GearShifter
    ///
    /// - Start bit: 17
    /// - Signal size: 2 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn gear_shifter_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[22..24].load_be::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of GearShifter
    #[inline(always)]
    pub fn set_gear_shifter(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 3_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: GearShifter::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: GearShifter::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Msb0>()[22..24].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for GearShifter {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for GearShifter {
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
/// Defined values for RightBlinker
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
pub enum GearShifterRightBlinker {
    Active,
    Inactive,
    _Other(bool),
}

impl From<GearShifterRightBlinker> for bool {
    fn from(val: GearShifterRightBlinker) -> bool {
        match val {
            GearShifterRightBlinker::Active => true,
            GearShifterRightBlinker::Inactive => false,
            GearShifterRightBlinker::_Other(x) => x,
        }
    }
}

/// Defined values for LeftBlinker
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
pub enum GearShifterLeftBlinker {
    Active,
    Inactive,
    _Other(bool),
}

impl From<GearShifterLeftBlinker> for bool {
    fn from(val: GearShifterLeftBlinker) -> bool {
        match val {
            GearShifterLeftBlinker::Active => true,
            GearShifterLeftBlinker::Inactive => false,
            GearShifterLeftBlinker::_Other(x) => x,
        }
    }
}

/// Defined values for BlinkerLight
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
pub enum GearShifterBlinkerLight {
    Active,
    Inactive,
    _Other(bool),
}

impl From<GearShifterBlinkerLight> for bool {
    fn from(val: GearShifterBlinkerLight) -> bool {
        match val {
            GearShifterBlinkerLight::Active => true,
            GearShifterBlinkerLight::Inactive => false,
            GearShifterBlinkerLight::_Other(x) => x,
        }
    }
}


/// GasPedalRegenCruise
///
/// - Standard ID: 40960 (0xa000)
/// - Size: 8 bytes
/// - Transmitter: GMLAN
#[derive(Clone, Copy)]
pub struct GasPedalRegenCruise {
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
impl GasPedalRegenCruise {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0xa000)});
    
    pub const GAS_PEDAL_MIN: u8 = 0_u8;
    pub const GAS_PEDAL_MAX: u8 = 254_u8;
    pub const GEAR_SHIFTER2_NOT_USED_MIN: u8 = 0_u8;
    pub const GEAR_SHIFTER2_NOT_USED_MAX: u8 = 255_u8;
    
    /// Construct new GasPedalRegenCruise from values
    pub fn new(cruise_control_active: bool, max_regen: bool, gas_pedal: u8, gear_shifter2_not_used: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_cruise_control_active(cruise_control_active)?;
        res.set_max_regen(max_regen)?;
        res.set_gas_pedal(gas_pedal)?;
        res.set_gear_shifter2_not_used(gear_shifter2_not_used)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// CruiseControlActive
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: GMLAN
    #[inline(always)]
    pub fn cruise_control_active(&self) -> GasPedalRegenCruiseCruiseControlActive {
        let signal = self.raw.view_bits::<Msb0>()[63..64].load_be::<u8>();
        
        match signal {
            1 => GasPedalRegenCruiseCruiseControlActive::Active,
            0 => GasPedalRegenCruiseCruiseControlActive::Inactive,
            _ => GasPedalRegenCruiseCruiseControlActive::_Other(self.cruise_control_active_raw()),
        }
    }
    
    /// Get raw value of CruiseControlActive
    ///
    /// - Start bit: 56
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn cruise_control_active_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[63..64].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of CruiseControlActive
    #[inline(always)]
    pub fn set_cruise_control_active(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[63..64].store_be(value);
        Ok(())
    }
    
    /// MaxRegen
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: GMLAN, NEO
    #[inline(always)]
    pub fn max_regen(&self) -> bool {
        self.max_regen_raw()
    }
    
    /// Get raw value of MaxRegen
    ///
    /// - Start bit: 12
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn max_regen_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Msb0>()[11..12].load_be::<u8>();
        
        signal == 1
    }
    
    /// Set value of MaxRegen
    #[inline(always)]
    pub fn set_max_regen(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Msb0>()[11..12].store_be(value);
        Ok(())
    }
    
    /// GasPedal
    ///
    /// - Min: 0
    /// - Max: 254
    /// - Unit: ""
    /// - Receivers: GMLAN, NEO
    #[inline(always)]
    pub fn gas_pedal(&self) -> u8 {
        self.gas_pedal_raw()
    }
    
    /// Get raw value of GasPedal
    ///
    /// - Start bit: 47
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn gas_pedal_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[40..48].load_be::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of GasPedal
    #[inline(always)]
    pub fn set_gas_pedal(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 254_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: GasPedalRegenCruise::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: GasPedalRegenCruise::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Msb0>()[40..48].store_be(value);
        Ok(())
    }
    
    /// GearShifter2NotUsed
    ///
    /// - Min: 0
    /// - Max: 255
    /// - Unit: ""
    /// - Receivers: GMLAN, NEO
    #[inline(always)]
    pub fn gear_shifter2_not_used(&self) -> u8 {
        self.gear_shifter2_not_used_raw()
    }
    
    /// Get raw value of GearShifter2NotUsed
    ///
    /// - Start bit: 55
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn gear_shifter2_not_used_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[48..56].load_be::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of GearShifter2NotUsed
    #[inline(always)]
    pub fn set_gear_shifter2_not_used(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 255_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: GasPedalRegenCruise::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: GasPedalRegenCruise::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Msb0>()[48..56].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for GasPedalRegenCruise {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for GasPedalRegenCruise {
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
/// Defined values for CruiseControlActive
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
pub enum GasPedalRegenCruiseCruiseControlActive {
    Active,
    Inactive,
    _Other(bool),
}

impl From<GasPedalRegenCruiseCruiseControlActive> for bool {
    fn from(val: GasPedalRegenCruiseCruiseControlActive) -> bool {
        match val {
            GasPedalRegenCruiseCruiseControlActive::Active => true,
            GasPedalRegenCruiseCruiseControlActive::Inactive => false,
            GasPedalRegenCruiseCruiseControlActive::_Other(x) => x,
        }
    }
}


/// BrakePedal
///
/// - Standard ID: 0 (0x0)
/// - Size: 2 bytes
/// - Transmitter: GMLAN
#[derive(Clone, Copy)]
pub struct BrakePedal {
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
impl BrakePedal {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x0)});
    
    pub const BRAKE_LEVEL_MIN: u8 = 0_u8;
    pub const BRAKE_LEVEL_MAX: u8 = 3_u8;
    pub const BRAKE_SENSOR_MIN: u8 = 0_u8;
    pub const BRAKE_SENSOR_MAX: u8 = 255_u8;
    
    /// Construct new BrakePedal from values
    pub fn new(brake_level: u8, brake_sensor: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 2] };
        res.set_brake_level(brake_level)?;
        res.set_brake_sensor(brake_sensor)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 2] {
        &self.raw
    }
    
    /// BrakeLevel
    ///
    /// - Min: 0
    /// - Max: 3
    /// - Unit: ""
    /// - Receivers: NEO
    #[inline(always)]
    pub fn brake_level(&self) -> u8 {
        self.brake_level_raw()
    }
    
    /// Get raw value of BrakeLevel
    ///
    /// - Start bit: 2
    /// - Signal size: 2 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn brake_level_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[5..7].load_be::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of BrakeLevel
    #[inline(always)]
    pub fn set_brake_level(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 3_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: BrakePedal::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: BrakePedal::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Msb0>()[5..7].store_be(value);
        Ok(())
    }
    
    /// BrakeSensor
    ///
    /// - Min: 0
    /// - Max: 255
    /// - Unit: ""
    /// - Receivers: NEO
    #[inline(always)]
    pub fn brake_sensor(&self) -> u8 {
        self.brake_sensor_raw()
    }
    
    /// Get raw value of BrakeSensor
    ///
    /// - Start bit: 15
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn brake_sensor_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[8..16].load_be::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of BrakeSensor
    #[inline(always)]
    pub fn set_brake_sensor(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 255_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: BrakePedal::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: BrakePedal::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Msb0>()[8..16].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for BrakePedal {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 2 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 2];
        raw.copy_from_slice(&payload[..2]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for BrakePedal {
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

/// WheelSpeed
///
/// - Standard ID: 32768 (0x8000)
/// - Size: 8 bytes
/// - Transmitter: GMLAN
#[derive(Clone, Copy)]
pub struct WheelSpeed {
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
impl WheelSpeed {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x8000)});
    
    pub const WHEEL_SPEED_FL_MIN: f32 = 0_f32;
    pub const WHEEL_SPEED_FL_MAX: f32 = 70_f32;
    pub const WHEEL_SPEED_FR_MIN: f32 = 0_f32;
    pub const WHEEL_SPEED_FR_MAX: f32 = 70_f32;
    pub const WHEEL_SPEED_RL_MIN: f32 = 0_f32;
    pub const WHEEL_SPEED_RL_MAX: f32 = 70_f32;
    pub const WHEEL_SPEED_RR_MIN: f32 = 0_f32;
    pub const WHEEL_SPEED_RR_MAX: f32 = 70_f32;
    
    /// Construct new WheelSpeed from values
    pub fn new(wheel_speed_fl: f32, wheel_speed_fr: f32, wheel_speed_rl: f32, wheel_speed_rr: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_wheel_speed_fl(wheel_speed_fl)?;
        res.set_wheel_speed_fr(wheel_speed_fr)?;
        res.set_wheel_speed_rl(wheel_speed_rl)?;
        res.set_wheel_speed_rr(wheel_speed_rr)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// WheelSpeedFL
    ///
    /// - Min: 0
    /// - Max: 70
    /// - Unit: "yd/s"
    /// - Receivers: NEO
    #[inline(always)]
    pub fn wheel_speed_fl(&self) -> f32 {
        self.wheel_speed_fl_raw()
    }
    
    /// Get raw value of WheelSpeedFL
    ///
    /// - Start bit: 7
    /// - Signal size: 16 bits
    /// - Factor: 0.01
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn wheel_speed_fl_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Msb0>()[0..16].load_be::<u16>();
        
        let factor = 0.01_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of WheelSpeedFL
    #[inline(always)]
    pub fn set_wheel_speed_fl(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 70_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: WheelSpeed::MESSAGE_ID });
        }
        let factor = 0.01_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Msb0>()[0..16].store_be(value);
        Ok(())
    }
    
    /// WheelSpeedFR
    ///
    /// - Min: 0
    /// - Max: 70
    /// - Unit: "yd/s"
    /// - Receivers: NEO
    #[inline(always)]
    pub fn wheel_speed_fr(&self) -> f32 {
        self.wheel_speed_fr_raw()
    }
    
    /// Get raw value of WheelSpeedFR
    ///
    /// - Start bit: 39
    /// - Signal size: 16 bits
    /// - Factor: 0.01
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn wheel_speed_fr_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Msb0>()[32..48].load_be::<u16>();
        
        let factor = 0.01_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of WheelSpeedFR
    #[inline(always)]
    pub fn set_wheel_speed_fr(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 70_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: WheelSpeed::MESSAGE_ID });
        }
        let factor = 0.01_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Msb0>()[32..48].store_be(value);
        Ok(())
    }
    
    /// WheelSpeedRL
    ///
    /// - Min: 0
    /// - Max: 70
    /// - Unit: "yd/s"
    /// - Receivers: NEO
    #[inline(always)]
    pub fn wheel_speed_rl(&self) -> f32 {
        self.wheel_speed_rl_raw()
    }
    
    /// Get raw value of WheelSpeedRL
    ///
    /// - Start bit: 23
    /// - Signal size: 16 bits
    /// - Factor: 0.01
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn wheel_speed_rl_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Msb0>()[16..32].load_be::<u16>();
        
        let factor = 0.01_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of WheelSpeedRL
    #[inline(always)]
    pub fn set_wheel_speed_rl(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 70_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: WheelSpeed::MESSAGE_ID });
        }
        let factor = 0.01_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Msb0>()[16..32].store_be(value);
        Ok(())
    }
    
    /// WheelSpeedRR
    ///
    /// - Min: 0
    /// - Max: 70
    /// - Unit: "yd/s"
    /// - Receivers: NEO
    #[inline(always)]
    pub fn wheel_speed_rr(&self) -> f32 {
        self.wheel_speed_rr_raw()
    }
    
    /// Get raw value of WheelSpeedRR
    ///
    /// - Start bit: 55
    /// - Signal size: 16 bits
    /// - Factor: 0.01
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn wheel_speed_rr_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Msb0>()[48..64].load_be::<u16>();
        
        let factor = 0.01_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of WheelSpeedRR
    #[inline(always)]
    pub fn set_wheel_speed_rr(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 70_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: WheelSpeed::MESSAGE_ID });
        }
        let factor = 0.01_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Msb0>()[48..64].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for WheelSpeed {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for WheelSpeed {
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

/// VehicleSpeed
///
/// - Standard ID: 0 (0x0)
/// - Size: 8 bytes
/// - Transmitter: GMLAN
#[derive(Clone, Copy)]
pub struct VehicleSpeed {
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
impl VehicleSpeed {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x0)});
    
    pub const VEHICLE_SPEED1_MIN: f32 = 0_f32;
    pub const VEHICLE_SPEED1_MAX: f32 = 100_f32;
    pub const VEHICLE_SPEED2_MIN: f32 = 0_f32;
    pub const VEHICLE_SPEED2_MAX: f32 = 100_f32;
    
    /// Construct new VehicleSpeed from values
    pub fn new(vehicle_speed1: f32, vehicle_speed2: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_vehicle_speed1(vehicle_speed1)?;
        res.set_vehicle_speed2(vehicle_speed2)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// VehicleSpeed1
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "mph"
    /// - Receivers: NEO
    #[inline(always)]
    pub fn vehicle_speed1(&self) -> f32 {
        self.vehicle_speed1_raw()
    }
    
    /// Get raw value of VehicleSpeed1
    ///
    /// - Start bit: 7
    /// - Signal size: 16 bits
    /// - Factor: 0.01
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn vehicle_speed1_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Msb0>()[0..16].load_be::<u16>();
        
        let factor = 0.01_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of VehicleSpeed1
    #[inline(always)]
    pub fn set_vehicle_speed1(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 100_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: VehicleSpeed::MESSAGE_ID });
        }
        let factor = 0.01_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Msb0>()[0..16].store_be(value);
        Ok(())
    }
    
    /// VehicleSpeed2
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "mph"
    /// - Receivers: NEO
    #[inline(always)]
    pub fn vehicle_speed2(&self) -> f32 {
        self.vehicle_speed2_raw()
    }
    
    /// Get raw value of VehicleSpeed2
    ///
    /// - Start bit: 39
    /// - Signal size: 16 bits
    /// - Factor: 0.01
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn vehicle_speed2_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Msb0>()[32..48].load_be::<u16>();
        
        let factor = 0.01_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of VehicleSpeed2
    #[inline(always)]
    pub fn set_vehicle_speed2(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 100_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: VehicleSpeed::MESSAGE_ID });
        }
        let factor = 0.01_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Msb0>()[32..48].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for VehicleSpeed {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for VehicleSpeed {
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

/// CruiseButtons
///
/// - Standard ID: 32768 (0x8000)
/// - Size: 3 bytes
/// - Transmitter: GMLAN
#[derive(Clone, Copy)]
pub struct CruiseButtons {
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
impl CruiseButtons {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x8000)});
    
    pub const CRUISE_BUTTONS_MIN: u8 = 0_u8;
    pub const CRUISE_BUTTONS_MAX: u8 = 12_u8;
    
    /// Construct new CruiseButtons from values
    pub fn new(cruise_buttons: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 3] };
        res.set_cruise_buttons(cruise_buttons)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 3] {
        &self.raw
    }
    
    /// CruiseButtons
    ///
    /// - Min: 0
    /// - Max: 12
    /// - Unit: ""
    /// - Receivers: NEO
    #[inline(always)]
    pub fn cruise_buttons(&self) -> CruiseButtonsCruiseButtons {
        let signal = self.raw.view_bits::<Msb0>()[4..7].load_be::<u8>();
        
        match signal {
            6 => CruiseButtonsCruiseButtons::Cancel,
            5 => CruiseButtonsCruiseButtons::Main,
            3 => CruiseButtonsCruiseButtons::Set,
            2 => CruiseButtonsCruiseButtons::Resume,
            1 => CruiseButtonsCruiseButtons::None,
            _ => CruiseButtonsCruiseButtons::_Other(self.cruise_buttons_raw()),
        }
    }
    
    /// Get raw value of CruiseButtons
    ///
    /// - Start bit: 3
    /// - Signal size: 3 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn cruise_buttons_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[4..7].load_be::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of CruiseButtons
    #[inline(always)]
    pub fn set_cruise_buttons(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 12_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: CruiseButtons::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: CruiseButtons::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Msb0>()[4..7].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for CruiseButtons {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 3 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 3];
        raw.copy_from_slice(&payload[..3]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for CruiseButtons {
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

/// CruiseButtons2
///
/// - Standard ID: 24576 (0x6000)
/// - Size: 1 bytes
/// - Transmitter: GMLAN
#[derive(Clone, Copy)]
pub struct CruiseButtons2 {
    raw: [u8; 1],
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
impl CruiseButtons2 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x6000)});
    
    pub const LKA_GAP_BUTTON_MIN: u8 = 0_u8;
    pub const LKA_GAP_BUTTON_MAX: u8 = 2_u8;
    
    /// Construct new CruiseButtons2 from values
    pub fn new(lka_gap_button: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_lka_gap_button(lka_gap_button)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// LKAGapButton
    ///
    /// - Min: 0
    /// - Max: 2
    /// - Unit: ""
    /// - Receivers: NEO
    #[inline(always)]
    pub fn lka_gap_button(&self) -> CruiseButtons2LkaGapButton {
        let signal = self.raw.view_bits::<Msb0>()[6..8].load_be::<u8>();
        
        match signal {
            2 => CruiseButtons2LkaGapButton::X,
            1 => CruiseButtons2LkaGapButton::X,
            0 => CruiseButtons2LkaGapButton::None,
            _ => CruiseButtons2LkaGapButton::_Other(self.lka_gap_button_raw()),
        }
    }
    
    /// Get raw value of LKAGapButton
    ///
    /// - Start bit: 1
    /// - Signal size: 2 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn lka_gap_button_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[6..8].load_be::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of LKAGapButton
    #[inline(always)]
    pub fn set_lka_gap_button(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 2_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: CruiseButtons2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: CruiseButtons2::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Msb0>()[6..8].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for CruiseButtons2 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for CruiseButtons2 {
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
/// Defined values for LKAGapButton
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
pub enum CruiseButtons2LkaGapButton {
    X,
    X,
    None,
    _Other(u8),
}

impl From<CruiseButtons2LkaGapButton> for u8 {
    fn from(val: CruiseButtons2LkaGapButton) -> u8 {
        match val {
            CruiseButtons2LkaGapButton::X => 2,
            CruiseButtons2LkaGapButton::X => 1,
            CruiseButtons2LkaGapButton::None => 0,
            CruiseButtons2LkaGapButton::_Other(x) => x,
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

