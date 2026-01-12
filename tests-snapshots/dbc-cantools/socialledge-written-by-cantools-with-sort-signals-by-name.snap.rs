// Generated code!
//
// Message definitions from file `socialledge-written-by-cantools-with-sort-signals-by-name`
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
    /// DRIVER_HEARTBEAT
    DriverHeartbeat(DriverHeartbeat),
    /// IO_DEBUG
    IoDebug(IoDebug),
    /// MOTOR_CMD
    MotorCmd(MotorCmd),
    /// MOTOR_STATUS
    MotorStatus(MotorStatus),
    /// SENSOR_SONARS
    SensorSonars(SensorSonars),
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
            DriverHeartbeat::MESSAGE_ID => Messages::DriverHeartbeat(DriverHeartbeat::try_from(payload)?),
            IoDebug::MESSAGE_ID => Messages::IoDebug(IoDebug::try_from(payload)?),
            MotorCmd::MESSAGE_ID => Messages::MotorCmd(MotorCmd::try_from(payload)?),
            MotorStatus::MESSAGE_ID => Messages::MotorStatus(MotorStatus::try_from(payload)?),
            SensorSonars::MESSAGE_ID => Messages::SensorSonars(SensorSonars::try_from(payload)?),
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}

/// DRIVER_HEARTBEAT
///
/// - Standard ID: 100 (0x64)
/// - Size: 1 bytes
/// - Transmitter: DRIVER
///
/// Sync message used to synchronize the controllers
#[derive(Clone, Copy)]
pub struct DriverHeartbeat {
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
impl DriverHeartbeat {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x64)});
    
    pub const DRIVER_HEARTBEAT_CMD_MIN: u8 = 0_u8;
    pub const DRIVER_HEARTBEAT_CMD_MAX: u8 = 0_u8;
    
    /// Construct new DRIVER_HEARTBEAT from values
    pub fn new(driver_heartbeat_cmd: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_driver_heartbeat_cmd(driver_heartbeat_cmd)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// DRIVER_HEARTBEAT_cmd
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: SENSOR, MOTOR
    #[inline(always)]
    pub fn driver_heartbeat_cmd(&self) -> DriverHeartbeatDriverHeartbeatCmd {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        match signal {
            2 => DriverHeartbeatDriverHeartbeatCmd::DriverHeartbeatCmdReboot,
            1 => DriverHeartbeatDriverHeartbeatCmd::DriverHeartbeatCmdSync,
            0 => DriverHeartbeatDriverHeartbeatCmd::DriverHeartbeatCmdNoop,
            _ => DriverHeartbeatDriverHeartbeatCmd::_Other(self.driver_heartbeat_cmd_raw()),
        }
    }
    
    /// Get raw value of DRIVER_HEARTBEAT_cmd
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn driver_heartbeat_cmd_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of DRIVER_HEARTBEAT_cmd
    #[inline(always)]
    pub fn set_driver_heartbeat_cmd(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: DriverHeartbeat::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: DriverHeartbeat::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for DriverHeartbeat {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for DriverHeartbeat {
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
/// Defined values for DRIVER_HEARTBEAT_cmd
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
pub enum DriverHeartbeatDriverHeartbeatCmd {
    DriverHeartbeatCmdReboot,
    DriverHeartbeatCmdSync,
    DriverHeartbeatCmdNoop,
    _Other(u8),
}

impl From<DriverHeartbeatDriverHeartbeatCmd> for u8 {
    fn from(val: DriverHeartbeatDriverHeartbeatCmd) -> u8 {
        match val {
            DriverHeartbeatDriverHeartbeatCmd::DriverHeartbeatCmdReboot => 2,
            DriverHeartbeatDriverHeartbeatCmd::DriverHeartbeatCmdSync => 1,
            DriverHeartbeatDriverHeartbeatCmd::DriverHeartbeatCmdNoop => 0,
            DriverHeartbeatDriverHeartbeatCmd::_Other(x) => x,
        }
    }
}


/// IO_DEBUG
///
/// - Standard ID: 500 (0x1f4)
/// - Size: 4 bytes
/// - Transmitter: IO
#[derive(Clone, Copy)]
pub struct IoDebug {
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
impl IoDebug {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x1f4)});
    
    pub const IO_DEBUG_TEST_ENUM_MIN: u8 = 0_u8;
    pub const IO_DEBUG_TEST_ENUM_MAX: u8 = 0_u8;
    pub const IO_DEBUG_TEST_FLOAT_MIN: f32 = 0_f32;
    pub const IO_DEBUG_TEST_FLOAT_MAX: f32 = 0_f32;
    pub const IO_DEBUG_TEST_SIGNED_MIN: i8 = 0_i8;
    pub const IO_DEBUG_TEST_SIGNED_MAX: i8 = 0_i8;
    pub const IO_DEBUG_TEST_UNSIGNED_MIN: u8 = 0_u8;
    pub const IO_DEBUG_TEST_UNSIGNED_MAX: u8 = 0_u8;
    
    /// Construct new IO_DEBUG from values
    pub fn new(io_debug_test_enum: u8, io_debug_test_float: f32, io_debug_test_signed: i8, io_debug_test_unsigned: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 4] };
        res.set_io_debug_test_enum(io_debug_test_enum)?;
        res.set_io_debug_test_float(io_debug_test_float)?;
        res.set_io_debug_test_signed(io_debug_test_signed)?;
        res.set_io_debug_test_unsigned(io_debug_test_unsigned)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 4] {
        &self.raw
    }
    
    /// IO_DEBUG_test_enum
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: DBG
    #[inline(always)]
    pub fn io_debug_test_enum(&self) -> IoDebugIoDebugTestEnum {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
        
        match signal {
            2 => IoDebugIoDebugTestEnum::IoDebugTest2EnumTwo,
            1 => IoDebugIoDebugTestEnum::IoDebugTest2EnumOne,
            _ => IoDebugIoDebugTestEnum::_Other(self.io_debug_test_enum_raw()),
        }
    }
    
    /// Get raw value of IO_DEBUG_test_enum
    ///
    /// - Start bit: 8
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn io_debug_test_enum_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of IO_DEBUG_test_enum
    #[inline(always)]
    pub fn set_io_debug_test_enum(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: IoDebug::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: IoDebug::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
        Ok(())
    }
    
    /// IO_DEBUG_test_float
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: DBG
    #[inline(always)]
    pub fn io_debug_test_float(&self) -> f32 {
        self.io_debug_test_float_raw()
    }
    
    /// Get raw value of IO_DEBUG_test_float
    ///
    /// - Start bit: 24
    /// - Signal size: 8 bits
    /// - Factor: 0.5
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn io_debug_test_float_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[24..32].load_le::<u8>();
        
        let factor = 0.5_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of IO_DEBUG_test_float
    #[inline(always)]
    pub fn set_io_debug_test_float(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: IoDebug::MESSAGE_ID });
        }
        let factor = 0.5_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[24..32].store_le(value);
        Ok(())
    }
    
    /// IO_DEBUG_test_signed
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: DBG
    #[inline(always)]
    pub fn io_debug_test_signed(&self) -> i8 {
        self.io_debug_test_signed_raw()
    }
    
    /// Get raw value of IO_DEBUG_test_signed
    ///
    /// - Start bit: 16
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn io_debug_test_signed_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[16..24].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of IO_DEBUG_test_signed
    #[inline(always)]
    pub fn set_io_debug_test_signed(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: IoDebug::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: IoDebug::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[16..24].store_le(value);
        Ok(())
    }
    
    /// IO_DEBUG_test_unsigned
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: DBG
    #[inline(always)]
    pub fn io_debug_test_unsigned(&self) -> u8 {
        self.io_debug_test_unsigned_raw()
    }
    
    /// Get raw value of IO_DEBUG_test_unsigned
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn io_debug_test_unsigned_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of IO_DEBUG_test_unsigned
    #[inline(always)]
    pub fn set_io_debug_test_unsigned(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: IoDebug::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: IoDebug::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for IoDebug {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 4 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 4];
        raw.copy_from_slice(&payload[..4]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for IoDebug {
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
/// Defined values for IO_DEBUG_test_enum
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
pub enum IoDebugIoDebugTestEnum {
    IoDebugTest2EnumTwo,
    IoDebugTest2EnumOne,
    _Other(u8),
}

impl From<IoDebugIoDebugTestEnum> for u8 {
    fn from(val: IoDebugIoDebugTestEnum) -> u8 {
        match val {
            IoDebugIoDebugTestEnum::IoDebugTest2EnumTwo => 2,
            IoDebugIoDebugTestEnum::IoDebugTest2EnumOne => 1,
            IoDebugIoDebugTestEnum::_Other(x) => x,
        }
    }
}


/// MOTOR_CMD
///
/// - Standard ID: 101 (0x65)
/// - Size: 1 bytes
/// - Transmitter: DRIVER
#[derive(Clone, Copy)]
pub struct MotorCmd {
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
impl MotorCmd {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x65)});
    
    pub const MOTOR_CMD_DRIVE_MIN: u8 = 0_u8;
    pub const MOTOR_CMD_DRIVE_MAX: u8 = 9_u8;
    pub const MOTOR_CMD_STEER_MIN: i8 = -5_i8;
    pub const MOTOR_CMD_STEER_MAX: i8 = 5_i8;
    
    /// Construct new MOTOR_CMD from values
    pub fn new(motor_cmd_drive: u8, motor_cmd_steer: i8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_motor_cmd_drive(motor_cmd_drive)?;
        res.set_motor_cmd_steer(motor_cmd_steer)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// MOTOR_CMD_drive
    ///
    /// - Min: 0
    /// - Max: 9
    /// - Unit: ""
    /// - Receivers: MOTOR
    #[inline(always)]
    pub fn motor_cmd_drive(&self) -> u8 {
        self.motor_cmd_drive_raw()
    }
    
    /// Get raw value of MOTOR_CMD_drive
    ///
    /// - Start bit: 4
    /// - Signal size: 4 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn motor_cmd_drive_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[4..8].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of MOTOR_CMD_drive
    #[inline(always)]
    pub fn set_motor_cmd_drive(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 9_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: MotorCmd::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: MotorCmd::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[4..8].store_le(value);
        Ok(())
    }
    
    /// MOTOR_CMD_steer
    ///
    /// - Min: -5
    /// - Max: 5
    /// - Unit: ""
    /// - Receivers: MOTOR
    #[inline(always)]
    pub fn motor_cmd_steer(&self) -> i8 {
        self.motor_cmd_steer_raw()
    }
    
    /// Get raw value of MOTOR_CMD_steer
    ///
    /// - Start bit: 0
    /// - Signal size: 4 bits
    /// - Factor: 1
    /// - Offset: -5
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn motor_cmd_steer_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..4].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_sub(5)
    }
    
    /// Set value of MOTOR_CMD_steer
    #[inline(always)]
    pub fn set_motor_cmd_steer(&mut self, value: i8) -> Result<(), CanError> {
        if value < -5_i8 || 5_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: MotorCmd::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_add(5)
            .ok_or(CanError::ParameterOutOfRange { message_id: MotorCmd::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..4].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for MotorCmd {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for MotorCmd {
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

/// MOTOR_STATUS
///
/// - Standard ID: 400 (0x190)
/// - Size: 3 bytes
/// - Transmitter: MOTOR
#[derive(Clone, Copy)]
pub struct MotorStatus {
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
impl MotorStatus {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x190)});
    
    pub const MOTOR_STATUS_SPEED_KPH_MIN: f32 = 0_f32;
    pub const MOTOR_STATUS_SPEED_KPH_MAX: f32 = 0_f32;
    
    /// Construct new MOTOR_STATUS from values
    pub fn new(motor_status_speed_kph: f32, motor_status_wheel_error: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 3] };
        res.set_motor_status_speed_kph(motor_status_speed_kph)?;
        res.set_motor_status_wheel_error(motor_status_wheel_error)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 3] {
        &self.raw
    }
    
    /// MOTOR_STATUS_speed_kph
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "kph"
    /// - Receivers: DRIVER, IO
    #[inline(always)]
    pub fn motor_status_speed_kph(&self) -> f32 {
        self.motor_status_speed_kph_raw()
    }
    
    /// Get raw value of MOTOR_STATUS_speed_kph
    ///
    /// - Start bit: 8
    /// - Signal size: 16 bits
    /// - Factor: 0.001
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn motor_status_speed_kph_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[8..24].load_le::<u16>();
        
        let factor = 0.001_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of MOTOR_STATUS_speed_kph
    #[inline(always)]
    pub fn set_motor_status_speed_kph(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: MotorStatus::MESSAGE_ID });
        }
        let factor = 0.001_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[8..24].store_le(value);
        Ok(())
    }
    
    /// MOTOR_STATUS_wheel_error
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: DRIVER, IO
    #[inline(always)]
    pub fn motor_status_wheel_error(&self) -> bool {
        self.motor_status_wheel_error_raw()
    }
    
    /// Get raw value of MOTOR_STATUS_wheel_error
    ///
    /// - Start bit: 0
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn motor_status_wheel_error_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[0..1].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of MOTOR_STATUS_wheel_error
    #[inline(always)]
    pub fn set_motor_status_wheel_error(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[0..1].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for MotorStatus {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 3 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 3];
        raw.copy_from_slice(&payload[..3]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for MotorStatus {
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

/// SENSOR_SONARS
///
/// - Standard ID: 200 (0xc8)
/// - Size: 8 bytes
/// - Transmitter: SENSOR
#[derive(Clone, Copy)]
pub struct SensorSonars {
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
impl SensorSonars {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0xc8)});
    
    pub const SENSOR_SONARS_ERR_COUNT_MIN: u16 = 0_u16;
    pub const SENSOR_SONARS_ERR_COUNT_MAX: u16 = 0_u16;
    pub const SENSOR_SONARS_LEFT_MIN: f32 = 0_f32;
    pub const SENSOR_SONARS_LEFT_MAX: f32 = 0_f32;
    pub const SENSOR_SONARS_MIDDLE_MIN: f32 = 0_f32;
    pub const SENSOR_SONARS_MIDDLE_MAX: f32 = 0_f32;
    pub const SENSOR_SONARS_MUX_MIN: u8 = 0_u8;
    pub const SENSOR_SONARS_MUX_MAX: u8 = 0_u8;
    pub const SENSOR_SONARS_NO_FILT_LEFT_MIN: f32 = 0_f32;
    pub const SENSOR_SONARS_NO_FILT_LEFT_MAX: f32 = 0_f32;
    pub const SENSOR_SONARS_NO_FILT_MIDDLE_MIN: f32 = 0_f32;
    pub const SENSOR_SONARS_NO_FILT_MIDDLE_MAX: f32 = 0_f32;
    pub const SENSOR_SONARS_NO_FILT_REAR_MIN: f32 = 0_f32;
    pub const SENSOR_SONARS_NO_FILT_REAR_MAX: f32 = 0_f32;
    pub const SENSOR_SONARS_NO_FILT_RIGHT_MIN: f32 = 0_f32;
    pub const SENSOR_SONARS_NO_FILT_RIGHT_MAX: f32 = 0_f32;
    pub const SENSOR_SONARS_REAR_MIN: f32 = 0_f32;
    pub const SENSOR_SONARS_REAR_MAX: f32 = 0_f32;
    pub const SENSOR_SONARS_RIGHT_MIN: f32 = 0_f32;
    pub const SENSOR_SONARS_RIGHT_MAX: f32 = 0_f32;
    
    /// Construct new SENSOR_SONARS from values
    pub fn new(sensor_sonars_err_count: u16, sensor_sonars_mux: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_sensor_sonars_err_count(sensor_sonars_err_count)?;
        res.set_sensor_sonars_mux(sensor_sonars_mux)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// SENSOR_SONARS_err_count
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: DRIVER, IO
    #[inline(always)]
    pub fn sensor_sonars_err_count(&self) -> u16 {
        self.sensor_sonars_err_count_raw()
    }
    
    /// Get raw value of SENSOR_SONARS_err_count
    ///
    /// - Start bit: 4
    /// - Signal size: 12 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn sensor_sonars_err_count_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[4..16].load_le::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of SENSOR_SONARS_err_count
    #[inline(always)]
    pub fn set_sensor_sonars_err_count(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: SensorSonars::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: SensorSonars::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[4..16].store_le(value);
        Ok(())
    }
    
    /// Get raw value of SENSOR_SONARS_mux
    ///
    /// - Start bit: 0
    /// - Signal size: 4 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn sensor_sonars_mux_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..4].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    pub fn sensor_sonars_mux(&mut self) -> Result<SensorSonarsSensorSonarsMuxIndex, CanError> {
        match self.sensor_sonars_mux_raw() {
            0 => Ok(SensorSonarsSensorSonarsMuxIndex::M0(SensorSonarsSensorSonarsMuxM0{ raw: self.raw })),
            1 => Ok(SensorSonarsSensorSonarsMuxIndex::M1(SensorSonarsSensorSonarsMuxM1{ raw: self.raw })),
            multiplexor => Err(CanError::InvalidMultiplexor { message_id: SensorSonars::MESSAGE_ID, multiplexor: multiplexor.into() }),
        }
    }
    /// Set value of SENSOR_SONARS_mux
    #[inline(always)]
    fn set_sensor_sonars_mux(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: SensorSonars::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: SensorSonars::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..4].store_le(value);
        Ok(())
    }
    
    /// Set value of SENSOR_SONARS_mux
    #[inline(always)]
    pub fn set_m0(&mut self, value: SensorSonarsSensorSonarsMuxM0) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_sensor_sonars_mux(0)?;
        Ok(())
    }
    
    /// Set value of SENSOR_SONARS_mux
    #[inline(always)]
    pub fn set_m1(&mut self, value: SensorSonarsSensorSonarsMuxM1) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_sensor_sonars_mux(1)?;
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for SensorSonars {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for SensorSonars {
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
/// Defined values for multiplexed signal SENSOR_SONARS
#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
pub enum SensorSonarsSensorSonarsMuxIndex {
    M0(SensorSonarsSensorSonarsMuxM0),
    M1(SensorSonarsSensorSonarsMuxM1),
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
pub struct SensorSonarsSensorSonarsMuxM0 { raw: [u8; 8] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl SensorSonarsSensorSonarsMuxM0 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// SENSOR_SONARS_left
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: DRIVER, IO
#[inline(always)]
pub fn sensor_sonars_left(&self) -> f32 {
    self.sensor_sonars_left_raw()
}

/// Get raw value of SENSOR_SONARS_left
///
/// - Start bit: 16
/// - Signal size: 12 bits
/// - Factor: 0.1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn sensor_sonars_left_raw(&self) -> f32 {
    let signal = self.raw.view_bits::<Lsb0>()[16..28].load_le::<u16>();
    
    let factor = 0.1_f32;
    let offset = 0_f32;
    (signal as f32) * factor + offset
}

/// Set value of SENSOR_SONARS_left
#[inline(always)]
pub fn set_sensor_sonars_left(&mut self, value: f32) -> Result<(), CanError> {
    if value < 0_f32 || 0_f32 < value {
        return Err(CanError::ParameterOutOfRange { message_id: SensorSonars::MESSAGE_ID });
    }
    let factor = 0.1_f32;
    let offset = 0_f32;
    let value = ((value - offset) / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..28].store_le(value);
    Ok(())
}

/// SENSOR_SONARS_middle
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: DRIVER, IO
#[inline(always)]
pub fn sensor_sonars_middle(&self) -> f32 {
    self.sensor_sonars_middle_raw()
}

/// Get raw value of SENSOR_SONARS_middle
///
/// - Start bit: 28
/// - Signal size: 12 bits
/// - Factor: 0.1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn sensor_sonars_middle_raw(&self) -> f32 {
    let signal = self.raw.view_bits::<Lsb0>()[28..40].load_le::<u16>();
    
    let factor = 0.1_f32;
    let offset = 0_f32;
    (signal as f32) * factor + offset
}

/// Set value of SENSOR_SONARS_middle
#[inline(always)]
pub fn set_sensor_sonars_middle(&mut self, value: f32) -> Result<(), CanError> {
    if value < 0_f32 || 0_f32 < value {
        return Err(CanError::ParameterOutOfRange { message_id: SensorSonars::MESSAGE_ID });
    }
    let factor = 0.1_f32;
    let offset = 0_f32;
    let value = ((value - offset) / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[28..40].store_le(value);
    Ok(())
}

/// SENSOR_SONARS_rear
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: DRIVER, IO
#[inline(always)]
pub fn sensor_sonars_rear(&self) -> f32 {
    self.sensor_sonars_rear_raw()
}

/// Get raw value of SENSOR_SONARS_rear
///
/// - Start bit: 52
/// - Signal size: 12 bits
/// - Factor: 0.1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn sensor_sonars_rear_raw(&self) -> f32 {
    let signal = self.raw.view_bits::<Lsb0>()[52..64].load_le::<u16>();
    
    let factor = 0.1_f32;
    let offset = 0_f32;
    (signal as f32) * factor + offset
}

/// Set value of SENSOR_SONARS_rear
#[inline(always)]
pub fn set_sensor_sonars_rear(&mut self, value: f32) -> Result<(), CanError> {
    if value < 0_f32 || 0_f32 < value {
        return Err(CanError::ParameterOutOfRange { message_id: SensorSonars::MESSAGE_ID });
    }
    let factor = 0.1_f32;
    let offset = 0_f32;
    let value = ((value - offset) / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[52..64].store_le(value);
    Ok(())
}

/// SENSOR_SONARS_right
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: DRIVER, IO
#[inline(always)]
pub fn sensor_sonars_right(&self) -> f32 {
    self.sensor_sonars_right_raw()
}

/// Get raw value of SENSOR_SONARS_right
///
/// - Start bit: 40
/// - Signal size: 12 bits
/// - Factor: 0.1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn sensor_sonars_right_raw(&self) -> f32 {
    let signal = self.raw.view_bits::<Lsb0>()[40..52].load_le::<u16>();
    
    let factor = 0.1_f32;
    let offset = 0_f32;
    (signal as f32) * factor + offset
}

/// Set value of SENSOR_SONARS_right
#[inline(always)]
pub fn set_sensor_sonars_right(&mut self, value: f32) -> Result<(), CanError> {
    if value < 0_f32 || 0_f32 < value {
        return Err(CanError::ParameterOutOfRange { message_id: SensorSonars::MESSAGE_ID });
    }
    let factor = 0.1_f32;
    let offset = 0_f32;
    let value = ((value - offset) / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[40..52].store_le(value);
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
pub struct SensorSonarsSensorSonarsMuxM1 { raw: [u8; 8] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl SensorSonarsSensorSonarsMuxM1 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// SENSOR_SONARS_no_filt_left
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: DBG
#[inline(always)]
pub fn sensor_sonars_no_filt_left(&self) -> f32 {
    self.sensor_sonars_no_filt_left_raw()
}

/// Get raw value of SENSOR_SONARS_no_filt_left
///
/// - Start bit: 16
/// - Signal size: 12 bits
/// - Factor: 0.1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn sensor_sonars_no_filt_left_raw(&self) -> f32 {
    let signal = self.raw.view_bits::<Lsb0>()[16..28].load_le::<u16>();
    
    let factor = 0.1_f32;
    let offset = 0_f32;
    (signal as f32) * factor + offset
}

/// Set value of SENSOR_SONARS_no_filt_left
#[inline(always)]
pub fn set_sensor_sonars_no_filt_left(&mut self, value: f32) -> Result<(), CanError> {
    if value < 0_f32 || 0_f32 < value {
        return Err(CanError::ParameterOutOfRange { message_id: SensorSonars::MESSAGE_ID });
    }
    let factor = 0.1_f32;
    let offset = 0_f32;
    let value = ((value - offset) / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..28].store_le(value);
    Ok(())
}

/// SENSOR_SONARS_no_filt_middle
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: DBG
#[inline(always)]
pub fn sensor_sonars_no_filt_middle(&self) -> f32 {
    self.sensor_sonars_no_filt_middle_raw()
}

/// Get raw value of SENSOR_SONARS_no_filt_middle
///
/// - Start bit: 28
/// - Signal size: 12 bits
/// - Factor: 0.1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn sensor_sonars_no_filt_middle_raw(&self) -> f32 {
    let signal = self.raw.view_bits::<Lsb0>()[28..40].load_le::<u16>();
    
    let factor = 0.1_f32;
    let offset = 0_f32;
    (signal as f32) * factor + offset
}

/// Set value of SENSOR_SONARS_no_filt_middle
#[inline(always)]
pub fn set_sensor_sonars_no_filt_middle(&mut self, value: f32) -> Result<(), CanError> {
    if value < 0_f32 || 0_f32 < value {
        return Err(CanError::ParameterOutOfRange { message_id: SensorSonars::MESSAGE_ID });
    }
    let factor = 0.1_f32;
    let offset = 0_f32;
    let value = ((value - offset) / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[28..40].store_le(value);
    Ok(())
}

/// SENSOR_SONARS_no_filt_rear
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: DBG
#[inline(always)]
pub fn sensor_sonars_no_filt_rear(&self) -> f32 {
    self.sensor_sonars_no_filt_rear_raw()
}

/// Get raw value of SENSOR_SONARS_no_filt_rear
///
/// - Start bit: 52
/// - Signal size: 12 bits
/// - Factor: 0.1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn sensor_sonars_no_filt_rear_raw(&self) -> f32 {
    let signal = self.raw.view_bits::<Lsb0>()[52..64].load_le::<u16>();
    
    let factor = 0.1_f32;
    let offset = 0_f32;
    (signal as f32) * factor + offset
}

/// Set value of SENSOR_SONARS_no_filt_rear
#[inline(always)]
pub fn set_sensor_sonars_no_filt_rear(&mut self, value: f32) -> Result<(), CanError> {
    if value < 0_f32 || 0_f32 < value {
        return Err(CanError::ParameterOutOfRange { message_id: SensorSonars::MESSAGE_ID });
    }
    let factor = 0.1_f32;
    let offset = 0_f32;
    let value = ((value - offset) / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[52..64].store_le(value);
    Ok(())
}

/// SENSOR_SONARS_no_filt_right
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: DBG
#[inline(always)]
pub fn sensor_sonars_no_filt_right(&self) -> f32 {
    self.sensor_sonars_no_filt_right_raw()
}

/// Get raw value of SENSOR_SONARS_no_filt_right
///
/// - Start bit: 40
/// - Signal size: 12 bits
/// - Factor: 0.1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn sensor_sonars_no_filt_right_raw(&self) -> f32 {
    let signal = self.raw.view_bits::<Lsb0>()[40..52].load_le::<u16>();
    
    let factor = 0.1_f32;
    let offset = 0_f32;
    (signal as f32) * factor + offset
}

/// Set value of SENSOR_SONARS_no_filt_right
#[inline(always)]
pub fn set_sensor_sonars_no_filt_right(&mut self, value: f32) -> Result<(), CanError> {
    if value < 0_f32 || 0_f32 < value {
        return Err(CanError::ParameterOutOfRange { message_id: SensorSonars::MESSAGE_ID });
    }
    let factor = 0.1_f32;
    let offset = 0_f32;
    let value = ((value - offset) / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[40..52].store_le(value);
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

