// Generated code!
//
// Message definitions from file `open_actuator`
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
    /// ControlCmd
    ControlCmd(ControlCmd),
    /// LimitsCmd
    LimitsCmd(LimitsCmd),
    /// ControlStatus
    ControlStatus(ControlStatus),
    /// SystemStatus
    SystemStatus(SystemStatus),
    /// TorqueSensorData
    TorqueSensorData(TorqueSensorData),
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
            ControlCmd::MESSAGE_ID => Messages::ControlCmd(ControlCmd::try_from(payload)?),
            LimitsCmd::MESSAGE_ID => Messages::LimitsCmd(LimitsCmd::try_from(payload)?),
            ControlStatus::MESSAGE_ID => Messages::ControlStatus(ControlStatus::try_from(payload)?),
            SystemStatus::MESSAGE_ID => Messages::SystemStatus(SystemStatus::try_from(payload)?),
            TorqueSensorData::MESSAGE_ID => Messages::TorqueSensorData(TorqueSensorData::try_from(payload)?),
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}

/// ControlCmd
///
/// - Standard ID: 250 (0xfa)
/// - Size: 7 bytes
/// - Transmitter: Driver
#[derive(Clone, Copy)]
pub struct ControlCmd {
    raw: [u8; 7],
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
impl ControlCmd {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0xfa)});
    
    pub const CRC8_CMD1_MIN: u8 = 0_u8;
    pub const CRC8_CMD1_MAX: u8 = 255_u8;
    pub const COUNTER_CMD1_MIN: u8 = 0_u8;
    pub const COUNTER_CMD1_MAX: u8 = 15_u8;
    pub const TARGET_MOTOR_ID_CMD1_MIN: u8 = 0_u8;
    pub const TARGET_MOTOR_ID_CMD1_MAX: u8 = 3_u8;
    pub const TARGET_MODE_MIN: u8 = 0_u8;
    pub const TARGET_MODE_MAX: u8 = 3_u8;
    pub const POSITION_CMD_64_MIN: f32 = -450_f32;
    pub const POSITION_CMD_64_MAX: f32 = 450_f32;
    pub const TORQUE_COMMAND_8_MIN: f32 = -8_f32;
    pub const TORQUE_COMMAND_8_MAX: f32 = 8_f32;
    pub const TORQUE_CLOSE_LOOP_MAX_32_MIN: f32 = 0_f32;
    pub const TORQUE_CLOSE_LOOP_MAX_32_MAX: f32 = 8_f32;
    
    /// Construct new ControlCmd from values
    pub fn new(crc8_cmd1: u8, counter_cmd1: u8, target_motor_id_cmd1: u8, target_mode: u8, position_cmd_64: f32, torque_command_8: f32, torque_close_loop_max_32: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 7] };
        res.set_crc8_cmd1(crc8_cmd1)?;
        res.set_counter_cmd1(counter_cmd1)?;
        res.set_target_motor_id_cmd1(target_motor_id_cmd1)?;
        res.set_target_mode(target_mode)?;
        res.set_position_cmd_64(position_cmd_64)?;
        res.set_torque_command_8(torque_command_8)?;
        res.set_torque_close_loop_max_32(torque_close_loop_max_32)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 7] {
        &self.raw
    }
    
    /// CRC8_CMD1
    ///
    /// - Min: 0
    /// - Max: 255
    /// - Unit: ""
    /// - Receivers: Actuator
    #[inline(always)]
    pub fn crc8_cmd1(&self) -> u8 {
        self.crc8_cmd1_raw()
    }
    
    /// Get raw value of CRC8_CMD1
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn crc8_cmd1_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of CRC8_CMD1
    #[inline(always)]
    pub fn set_crc8_cmd1(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 255_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: ControlCmd::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: ControlCmd::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
    /// Counter_CMD1
    ///
    /// - Min: 0
    /// - Max: 15
    /// - Unit: ""
    /// - Receivers: Actuator
    #[inline(always)]
    pub fn counter_cmd1(&self) -> u8 {
        self.counter_cmd1_raw()
    }
    
    /// Get raw value of Counter_CMD1
    ///
    /// - Start bit: 48
    /// - Signal size: 4 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn counter_cmd1_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[48..52].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Counter_CMD1
    #[inline(always)]
    pub fn set_counter_cmd1(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 15_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: ControlCmd::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: ControlCmd::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[48..52].store_le(value);
        Ok(())
    }
    
    /// TargetMotorID_CMD1
    ///
    /// - Min: 0
    /// - Max: 3
    /// - Unit: ""
    /// - Receivers: Actuator
    #[inline(always)]
    pub fn target_motor_id_cmd1(&self) -> u8 {
        self.target_motor_id_cmd1_raw()
    }
    
    /// Get raw value of TargetMotorID_CMD1
    ///
    /// - Start bit: 12
    /// - Signal size: 2 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn target_motor_id_cmd1_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[12..14].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of TargetMotorID_CMD1
    #[inline(always)]
    pub fn set_target_motor_id_cmd1(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 3_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: ControlCmd::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: ControlCmd::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[12..14].store_le(value);
        Ok(())
    }
    
    /// TargetMode
    ///
    /// - Min: 0
    /// - Max: 3
    /// - Unit: ""
    /// - Receivers: Actuator
    #[inline(always)]
    pub fn target_mode(&self) -> ControlCmdTargetMode {
        let signal = self.raw.view_bits::<Lsb0>()[8..11].load_le::<u8>();
        
        match signal {
            0 => ControlCmdTargetMode::Off,
            1 => ControlCmdTargetMode::Assist,
            2 => ControlCmdTargetMode::PositionRelative,
            3 => ControlCmdTargetMode::Torque,
            4 => ControlCmdTargetMode::PositionAbsolute,
            _ => ControlCmdTargetMode::_Other(self.target_mode_raw()),
        }
    }
    
    /// Get raw value of TargetMode
    ///
    /// - Start bit: 8
    /// - Signal size: 3 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn target_mode_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..11].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of TargetMode
    #[inline(always)]
    pub fn set_target_mode(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 3_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: ControlCmd::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: ControlCmd::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[8..11].store_le(value);
        Ok(())
    }
    
    /// PositionCmd_64
    ///
    /// Output relative position.
    /// Alternative usage - absolute output position
    /// Factor = 64_const / 200steps / 256microsteps *360deg / FinalGearRatio / GearboxRatio
    ///
    /// - Min: -450
    /// - Max: 450
    /// - Unit: "deg"
    /// - Receivers: Actuator
    #[inline(always)]
    pub fn position_cmd_64(&self) -> f32 {
        self.position_cmd_64_raw()
    }
    
    /// Get raw value of PositionCmd_64
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.0154286
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn position_cmd_64_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<i16>();
        
        let factor = 0.0154286_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of PositionCmd_64
    #[inline(always)]
    pub fn set_position_cmd_64(&mut self, value: f32) -> Result<(), CanError> {
        if value < -450_f32 || 450_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: ControlCmd::MESSAGE_ID });
        }
        let factor = 0.0154286_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// TorqueCommand_8
    ///
    /// Factor: 
    /// 8_const * 1A/1000mA * MotorRatedTorque / MotorRatedCurrent * GearboxRatio * FinalGearRatio
    ///
    /// - Min: -8
    /// - Max: 8
    /// - Unit: "N*m"
    /// - Receivers: Actuator
    #[inline(always)]
    pub fn torque_command_8(&self) -> f32 {
        self.torque_command_8_raw()
    }
    
    /// Get raw value of TorqueCommand_8
    ///
    /// - Start bit: 32
    /// - Signal size: 10 bits
    /// - Factor: 0.0166667
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn torque_command_8_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..42].load_le::<i16>();
        
        let factor = 0.0166667_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of TorqueCommand_8
    #[inline(always)]
    pub fn set_torque_command_8(&mut self, value: f32) -> Result<(), CanError> {
        if value < -8_f32 || 8_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: ControlCmd::MESSAGE_ID });
        }
        let factor = 0.0166667_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..42].store_le(value);
        Ok(())
    }
    
    /// TorqueCloseLoopMax_32
    ///
    /// For TorqueCmd > 0 
    /// Max positive close loop torque on top of TorqueCmd (outward torque) and below 0 (centering torque). 
    /// For TorqueCmd < 0;
    /// Max negative close loop torque on top of TorqueCmd (outward torque) and above 0 (centering torque).
    /// Factor: 
    /// 32_const * 1A/1000mA * MotorRatedTorque / MotorRatedCurrent * GearboxRatio * FinalGearRatio
    ///
    /// - Min: 0
    /// - Max: 8
    /// - Unit: "N*m"
    /// - Receivers: Actuator
    #[inline(always)]
    pub fn torque_close_loop_max_32(&self) -> f32 {
        self.torque_close_loop_max_32_raw()
    }
    
    /// Get raw value of TorqueCloseLoopMax_32
    ///
    /// - Start bit: 42
    /// - Signal size: 6 bits
    /// - Factor: 0.186666
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn torque_close_loop_max_32_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[42..48].load_le::<u8>();
        
        let factor = 0.186666_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of TorqueCloseLoopMax_32
    #[inline(always)]
    pub fn set_torque_close_loop_max_32(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 8_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: ControlCmd::MESSAGE_ID });
        }
        let factor = 0.186666_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[42..48].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for ControlCmd {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 7 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 7];
        raw.copy_from_slice(&payload[..7]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for ControlCmd {
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
/// Defined values for TargetMode
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
pub enum ControlCmdTargetMode {
    Off,
    Assist,
    PositionRelative,
    Torque,
    PositionAbsolute,
    _Other(u8),
}

impl From<ControlCmdTargetMode> for u8 {
    fn from(val: ControlCmdTargetMode) -> u8 {
        match val {
            ControlCmdTargetMode::Off => 0,
            ControlCmdTargetMode::Assist => 1,
            ControlCmdTargetMode::PositionRelative => 2,
            ControlCmdTargetMode::Torque => 3,
            ControlCmdTargetMode::PositionAbsolute => 4,
            ControlCmdTargetMode::_Other(x) => x,
        }
    }
}


/// LimitsCmd
///
/// - Standard ID: 251 (0xfb)
/// - Size: 6 bytes
/// - Transmitter: Driver
#[derive(Clone, Copy)]
pub struct LimitsCmd {
    raw: [u8; 6],
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
impl LimitsCmd {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0xfb)});
    
    pub const CRC8_CMD2_MIN: u8 = 0_u8;
    pub const CRC8_CMD2_MAX: u8 = 255_u8;
    pub const COUNTER_CMD2_MIN: u8 = 0_u8;
    pub const COUNTER_CMD2_MAX: u8 = 15_u8;
    pub const VELOCITY_LIMIT_MIN: u16 = 0_u16;
    pub const VELOCITY_LIMIT_MAX: u16 = 0_u16;
    pub const ACCEL_LIMIT_MIN: u16 = 0_u16;
    pub const ACCEL_LIMIT_MAX: u16 = 0_u16;
    
    /// Construct new LimitsCmd from values
    pub fn new(crc8_cmd2: u8, counter_cmd2: u8, velocity_limit: u16, accel_limit: u16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 6] };
        res.set_crc8_cmd2(crc8_cmd2)?;
        res.set_counter_cmd2(counter_cmd2)?;
        res.set_velocity_limit(velocity_limit)?;
        res.set_accel_limit(accel_limit)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 6] {
        &self.raw
    }
    
    /// CRC8_CMD2
    ///
    /// - Min: 0
    /// - Max: 255
    /// - Unit: ""
    /// - Receivers: Actuator
    #[inline(always)]
    pub fn crc8_cmd2(&self) -> u8 {
        self.crc8_cmd2_raw()
    }
    
    /// Get raw value of CRC8_CMD2
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn crc8_cmd2_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of CRC8_CMD2
    #[inline(always)]
    pub fn set_crc8_cmd2(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 255_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: LimitsCmd::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: LimitsCmd::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
    /// Counter_CMD2
    ///
    /// - Min: 0
    /// - Max: 15
    /// - Unit: ""
    /// - Receivers: Actuator
    #[inline(always)]
    pub fn counter_cmd2(&self) -> u8 {
        self.counter_cmd2_raw()
    }
    
    /// Get raw value of Counter_CMD2
    ///
    /// - Start bit: 12
    /// - Signal size: 4 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn counter_cmd2_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[12..16].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Counter_CMD2
    #[inline(always)]
    pub fn set_counter_cmd2(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 15_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: LimitsCmd::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: LimitsCmd::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[12..16].store_le(value);
        Ok(())
    }
    
    /// VelocityLimit
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Actuator
    #[inline(always)]
    pub fn velocity_limit(&self) -> u16 {
        self.velocity_limit_raw()
    }
    
    /// Get raw value of VelocityLimit
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn velocity_limit_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of VelocityLimit
    #[inline(always)]
    pub fn set_velocity_limit(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: LimitsCmd::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: LimitsCmd::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// AccelLimit
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Actuator
    #[inline(always)]
    pub fn accel_limit(&self) -> u16 {
        self.accel_limit_raw()
    }
    
    /// Get raw value of AccelLimit
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn accel_limit_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of AccelLimit
    #[inline(always)]
    pub fn set_accel_limit(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: LimitsCmd::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: LimitsCmd::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for LimitsCmd {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 6 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 6];
        raw.copy_from_slice(&payload[..6]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for LimitsCmd {
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

/// ControlStatus
///
/// - Standard ID: 252 (0xfc)
/// - Size: 4 bytes
/// - Transmitter: Actuator
#[derive(Clone, Copy)]
pub struct ControlStatus {
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
impl ControlStatus {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0xfc)});
    
    pub const CRC8_STAT1_MIN: u8 = 0_u8;
    pub const CRC8_STAT1_MAX: u8 = 255_u8;
    pub const COUNTER_STAT1_MIN: u8 = 0_u8;
    pub const COUNTER_STAT1_MAX: u8 = 15_u8;
    pub const TORQUE_ACTUAL_MIN: f32 = -8_f32;
    pub const TORQUE_ACTUAL_MAX: f32 = 8_f32;
    pub const TORQUE_CLOSE_LOOP_ACTUAL_MIN: f32 = 0_f32;
    pub const TORQUE_CLOSE_LOOP_ACTUAL_MAX: f32 = 8_f32;
    
    /// Construct new ControlStatus from values
    pub fn new(crc8_stat1: u8, counter_stat1: u8, torque_actual: f32, torque_close_loop_actual: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 4] };
        res.set_crc8_stat1(crc8_stat1)?;
        res.set_counter_stat1(counter_stat1)?;
        res.set_torque_actual(torque_actual)?;
        res.set_torque_close_loop_actual(torque_close_loop_actual)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 4] {
        &self.raw
    }
    
    /// CRC8_STAT1
    ///
    /// - Min: 0
    /// - Max: 255
    /// - Unit: ""
    /// - Receivers: Driver, Actuator
    #[inline(always)]
    pub fn crc8_stat1(&self) -> u8 {
        self.crc8_stat1_raw()
    }
    
    /// Get raw value of CRC8_STAT1
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn crc8_stat1_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of CRC8_STAT1
    #[inline(always)]
    pub fn set_crc8_stat1(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 255_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: ControlStatus::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: ControlStatus::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
    /// Counter_STAT1
    ///
    /// - Min: 0
    /// - Max: 15
    /// - Unit: ""
    /// - Receivers: Driver
    #[inline(always)]
    pub fn counter_stat1(&self) -> u8 {
        self.counter_stat1_raw()
    }
    
    /// Get raw value of Counter_STAT1
    ///
    /// - Start bit: 12
    /// - Signal size: 4 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn counter_stat1_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[12..16].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Counter_STAT1
    #[inline(always)]
    pub fn set_counter_stat1(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 15_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: ControlStatus::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: ControlStatus::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[12..16].store_le(value);
        Ok(())
    }
    
    /// TorqueActual
    ///
    /// - Min: -8
    /// - Max: 8
    /// - Unit: "N*m"
    /// - Receivers: Driver
    #[inline(always)]
    pub fn torque_actual(&self) -> f32 {
        self.torque_actual_raw()
    }
    
    /// Get raw value of TorqueActual
    ///
    /// - Start bit: 16
    /// - Signal size: 10 bits
    /// - Factor: 0.015625
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn torque_actual_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..26].load_le::<i16>();
        
        let factor = 0.015625_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of TorqueActual
    #[inline(always)]
    pub fn set_torque_actual(&mut self, value: f32) -> Result<(), CanError> {
        if value < -8_f32 || 8_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: ControlStatus::MESSAGE_ID });
        }
        let factor = 0.015625_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[16..26].store_le(value);
        Ok(())
    }
    
    /// TorqueCloseLoopActual
    ///
    /// - Min: 0
    /// - Max: 8
    /// - Unit: "N*m"
    /// - Receivers: Driver
    #[inline(always)]
    pub fn torque_close_loop_actual(&self) -> f32 {
        self.torque_close_loop_actual_raw()
    }
    
    /// Get raw value of TorqueCloseLoopActual
    ///
    /// - Start bit: 26
    /// - Signal size: 6 bits
    /// - Factor: 0.125
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn torque_close_loop_actual_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[26..32].load_le::<u8>();
        
        let factor = 0.125_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of TorqueCloseLoopActual
    #[inline(always)]
    pub fn set_torque_close_loop_actual(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 8_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: ControlStatus::MESSAGE_ID });
        }
        let factor = 0.125_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[26..32].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for ControlStatus {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 4 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 4];
        raw.copy_from_slice(&payload[..4]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for ControlStatus {
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

/// SystemStatus
///
/// - Standard ID: 253 (0xfd)
/// - Size: 3 bytes
/// - Transmitter: Actuator
#[derive(Clone, Copy)]
pub struct SystemStatus {
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
impl SystemStatus {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0xfd)});
    
    pub const CRC8_STAT2_MIN: u8 = 0_u8;
    pub const CRC8_STAT2_MAX: u8 = 255_u8;
    pub const COUNTER_STAT2_MIN: u8 = 0_u8;
    pub const COUNTER_STAT2_MAX: u8 = 15_u8;
    pub const CHIP_TEMP_MIN: i16 = -60_i16;
    pub const CHIP_TEMP_MAX: i16 = 195_i16;
    
    /// Construct new SystemStatus from values
    pub fn new(crc8_stat2: u8, counter_stat2: u8, chip_temp: i16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 3] };
        res.set_crc8_stat2(crc8_stat2)?;
        res.set_counter_stat2(counter_stat2)?;
        res.set_chip_temp(chip_temp)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 3] {
        &self.raw
    }
    
    /// CRC8_STAT2
    ///
    /// - Min: 0
    /// - Max: 255
    /// - Unit: ""
    /// - Receivers: Driver
    #[inline(always)]
    pub fn crc8_stat2(&self) -> u8 {
        self.crc8_stat2_raw()
    }
    
    /// Get raw value of CRC8_STAT2
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn crc8_stat2_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of CRC8_STAT2
    #[inline(always)]
    pub fn set_crc8_stat2(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 255_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: SystemStatus::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: SystemStatus::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
    /// Counter_STAT2
    ///
    /// - Min: 0
    /// - Max: 15
    /// - Unit: ""
    /// - Receivers: Driver
    #[inline(always)]
    pub fn counter_stat2(&self) -> u8 {
        self.counter_stat2_raw()
    }
    
    /// Get raw value of Counter_STAT2
    ///
    /// - Start bit: 12
    /// - Signal size: 4 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn counter_stat2_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[12..16].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Counter_STAT2
    #[inline(always)]
    pub fn set_counter_stat2(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 15_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: SystemStatus::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: SystemStatus::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[12..16].store_le(value);
        Ok(())
    }
    
    /// ChipTemp
    ///
    /// - Min: -60
    /// - Max: 195
    /// - Unit: "C"
    /// - Receivers: Driver
    #[inline(always)]
    pub fn chip_temp(&self) -> i16 {
        self.chip_temp_raw()
    }
    
    /// Get raw value of ChipTemp
    ///
    /// - Start bit: 16
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: -60
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn chip_temp_raw(&self) -> i16 {
        let signal = self.raw.view_bits::<Lsb0>()[16..24].load_le::<u8>();
        
        let factor = 1;
        i16::from(signal).saturating_mul(factor).saturating_sub(60)
    }
    
    /// Set value of ChipTemp
    #[inline(always)]
    pub fn set_chip_temp(&mut self, value: i16) -> Result<(), CanError> {
        if value < -60_i16 || 195_i16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: SystemStatus::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_add(60)
            .ok_or(CanError::ParameterOutOfRange { message_id: SystemStatus::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[16..24].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for SystemStatus {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 3 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 3];
        raw.copy_from_slice(&payload[..3]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for SystemStatus {
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

/// TorqueSensorData
///
/// - Standard ID: 113 (0x71)
/// - Size: 3 bytes
/// - Transmitter: Sensor
#[derive(Clone, Copy)]
pub struct TorqueSensorData {
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
impl TorqueSensorData {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x71)});
    
    pub const CRC8_DATA1_MIN: u8 = 0_u8;
    pub const CRC8_DATA1_MAX: u8 = 255_u8;
    pub const COUNTER_DATA1_MIN: u8 = 0_u8;
    pub const COUNTER_DATA1_MAX: u8 = 15_u8;
    pub const TORQUE_SENSE_MIN: f32 = -20_f32;
    pub const TORQUE_SENSE_MAX: f32 = 20_f32;
    
    /// Construct new TorqueSensorData from values
    pub fn new(crc8_data1: u8, counter_data1: u8, torque_sense: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 3] };
        res.set_crc8_data1(crc8_data1)?;
        res.set_counter_data1(counter_data1)?;
        res.set_torque_sense(torque_sense)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 3] {
        &self.raw
    }
    
    /// CRC8_DATA1
    ///
    /// - Min: 0
    /// - Max: 255
    /// - Unit: ""
    /// - Receivers: Driver
    #[inline(always)]
    pub fn crc8_data1(&self) -> u8 {
        self.crc8_data1_raw()
    }
    
    /// Get raw value of CRC8_DATA1
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn crc8_data1_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of CRC8_DATA1
    #[inline(always)]
    pub fn set_crc8_data1(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 255_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: TorqueSensorData::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: TorqueSensorData::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
    /// Counter_DATA1
    ///
    /// - Min: 0
    /// - Max: 15
    /// - Unit: ""
    /// - Receivers: Driver
    #[inline(always)]
    pub fn counter_data1(&self) -> u8 {
        self.counter_data1_raw()
    }
    
    /// Get raw value of Counter_DATA1
    ///
    /// - Start bit: 8
    /// - Signal size: 4 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn counter_data1_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..12].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Counter_DATA1
    #[inline(always)]
    pub fn set_counter_data1(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 15_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: TorqueSensorData::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: TorqueSensorData::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[8..12].store_le(value);
        Ok(())
    }
    
    /// TorqueSense
    ///
    /// Strain gauge torque measured
    ///
    /// - Min: -20
    /// - Max: 20
    /// - Unit: "N*m"
    /// - Receivers: Driver
    #[inline(always)]
    pub fn torque_sense(&self) -> f32 {
        self.torque_sense_raw()
    }
    
    /// Get raw value of TorqueSense
    ///
    /// - Start bit: 12
    /// - Signal size: 12 bits
    /// - Factor: 0.01
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn torque_sense_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[12..24].load_le::<i16>();
        
        let factor = 0.01_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of TorqueSense
    #[inline(always)]
    pub fn set_torque_sense(&mut self, value: f32) -> Result<(), CanError> {
        if value < -20_f32 || 20_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: TorqueSensorData::MESSAGE_ID });
        }
        let factor = 0.01_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[12..24].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for TorqueSensorData {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 3 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 3];
        raw.copy_from_slice(&payload[..3]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for TorqueSensorData {
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

