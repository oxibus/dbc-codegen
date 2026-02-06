# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0](https://github.com/oxibus/dbc-codegen/compare/v0.3.0...v0.4.0) - 2026-02-06

### Added

- add `DBC_FILE_NAME` + ver, proper code formatting ([#131](https://github.com/oxibus/dbc-codegen/pull/131))
- [**breaking**] streamline DBC code generation and file writing ([#128](https://github.com/oxibus/dbc-codegen/pull/128))

### Fixed

- handle duplicate signal names ([#126](https://github.com/oxibus/dbc-codegen/pull/126))
- handle empty relevant msgs in from_can_message ([#125](https://github.com/oxibus/dbc-codegen/pull/125))
- fix typo
- fix formatting errors
- fix factor/offset not getting applied to 0 for unsigned case
- fix duplicate enum names on multiplex messages
- fix issue where signed signals would be rendered as unsigned

### Other

- *(deps)* bump the all-cargo-version-updates group across 1 directory with 2 updates ([#145](https://github.com/oxibus/dbc-codegen/pull/145))
- *(deps)* bump insta from 1.46.2 to 1.46.3 in the all-cargo-version-updates group ([#143](https://github.com/oxibus/dbc-codegen/pull/143))
- upgrade to rust 1.93 stderr ([#142](https://github.com/oxibus/dbc-codegen/pull/142))
- *(deps)* bump the all-cargo-version-updates group across 1 directory with 4 updates ([#141](https://github.com/oxibus/dbc-codegen/pull/141))
- [**breaking**] remove main fn from generated code ([#138](https://github.com/oxibus/dbc-codegen/pull/138))
- make many rendering fn methods on config ([#133](https://github.com/oxibus/dbc-codegen/pull/133))
- *(deps)* bump clap from 4.5.54 to 4.5.55 in the all-cargo-version-updates group ([#134](https://github.com/oxibus/dbc-codegen/pull/134))
- use `quote` crate to handle str escaping ([#132](https://github.com/oxibus/dbc-codegen/pull/132))
- noop - dedup, cleanup, new type abstraction ([#130](https://github.com/oxibus/dbc-codegen/pull/130))
- update shared tests, bump deps, freeze to rust 1.92  ([#127](https://github.com/oxibus/dbc-codegen/pull/127))
- *(deps)* bump can-dbc from 8.0.0 to 8.0.1 in the all-cargo-version-updates group ([#129](https://github.com/oxibus/dbc-codegen/pull/129))
- do not clean lock
- bump dependencies and enable verbose output for release actions ([#120](https://github.com/oxibus/dbc-codegen/pull/120))
- [pre-commit.ci] pre-commit autoupdate ([#119](https://github.com/oxibus/dbc-codegen/pull/119))
- Bump the all-cargo-version-updates group across 1 directory with 2 updates ([#118](https://github.com/oxibus/dbc-codegen/pull/118))
- refactor write functions, improve CI, MSRV ([#116](https://github.com/oxibus/dbc-codegen/pull/116))
- minor code cleanup ([#114](https://github.com/oxibus/dbc-codegen/pull/114))
- generate and compile all .dbc files from test repo ([#113](https://github.com/oxibus/dbc-codegen/pull/113))
- allow generated file to be used with `include!` ([#108](https://github.com/oxibus/dbc-codegen/pull/108))
- Bump the all-cargo-version-updates group across 1 directory with 2 updates ([#111](https://github.com/oxibus/dbc-codegen/pull/111))
- minor justfile fix, editorconfig ([#112](https://github.com/oxibus/dbc-codegen/pull/112))
- Bump actions/cache from 4 to 5 in the all-actions-version-updates group ([#110](https://github.com/oxibus/dbc-codegen/pull/110))
- OxiBUS onboarding cleanup ([#102](https://github.com/oxibus/dbc-codegen/pull/102))
- run `just fmt` to cleanup imports ([#107](https://github.com/oxibus/dbc-codegen/pull/107))
- Bump the all-cargo-version-updates group across 1 directory with 9 updates ([#106](https://github.com/oxibus/dbc-codegen/pull/106))
- upgrade to can-dbc v8 ([#101](https://github.com/oxibus/dbc-codegen/pull/101))
- Bump actions/checkout from 1 to 6 in the all-actions-version-updates group ([#104](https://github.com/oxibus/dbc-codegen/pull/104))
- *(CI)* minor CI updates, automate dependabot ([#103](https://github.com/oxibus/dbc-codegen/pull/103))
- Add optional defmt::Format support for generated types
- Merge pull request #85 from inomotech-foss/fix-mux-msg-id
- Use message name instead of self
- Generate embedded_can::Frame trait for each frame
- Represent CAN message IDs as embedded_can::Id to support extended IDs
- Fix issue with decoding signed values of non-standard length
- revert change to Cargo.toml, not necessary
- address clippy's concerns (thanks clippy), enhance comments
- make can-messages dependency optional so it doesn't pull in std
- reduce diff against main
- handle more edge cases, use i128 as the maximum type
- reorder lines to match signals
- drop symlink in favor of copy so it works on Windows
- revert because I missed the case of check_ranges being false
- add tests, edge cases where the min/max is way less than signal width
- clean up warning
- add unsigned to the name, add test case
- change signal logic to handle negative factors, add tests
- Bump rustc to latest stable, 1.78
- Merge pull request #68 from projectgus/tweaks/codegen_warnings
- Rename .envrc for non-Nix users, add note in README
- dont assign signed integers to unsigned integers
- Update README
- Fix clippy warnings
- Use checked_sub(offset) for set_signal methods
- Don't treat signals as floats if factor is integer
- Add flag to impl std Error
- Tweak docs and optional features
- Make feature-gated impls configurable
