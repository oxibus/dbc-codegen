// Generated code!
//
// Message definitions from file `msxii_system_can`
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
    /// BATTERY_VT
    BatteryVt(BatteryVt),
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
            BatteryVt::MESSAGE_ID => Messages::BatteryVt(BatteryVt::try_from(payload)?),
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}

/// BATTERY_VT
///
/// - Standard ID: 1025 (0x401)
/// - Size: 6 bytes
/// - Transmitter: BMS
#[derive(Clone, Copy)]
pub struct BatteryVt {
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
impl BatteryVt {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x401)});
    
    pub const MODULE_TEMP_35_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_35_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_34_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_34_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_33_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_33_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_32_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_32_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_31_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_31_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_30_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_30_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_29_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_29_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_28_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_28_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_27_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_27_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_26_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_26_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_25_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_25_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_24_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_24_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_23_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_23_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_22_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_22_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_21_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_21_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_20_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_20_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_19_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_19_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_18_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_18_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_17_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_17_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_16_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_16_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_15_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_15_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_14_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_14_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_13_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_13_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_12_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_12_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_11_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_11_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_10_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_10_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_09_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_09_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_08_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_08_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_07_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_07_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_06_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_06_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_05_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_05_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_04_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_04_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_03_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_03_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_02_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_02_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_01_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_01_MAX: u16 = 0_u16;
    pub const MODULE_TEMP_00_MIN: u16 = 0_u16;
    pub const MODULE_TEMP_00_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_35_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_35_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_34_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_34_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_33_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_33_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_32_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_32_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_31_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_31_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_30_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_30_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_29_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_29_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_28_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_28_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_27_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_27_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_26_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_26_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_25_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_25_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_24_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_24_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_23_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_23_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_22_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_22_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_21_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_21_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_20_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_20_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_19_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_19_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_18_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_18_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_17_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_17_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_16_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_16_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_15_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_15_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_14_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_14_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_13_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_13_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_12_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_12_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_11_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_11_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_10_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_10_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_09_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_09_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_08_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_08_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_07_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_07_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_06_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_06_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_05_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_05_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_04_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_04_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_03_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_03_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_02_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_02_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_01_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_01_MAX: u16 = 0_u16;
    pub const MODULE_VOLTAGE_00_MIN: u16 = 0_u16;
    pub const MODULE_VOLTAGE_00_MAX: u16 = 0_u16;
    pub const BATTERY_VT_INDEX_MIN: u16 = 0_u16;
    pub const BATTERY_VT_INDEX_MAX: u16 = 0_u16;
    
    /// Construct new BATTERY_VT from values
    pub fn new(battery_vt_index: u16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 6] };
        res.set_battery_vt_index(battery_vt_index)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 6] {
        &self.raw
    }
    
    /// Get raw value of BATTERY_VT_INDEX
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn battery_vt_index_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    pub fn battery_vt_index(&mut self) -> Result<BatteryVtBatteryVtIndexIndex, CanError> {
        match self.battery_vt_index_raw() {
            0 => Ok(BatteryVtBatteryVtIndexIndex::M0(BatteryVtBatteryVtIndexM0{ raw: self.raw })),
            1 => Ok(BatteryVtBatteryVtIndexIndex::M1(BatteryVtBatteryVtIndexM1{ raw: self.raw })),
            2 => Ok(BatteryVtBatteryVtIndexIndex::M2(BatteryVtBatteryVtIndexM2{ raw: self.raw })),
            3 => Ok(BatteryVtBatteryVtIndexIndex::M3(BatteryVtBatteryVtIndexM3{ raw: self.raw })),
            4 => Ok(BatteryVtBatteryVtIndexIndex::M4(BatteryVtBatteryVtIndexM4{ raw: self.raw })),
            5 => Ok(BatteryVtBatteryVtIndexIndex::M5(BatteryVtBatteryVtIndexM5{ raw: self.raw })),
            6 => Ok(BatteryVtBatteryVtIndexIndex::M6(BatteryVtBatteryVtIndexM6{ raw: self.raw })),
            7 => Ok(BatteryVtBatteryVtIndexIndex::M7(BatteryVtBatteryVtIndexM7{ raw: self.raw })),
            8 => Ok(BatteryVtBatteryVtIndexIndex::M8(BatteryVtBatteryVtIndexM8{ raw: self.raw })),
            9 => Ok(BatteryVtBatteryVtIndexIndex::M9(BatteryVtBatteryVtIndexM9{ raw: self.raw })),
            10 => Ok(BatteryVtBatteryVtIndexIndex::M10(BatteryVtBatteryVtIndexM10{ raw: self.raw })),
            11 => Ok(BatteryVtBatteryVtIndexIndex::M11(BatteryVtBatteryVtIndexM11{ raw: self.raw })),
            12 => Ok(BatteryVtBatteryVtIndexIndex::M12(BatteryVtBatteryVtIndexM12{ raw: self.raw })),
            13 => Ok(BatteryVtBatteryVtIndexIndex::M13(BatteryVtBatteryVtIndexM13{ raw: self.raw })),
            14 => Ok(BatteryVtBatteryVtIndexIndex::M14(BatteryVtBatteryVtIndexM14{ raw: self.raw })),
            15 => Ok(BatteryVtBatteryVtIndexIndex::M15(BatteryVtBatteryVtIndexM15{ raw: self.raw })),
            16 => Ok(BatteryVtBatteryVtIndexIndex::M16(BatteryVtBatteryVtIndexM16{ raw: self.raw })),
            17 => Ok(BatteryVtBatteryVtIndexIndex::M17(BatteryVtBatteryVtIndexM17{ raw: self.raw })),
            18 => Ok(BatteryVtBatteryVtIndexIndex::M18(BatteryVtBatteryVtIndexM18{ raw: self.raw })),
            19 => Ok(BatteryVtBatteryVtIndexIndex::M19(BatteryVtBatteryVtIndexM19{ raw: self.raw })),
            20 => Ok(BatteryVtBatteryVtIndexIndex::M20(BatteryVtBatteryVtIndexM20{ raw: self.raw })),
            21 => Ok(BatteryVtBatteryVtIndexIndex::M21(BatteryVtBatteryVtIndexM21{ raw: self.raw })),
            22 => Ok(BatteryVtBatteryVtIndexIndex::M22(BatteryVtBatteryVtIndexM22{ raw: self.raw })),
            23 => Ok(BatteryVtBatteryVtIndexIndex::M23(BatteryVtBatteryVtIndexM23{ raw: self.raw })),
            24 => Ok(BatteryVtBatteryVtIndexIndex::M24(BatteryVtBatteryVtIndexM24{ raw: self.raw })),
            25 => Ok(BatteryVtBatteryVtIndexIndex::M25(BatteryVtBatteryVtIndexM25{ raw: self.raw })),
            26 => Ok(BatteryVtBatteryVtIndexIndex::M26(BatteryVtBatteryVtIndexM26{ raw: self.raw })),
            27 => Ok(BatteryVtBatteryVtIndexIndex::M27(BatteryVtBatteryVtIndexM27{ raw: self.raw })),
            28 => Ok(BatteryVtBatteryVtIndexIndex::M28(BatteryVtBatteryVtIndexM28{ raw: self.raw })),
            29 => Ok(BatteryVtBatteryVtIndexIndex::M29(BatteryVtBatteryVtIndexM29{ raw: self.raw })),
            30 => Ok(BatteryVtBatteryVtIndexIndex::M30(BatteryVtBatteryVtIndexM30{ raw: self.raw })),
            31 => Ok(BatteryVtBatteryVtIndexIndex::M31(BatteryVtBatteryVtIndexM31{ raw: self.raw })),
            32 => Ok(BatteryVtBatteryVtIndexIndex::M32(BatteryVtBatteryVtIndexM32{ raw: self.raw })),
            33 => Ok(BatteryVtBatteryVtIndexIndex::M33(BatteryVtBatteryVtIndexM33{ raw: self.raw })),
            34 => Ok(BatteryVtBatteryVtIndexIndex::M34(BatteryVtBatteryVtIndexM34{ raw: self.raw })),
            35 => Ok(BatteryVtBatteryVtIndexIndex::M35(BatteryVtBatteryVtIndexM35{ raw: self.raw })),
            multiplexor => Err(CanError::InvalidMultiplexor { message_id: BatteryVt::MESSAGE_ID, multiplexor: multiplexor.into() }),
        }
    }
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    fn set_battery_vt_index(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m0(&mut self, value: BatteryVtBatteryVtIndexM0) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(0)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m1(&mut self, value: BatteryVtBatteryVtIndexM1) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(1)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m2(&mut self, value: BatteryVtBatteryVtIndexM2) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(2)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m3(&mut self, value: BatteryVtBatteryVtIndexM3) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(3)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m4(&mut self, value: BatteryVtBatteryVtIndexM4) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(4)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m5(&mut self, value: BatteryVtBatteryVtIndexM5) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(5)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m6(&mut self, value: BatteryVtBatteryVtIndexM6) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(6)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m7(&mut self, value: BatteryVtBatteryVtIndexM7) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(7)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m8(&mut self, value: BatteryVtBatteryVtIndexM8) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(8)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m9(&mut self, value: BatteryVtBatteryVtIndexM9) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(9)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m10(&mut self, value: BatteryVtBatteryVtIndexM10) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(10)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m11(&mut self, value: BatteryVtBatteryVtIndexM11) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(11)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m12(&mut self, value: BatteryVtBatteryVtIndexM12) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(12)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m13(&mut self, value: BatteryVtBatteryVtIndexM13) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(13)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m14(&mut self, value: BatteryVtBatteryVtIndexM14) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(14)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m15(&mut self, value: BatteryVtBatteryVtIndexM15) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(15)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m16(&mut self, value: BatteryVtBatteryVtIndexM16) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(16)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m17(&mut self, value: BatteryVtBatteryVtIndexM17) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(17)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m18(&mut self, value: BatteryVtBatteryVtIndexM18) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(18)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m19(&mut self, value: BatteryVtBatteryVtIndexM19) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(19)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m20(&mut self, value: BatteryVtBatteryVtIndexM20) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(20)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m21(&mut self, value: BatteryVtBatteryVtIndexM21) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(21)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m22(&mut self, value: BatteryVtBatteryVtIndexM22) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(22)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m23(&mut self, value: BatteryVtBatteryVtIndexM23) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(23)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m24(&mut self, value: BatteryVtBatteryVtIndexM24) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(24)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m25(&mut self, value: BatteryVtBatteryVtIndexM25) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(25)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m26(&mut self, value: BatteryVtBatteryVtIndexM26) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(26)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m27(&mut self, value: BatteryVtBatteryVtIndexM27) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(27)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m28(&mut self, value: BatteryVtBatteryVtIndexM28) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(28)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m29(&mut self, value: BatteryVtBatteryVtIndexM29) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(29)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m30(&mut self, value: BatteryVtBatteryVtIndexM30) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(30)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m31(&mut self, value: BatteryVtBatteryVtIndexM31) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(31)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m32(&mut self, value: BatteryVtBatteryVtIndexM32) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(32)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m33(&mut self, value: BatteryVtBatteryVtIndexM33) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(33)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m34(&mut self, value: BatteryVtBatteryVtIndexM34) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(34)?;
        Ok(())
    }
    
    /// Set value of BATTERY_VT_INDEX
    #[inline(always)]
    pub fn set_m35(&mut self, value: BatteryVtBatteryVtIndexM35) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_battery_vt_index(35)?;
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for BatteryVt {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 6 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 6];
        raw.copy_from_slice(&payload[..6]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for BatteryVt {
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
/// Defined values for multiplexed signal BATTERY_VT
#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
pub enum BatteryVtBatteryVtIndexIndex {
    M0(BatteryVtBatteryVtIndexM0),
    M1(BatteryVtBatteryVtIndexM1),
    M2(BatteryVtBatteryVtIndexM2),
    M3(BatteryVtBatteryVtIndexM3),
    M4(BatteryVtBatteryVtIndexM4),
    M5(BatteryVtBatteryVtIndexM5),
    M6(BatteryVtBatteryVtIndexM6),
    M7(BatteryVtBatteryVtIndexM7),
    M8(BatteryVtBatteryVtIndexM8),
    M9(BatteryVtBatteryVtIndexM9),
    M10(BatteryVtBatteryVtIndexM10),
    M11(BatteryVtBatteryVtIndexM11),
    M12(BatteryVtBatteryVtIndexM12),
    M13(BatteryVtBatteryVtIndexM13),
    M14(BatteryVtBatteryVtIndexM14),
    M15(BatteryVtBatteryVtIndexM15),
    M16(BatteryVtBatteryVtIndexM16),
    M17(BatteryVtBatteryVtIndexM17),
    M18(BatteryVtBatteryVtIndexM18),
    M19(BatteryVtBatteryVtIndexM19),
    M20(BatteryVtBatteryVtIndexM20),
    M21(BatteryVtBatteryVtIndexM21),
    M22(BatteryVtBatteryVtIndexM22),
    M23(BatteryVtBatteryVtIndexM23),
    M24(BatteryVtBatteryVtIndexM24),
    M25(BatteryVtBatteryVtIndexM25),
    M26(BatteryVtBatteryVtIndexM26),
    M27(BatteryVtBatteryVtIndexM27),
    M28(BatteryVtBatteryVtIndexM28),
    M29(BatteryVtBatteryVtIndexM29),
    M30(BatteryVtBatteryVtIndexM30),
    M31(BatteryVtBatteryVtIndexM31),
    M32(BatteryVtBatteryVtIndexM32),
    M33(BatteryVtBatteryVtIndexM33),
    M34(BatteryVtBatteryVtIndexM34),
    M35(BatteryVtBatteryVtIndexM35),
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
pub struct BatteryVtBatteryVtIndexM0 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM0 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_00
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_00(&self) -> u16 {
    self.module_temp_00_raw()
}

/// Get raw value of MODULE_TEMP_00
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_00_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_00
#[inline(always)]
pub fn set_module_temp_00(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_00
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_00(&self) -> u16 {
    self.module_voltage_00_raw()
}

/// Get raw value of MODULE_VOLTAGE_00
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_00_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_00
#[inline(always)]
pub fn set_module_voltage_00(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM1 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM1 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_01
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_01(&self) -> u16 {
    self.module_temp_01_raw()
}

/// Get raw value of MODULE_TEMP_01
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_01_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_01
#[inline(always)]
pub fn set_module_temp_01(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_01
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_01(&self) -> u16 {
    self.module_voltage_01_raw()
}

/// Get raw value of MODULE_VOLTAGE_01
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_01_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_01
#[inline(always)]
pub fn set_module_voltage_01(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM2 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM2 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_02
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_02(&self) -> u16 {
    self.module_temp_02_raw()
}

/// Get raw value of MODULE_TEMP_02
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_02_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_02
#[inline(always)]
pub fn set_module_temp_02(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_02
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_02(&self) -> u16 {
    self.module_voltage_02_raw()
}

/// Get raw value of MODULE_VOLTAGE_02
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_02_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_02
#[inline(always)]
pub fn set_module_voltage_02(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM3 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM3 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_03
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_03(&self) -> u16 {
    self.module_temp_03_raw()
}

/// Get raw value of MODULE_TEMP_03
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_03_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_03
#[inline(always)]
pub fn set_module_temp_03(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_03
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_03(&self) -> u16 {
    self.module_voltage_03_raw()
}

/// Get raw value of MODULE_VOLTAGE_03
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_03_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_03
#[inline(always)]
pub fn set_module_voltage_03(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM4 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM4 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_04
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_04(&self) -> u16 {
    self.module_temp_04_raw()
}

/// Get raw value of MODULE_TEMP_04
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_04_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_04
#[inline(always)]
pub fn set_module_temp_04(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_04
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_04(&self) -> u16 {
    self.module_voltage_04_raw()
}

/// Get raw value of MODULE_VOLTAGE_04
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_04_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_04
#[inline(always)]
pub fn set_module_voltage_04(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM5 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM5 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_05
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_05(&self) -> u16 {
    self.module_temp_05_raw()
}

/// Get raw value of MODULE_TEMP_05
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_05_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_05
#[inline(always)]
pub fn set_module_temp_05(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_05
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_05(&self) -> u16 {
    self.module_voltage_05_raw()
}

/// Get raw value of MODULE_VOLTAGE_05
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_05_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_05
#[inline(always)]
pub fn set_module_voltage_05(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM6 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM6 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_06
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_06(&self) -> u16 {
    self.module_temp_06_raw()
}

/// Get raw value of MODULE_TEMP_06
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_06_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_06
#[inline(always)]
pub fn set_module_temp_06(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_06
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_06(&self) -> u16 {
    self.module_voltage_06_raw()
}

/// Get raw value of MODULE_VOLTAGE_06
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_06_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_06
#[inline(always)]
pub fn set_module_voltage_06(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM7 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM7 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_07
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_07(&self) -> u16 {
    self.module_temp_07_raw()
}

/// Get raw value of MODULE_TEMP_07
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_07_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_07
#[inline(always)]
pub fn set_module_temp_07(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_07
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_07(&self) -> u16 {
    self.module_voltage_07_raw()
}

/// Get raw value of MODULE_VOLTAGE_07
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_07_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_07
#[inline(always)]
pub fn set_module_voltage_07(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM8 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM8 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_08
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_08(&self) -> u16 {
    self.module_temp_08_raw()
}

/// Get raw value of MODULE_TEMP_08
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_08_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_08
#[inline(always)]
pub fn set_module_temp_08(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_08
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_08(&self) -> u16 {
    self.module_voltage_08_raw()
}

/// Get raw value of MODULE_VOLTAGE_08
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_08_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_08
#[inline(always)]
pub fn set_module_voltage_08(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM9 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM9 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_09
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_09(&self) -> u16 {
    self.module_temp_09_raw()
}

/// Get raw value of MODULE_TEMP_09
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_09_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_09
#[inline(always)]
pub fn set_module_temp_09(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_09
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_09(&self) -> u16 {
    self.module_voltage_09_raw()
}

/// Get raw value of MODULE_VOLTAGE_09
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_09_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_09
#[inline(always)]
pub fn set_module_voltage_09(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM10 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM10 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_10
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_10(&self) -> u16 {
    self.module_temp_10_raw()
}

/// Get raw value of MODULE_TEMP_10
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_10_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_10
#[inline(always)]
pub fn set_module_temp_10(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_10
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_10(&self) -> u16 {
    self.module_voltage_10_raw()
}

/// Get raw value of MODULE_VOLTAGE_10
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_10_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_10
#[inline(always)]
pub fn set_module_voltage_10(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM11 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM11 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_11
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_11(&self) -> u16 {
    self.module_temp_11_raw()
}

/// Get raw value of MODULE_TEMP_11
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_11_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_11
#[inline(always)]
pub fn set_module_temp_11(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_11
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_11(&self) -> u16 {
    self.module_voltage_11_raw()
}

/// Get raw value of MODULE_VOLTAGE_11
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_11_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_11
#[inline(always)]
pub fn set_module_voltage_11(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM12 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM12 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_12
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_12(&self) -> u16 {
    self.module_temp_12_raw()
}

/// Get raw value of MODULE_TEMP_12
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_12_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_12
#[inline(always)]
pub fn set_module_temp_12(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_12
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_12(&self) -> u16 {
    self.module_voltage_12_raw()
}

/// Get raw value of MODULE_VOLTAGE_12
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_12_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_12
#[inline(always)]
pub fn set_module_voltage_12(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM13 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM13 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_13
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_13(&self) -> u16 {
    self.module_temp_13_raw()
}

/// Get raw value of MODULE_TEMP_13
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_13_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_13
#[inline(always)]
pub fn set_module_temp_13(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_13
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_13(&self) -> u16 {
    self.module_voltage_13_raw()
}

/// Get raw value of MODULE_VOLTAGE_13
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_13_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_13
#[inline(always)]
pub fn set_module_voltage_13(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM14 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM14 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_14
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_14(&self) -> u16 {
    self.module_temp_14_raw()
}

/// Get raw value of MODULE_TEMP_14
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_14_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_14
#[inline(always)]
pub fn set_module_temp_14(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_14
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_14(&self) -> u16 {
    self.module_voltage_14_raw()
}

/// Get raw value of MODULE_VOLTAGE_14
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_14_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_14
#[inline(always)]
pub fn set_module_voltage_14(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM15 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM15 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_15
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_15(&self) -> u16 {
    self.module_temp_15_raw()
}

/// Get raw value of MODULE_TEMP_15
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_15_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_15
#[inline(always)]
pub fn set_module_temp_15(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_15
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_15(&self) -> u16 {
    self.module_voltage_15_raw()
}

/// Get raw value of MODULE_VOLTAGE_15
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_15_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_15
#[inline(always)]
pub fn set_module_voltage_15(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM16 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM16 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_16
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_16(&self) -> u16 {
    self.module_temp_16_raw()
}

/// Get raw value of MODULE_TEMP_16
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_16_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_16
#[inline(always)]
pub fn set_module_temp_16(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_16
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_16(&self) -> u16 {
    self.module_voltage_16_raw()
}

/// Get raw value of MODULE_VOLTAGE_16
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_16_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_16
#[inline(always)]
pub fn set_module_voltage_16(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM17 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM17 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_17
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_17(&self) -> u16 {
    self.module_temp_17_raw()
}

/// Get raw value of MODULE_TEMP_17
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_17_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_17
#[inline(always)]
pub fn set_module_temp_17(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_17
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_17(&self) -> u16 {
    self.module_voltage_17_raw()
}

/// Get raw value of MODULE_VOLTAGE_17
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_17_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_17
#[inline(always)]
pub fn set_module_voltage_17(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM18 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM18 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_18
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_18(&self) -> u16 {
    self.module_temp_18_raw()
}

/// Get raw value of MODULE_TEMP_18
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_18_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_18
#[inline(always)]
pub fn set_module_temp_18(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_18
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_18(&self) -> u16 {
    self.module_voltage_18_raw()
}

/// Get raw value of MODULE_VOLTAGE_18
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_18_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_18
#[inline(always)]
pub fn set_module_voltage_18(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM19 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM19 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_19
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_19(&self) -> u16 {
    self.module_temp_19_raw()
}

/// Get raw value of MODULE_TEMP_19
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_19_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_19
#[inline(always)]
pub fn set_module_temp_19(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_19
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_19(&self) -> u16 {
    self.module_voltage_19_raw()
}

/// Get raw value of MODULE_VOLTAGE_19
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_19_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_19
#[inline(always)]
pub fn set_module_voltage_19(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM20 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM20 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_20
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_20(&self) -> u16 {
    self.module_temp_20_raw()
}

/// Get raw value of MODULE_TEMP_20
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_20_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_20
#[inline(always)]
pub fn set_module_temp_20(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_20
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_20(&self) -> u16 {
    self.module_voltage_20_raw()
}

/// Get raw value of MODULE_VOLTAGE_20
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_20_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_20
#[inline(always)]
pub fn set_module_voltage_20(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM21 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM21 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_21
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_21(&self) -> u16 {
    self.module_temp_21_raw()
}

/// Get raw value of MODULE_TEMP_21
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_21_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_21
#[inline(always)]
pub fn set_module_temp_21(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_21
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_21(&self) -> u16 {
    self.module_voltage_21_raw()
}

/// Get raw value of MODULE_VOLTAGE_21
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_21_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_21
#[inline(always)]
pub fn set_module_voltage_21(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM22 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM22 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_22
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_22(&self) -> u16 {
    self.module_temp_22_raw()
}

/// Get raw value of MODULE_TEMP_22
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_22_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_22
#[inline(always)]
pub fn set_module_temp_22(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_22
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_22(&self) -> u16 {
    self.module_voltage_22_raw()
}

/// Get raw value of MODULE_VOLTAGE_22
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_22_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_22
#[inline(always)]
pub fn set_module_voltage_22(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM23 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM23 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_23
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_23(&self) -> u16 {
    self.module_temp_23_raw()
}

/// Get raw value of MODULE_TEMP_23
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_23_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_23
#[inline(always)]
pub fn set_module_temp_23(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_23
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_23(&self) -> u16 {
    self.module_voltage_23_raw()
}

/// Get raw value of MODULE_VOLTAGE_23
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_23_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_23
#[inline(always)]
pub fn set_module_voltage_23(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM24 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM24 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_24
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_24(&self) -> u16 {
    self.module_temp_24_raw()
}

/// Get raw value of MODULE_TEMP_24
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_24_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_24
#[inline(always)]
pub fn set_module_temp_24(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_24
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_24(&self) -> u16 {
    self.module_voltage_24_raw()
}

/// Get raw value of MODULE_VOLTAGE_24
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_24_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_24
#[inline(always)]
pub fn set_module_voltage_24(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM25 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM25 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_25
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_25(&self) -> u16 {
    self.module_temp_25_raw()
}

/// Get raw value of MODULE_TEMP_25
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_25_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_25
#[inline(always)]
pub fn set_module_temp_25(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_25
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_25(&self) -> u16 {
    self.module_voltage_25_raw()
}

/// Get raw value of MODULE_VOLTAGE_25
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_25_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_25
#[inline(always)]
pub fn set_module_voltage_25(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM26 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM26 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_26
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_26(&self) -> u16 {
    self.module_temp_26_raw()
}

/// Get raw value of MODULE_TEMP_26
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_26_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_26
#[inline(always)]
pub fn set_module_temp_26(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_26
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_26(&self) -> u16 {
    self.module_voltage_26_raw()
}

/// Get raw value of MODULE_VOLTAGE_26
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_26_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_26
#[inline(always)]
pub fn set_module_voltage_26(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM27 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM27 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_27
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_27(&self) -> u16 {
    self.module_temp_27_raw()
}

/// Get raw value of MODULE_TEMP_27
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_27_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_27
#[inline(always)]
pub fn set_module_temp_27(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_27
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_27(&self) -> u16 {
    self.module_voltage_27_raw()
}

/// Get raw value of MODULE_VOLTAGE_27
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_27_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_27
#[inline(always)]
pub fn set_module_voltage_27(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM28 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM28 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_28
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_28(&self) -> u16 {
    self.module_temp_28_raw()
}

/// Get raw value of MODULE_TEMP_28
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_28_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_28
#[inline(always)]
pub fn set_module_temp_28(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_28
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_28(&self) -> u16 {
    self.module_voltage_28_raw()
}

/// Get raw value of MODULE_VOLTAGE_28
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_28_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_28
#[inline(always)]
pub fn set_module_voltage_28(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM29 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM29 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_29
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_29(&self) -> u16 {
    self.module_temp_29_raw()
}

/// Get raw value of MODULE_TEMP_29
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_29_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_29
#[inline(always)]
pub fn set_module_temp_29(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_29
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_29(&self) -> u16 {
    self.module_voltage_29_raw()
}

/// Get raw value of MODULE_VOLTAGE_29
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_29_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_29
#[inline(always)]
pub fn set_module_voltage_29(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM30 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM30 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_30
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_30(&self) -> u16 {
    self.module_temp_30_raw()
}

/// Get raw value of MODULE_TEMP_30
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_30_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_30
#[inline(always)]
pub fn set_module_temp_30(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_30
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_30(&self) -> u16 {
    self.module_voltage_30_raw()
}

/// Get raw value of MODULE_VOLTAGE_30
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_30_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_30
#[inline(always)]
pub fn set_module_voltage_30(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM31 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM31 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_31
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_31(&self) -> u16 {
    self.module_temp_31_raw()
}

/// Get raw value of MODULE_TEMP_31
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_31_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_31
#[inline(always)]
pub fn set_module_temp_31(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_31
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_31(&self) -> u16 {
    self.module_voltage_31_raw()
}

/// Get raw value of MODULE_VOLTAGE_31
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_31_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_31
#[inline(always)]
pub fn set_module_voltage_31(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM32 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM32 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_32
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_32(&self) -> u16 {
    self.module_temp_32_raw()
}

/// Get raw value of MODULE_TEMP_32
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_32_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_32
#[inline(always)]
pub fn set_module_temp_32(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_32
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_32(&self) -> u16 {
    self.module_voltage_32_raw()
}

/// Get raw value of MODULE_VOLTAGE_32
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_32_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_32
#[inline(always)]
pub fn set_module_voltage_32(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM33 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM33 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_33
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_33(&self) -> u16 {
    self.module_temp_33_raw()
}

/// Get raw value of MODULE_TEMP_33
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_33_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_33
#[inline(always)]
pub fn set_module_temp_33(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_33
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_33(&self) -> u16 {
    self.module_voltage_33_raw()
}

/// Get raw value of MODULE_VOLTAGE_33
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_33_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_33
#[inline(always)]
pub fn set_module_voltage_33(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM34 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM34 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_34
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_34(&self) -> u16 {
    self.module_temp_34_raw()
}

/// Get raw value of MODULE_TEMP_34
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_34_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_34
#[inline(always)]
pub fn set_module_temp_34(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_34
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_34(&self) -> u16 {
    self.module_voltage_34_raw()
}

/// Get raw value of MODULE_VOLTAGE_34
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_34_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_34
#[inline(always)]
pub fn set_module_voltage_34(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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
pub struct BatteryVtBatteryVtIndexM35 { raw: [u8; 6] }

#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
impl BatteryVtBatteryVtIndexM35 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// MODULE_TEMP_35
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_temp_35(&self) -> u16 {
    self.module_temp_35_raw()
}

/// Get raw value of MODULE_TEMP_35
///
/// - Start bit: 32
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_temp_35_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_TEMP_35
#[inline(always)]
pub fn set_module_temp_35(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
    Ok(())
}

/// MODULE_VOLTAGE_35
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn module_voltage_35(&self) -> u16 {
    self.module_voltage_35_raw()
}

/// Get raw value of MODULE_VOLTAGE_35
///
/// - Start bit: 16
/// - Signal size: 16 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn module_voltage_35_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of MODULE_VOLTAGE_35
#[inline(always)]
pub fn set_module_voltage_35(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 0_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: BatteryVt::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
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

