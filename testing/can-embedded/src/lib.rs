#![no_std]

#[rustfmt::skip]
#[expect(dead_code, unused_comparisons)]
#[expect(clippy::excessive_precision, clippy::manual_range_contains, clippy::useless_conversion, clippy::absurd_extreme_comparisons, clippy::unnecessary_cast, clippy::disallowed_names)]
mod messages {
    include!(concat!(env!("OUT_DIR"), "/messages.rs"));
}
