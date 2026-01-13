#![no_std]

#[expect(clippy::disallowed_names)]
mod messages {
    include!(concat!(env!("OUT_DIR"), "/messages.rs"));
}
