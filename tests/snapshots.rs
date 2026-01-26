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
    let test_path = Path::new("tests/fixtures/shared-test-files/dbc-cantools/multiplex_2.dbc");
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
    pub const DBC_FILE_NAME: &str = "multiplex_2";
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
        /// Shared
        Shared(Shared),
        /// Normal
        Normal(Normal),
        /// Extended
        Extended(Extended),
        /// ExtendedTypes
        ExtendedTypes(ExtendedTypes),
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
                Shared::MESSAGE_ID => Messages::Shared(Shared::try_from(payload)?),
                Normal::MESSAGE_ID => Messages::Normal(Normal::try_from(payload)?),
                Extended::MESSAGE_ID => Messages::Extended(Extended::try_from(payload)?),
                ExtendedTypes::MESSAGE_ID => {
                    Messages::ExtendedTypes(ExtendedTypes::try_from(payload)?)
                }
                id => return Err(CanError::UnknownMessageId(id)),
            };
            Ok(res)
        }
    }
    /// Shared
    ///
    /// - Extended ID: 201522942 (0xc02fefe)
    /// - Size: 8 bytes
    #[derive(Clone, Copy)]
    pub struct Shared {
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
    impl Shared {
        pub const MESSAGE_ID: embedded_can::Id = Id::Extended(unsafe {
            ExtendedId::new_unchecked(0xc02fefe)
        });
        pub const S2_MIN: i8 = 0_i8;
        pub const S2_MAX: i8 = 0_i8;
        pub const S1_MIN: i8 = 0_i8;
        pub const S1_MAX: i8 = 0_i8;
        pub const S0_MIN: i8 = 0_i8;
        pub const S0_MAX: i8 = 0_i8;
        /// Construct new Shared from values
        pub fn new(s0: i8) -> Result<Self, CanError> {
            let mut res = Self { raw: [0u8; 8] };
            res.set_s0(s0)?;
            Ok(res)
        }
        /// Access message payload raw value
        pub fn raw(&self) -> &[u8; 8] {
            &self.raw
        }
        /// Get raw value of S0
        ///
        /// - Start bit: 0
        /// - Signal size: 4 bits
        /// - Factor: 1
        /// - Offset: 0
        /// - Byte order: LittleEndian
        /// - Value type: Signed
        #[inline(always)]
        pub fn s0_raw(&self) -> i8 {
            let signal = self.raw.view_bits::<Lsb0>()[0..4].load_le::<i8>();
            let factor = 1;
            let signal = signal as i8;
            i8::from(signal).saturating_mul(factor).saturating_add(0)
        }
        pub fn s0(&mut self) -> Result<SharedS0Index, CanError> {
            match self.s0_raw() {
                1 => Ok(SharedS0Index::M1(SharedS0M1 { raw: self.raw })),
                2 => Ok(SharedS0Index::M2(SharedS0M2 { raw: self.raw })),
                multiplexor => {
                    Err(CanError::InvalidMultiplexor {
                        message_id: Shared::MESSAGE_ID,
                        multiplexor: multiplexor.into(),
                    })
                }
            }
        }
        /// Set value of S0
        #[inline(always)]
        fn set_s0(&mut self, value: i8) -> Result<(), CanError> {
            if value < 0_i8 || 0_i8 < value {
                return Err(CanError::ParameterOutOfRange {
                    message_id: Shared::MESSAGE_ID,
                });
            }
            let factor = 1;
            let value = value
                .checked_sub(0)
                .ok_or(CanError::ParameterOutOfRange {
                    message_id: Shared::MESSAGE_ID,
                })?;
            let value = (value / factor) as i8;
            let value = u8::from_ne_bytes(value.to_ne_bytes());
            self.raw.view_bits_mut::<Lsb0>()[0..4].store_le(value);
            Ok(())
        }
        /// Set value of S0
        #[inline(always)]
        pub fn set_m1(&mut self, value: SharedS0M1) -> Result<(), CanError> {
            let b0 = BitArray::<_, LocalBits>::new(self.raw);
            let b1 = BitArray::<_, LocalBits>::new(value.raw);
            self.raw = b0.bitor(b1).into_inner();
            self.set_s0(1)?;
            Ok(())
        }
        /// Set value of S0
        #[inline(always)]
        pub fn set_m2(&mut self, value: SharedS0M2) -> Result<(), CanError> {
            let b0 = BitArray::<_, LocalBits>::new(self.raw);
            let b1 = BitArray::<_, LocalBits>::new(value.raw);
            self.raw = b0.bitor(b1).into_inner();
            self.set_s0(2)?;
            Ok(())
        }
    }
    impl core::convert::TryFrom<&[u8]> for Shared {
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
    impl embedded_can::Frame for Shared {
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
    /// Defined values for multiplexed signal Shared
    #[allow(
        clippy::absurd_extreme_comparisons,
        clippy::excessive_precision,
        clippy::manual_range_contains,
        clippy::unnecessary_cast,
        clippy::useless_conversion,
        unused_comparisons,
        unused_variables,
    )]
    pub enum SharedS0Index {
        M1(SharedS0M1),
        M2(SharedS0M2),
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
    pub struct SharedS0M1 {
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
    impl SharedS0M1 {
        pub fn new() -> Self {
            Self { raw: [0u8; 8] }
        }
        /// S1
        ///
        /// - Min: 0
        /// - Max: 0
        /// - Unit: ""
        /// - Receivers: Vector__XXX
        #[inline(always)]
        pub fn s1(&self) -> i8 {
            self.s1_raw()
        }
        /// Get raw value of S1
        ///
        /// - Start bit: 4
        /// - Signal size: 4 bits
        /// - Factor: 1
        /// - Offset: 0
        /// - Byte order: LittleEndian
        /// - Value type: Signed
        #[inline(always)]
        pub fn s1_raw(&self) -> i8 {
            let signal = self.raw.view_bits::<Lsb0>()[4..8].load_le::<i8>();
            let factor = 1;
            let signal = signal as i8;
            i8::from(signal).saturating_mul(factor).saturating_add(0)
        }
        /// Set value of S1
        #[inline(always)]
        pub fn set_s1(&mut self, value: i8) -> Result<(), CanError> {
            if value < 0_i8 || 0_i8 < value {
                return Err(CanError::ParameterOutOfRange {
                    message_id: Shared::MESSAGE_ID,
                });
            }
            let factor = 1;
            let value = value
                .checked_sub(0)
                .ok_or(CanError::ParameterOutOfRange {
                    message_id: Shared::MESSAGE_ID,
                })?;
            let value = (value / factor) as i8;
            let value = u8::from_ne_bytes(value.to_ne_bytes());
            self.raw.view_bits_mut::<Lsb0>()[4..8].store_le(value);
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
    pub struct SharedS0M2 {
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
    impl SharedS0M2 {
        pub fn new() -> Self {
            Self { raw: [0u8; 8] }
        }
        /// S2
        ///
        /// - Min: 0
        /// - Max: 0
        /// - Unit: ""
        /// - Receivers: Vector__XXX
        #[inline(always)]
        pub fn s2(&self) -> i8 {
            self.s2_raw()
        }
        /// Get raw value of S2
        ///
        /// - Start bit: 8
        /// - Signal size: 8 bits
        /// - Factor: 1
        /// - Offset: 0
        /// - Byte order: LittleEndian
        /// - Value type: Signed
        #[inline(always)]
        pub fn s2_raw(&self) -> i8 {
            let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<i8>();
            let factor = 1;
            let signal = signal as i8;
            i8::from(signal).saturating_mul(factor).saturating_add(0)
        }
        /// Set value of S2
        #[inline(always)]
        pub fn set_s2(&mut self, value: i8) -> Result<(), CanError> {
            if value < 0_i8 || 0_i8 < value {
                return Err(CanError::ParameterOutOfRange {
                    message_id: Shared::MESSAGE_ID,
                });
            }
            let factor = 1;
            let value = value
                .checked_sub(0)
                .ok_or(CanError::ParameterOutOfRange {
                    message_id: Shared::MESSAGE_ID,
                })?;
            let value = (value / factor) as i8;
            let value = u8::from_ne_bytes(value.to_ne_bytes());
            self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
            Ok(())
        }
    }
    /// Normal
    ///
    /// - Extended ID: 201457406 (0xc01fefe)
    /// - Size: 8 bytes
    #[derive(Clone, Copy)]
    pub struct Normal {
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
    impl Normal {
        pub const MESSAGE_ID: embedded_can::Id = Id::Extended(unsafe {
            ExtendedId::new_unchecked(0xc01fefe)
        });
        pub const S2_MIN: i8 = 0_i8;
        pub const S2_MAX: i8 = 0_i8;
        pub const S1_MIN: i8 = 0_i8;
        pub const S1_MAX: i8 = 0_i8;
        pub const S0_MIN: i8 = 0_i8;
        pub const S0_MAX: i8 = 0_i8;
        /// Construct new Normal from values
        pub fn new(s0: i8) -> Result<Self, CanError> {
            let mut res = Self { raw: [0u8; 8] };
            res.set_s0(s0)?;
            Ok(res)
        }
        /// Access message payload raw value
        pub fn raw(&self) -> &[u8; 8] {
            &self.raw
        }
        /// Get raw value of S0
        ///
        /// - Start bit: 0
        /// - Signal size: 4 bits
        /// - Factor: 1
        /// - Offset: 0
        /// - Byte order: LittleEndian
        /// - Value type: Signed
        #[inline(always)]
        pub fn s0_raw(&self) -> i8 {
            let signal = self.raw.view_bits::<Lsb0>()[0..4].load_le::<i8>();
            let factor = 1;
            let signal = signal as i8;
            i8::from(signal).saturating_mul(factor).saturating_add(0)
        }
        pub fn s0(&mut self) -> Result<NormalS0Index, CanError> {
            match self.s0_raw() {
                0 => Ok(NormalS0Index::M0(NormalS0M0 { raw: self.raw })),
                1 => Ok(NormalS0Index::M1(NormalS0M1 { raw: self.raw })),
                multiplexor => {
                    Err(CanError::InvalidMultiplexor {
                        message_id: Normal::MESSAGE_ID,
                        multiplexor: multiplexor.into(),
                    })
                }
            }
        }
        /// Set value of S0
        #[inline(always)]
        fn set_s0(&mut self, value: i8) -> Result<(), CanError> {
            if value < 0_i8 || 0_i8 < value {
                return Err(CanError::ParameterOutOfRange {
                    message_id: Normal::MESSAGE_ID,
                });
            }
            let factor = 1;
            let value = value
                .checked_sub(0)
                .ok_or(CanError::ParameterOutOfRange {
                    message_id: Normal::MESSAGE_ID,
                })?;
            let value = (value / factor) as i8;
            let value = u8::from_ne_bytes(value.to_ne_bytes());
            self.raw.view_bits_mut::<Lsb0>()[0..4].store_le(value);
            Ok(())
        }
        /// Set value of S0
        #[inline(always)]
        pub fn set_m0(&mut self, value: NormalS0M0) -> Result<(), CanError> {
            let b0 = BitArray::<_, LocalBits>::new(self.raw);
            let b1 = BitArray::<_, LocalBits>::new(value.raw);
            self.raw = b0.bitor(b1).into_inner();
            self.set_s0(0)?;
            Ok(())
        }
        /// Set value of S0
        #[inline(always)]
        pub fn set_m1(&mut self, value: NormalS0M1) -> Result<(), CanError> {
            let b0 = BitArray::<_, LocalBits>::new(self.raw);
            let b1 = BitArray::<_, LocalBits>::new(value.raw);
            self.raw = b0.bitor(b1).into_inner();
            self.set_s0(1)?;
            Ok(())
        }
    }
    impl core::convert::TryFrom<&[u8]> for Normal {
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
    impl embedded_can::Frame for Normal {
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
    /// Defined values for multiplexed signal Normal
    #[allow(
        clippy::absurd_extreme_comparisons,
        clippy::excessive_precision,
        clippy::manual_range_contains,
        clippy::unnecessary_cast,
        clippy::useless_conversion,
        unused_comparisons,
        unused_variables,
    )]
    pub enum NormalS0Index {
        M0(NormalS0M0),
        M1(NormalS0M1),
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
    pub struct NormalS0M0 {
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
    impl NormalS0M0 {
        pub fn new() -> Self {
            Self { raw: [0u8; 8] }
        }
        /// S1
        ///
        /// - Min: 0
        /// - Max: 0
        /// - Unit: ""
        /// - Receivers: Vector__XXX
        #[inline(always)]
        pub fn s1(&self) -> i8 {
            self.s1_raw()
        }
        /// Get raw value of S1
        ///
        /// - Start bit: 4
        /// - Signal size: 4 bits
        /// - Factor: 1
        /// - Offset: 0
        /// - Byte order: LittleEndian
        /// - Value type: Signed
        #[inline(always)]
        pub fn s1_raw(&self) -> i8 {
            let signal = self.raw.view_bits::<Lsb0>()[4..8].load_le::<i8>();
            let factor = 1;
            let signal = signal as i8;
            i8::from(signal).saturating_mul(factor).saturating_add(0)
        }
        /// Set value of S1
        #[inline(always)]
        pub fn set_s1(&mut self, value: i8) -> Result<(), CanError> {
            if value < 0_i8 || 0_i8 < value {
                return Err(CanError::ParameterOutOfRange {
                    message_id: Normal::MESSAGE_ID,
                });
            }
            let factor = 1;
            let value = value
                .checked_sub(0)
                .ok_or(CanError::ParameterOutOfRange {
                    message_id: Normal::MESSAGE_ID,
                })?;
            let value = (value / factor) as i8;
            let value = u8::from_ne_bytes(value.to_ne_bytes());
            self.raw.view_bits_mut::<Lsb0>()[4..8].store_le(value);
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
    pub struct NormalS0M1 {
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
    impl NormalS0M1 {
        pub fn new() -> Self {
            Self { raw: [0u8; 8] }
        }
        /// S2
        ///
        /// - Min: 0
        /// - Max: 0
        /// - Unit: ""
        /// - Receivers: Vector__XXX
        #[inline(always)]
        pub fn s2(&self) -> i8 {
            self.s2_raw()
        }
        /// Get raw value of S2
        ///
        /// - Start bit: 8
        /// - Signal size: 8 bits
        /// - Factor: 1
        /// - Offset: 0
        /// - Byte order: LittleEndian
        /// - Value type: Signed
        #[inline(always)]
        pub fn s2_raw(&self) -> i8 {
            let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<i8>();
            let factor = 1;
            let signal = signal as i8;
            i8::from(signal).saturating_mul(factor).saturating_add(0)
        }
        /// Set value of S2
        #[inline(always)]
        pub fn set_s2(&mut self, value: i8) -> Result<(), CanError> {
            if value < 0_i8 || 0_i8 < value {
                return Err(CanError::ParameterOutOfRange {
                    message_id: Normal::MESSAGE_ID,
                });
            }
            let factor = 1;
            let value = value
                .checked_sub(0)
                .ok_or(CanError::ParameterOutOfRange {
                    message_id: Normal::MESSAGE_ID,
                })?;
            let value = (value / factor) as i8;
            let value = u8::from_ne_bytes(value.to_ne_bytes());
            self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
            Ok(())
        }
    }
    /// Extended
    ///
    /// - Extended ID: 201391870 (0xc00fefe)
    /// - Size: 8 bytes
    #[derive(Clone, Copy)]
    pub struct Extended {
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
    impl Extended {
        pub const MESSAGE_ID: embedded_can::Id = Id::Extended(unsafe {
            ExtendedId::new_unchecked(0xc00fefe)
        });
        pub const S8_MIN: i8 = 0_i8;
        pub const S8_MAX: i8 = 0_i8;
        pub const S7_MIN: i32 = 0_i32;
        pub const S7_MAX: i32 = 0_i32;
        pub const S6_MIN: i8 = 0_i8;
        pub const S6_MAX: i8 = 0_i8;
        pub const S5_MIN: i32 = 0_i32;
        pub const S5_MAX: i32 = 0_i32;
        pub const S4_MIN: i32 = 0_i32;
        pub const S4_MAX: i32 = 0_i32;
        pub const S3_MIN: i16 = 0_i16;
        pub const S3_MAX: i16 = 0_i16;
        pub const S2_MIN: i8 = 0_i8;
        pub const S2_MAX: i8 = 0_i8;
        pub const S1_MIN: i8 = 0_i8;
        pub const S1_MAX: i8 = 0_i8;
        pub const S0_MIN: i8 = 0_i8;
        pub const S0_MAX: i8 = 0_i8;
        /// Construct new Extended from values
        pub fn new(s6: i8, s0: i8) -> Result<Self, CanError> {
            let mut res = Self { raw: [0u8; 8] };
            res.set_s6(s6)?;
            res.set_s0(s0)?;
            Ok(res)
        }
        /// Access message payload raw value
        pub fn raw(&self) -> &[u8; 8] {
            &self.raw
        }
        /// Get raw value of S6
        ///
        /// - Start bit: 32
        /// - Signal size: 8 bits
        /// - Factor: 1
        /// - Offset: 0
        /// - Byte order: LittleEndian
        /// - Value type: Signed
        #[inline(always)]
        pub fn s6_raw(&self) -> i8 {
            let signal = self.raw.view_bits::<Lsb0>()[32..40].load_le::<i8>();
            let factor = 1;
            let signal = signal as i8;
            i8::from(signal).saturating_mul(factor).saturating_add(0)
        }
        pub fn s6(&mut self) -> Result<ExtendedS6Index, CanError> {
            match self.s6_raw() {
                0 => Ok(ExtendedS6Index::M0(ExtendedS6M0 { raw: self.raw })),
                1 => Ok(ExtendedS6Index::M1(ExtendedS6M1 { raw: self.raw })),
                2 => Ok(ExtendedS6Index::M2(ExtendedS6M2 { raw: self.raw })),
                multiplexor => {
                    Err(CanError::InvalidMultiplexor {
                        message_id: Extended::MESSAGE_ID,
                        multiplexor: multiplexor.into(),
                    })
                }
            }
        }
        /// Set value of S6
        #[inline(always)]
        fn set_s6(&mut self, value: i8) -> Result<(), CanError> {
            if value < 0_i8 || 0_i8 < value {
                return Err(CanError::ParameterOutOfRange {
                    message_id: Extended::MESSAGE_ID,
                });
            }
            let factor = 1;
            let value = value
                .checked_sub(0)
                .ok_or(CanError::ParameterOutOfRange {
                    message_id: Extended::MESSAGE_ID,
                })?;
            let value = (value / factor) as i8;
            let value = u8::from_ne_bytes(value.to_ne_bytes());
            self.raw.view_bits_mut::<Lsb0>()[32..40].store_le(value);
            Ok(())
        }
        /// Set value of S6
        #[inline(always)]
        pub fn set_m0(&mut self, value: ExtendedS6M0) -> Result<(), CanError> {
            let b0 = BitArray::<_, LocalBits>::new(self.raw);
            let b1 = BitArray::<_, LocalBits>::new(value.raw);
            self.raw = b0.bitor(b1).into_inner();
            self.set_s6(0)?;
            Ok(())
        }
        /// Set value of S6
        #[inline(always)]
        pub fn set_m1(&mut self, value: ExtendedS6M1) -> Result<(), CanError> {
            let b0 = BitArray::<_, LocalBits>::new(self.raw);
            let b1 = BitArray::<_, LocalBits>::new(value.raw);
            self.raw = b0.bitor(b1).into_inner();
            self.set_s6(1)?;
            Ok(())
        }
        /// Set value of S6
        #[inline(always)]
        pub fn set_m2(&mut self, value: ExtendedS6M2) -> Result<(), CanError> {
            let b0 = BitArray::<_, LocalBits>::new(self.raw);
            let b1 = BitArray::<_, LocalBits>::new(value.raw);
            self.raw = b0.bitor(b1).into_inner();
            self.set_s6(2)?;
            Ok(())
        }
        /// Get raw value of S0
        ///
        /// - Start bit: 0
        /// - Signal size: 4 bits
        /// - Factor: 1
        /// - Offset: 0
        /// - Byte order: LittleEndian
        /// - Value type: Signed
        #[inline(always)]
        pub fn s0_raw(&self) -> i8 {
            let signal = self.raw.view_bits::<Lsb0>()[0..4].load_le::<i8>();
            let factor = 1;
            let signal = signal as i8;
            i8::from(signal).saturating_mul(factor).saturating_add(0)
        }
        pub fn s0(&mut self) -> Result<ExtendedS0Index, CanError> {
            match self.s0_raw() {
                0 => Ok(ExtendedS0Index::M0(ExtendedS0M0 { raw: self.raw })),
                1 => Ok(ExtendedS0Index::M1(ExtendedS0M1 { raw: self.raw })),
                2 => Ok(ExtendedS0Index::M2(ExtendedS0M2 { raw: self.raw })),
                multiplexor => {
                    Err(CanError::InvalidMultiplexor {
                        message_id: Extended::MESSAGE_ID,
                        multiplexor: multiplexor.into(),
                    })
                }
            }
        }
        /// Set value of S0
        #[inline(always)]
        fn set_s0(&mut self, value: i8) -> Result<(), CanError> {
            if value < 0_i8 || 0_i8 < value {
                return Err(CanError::ParameterOutOfRange {
                    message_id: Extended::MESSAGE_ID,
                });
            }
            let factor = 1;
            let value = value
                .checked_sub(0)
                .ok_or(CanError::ParameterOutOfRange {
                    message_id: Extended::MESSAGE_ID,
                })?;
            let value = (value / factor) as i8;
            let value = u8::from_ne_bytes(value.to_ne_bytes());
            self.raw.view_bits_mut::<Lsb0>()[0..4].store_le(value);
            Ok(())
        }
        /// Set value of S0
        #[inline(always)]
        pub fn set_m0(&mut self, value: ExtendedS0M0) -> Result<(), CanError> {
            let b0 = BitArray::<_, LocalBits>::new(self.raw);
            let b1 = BitArray::<_, LocalBits>::new(value.raw);
            self.raw = b0.bitor(b1).into_inner();
            self.set_s0(0)?;
            Ok(())
        }
        /// Set value of S0
        #[inline(always)]
        pub fn set_m1(&mut self, value: ExtendedS0M1) -> Result<(), CanError> {
            let b0 = BitArray::<_, LocalBits>::new(self.raw);
            let b1 = BitArray::<_, LocalBits>::new(value.raw);
            self.raw = b0.bitor(b1).into_inner();
            self.set_s0(1)?;
            Ok(())
        }
        /// Set value of S0
        #[inline(always)]
        pub fn set_m2(&mut self, value: ExtendedS0M2) -> Result<(), CanError> {
            let b0 = BitArray::<_, LocalBits>::new(self.raw);
            let b1 = BitArray::<_, LocalBits>::new(value.raw);
            self.raw = b0.bitor(b1).into_inner();
            self.set_s0(2)?;
            Ok(())
        }
    }
    impl core::convert::TryFrom<&[u8]> for Extended {
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
    impl embedded_can::Frame for Extended {
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
    /// Defined values for multiplexed signal Extended
    #[allow(
        clippy::absurd_extreme_comparisons,
        clippy::excessive_precision,
        clippy::manual_range_contains,
        clippy::unnecessary_cast,
        clippy::useless_conversion,
        unused_comparisons,
        unused_variables,
    )]
    pub enum ExtendedS6Index {
        M0(ExtendedS6M0),
        M1(ExtendedS6M1),
        M2(ExtendedS6M2),
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
    pub struct ExtendedS6M0 {
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
    impl ExtendedS6M0 {
        pub fn new() -> Self {
            Self { raw: [0u8; 8] }
        }
        /// S3
        ///
        /// - Min: 0
        /// - Max: 0
        /// - Unit: ""
        /// - Receivers: Vector__XXX
        #[inline(always)]
        pub fn s3(&self) -> i16 {
            self.s3_raw()
        }
        /// Get raw value of S3
        ///
        /// - Start bit: 16
        /// - Signal size: 16 bits
        /// - Factor: 1
        /// - Offset: 0
        /// - Byte order: LittleEndian
        /// - Value type: Signed
        #[inline(always)]
        pub fn s3_raw(&self) -> i16 {
            let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<i16>();
            let factor = 1;
            let signal = signal as i16;
            i16::from(signal).saturating_mul(factor).saturating_add(0)
        }
        /// Set value of S3
        #[inline(always)]
        pub fn set_s3(&mut self, value: i16) -> Result<(), CanError> {
            if value < 0_i16 || 0_i16 < value {
                return Err(CanError::ParameterOutOfRange {
                    message_id: Extended::MESSAGE_ID,
                });
            }
            let factor = 1;
            let value = value
                .checked_sub(0)
                .ok_or(CanError::ParameterOutOfRange {
                    message_id: Extended::MESSAGE_ID,
                })?;
            let value = (value / factor) as i16;
            let value = u16::from_ne_bytes(value.to_ne_bytes());
            self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
            Ok(())
        }
        /// S2
        ///
        /// - Min: 0
        /// - Max: 0
        /// - Unit: ""
        /// - Receivers: Vector__XXX
        #[inline(always)]
        pub fn s2(&self) -> i8 {
            self.s2_raw()
        }
        /// Get raw value of S2
        ///
        /// - Start bit: 8
        /// - Signal size: 8 bits
        /// - Factor: 1
        /// - Offset: 0
        /// - Byte order: LittleEndian
        /// - Value type: Signed
        #[inline(always)]
        pub fn s2_raw(&self) -> i8 {
            let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<i8>();
            let factor = 1;
            let signal = signal as i8;
            i8::from(signal).saturating_mul(factor).saturating_add(0)
        }
        /// Set value of S2
        #[inline(always)]
        pub fn set_s2(&mut self, value: i8) -> Result<(), CanError> {
            if value < 0_i8 || 0_i8 < value {
                return Err(CanError::ParameterOutOfRange {
                    message_id: Extended::MESSAGE_ID,
                });
            }
            let factor = 1;
            let value = value
                .checked_sub(0)
                .ok_or(CanError::ParameterOutOfRange {
                    message_id: Extended::MESSAGE_ID,
                })?;
            let value = (value / factor) as i8;
            let value = u8::from_ne_bytes(value.to_ne_bytes());
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
    pub struct ExtendedS6M1 {
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
    impl ExtendedS6M1 {
        pub fn new() -> Self {
            Self { raw: [0u8; 8] }
        }
        /// S7
        ///
        /// - Min: 0
        /// - Max: 0
        /// - Unit: ""
        /// - Receivers: Vector__XXX
        #[inline(always)]
        pub fn s7(&self) -> i32 {
            self.s7_raw()
        }
        /// Get raw value of S7
        ///
        /// - Start bit: 40
        /// - Signal size: 24 bits
        /// - Factor: 1
        /// - Offset: 0
        /// - Byte order: LittleEndian
        /// - Value type: Signed
        #[inline(always)]
        pub fn s7_raw(&self) -> i32 {
            let signal = self.raw.view_bits::<Lsb0>()[40..64].load_le::<i32>();
            let factor = 1;
            let signal = signal as i32;
            i32::from(signal).saturating_mul(factor).saturating_add(0)
        }
        /// Set value of S7
        #[inline(always)]
        pub fn set_s7(&mut self, value: i32) -> Result<(), CanError> {
            if value < 0_i32 || 0_i32 < value {
                return Err(CanError::ParameterOutOfRange {
                    message_id: Extended::MESSAGE_ID,
                });
            }
            let factor = 1;
            let value = value
                .checked_sub(0)
                .ok_or(CanError::ParameterOutOfRange {
                    message_id: Extended::MESSAGE_ID,
                })?;
            let value = (value / factor) as i32;
            let value = u32::from_ne_bytes(value.to_ne_bytes());
            self.raw.view_bits_mut::<Lsb0>()[40..64].store_le(value);
            Ok(())
        }
        /// S5
        ///
        /// - Min: 0
        /// - Max: 0
        /// - Unit: ""
        /// - Receivers: Vector__XXX
        #[inline(always)]
        pub fn s5(&self) -> i32 {
            self.s5_raw()
        }
        /// Get raw value of S5
        ///
        /// - Start bit: 4
        /// - Signal size: 28 bits
        /// - Factor: 1
        /// - Offset: 0
        /// - Byte order: LittleEndian
        /// - Value type: Signed
        #[inline(always)]
        pub fn s5_raw(&self) -> i32 {
            let signal = self.raw.view_bits::<Lsb0>()[4..32].load_le::<i32>();
            let factor = 1;
            let signal = signal as i32;
            i32::from(signal).saturating_mul(factor).saturating_add(0)
        }
        /// Set value of S5
        #[inline(always)]
        pub fn set_s5(&mut self, value: i32) -> Result<(), CanError> {
            if value < 0_i32 || 0_i32 < value {
                return Err(CanError::ParameterOutOfRange {
                    message_id: Extended::MESSAGE_ID,
                });
            }
            let factor = 1;
            let value = value
                .checked_sub(0)
                .ok_or(CanError::ParameterOutOfRange {
                    message_id: Extended::MESSAGE_ID,
                })?;
            let value = (value / factor) as i32;
            let value = u32::from_ne_bytes(value.to_ne_bytes());
            self.raw.view_bits_mut::<Lsb0>()[4..32].store_le(value);
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
    pub struct ExtendedS6M2 {
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
    impl ExtendedS6M2 {
        pub fn new() -> Self {
            Self { raw: [0u8; 8] }
        }
        /// S8
        ///
        /// - Min: 0
        /// - Max: 0
        /// - Unit: ""
        /// - Receivers: Vector__XXX
        #[inline(always)]
        pub fn s8(&self) -> i8 {
            self.s8_raw()
        }
        /// Get raw value of S8
        ///
        /// - Start bit: 40
        /// - Signal size: 8 bits
        /// - Factor: 1
        /// - Offset: 0
        /// - Byte order: LittleEndian
        /// - Value type: Signed
        #[inline(always)]
        pub fn s8_raw(&self) -> i8 {
            let signal = self.raw.view_bits::<Lsb0>()[40..48].load_le::<i8>();
            let factor = 1;
            let signal = signal as i8;
            i8::from(signal).saturating_mul(factor).saturating_add(0)
        }
        /// Set value of S8
        #[inline(always)]
        pub fn set_s8(&mut self, value: i8) -> Result<(), CanError> {
            if value < 0_i8 || 0_i8 < value {
                return Err(CanError::ParameterOutOfRange {
                    message_id: Extended::MESSAGE_ID,
                });
            }
            let factor = 1;
            let value = value
                .checked_sub(0)
                .ok_or(CanError::ParameterOutOfRange {
                    message_id: Extended::MESSAGE_ID,
                })?;
            let value = (value / factor) as i8;
            let value = u8::from_ne_bytes(value.to_ne_bytes());
            self.raw.view_bits_mut::<Lsb0>()[40..48].store_le(value);
            Ok(())
        }
        /// S4
        ///
        /// - Min: 0
        /// - Max: 0
        /// - Unit: ""
        /// - Receivers: Vector__XXX
        #[inline(always)]
        pub fn s4(&self) -> i32 {
            self.s4_raw()
        }
        /// Get raw value of S4
        ///
        /// - Start bit: 8
        /// - Signal size: 24 bits
        /// - Factor: 1
        /// - Offset: 0
        /// - Byte order: LittleEndian
        /// - Value type: Signed
        #[inline(always)]
        pub fn s4_raw(&self) -> i32 {
            let signal = self.raw.view_bits::<Lsb0>()[8..32].load_le::<i32>();
            let factor = 1;
            let signal = signal as i32;
            i32::from(signal).saturating_mul(factor).saturating_add(0)
        }
        /// Set value of S4
        #[inline(always)]
        pub fn set_s4(&mut self, value: i32) -> Result<(), CanError> {
            if value < 0_i32 || 0_i32 < value {
                return Err(CanError::ParameterOutOfRange {
                    message_id: Extended::MESSAGE_ID,
                });
            }
            let factor = 1;
            let value = value
                .checked_sub(0)
                .ok_or(CanError::ParameterOutOfRange {
                    message_id: Extended::MESSAGE_ID,
                })?;
            let value = (value / factor) as i32;
            let value = u32::from_ne_bytes(value.to_ne_bytes());
            self.raw.view_bits_mut::<Lsb0>()[8..32].store_le(value);
            Ok(())
        }
    }
    /// ExtendedTypes
    ///
    /// - Extended ID: 201588478 (0xc03fefe)
    /// - Size: 8 bytes
    #[derive(Clone, Copy)]
    pub struct ExtendedTypes {
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
    impl ExtendedTypes {
        pub const MESSAGE_ID: embedded_can::Id = Id::Extended(unsafe {
            ExtendedId::new_unchecked(0xc03fefe)
        });
        pub const S9_MIN: i32 = -1.34_i32;
        pub const S9_MAX: i32 = 1235_i32;
        pub const S10_MIN: i32 = -340000000000000000000000000000000000000_i32;
        pub const S10_MAX: i32 = 340000000000000000000000000000000000000_i32;
        pub const S0_MIN: i8 = 0_i8;
        pub const S0_MAX: i8 = 0_i8;
        pub const S11_MIN: u8 = 2_u8;
        pub const S11_MAX: u8 = 6_u8;
        /// Construct new ExtendedTypes from values
        pub fn new(s11: u8) -> Result<Self, CanError> {
            let mut res = Self { raw: [0u8; 8] };
            res.set_s11(s11)?;
            Ok(res)
        }
        /// Access message payload raw value
        pub fn raw(&self) -> &[u8; 8] {
            &self.raw
        }
        /// Get raw value of S11
        ///
        /// - Start bit: 0
        /// - Signal size: 5 bits
        /// - Factor: 1
        /// - Offset: 0
        /// - Byte order: LittleEndian
        /// - Value type: Unsigned
        #[inline(always)]
        pub fn s11_raw(&self) -> u8 {
            let signal = self.raw.view_bits::<Lsb0>()[0..5].load_le::<u8>();
            let factor = 1;
            u8::from(signal).saturating_mul(factor).saturating_add(0)
        }
        pub fn s11(&mut self) -> Result<ExtendedTypesS11Index, CanError> {
            match self.s11_raw() {
                0 => {
                    Ok(
                        ExtendedTypesS11Index::M0(ExtendedTypesS11M0 {
                            raw: self.raw,
                        }),
                    )
                }
                5 => {
                    Ok(
                        ExtendedTypesS11Index::M5(ExtendedTypesS11M5 {
                            raw: self.raw,
                        }),
                    )
                }
                multiplexor => {
                    Err(CanError::InvalidMultiplexor {
                        message_id: ExtendedTypes::MESSAGE_ID,
                        multiplexor: multiplexor.into(),
                    })
                }
            }
        }
        /// Set value of S11
        #[inline(always)]
        fn set_s11(&mut self, value: u8) -> Result<(), CanError> {
            if value < 2_u8 || 6_u8 < value {
                return Err(CanError::ParameterOutOfRange {
                    message_id: ExtendedTypes::MESSAGE_ID,
                });
            }
            let factor = 1;
            let value = value
                .checked_sub(0)
                .ok_or(CanError::ParameterOutOfRange {
                    message_id: ExtendedTypes::MESSAGE_ID,
                })?;
            let value = (value / factor) as u8;
            self.raw.view_bits_mut::<Lsb0>()[0..5].store_le(value);
            Ok(())
        }
        /// Set value of S11
        #[inline(always)]
        pub fn set_m0(&mut self, value: ExtendedTypesS11M0) -> Result<(), CanError> {
            let b0 = BitArray::<_, LocalBits>::new(self.raw);
            let b1 = BitArray::<_, LocalBits>::new(value.raw);
            self.raw = b0.bitor(b1).into_inner();
            self.set_s11(0)?;
            Ok(())
        }
        /// Set value of S11
        #[inline(always)]
        pub fn set_m5(&mut self, value: ExtendedTypesS11M5) -> Result<(), CanError> {
            let b0 = BitArray::<_, LocalBits>::new(self.raw);
            let b1 = BitArray::<_, LocalBits>::new(value.raw);
            self.raw = b0.bitor(b1).into_inner();
            self.set_s11(5)?;
            Ok(())
        }
    }
    impl core::convert::TryFrom<&[u8]> for ExtendedTypes {
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
    impl embedded_can::Frame for ExtendedTypes {
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
    /// Defined values for multiplexed signal ExtendedTypes
    #[allow(
        clippy::absurd_extreme_comparisons,
        clippy::excessive_precision,
        clippy::manual_range_contains,
        clippy::unnecessary_cast,
        clippy::useless_conversion,
        unused_comparisons,
        unused_variables,
    )]
    pub enum ExtendedTypesS11Index {
        M0(ExtendedTypesS11M0),
        M5(ExtendedTypesS11M5),
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
    pub struct ExtendedTypesS11M0 {
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
    impl ExtendedTypesS11M0 {
        pub fn new() -> Self {
            Self { raw: [0u8; 8] }
        }
        /// S10
        ///
        /// - Min: -340000000000000000000000000000000000000
        /// - Max: 340000000000000000000000000000000000000
        /// - Unit: ""
        /// - Receivers: Vector__XXX
        #[inline(always)]
        pub fn s10(&self) -> i32 {
            self.s10_raw()
        }
        /// Get raw value of S10
        ///
        /// - Start bit: 16
        /// - Signal size: 32 bits
        /// - Factor: 1
        /// - Offset: 0
        /// - Byte order: LittleEndian
        /// - Value type: Signed
        #[inline(always)]
        pub fn s10_raw(&self) -> i32 {
            let signal = self.raw.view_bits::<Lsb0>()[16..48].load_le::<i32>();
            let factor = 1;
            let signal = signal as i32;
            i32::from(signal).saturating_mul(factor).saturating_add(0)
        }
        /// Set value of S10
        #[inline(always)]
        pub fn set_s10(&mut self, value: i32) -> Result<(), CanError> {
            if value < -340000000000000000000000000000000000000_i32
                || 340000000000000000000000000000000000000_i32 < value
            {
                return Err(CanError::ParameterOutOfRange {
                    message_id: ExtendedTypes::MESSAGE_ID,
                });
            }
            let factor = 1;
            let value = value
                .checked_sub(0)
                .ok_or(CanError::ParameterOutOfRange {
                    message_id: ExtendedTypes::MESSAGE_ID,
                })?;
            let value = (value / factor) as i32;
            let value = u32::from_ne_bytes(value.to_ne_bytes());
            self.raw.view_bits_mut::<Lsb0>()[16..48].store_le(value);
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
    pub struct ExtendedTypesS11M5 {
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
    impl ExtendedTypesS11M5 {
        pub fn new() -> Self {
            Self { raw: [0u8; 8] }
        }
        /// S9
        ///
        /// - Min: -1.34
        /// - Max: 1235
        /// - Unit: ""
        /// - Receivers: Vector__XXX
        #[inline(always)]
        pub fn s9(&self) -> i32 {
            self.s9_raw()
        }
        /// Get raw value of S9
        ///
        /// - Start bit: 24
        /// - Signal size: 32 bits
        /// - Factor: 1
        /// - Offset: 0
        /// - Byte order: LittleEndian
        /// - Value type: Signed
        #[inline(always)]
        pub fn s9_raw(&self) -> i32 {
            let signal = self.raw.view_bits::<Lsb0>()[24..56].load_le::<i32>();
            let factor = 1;
            let signal = signal as i32;
            i32::from(signal).saturating_mul(factor).saturating_add(0)
        }
        /// Set value of S9
        #[inline(always)]
        pub fn set_s9(&mut self, value: i32) -> Result<(), CanError> {
            if value < -1.34_i32 || 1235_i32 < value {
                return Err(CanError::ParameterOutOfRange {
                    message_id: ExtendedTypes::MESSAGE_ID,
                });
            }
            let factor = 1;
            let value = value
                .checked_sub(0)
                .ok_or(CanError::ParameterOutOfRange {
                    message_id: ExtendedTypes::MESSAGE_ID,
                })?;
            let value = (value / factor) as i32;
            let value = u32::from_ne_bytes(value.to_ne_bytes());
            self.raw.view_bits_mut::<Lsb0>()[24..56].store_le(value);
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
