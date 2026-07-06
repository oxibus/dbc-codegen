use std::env::var;
use std::fs::read_to_string;
use std::path::PathBuf;

use anyhow::Result;
use dbc_codegen::Config;

fn main() -> Result<()> {
    let dbc_file = read_to_string("../dbc-examples/multiple_devices.dbc")?;
    println!("cargo:rerun-if-changed=../dbc-examples/multiple_devices.dbc");
    println!("cargo:rerun-if-changed=../../src");

    let out_dir = PathBuf::from(var("OUT_DIR")?);

    Config::builder()
        .dbc_name("multiple_devices.dbc")
        .dbc_content(&dbc_file)
        .allow_dead_code(true)
        .build()
        .write_split_by_sender(&out_dir)
}
