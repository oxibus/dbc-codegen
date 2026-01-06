# CAN DBC code generator for Rust

[![GitHub repo](https://img.shields.io/badge/github-oxibus/dbc--codegen-8da0cb?logo=github)](https://github.com/oxibus/dbc-codegen)
[![crates.io version](https://img.shields.io/crates/v/dbc-codegen)](https://crates.io/crates/dbc-codegen)
[![crate usage](https://img.shields.io/crates/d/dbc-codegen)](https://crates.io/crates/dbc-codegen)
[![docs.rs status](https://img.shields.io/docsrs/dbc-codegen)](https://docs.rs/dbc-codegen)
[![crates.io license](https://img.shields.io/crates/l/dbc-codegen)](https://github.com/oxibus/dbc-codegen)
[![CI build status](https://github.com/oxibus/dbc-codegen/actions/workflows/ci.yml/badge.svg)](https://github.com/oxibus/dbc-codegen/actions)
[![Codecov](https://img.shields.io/codecov/c/github/oxibus/dbc-codegen)](https://app.codecov.io/gh/oxibus/dbc-codegen)

Generates Rust messages from a `dbc` file. DBC files are descriptions of CAN frames.
See [this post](https://www.kvaser.com/developer-blog/an-introduction-j1939-and-dbc-files/)
for an introduction.

## Installation

Install published version using [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html):

```bash
cargo install dbc-codegen-cli
```
Install latest version from the git repository:

```bash
cargo install dbc-codegen-cli --git https://github.com/technocreatives/dbc-codegen --branch main
```

## Using dbc-codegen

Generate `messages.rs` from `example.dbc` using the CLI:

```bash
dbc-codegen testing/dbc-examples/example.dbc dir/where/messages_rs/file/is/written
```

Or put something like this into your `build.rs` file. Create a `Config` and pass it to `codegen` along with the contents of a DBC-file. See `Config` docs for a complete list of options.


```rust,no_run
use std::env;
use std::path::PathBuf;
use std::fs;

use dbc_codegen::{codegen, Config, FeatureConfig};

fn main() {
    let dbc_path = "../dbc-examples/example.dbc";
    let dbc_file = fs::read_to_string(dbc_path).unwrap();
    println!("cargo:rerun-if-changed={dbc_path}");

    let config = Config::builder()
        .dbc_name("example.dbc")
        .dbc_content(&dbc_file)
        //.impl_arbitrary(FeatureConfig::Gated("arbitrary")) // optional
        //.impl_debug(FeatureConfig::Always)                 // optional
        .build();

    let mut out = Vec::<u8>::new();
    dbc_codegen::codegen(config, &mut out).expect("dbc-codegen failed");
    fs::write(
        PathBuf::from(env::var("OUT_DIR").unwrap()).join("messages.rs"),
        String::from_utf8(out).unwrap(),
    ).unwrap();
}
```

## Using generated Rust code

dbc-codegen generates a Rust file that is usually placed into the `OUT_DIR` directory.
Here is an example [`testing/can-messages/Cargo.toml`](testing/can-messages/Cargo.toml) which defines dependencies and features that are used in generated message file.

### Project setup

To use the code, add this code to your `lib.rs` (or `main.rs`):

```rust,ignore
// Import the generated code.
mod messages {
    include!(concat!(env!("OUT_DIR"), "/messages.rs"));
}
```

You will most likely want to interact with the generated `Messages` enum, and call `Messages::from_can_message(id, &payload)`.

Note: The generated code contains a lot of documentation.
Give it a try:

```bash
cargo doc --open
```

### Optional impls

The generator config has the following flags that control what code gets generated:

- `impl_debug`: enables `#[derive(Debug)]` for messages.
- `impl_arbitrary`: enables implementation of [`Arbitrary`](https://docs.rs/arbitrary/1.0.0/arbitrary/trait.Arbitrary.html) trait.
  Also requires you to add `arbitrary` crate (version 1.x) as a dependency of the crate.
- `impl_error`: Implements `core::error::Error` for `CanError`. This makes it easy to use crates like `anyhow` for error handling.
- `check_ranges`: adds range checks in signal setters. (Enabled by default)

These implementations can be enabled, disabled, or placed behind feature guards, like so:

```rust,no_run
use dbc_codegen::{Config, FeatureConfig};

Config::builder()
    // this will generate Debug implementations
    .impl_debug(FeatureConfig::Always)

    // this will generate Error implementations behind `#[cfg(feature = "std")]` guards
    .impl_error(FeatureConfig::Gated("std"))

    // this will disable range checks
    .check_ranges(FeatureConfig::Never);
```

### no_std

The generated code is no_std compatible, unless you enable `impl_error`.

### Field/variant rename rules

If some field name starts with a non-alphabetic character or is a Rust keyword then it is prefixed with `x`.

For example:

```dbc
VAL_ 512 Five 0 "0Off" 1 "1On" 2 "2Oner" 3 "3Onest";
```

…is generated as:

```rust
pub enum BarFive {
    X0off,
    X1on,
    X2oner,
    X3onest,
    _Other(bool),
}
```

`Type` here:

```dbc
SG_ Type : 30|1@0+ (1,0) [0|1] "boolean" Dolor
```

…conflicts with the Rust keyword `type`. Therefore, we prefix it with `x`:

```rust,ignore
pub fn xtype(&self) -> BarType {
    match self.xtype_raw() {
        false => BarType::X0off,
        true => BarType::X1on,
        x => BarType::_Other(x),
    }
}
```

## Development
* This project is easier to develop with [just](https://just.systems/man/en/), a modern alternative to `make`.
* To get a list of available commands, run `just`.
* To run tests, use `just test`.

### lorri for Nix

If using Nix, dbc-codegen is integrated with [lorri](https://github.com/nix-community/lorri) for easy project dependency management. To enable, create a symlink in the top-level working directory:

```sh
ln -s envrc.lorri .envrc
```

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
  at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual-licensed as above, without any
additional terms or conditions.
