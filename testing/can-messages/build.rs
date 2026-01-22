use std::env::var;
use std::fs::read_to_string;
use std::path::PathBuf;

use anyhow::Result;
use dbc_codegen::{Config, FeatureConfig};

fn main() -> Result<()> {
    let dbc_file = read_to_string("../dbc-examples/example.dbc")?;
    println!("cargo:rerun-if-changed=../dbc-examples/example.dbc");
    println!("cargo:rerun-if-changed=../../src");
    println!("cargo:rerun-if-changed=../can-embedded/src");

    let path = PathBuf::from(var("OUT_DIR")?).join("messages.rs");

    Config::builder()
        .dbc_name("example.dbc")
        .dbc_content(&dbc_file)
        .debug_prints(true)
        .allow_dead_code(true)
        .impl_debug(FeatureConfig::Always)
        .impl_defmt(FeatureConfig::Always)
        .impl_error(FeatureConfig::Gated("std"))
        .impl_arbitrary(FeatureConfig::Gated("arb"))
        .check_ranges(FeatureConfig::Always)
        .build()
        .write_to_file(path)
}
