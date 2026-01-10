#![cfg(feature = "std")]

use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::{env, fs};

use insta::{assert_binary_snapshot, assert_debug_snapshot, with_settings};
use test_each_file::test_each_path;

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
    // TestConfig {
    //     test_root: "opendbc/opendbc/dbc",
    //     snapshot_suffix: "opendbc",
    //     use_cp1251: false,
    //     create_snapshot: false,
    // },
];

// test_each_path! { for ["dbc"] in "./tests/fixtures/opendbc/opendbc/dbc" as dbc => parse_one_file }
test_each_path! { for ["dbc"] in "./tests/fixtures/shared-test-files" as shared => parse_one_file }
// upper case extension
test_each_path! { for ["DBC"] in "./tests/fixtures/shared-test-files" as shared2 => parse_one_file }

/// Get `test root`, `snapshot name suffix`, `use cp1251`, `create snapshot` for the given path
fn get_test_info(path: &Path) -> Option<(PathBuf, &'static TestConfig)> {
    if !path
        .extension()
        .unwrap_or_default()
        .eq_ignore_ascii_case("dbc")
    {
        return None;
    }
    let path_str = path.to_str().unwrap();
    let parent = path.parent().unwrap();
    for item in TEST_DIRS {
        // Ensure slashes are there for easier matching
        let test_root = format!("/{}/", item.test_root);
        let mut path_dir = parent.to_str().unwrap().to_string();
        if !path_dir.ends_with('/') {
            path_dir.push('/');
        }
        if let Some(pos) = path_dir.find(&test_root) {
            let parent = PathBuf::from("../test-snapshots")
                .join(item.snapshot_suffix)
                .join(&path_dir[pos + test_root.len()..]);
            return Some((parent, item));
        }
    }
    panic!("Unknown test directory: {path_str}");
}

/// Test parsing all DBC files
#[test]
fn test_if_submodules_are_present() {
    for test in TEST_DIRS {
        let dir = Path::new("./tests/fixtures").join(test.test_root);
        fs::read_dir(&dir).unwrap_or_else(|e| {
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
    let Some((
        snapshot_path,
        &TestConfig {
            use_cp1251,
            create_snapshot,
            ..
        },
    )) = get_test_info(path)
    else {
        return;
    };

    let file_name = path.file_stem().unwrap().to_string_lossy().to_string();
    let buffer = fs::read(path).unwrap();
    let buffer = if use_cp1251 {
        can_dbc::decode_cp1252(&buffer)
            .unwrap_or_else(|| panic!("Failed to decode {} as cp1252", path.display()))
    } else {
        Cow::Borrowed(
            std::str::from_utf8(&buffer)
                .unwrap_or_else(|_| panic!("Failed to decode {} as utf-8", path.display())),
        )
    };

    let config = dbc_codegen::Config::builder()
        .dbc_name(&file_name)
        .dbc_content(&buffer)
        .build();

    let create_snapshot =
        env::var("SKIP_INSTA").is_err() && (create_snapshot || env::var("FORCE_INSTA").is_ok());

    let mut out = Vec::<u8>::new();
    if let Err(e) = dbc_codegen::codegen(config, &mut out) {
        if create_snapshot {
            with_settings! {
                {
                    omit_expression => true,
                    snapshot_path => snapshot_path,
                    prepend_module_to_snapshot => false
                },
                {
                    assert_debug_snapshot!(file_name, e);
                }
            }
            return;
        } else {
            panic!("{e}");
        }
    }

    match String::from_utf8(out) {
        Ok(result) => {
            if create_snapshot {
                with_settings! {
                    {
                        omit_expression => true,
                        snapshot_path => snapshot_path,
                        prepend_module_to_snapshot => false
                    },
                    {
                        assert_binary_snapshot!(&format!("{file_name}.rs"), result.into());
                    }
                }
            }
        }
        Err(e) => panic!("Failed to parse {file_name}.dbc: {e:#?}"),
    }
}

#[test]
#[ignore = "Run manually to verify that generated code compiles"]
fn compile_test() {
    let t = trybuild::TestCases::new();
    t.pass("test-snapshots/canpy/*.rs");
    t.pass("test-snapshots/dbc-cantools/*.rs");

    // // iterate over all *.rs files one by one
    // for entry in fs::read_dir("test-snapshots/dbc-cantools").unwrap() {
    //     let path = entry.unwrap().path();
    //     if path.extension().unwrap_or_default() != "rs" {
    //         continue;
    //     }
    //     if path.file_stem() == Some(std::ffi::OsStr::new("mod")) {
    //         continue;
    //     }
    // }
}
