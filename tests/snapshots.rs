#![cfg(feature = "std")]

use std::borrow::Cow;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::{env, fs};

use can_dbc::decode_cp1252;
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
    fn file_name(&self, is_error: bool) -> String {
        if is_error {
            format!("!error___{}", self.file_name)
        } else {
            format!("{}.rs", self.file_name)
        }
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

    let config = dbc_codegen::Config::builder()
        .dbc_name(&test.file_name)
        .dbc_content(&buffer)
        .build();

    let mut out = Vec::<u8>::new();
    let result = dbc_codegen::codegen(config, &mut out);
    let result = result.and_then(|()| String::from_utf8(out).map_err(anyhow::Error::from));
    let is_err = result.is_err();

    if let Some(snapshot_path) = test.snapshot_path(is_err) {
        with_settings! {
            {
                omit_expression => true,
                prepend_module_to_snapshot => false,
                snapshot_path => snapshot_path,
            },
            {
                match result {
                    Ok(v) => assert_binary_snapshot!(&test.file_name(is_err), v.as_bytes().to_vec()),
                    Err(e) => assert_debug_snapshot!(test.file_name(is_err), e.to_string()),
                }
            }
        }
    } else if let Err(e) = result {
        panic!("Failed to parse {}.dbc: {e:#?}", test.file_name);
    }
}

static BAD_TESTS: &[&str] = &[
    //
    "bus_comment_bare.snap.rs",
    "bus_comment_bare_out.snap.rs",
    "choices.snap.rs",
    "cp1252.snap.rs",
    "empty_choice.snap.rs",
    "empty_ns.snap.rs",
    "issue_184_extended_mux_cascaded.snap.rs",
    "issue_184_extended_mux_cascaded_dumped.snap.rs",
    "issue_184_extended_mux_independent_multiplexors.snap.rs",
    "issue_184_extended_mux_independent_multiplexors_dumped.snap.rs",
    "issue_184_extended_mux_multiple_values.snap.rs",
    "issue_184_extended_mux_multiple_values_dumped.snap.rs",
    "issue_199.snap.rs",
    "issue_199_extended.snap.rs",
    "issue_62.snap.rs",
    "long_names_multiple_relations.snap.rs",
    "long_names_multiple_relations_dumped.snap.rs",
    "multiplex_2.snap.rs",
    "multiplex_2_dumped.snap.rs",
    "padding_bit_order.snap.rs",
    "signed.snap.rs",
    "vehicle.snap.rs",
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
