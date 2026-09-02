/// The name of the DBC file this code was generated from
#[allow(dead_code)]
pub const DBC_FILE_NAME: &str = "test_multiplex_dump";
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
    /// MuxedFrame
    MuxedFrame(MuxedFrame),
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
            MuxedFrame::MESSAGE_ID => {
                Messages::MuxedFrame(MuxedFrame::try_from(payload)?)
            }
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}
/// MuxedFrame
///
/// - Standard ID: 256 (0x100)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct MuxedFrame {
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
impl MuxedFrame {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(
        StandardId::new(0x100).unwrap(),
    );
    pub const MESSAGE_SIZE: usize = 8;
    pub const UNMULTIPLEXED_SIG_MIN: i8 = 0_i8;
    pub const UNMULTIPLEXED_SIG_MAX: i8 = 0_i8;
    pub const MULTIPLEXED_SIG_MIN: i8 = 0_i8;
    pub const MULTIPLEXED_SIG_MAX: i8 = 0_i8;
    pub const MULTIPLEXOR_SIG_MIN: u8 = 0_u8;
    pub const MULTIPLEXOR_SIG_MAX: u8 = 0_u8;
    /// Constructs a new `MuxedFrame` message from values.
    pub fn new(unmultiplexed_sig: i8, multiplexor_sig: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0x00; 8] };
        res.set_unmultiplexed_sig(unmultiplexed_sig)?;
        res.set_multiplexor_sig(multiplexor_sig)?;
        Ok(res)
    }
    /// Returns the raw `MuxedFrame` message payload.
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    /// Returns the value of `UnmultiplexedSig`.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: Not specified
    /// - Receivers: Vector__XXX
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn unmultiplexed_sig(&self) -> i8 {
        self.unmultiplexed_sig_raw_val()
    }
    /// Returns the raw value of `UnmultiplexedSig`.
    ///
    /// - Start bit: 16
    /// - Signal size: 8 bits
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn unmultiplexed_sig_raw_val(&self) -> i8 {
        self.raw.view_bits::<Lsb0>()[16..24].load_le::<i8>()
    }
    /// Sets the raw value of `UnmultiplexedSig`.
    #[inline(always)]
    pub fn set_unmultiplexed_sig_raw_val(&mut self, value: i8) {
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[16..24].store_le(value);
    }
    /// Sets the value of `UnmultiplexedSig`.
    #[inline(always)]
    pub fn set_unmultiplexed_sig(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: MuxedFrame::MESSAGE_ID,
            });
        }
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[16..24].store_le(value);
        Ok(())
    }
    /// Returns the raw value of `MultiplexorSig`.
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn multiplexor_sig_raw_val(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>()
    }
    /// Sets the raw value of `MultiplexorSig`.
    #[allow(dead_code)]
    #[inline(always)]
    fn set_multiplexor_sig_raw_val(&mut self, value: u8) {
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
    }
    /// Selects the active multiplexed sub-message for `MultiplexorSig`.
    pub fn multiplexor_sig_multiplexed(
        &mut self,
    ) -> Result<MuxedFrameMultiplexorSigIndex, CanError> {
        match self.multiplexor_sig_raw_val() {
            42 => {
                Ok(
                    MuxedFrameMultiplexorSigIndex::M42(MuxedFrameMultiplexorSigM42 {
                        raw: self.raw,
                    }),
                )
            }
            multiplexor => {
                Err(CanError::InvalidMultiplexor {
                    message_id: MuxedFrame::MESSAGE_ID,
                    multiplexor: multiplexor.into(),
                })
            }
        }
    }
    /// Sets the value of `MultiplexorSig`.
    #[inline(always)]
    fn set_multiplexor_sig(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: MuxedFrame::MESSAGE_ID,
            });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    /// Sets the value of `MultiplexorSig`.
    #[inline(always)]
    pub fn set_m42(
        &mut self,
        value: MuxedFrameMultiplexorSigM42,
    ) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_multiplexor_sig(42)?;
        Ok(())
    }
}
impl core::convert::TryFrom<&[u8]> for MuxedFrame {
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
impl embedded_can::Frame for MuxedFrame {
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
/// Defined values for multiplexed signal MuxedFrame
#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]
pub enum MuxedFrameMultiplexorSigIndex {
    M42(MuxedFrameMultiplexorSigM42),
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
pub struct MuxedFrameMultiplexorSigM42 {
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
impl MuxedFrameMultiplexorSigM42 {
    pub fn new() -> Self {
        Self { raw: [0u8; 8] }
    }
    /// Returns the value of `MultiplexedSig`.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: Not specified
    /// - Receivers: Vector__XXX
    /// - Factor: 1
    /// - Offset: 0
    #[inline(always)]
    pub fn multiplexed_sig(&self) -> i8 {
        self.multiplexed_sig_raw_val()
    }
    /// Returns the raw value of `MultiplexedSig`.
    ///
    /// - Start bit: 8
    /// - Signal size: 8 bits
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn multiplexed_sig_raw_val(&self) -> i8 {
        self.raw.view_bits::<Lsb0>()[8..16].load_le::<i8>()
    }
    /// Sets the raw value of `MultiplexedSig`.
    #[inline(always)]
    pub fn set_multiplexed_sig_raw_val(&mut self, value: i8) {
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
    }
    /// Sets the value of `MultiplexedSig`.
    #[inline(always)]
    pub fn set_multiplexed_sig(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange {
                message_id: MuxedFrame::MESSAGE_ID,
            });
        }
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
        Ok(())
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
