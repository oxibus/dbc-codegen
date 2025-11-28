#[expect(clippy::disallowed_names)]
mod messages {
    include!(concat!(env!("OUT_DIR"), "/messages.rs"));
}

pub use messages::*;
