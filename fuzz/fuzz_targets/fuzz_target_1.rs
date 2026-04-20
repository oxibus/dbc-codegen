#![no_main]
use libfuzzer_sys::fuzz_target;
use std::ptr::addr_of;

use cantools_messages::{
    example_bar_four_encode, example_bar_one_encode, example_bar_pack, example_bar_t,
    example_bar_three_encode, example_bar_two_encode, example_bar_type_encode,
};

fuzz_target!(|dbc_codegen_bar: can_messages::Bar| {
    let one = unsafe { example_bar_one_encode(dbc_codegen_bar.one_raw() as f64) };
    let two = unsafe { example_bar_two_encode(dbc_codegen_bar.two_raw() as f64) };
    let three = unsafe { example_bar_three_encode(dbc_codegen_bar.three_raw() as f64) };
    let four = unsafe { example_bar_four_encode(dbc_codegen_bar.four_raw() as f64) };
    let type_ = unsafe { example_bar_type_encode(dbc_codegen_bar.xtype_raw() as u8 as f64) };

    let bar = example_bar_t {
        one,
        two,
        three,
        four,
        type_,
    };
    let mut buffer: [u8; 8] = [0; 8];
    unsafe { example_bar_pack(buffer.as_mut_ptr(), addr_of!(bar), buffer.len()) };

    // Compare dbc-codegen raw bytes with cantools C library output.
    // Cantools C uses truncation for encode (e.g. (uint8_t)(value/0.39)) while dbc-codegen
    // may round; allow off-by-one in the first byte (Two signal) due to this.
    let dbc_raw = dbc_codegen_bar.raw();
    if dbc_raw != buffer.as_slice() {
        let diff_ok = dbc_raw.len() == 8
            && buffer[1..] == dbc_raw[1..]
            && dbc_raw[0].abs_diff(buffer[0]) <= 1;
        if !diff_ok {
            panic!(
                "dbc-codegen and cantools disagree: {:?} vs {:?}",
                dbc_raw, &buffer
            );
        }
    }
});
