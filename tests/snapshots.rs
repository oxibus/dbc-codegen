#![cfg(feature = "std")]

use std::borrow::Cow;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::{env, fs};

use can_dbc::decode_cp1252;
use dbc_codegen::Config;
use insta::{assert_binary_snapshot, assert_debug_snapshot, with_settings};
use test_each_file::test_each_path;
use walkdir::WalkDir;

struct TestConfig {
    test_root: &'static str,
    snapshot_suffix: &'static str,
    use_cp1251: bool,
    create_snapshot: bool,
}

static TEST_DIRS: &[TestConfig] = &[
    TestConfig {
        test_root: "shared-test-files",
        snapshot_suffix: "",
        use_cp1251: true,
        create_snapshot: true,
    },
    TestConfig {
        test_root: "opendbc/opendbc/dbc",
        snapshot_suffix: "opendbc",
        use_cp1251: false,
        create_snapshot: false,
    },
];

test_each_path! { for ["dbc"] in "./tests/fixtures/opendbc/opendbc/dbc" as dbc => parse_one_file }
test_each_path! { for ["dbc"] in "./tests/fixtures/shared-test-files" as shared => parse_one_file }
// upper case extension
test_each_path! { for ["DBC"] in "./tests/fixtures/shared-test-files" as shared2 => parse_one_file }

struct Test {
    config: &'static TestConfig,
    path: PathBuf,
    file_name: String,
}

impl Test {
    fn new(config: &'static TestConfig, path: PathBuf, file_name: String) -> Self {
        Self {
            config,
            path,
            file_name,
        }
    }
    fn decode<'a>(&self, data: &'a [u8]) -> Cow<'a, str> {
        if self.config.use_cp1251 {
            decode_cp1252(data)
                .unwrap_or_else(|| panic!("Cannot decode {} as cp1252", self.path.display()))
        } else {
            std::str::from_utf8(data)
                .unwrap_or_else(|_| panic!("Cannot decode {} as utf-8", self.path.display()))
                .into()
        }
    }
    fn snapshot_path(&self, is_error: bool) -> Option<PathBuf> {
        (if is_error || self.config.create_snapshot || env::var("FORCE_INSTA").is_ok() {
            // forced content ignored in .gitignored should still go to normal snapshots
            Some("../tests-snapshots")
        } else {
            None
        })
        .map(|v| {
            PathBuf::from(v)
                .join(self.config.snapshot_suffix)
                .join(&self.path)
        })
    }
    fn file_name(&self) -> String {
        format!("{}.rs", self.file_name)
    }
    fn err_file_name(&self) -> String {
        format!("!error___{}", self.file_name)
    }
}

/// Get snapshot path (if snapshot should be created) and a decoding
/// function for a test file path
fn get_test_info(path: &Path) -> Test {
    let path_str = path.display().to_string();
    let parent = path.parent().unwrap();
    for item in TEST_DIRS {
        // Ensure slashes are there for easier matching
        let test_root = format!("/{}/", item.test_root);
        let mut path_dir = parent.to_str().unwrap().to_string();
        if !path_dir.ends_with('/') {
            path_dir.push('/');
        }
        if let Some(pos) = path_dir.find(&test_root) {
            let file_name = path.file_stem().unwrap().to_string_lossy().to_string();
            let path = PathBuf::from(&path_dir[pos + test_root.len()..]);
            return Test::new(item, path, file_name);
        }
    }
    panic!("Unknown test directory: {path_str}");
}

/// Test parsing all DBC files
#[test]
fn test_if_submodules_are_present() {
    for test in TEST_DIRS {
        let dir = Path::new("./tests/fixtures").join(test.test_root);
        fs::read_dir(&dir)
            .and_then(|v| {
                v.into_iter()
                    .next()
                    .map(|_| ())
                    .ok_or_else(|| Error::new(ErrorKind::NotFound, "No files or dirs found"))
            })
            .unwrap_or_else(|e| {
                let dir_display = dir.display();
                panic!(
                    "
--------------------------------------------------------------------------
Error reading dbc test files from   {dir_display}
{e}
Make sure git submodules are up to date by running
    git submodule update --init --recursive
--------------------------------------------------------------------------
"
                )
            });
    }
}

/// Parse a single DBC file and assert a snapshot of the result.
fn parse_one_file([path]: [&Path; 1]) {
    let test = get_test_info(path);
    let buffer = fs::read(path).unwrap();
    let buffer = test.decode(&buffer);

    let result = Config::builder()
        .dbc_name(&test.file_name)
        .dbc_content(&buffer)
        .build()
        .generate();

    if let Some(snapshot_path) = test.snapshot_path(result.is_err()) {
        with_settings! {
            {
                omit_expression => true,
                prepend_module_to_snapshot => false,
                snapshot_path => snapshot_path,
            },
            {
                match result {
                    Ok(v) => assert_binary_snapshot!(&test.file_name(), v.as_bytes().to_vec()),
                    Err(e) => assert_debug_snapshot!(test.err_file_name(), e),
                }
            }
        }
    } else if let Err(e) = result {
        panic!("Failed to parse {}.dbc: {e:#?}", test.file_name);
    }
}

static BAD_TESTS: &[&str] = &[
    //
    "empty_choice.snap.rs",
    "issue_184_extended_mux_cascaded.snap.rs",
    "issue_184_extended_mux_cascaded_dumped.snap.rs",
    "issue_184_extended_mux_independent_multiplexors.snap.rs",
    "issue_184_extended_mux_independent_multiplexors_dumped.snap.rs",
    "issue_184_extended_mux_multiple_values.snap.rs",
    "issue_184_extended_mux_multiple_values_dumped.snap.rs",
    "issue_199.snap.rs",
    "issue_199_extended.snap.rs",
    "long_names_multiple_relations.snap.rs",
    "long_names_multiple_relations_dumped.snap.rs",
    "multiplex_2.snap.rs",
    "multiplex_2_dumped.snap.rs",
    "padding_bit_order.snap.rs",
    "signed.snap.rs",
    //
    "FORD_CADS.snap.rs",
    "bmw_e9x_e8x.snap.rs",
    "cadillac_ct6_object.snap.rs",
    "cadillac_ct6_powertrain.snap.rs",
    "chrysler_pacifica_2017_hybrid_private_fusion.snap.rs",
    "ford_lincoln_base_pt.snap.rs",
    "gm_global_a_high_voltage_management.snap.rs",
    "gm_global_a_lowspeed.snap.rs",
    "gm_global_a_lowspeed_1818125.snap.rs",
    "hyundai_2015_ccan.snap.rs",
    "hyundai_2015_mcan.snap.rs",
    "hyundai_i30_2014.snap.rs",
    "hyundai_kia_generic.snap.rs",
    "luxgen_s5_2015.snap.rs",
    "mercedes_benz_e350_2010.snap.rs",
    "psa_aee2010_r3.snap.rs",
    "rivian_primary_actuator.snap.rs",
    "tesla_can.snap.rs",
    "tesla_model3_party.snap.rs",
    "tesla_powertrain.snap.rs",
    "toyota_iQ_2009_can.snap.rs",
    "toyota_prius_2010_pt.snap.rs",
    "toyota_radar_dsu_tssp.snap.rs",
    "volvo_v40_2017_pt.snap.rs",
    "volvo_v60_2015_pt.snap.rs",
    "vw_meb.snap.rs",
    "vw_mlb.snap.rs",
    "vw_mqb.snap.rs",
    "vw_mqbevo.snap.rs",
    /* generator/chrysler */ "_stellantis_common.snap.rs",
    /* generator/gm */ "gm_global_a_powertrain.snap.rs",
    /* generator/honda */ "_bosch_2018.snap.rs",
    /* generator/honda */ "_bosch_radar_acc.snap.rs",
    /* generator/honda */ "_gearbox_common.snap.rs",
    /* generator/honda */ "_honda_common.snap.rs",
    /* generator/honda */ "_nidec_common.snap.rs",
    /* generator/honda */ "_nidec_scm_group_a.snap.rs",
    /* generator/honda */ "_nidec_scm_group_b.snap.rs",
    /* generator/honda */ "_steering_control_b.snap.rs",
    /* generator/honda */ "acura_ilx_2016_can.snap.rs",
    /* generator/honda */ "acura_rdx_2020_can.snap.rs",
    /* generator/honda */ "honda_civic_hatchback_ex_2017_can.snap.rs",
    /* generator/honda */ "honda_common_canfd.snap.rs",
    /* generator/honda */ "honda_crv_touring_2016_can.snap.rs",
    /* generator/honda */ "honda_odyssey_exl_2018.snap.rs",
    /* generator/hyundai */ "hyundai_canfd.snap.rs",
    /* generator/nissan */ "_nissan_common.snap.rs",
    /* generator/nissan */ "nissan_leaf_2018.snap.rs",
    /* generator/nissan */ "nissan_x_trail_2017.snap.rs",
    /* generator/subaru */ "_subaru_global.snap.rs",
    /* generator/subaru */ "_subaru_preglobal_2015.snap.rs",
    /* generator/toyota */ "_toyota_2017.snap.rs",
    /* generator/toyota */ "_toyota_adas_standard.snap.rs",
    /* generator/toyota */ "toyota_secoc_pt.snap.rs",
];

#[test]
fn compile_test() {
    // Clear any rust flags possibly set in the justfile
    env::remove_var("RUSTFLAGS");
    env::remove_var("RUSTDOCFLAGS");
    env::remove_var("RUST_BACKTRACE");
    env::set_var("CARGO_ENCODED_RUSTFLAGS", "--deny=warnings");
    let t = trybuild::TestCases::new();

    // Once all tests are fixed, switch to this:
    // t.pass("tests-snapshots/**/*.rs");

    for entry in WalkDir::new("tests-snapshots")
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }
        if BAD_TESTS.iter().any(|bad| path.ends_with(bad)) {
            t.compile_fail(path);
        } else {
            t.pass(path);
        }
    }
}

#[test]
#[ignore = "use `just test-manual` to run"]
fn single_file_manual_test() {
    let test_path = Path::new("tests/fixtures/shared-test-files/dbc-cantools/big_numbers.dbc");
    let test = get_test_info(test_path);
    let buffer = fs::read(test_path).unwrap();

    let code = Config::builder()
        .dbc_name(&test.file_name)
        .dbc_content(&test.decode(&buffer))
        .build()
        .generate()
        .unwrap();

    insta::assert_snapshot!(
        code,
        @r#"
    /// The name of the DBC file this code was generated from
    #[allow(dead_code)]
    pub const DBC_FILE_NAME: &str = "big_numbers";
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
        /// TheMessage
        TheMessage(TheMessage),
        /// TheOtherMessage
        TheOtherMessage(TheOtherMessage),
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
                TheMessage::MESSAGE_ID => {
                    Messages::TheMessage(TheMessage::try_from(payload)?)
                }
                TheOtherMessage::MESSAGE_ID => {
                    Messages::TheOtherMessage(TheOtherMessage::try_from(payload)?)
                }
                id => return Err(CanError::UnknownMessageId(id)),
            };
            Ok(res)
        }
    }
    /// TheMessage
    ///
    /// - Extended ID: 57 (0x39)
    /// - Size: 8 bytes
    #[derive(Clone, Copy)]
    pub struct TheMessage {
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
    impl TheMessage {
        pub const MESSAGE_ID: embedded_can::Id = Id::Extended(unsafe {
            ExtendedId::new_unchecked(0x39)
        });
        pub const THE_SIGNAL_MIN: i8 = 0_i8;
        pub const THE_SIGNAL_MAX: i8 = 0_i8;
        /// Construct new TheMessage from values
        pub fn new(the_signal: i8) -> Result<Self, CanError> {
            let mut res = Self { raw: [0u8; 8] };
            res.set_the_signal(the_signal)?;
            Ok(res)
        }
        /// Access message payload raw value
        pub fn raw(&self) -> &[u8; 8] {
            &self.raw
        }
        /// TheSignal
        ///
        /// - Min: 0
        /// - Max: 0
        /// - Unit: ""
        /// - Receivers: Vector__XXX
        #[inline(always)]
        pub fn the_signal(&self) -> i8 {
            self.the_signal_raw()
        }
        /// Get raw value of TheSignal
        ///
        /// - Start bit: 0
        /// - Signal size: 8 bits
        /// - Factor: 1
        /// - Offset: 0
        /// - Byte order: LittleEndian
        /// - Value type: Signed
        #[inline(always)]
        pub fn the_signal_raw(&self) -> i8 {
            let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<i8>();
            let factor = 1;
            let signal = signal as i8;
            i8::from(signal).saturating_mul(factor).saturating_add(0)
        }
        /// Set value of TheSignal
        #[inline(always)]
        pub fn set_the_signal(&mut self, value: i8) -> Result<(), CanError> {
            if value < 0_i8 || 0_i8 < value {
                return Err(CanError::ParameterOutOfRange {
                    message_id: TheMessage::MESSAGE_ID,
                });
            }
            let factor = 1;
            let value = value
                .checked_sub(0)
                .ok_or(CanError::ParameterOutOfRange {
                    message_id: TheMessage::MESSAGE_ID,
                })?;
            let value = (value / factor) as i8;
            let value = u8::from_ne_bytes(value.to_ne_bytes());
            self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
            Ok(())
        }
    }
    impl core::convert::TryFrom<&[u8]> for TheMessage {
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
    impl embedded_can::Frame for TheMessage {
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
    /// TheOtherMessage
    ///
    /// - Standard ID: 42 (0x2a)
    /// - Size: 8 bytes
    #[derive(Clone, Copy)]
    pub struct TheOtherMessage {
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
    impl TheOtherMessage {
        pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe {
            StandardId::new_unchecked(0x2a)
        });
        pub const THE_SIGNAL_MIN: i8 = 0_i8;
        pub const THE_SIGNAL_MAX: i8 = 0_i8;
        /// Construct new TheOtherMessage from values
        pub fn new(the_signal: i8) -> Result<Self, CanError> {
            let mut res = Self { raw: [0u8; 8] };
            res.set_the_signal(the_signal)?;
            Ok(res)
        }
        /// Access message payload raw value
        pub fn raw(&self) -> &[u8; 8] {
            &self.raw
        }
        /// TheSignal
        ///
        /// - Min: 0
        /// - Max: 0
        /// - Unit: ""
        /// - Receivers: Vector__XXX
        #[inline(always)]
        pub fn the_signal(&self) -> i8 {
            self.the_signal_raw()
        }
        /// Get raw value of TheSignal
        ///
        /// - Start bit: 0
        /// - Signal size: 8 bits
        /// - Factor: 1
        /// - Offset: 0
        /// - Byte order: LittleEndian
        /// - Value type: Signed
        #[inline(always)]
        pub fn the_signal_raw(&self) -> i8 {
            let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<i8>();
            let factor = 1;
            let signal = signal as i8;
            i8::from(signal).saturating_mul(factor).saturating_add(0)
        }
        /// Set value of TheSignal
        #[inline(always)]
        pub fn set_the_signal(&mut self, value: i8) -> Result<(), CanError> {
            if value < 0_i8 || 0_i8 < value {
                return Err(CanError::ParameterOutOfRange {
                    message_id: TheOtherMessage::MESSAGE_ID,
                });
            }
            let factor = 1;
            let value = value
                .checked_sub(0)
                .ok_or(CanError::ParameterOutOfRange {
                    message_id: TheOtherMessage::MESSAGE_ID,
                })?;
            let value = (value / factor) as i8;
            let value = u8::from_ne_bytes(value.to_ne_bytes());
            self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
            Ok(())
        }
    }
    impl core::convert::TryFrom<&[u8]> for TheOtherMessage {
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
    impl embedded_can::Frame for TheOtherMessage {
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
    "#,
    )

    // let out_path = PathBuf::from("./target/manual/manual_test_output.rs");
    // fs::create_dir_all(out_path.parent().unwrap()).unwrap();
    // fs::write(&out_path, code).unwrap();
    //
    // env::set_var("CARGO_ENCODED_RUSTFLAGS", "--deny=warnings");
    // let t = trybuild::TestCases::new();
    // t.pass(out_path);
}
