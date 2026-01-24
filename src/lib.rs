#![deny(missing_docs)]
#![deny(clippy::arithmetic_side_effects)]
#![allow(clippy::needless_doctest_main)]
#![cfg_attr(feature = "std", doc = include_str!("../README.md"))]
#![cfg_attr(
    not(feature = "std"),
    doc = "Documentation is only available with the `std` feature."
)]

mod feature_config;
mod includes;
mod keywords;
mod pad;
mod utils;

use std::cmp::{max, min};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;

use anyhow::{anyhow, ensure, Context, Error, Result};
use can_dbc::ByteOrder::{BigEndian, LittleEndian};
use can_dbc::MultiplexIndicator::{
    MultiplexedSignal, Multiplexor, MultiplexorAndMultiplexedSignal, Plain,
};
use can_dbc::ValueType::{Signed, Unsigned};
use can_dbc::{
    Dbc, Message, MessageId, Signal, Transmitter, ValDescription, ValueDescription, ValueType,
};
pub use feature_config::FeatureConfig;
use heck::{ToPascalCase, ToSnakeCase};
use pad::PadAdapter;
use typed_builder::TypedBuilder;
use utils::{enum_variant_name, MessageExt as _, SignalExt as _};

static ALLOW_DEADCODE: &str = "#[allow(dead_code)]";
static ALLOW_LINTS: &str = r"#[allow(
    clippy::absurd_extreme_comparisons,
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::unnecessary_cast,
    clippy::useless_conversion,
    unused_comparisons,
    unused_variables,
)]";

/// Code generator configuration. See module-level docs for an example.
#[derive(TypedBuilder)]
#[non_exhaustive]
pub struct Config<'a> {
    /// Name of the dbc-file. Used for generated docs only.
    pub dbc_name: &'a str,

    /// Content of a dbc-file as a UTF-8 string. Use [`can_dbc::decode_cp1252`] or other encodings if needed.
    pub dbc_content: &'a str,

    /// Optional: Print debug info to stdout while generating code. Default: `false`.
    #[builder(default)]
    pub debug_prints: bool,

    /// Optional: `impl Debug` for generated types. Default: `Never`.
    #[builder(default)]
    pub impl_debug: FeatureConfig<'a>,

    /// Optional: `impl defmt::Format` for generated types. Default: `Never`.
    #[builder(default)]
    pub impl_defmt: FeatureConfig<'a>,

    /// Optional: `impl Arbitrary` for generated types. Default: `Never`.
    #[builder(default)]
    pub impl_arbitrary: FeatureConfig<'a>,

    /// Optional: `impl Serialize` and `impl Deserialize` for generated types.. Default: `Never`.
    #[builder(default)]
    pub impl_serde: FeatureConfig<'a>,

    /// Optional: `impl Error` for generated error type. Default: `Never`.
    ///
    /// Note: this feature depends on `std`.
    #[builder(default)]
    pub impl_error: FeatureConfig<'a>,

    /// Optional: Generate `embedded_can::Frame` impl for each frame. Default: `Always`
    #[builder(default = FeatureConfig::Always)]
    pub impl_embedded_can_frame: FeatureConfig<'a>,

    /// Optional: Validate min and max values in generated signal setters. Default: `Always`
    #[builder(default = FeatureConfig::Always)]
    pub check_ranges: FeatureConfig<'a>,

    /// Optional: Allow dead code in the generated module. Default: `false`.
    #[builder(default)]
    pub allow_dead_code: bool,
}

/// Write Rust structs matching DBC input description to `out` buffer
fn codegen(config: &Config<'_>, out: impl Write) -> Result<()> {
    let dbc = Dbc::try_from(config.dbc_content).map_err(|e| {
        let msg = "Could not parse dbc file";
        if config.debug_prints {
            anyhow!("{msg}: {e:#?}")
        } else {
            anyhow!("{msg}")
        }
    })?;
    if config.debug_prints {
        eprintln!("{dbc:#?}");
    }
    let mut w = BufWriter::new(out);

    writeln!(w, "// Generated code!")?;
    writeln!(w, "//")?;
    writeln!(w, "// Message definitions from file `{}`", config.dbc_name)?;
    writeln!(w, "// Version: {}", dbc.version.0)?;
    writeln!(w)?;
    writeln!(w, "#[allow(unused_imports)]")?;
    writeln!(w, "use core::ops::BitOr;")?;
    writeln!(w, "#[allow(unused_imports)]")?;
    writeln!(w, "use bitvec::prelude::*;")?;
    writeln!(w, "#[allow(unused_imports)]")?;
    writeln!(w, "use embedded_can::{{Id, StandardId, ExtendedId}};")?;

    config.impl_arbitrary.fmt_cfg(&mut w, |w| {
        writeln!(w, "use arbitrary::{{Arbitrary, Unstructured}};")
    })?;

    config.impl_serde.fmt_cfg(&mut w, |w| {
        writeln!(w, "use serde::{{Serialize, Deserialize}};")
    })?;

    writeln!(w)?;

    render_dbc(&mut w, config, &dbc).context("could not generate Rust code")?;

    writeln!(w)?;
    writeln!(w, "/// This is just to make testing easier")?;
    writeln!(w, "#[allow(dead_code)]")?;
    writeln!(w, "fn main() {{}}")?;
    writeln!(w)?;
    render_error(&mut w, config)?;
    render_arbitrary_helpers(&mut w, config)?;
    writeln!(w)?;

    Ok(())
}

fn render_dbc(w: &mut impl Write, config: &Config<'_>, dbc: &Dbc) -> Result<()> {
    render_root_enum(w, dbc, config)?;

    for msg in get_relevant_messages(dbc) {
        render_message(w, config, msg, dbc)
            .with_context(|| format!("write message `{}`", msg.name))?;
        writeln!(w)?;
    }

    Ok(())
}

fn render_root_enum(w: &mut impl Write, dbc: &Dbc, config: &Config<'_>) -> Result<()> {
    writeln!(w, "/// All messages")?;
    writeln!(w, "{ALLOW_LINTS}")?;
    config.write_allow_dead_code(w)?;
    writeln!(w, "#[derive(Clone)]")?;
    config.impl_debug.fmt_attr(w, "derive(Debug)")?;
    config.impl_defmt.fmt_attr(w, "derive(defmt::Format)")?;
    config.impl_serde.fmt_attr(w, "derive(Serialize)")?;
    config.impl_serde.fmt_attr(w, "derive(Deserialize)")?;
    writeln!(w, "pub enum Messages {{")?;
    {
        let mut w = PadAdapter::wrap(w);
        for msg in get_relevant_messages(dbc) {
            writeln!(w, "/// {}", msg.name)?;
            writeln!(w, "{0}({0}),", msg.type_name())?;
        }
    }
    writeln!(w, "}}")?;
    writeln!(w)?;

    writeln!(w, "{ALLOW_LINTS}")?;
    config.write_allow_dead_code(w)?;
    writeln!(w, "impl Messages {{")?;
    {
        let mut w = PadAdapter::wrap(w);
        writeln!(w, "/// Read message from CAN frame")?;
        writeln!(w, "#[inline(never)]")?;
        writeln!(
            w,
            "pub fn from_can_message(id: Id, payload: &[u8]) -> Result<Self, CanError> {{",
        )?;

        {
            let mut w = PadAdapter::wrap(&mut w);
            let messages: Vec<_> = get_relevant_messages(dbc).collect();
            if messages.is_empty() {
                writeln!(w, "Err(CanError::UnknownMessageId(id))")?;
            } else {
                writeln!(w)?;
                writeln!(w, "let res = match id {{")?;
                {
                    let mut w = PadAdapter::wrap(&mut w);
                    for msg in messages {
                        writeln!(
                            w,
                            "{0}::MESSAGE_ID => Messages::{0}({0}::try_from(payload)?),",
                            msg.type_name(),
                        )?;
                    }
                    writeln!(w, r"id => return Err(CanError::UnknownMessageId(id)),")?;
                }
                writeln!(w, "}};")?;
                writeln!(w, "Ok(res)")?;
            }
        }

        writeln!(w, "}}")?;
    }
    writeln!(w, "}}")?;
    writeln!(w)?;

    Ok(())
}

fn render_message(w: &mut impl Write, config: &Config<'_>, msg: &Message, dbc: &Dbc) -> Result<()> {
    writeln!(w, "/// {}", msg.name)?;
    writeln!(w, "///")?;
    match msg.id {
        MessageId::Standard(id) => writeln!(w, "/// - Standard ID: {id} (0x{id:x})"),
        MessageId::Extended(id) => writeln!(w, "/// - Extended ID: {id} (0x{id:x})"),
    }?;
    writeln!(w, "/// - Size: {} bytes", msg.size)?;
    if let Transmitter::NodeName(transmitter) = &msg.transmitter {
        writeln!(w, "/// - Transmitter: {transmitter}")?;
    }
    if let Some(comment) = dbc.message_comment(msg.id) {
        writeln!(w, "///")?;
        for line in comment.trim().lines() {
            writeln!(w, "/// {line}")?;
        }
    }
    writeln!(w, "#[derive(Clone, Copy)]")?;
    config.impl_serde.fmt_attr(w, "derive(Serialize)")?;
    config.impl_serde.fmt_attr(w, "derive(Deserialize)")?;
    writeln!(w, "pub struct {} {{", msg.type_name())?;
    {
        let mut w = PadAdapter::wrap(w);
        config
            .impl_serde
            .fmt_attr(&mut w, "serde(with = \"serde_bytes\")")?;
        writeln!(w, "raw: [u8; {}],", msg.size)?;
    }
    writeln!(w, "}}")?;
    writeln!(w)?;

    writeln!(w, "{ALLOW_LINTS}")?;
    config.write_allow_dead_code(w)?;
    writeln!(w, "impl {} {{", msg.type_name())?;
    {
        let mut w = PadAdapter::wrap(w);

        writeln!(
            w,
            "pub const MESSAGE_ID: embedded_can::Id = {};",
            match msg.id {
                // use StandardId::new().unwrap() once const_option is stable
                MessageId::Standard(id) =>
                    format!("Id::Standard(unsafe {{ StandardId::new_unchecked({id:#x})}})"),
                MessageId::Extended(id) =>
                    format!("Id::Extended(unsafe {{ ExtendedId::new_unchecked({id:#x})}})"),
            }
        )?;
        writeln!(w)?;

        for signal in &msg.signals {
            let typ = signal_to_rust_type(signal);
            if typ != "bool" {
                let sig = signal.field_name().to_uppercase();
                let min = signal.min;
                let max = signal.max;
                writeln!(w, "pub const {sig}_MIN: {typ} = {min}_{typ};")?;
                writeln!(w, "pub const {sig}_MAX: {typ} = {max}_{typ};")?;
            }
        }
        writeln!(w)?;

        writeln!(w, "/// Construct new {} from values", msg.name)?;
        let args = msg
            .signals
            .iter()
            .filter_map(|signal| {
                if matches!(signal.multiplexer_indicator, Plain | Multiplexor) {
                    let field = signal.field_name();
                    let typ = signal_to_rust_type(signal);
                    Some(format!("{field}: {typ}"))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .join(", ");
        writeln!(w, "pub fn new({args}) -> Result<Self, CanError> {{")?;
        {
            let mut w = PadAdapter::wrap(&mut w);
            let mutable = if msg.signals.is_empty() { "" } else { "mut " };
            let size = msg.size;
            writeln!(w, "let {mutable}res = Self {{ raw: [0u8; {size}] }};")?;
            for signal in &msg.signals {
                if matches!(signal.multiplexer_indicator, Plain | Multiplexor) {
                    writeln!(w, "res.set_{0}({0})?;", signal.field_name())?;
                }
            }
            writeln!(w, "Ok(res)")?;
        }
        writeln!(w, "}}")?;
        writeln!(w)?;

        writeln!(w, "/// Access message payload raw value")?;
        writeln!(w, "pub fn raw(&self) -> &[u8; {}] {{", msg.size)?;
        {
            let mut w = PadAdapter::wrap(&mut w);
            writeln!(w, "&self.raw")?;
        }
        writeln!(w, "}}")?;
        writeln!(w)?;

        for signal in &msg.signals {
            match signal.multiplexer_indicator {
                Plain => render_signal(&mut w, config, signal, dbc, msg)
                    .with_context(|| format!("write signal impl `{}`", signal.name))?,
                Multiplexor => {
                    render_multiplexor_signal(&mut w, config, signal, msg)?;
                }
                MultiplexedSignal(_) | MultiplexorAndMultiplexedSignal(_) => {}
            }
        }
    }

    writeln!(w, "}}")?;
    writeln!(w)?;

    let typ = msg.type_name();
    writeln!(w, "impl core::convert::TryFrom<&[u8]> for {typ} {{")?;
    {
        let mut w = PadAdapter::wrap(w);
        writeln!(w, "type Error = CanError;")?;
        writeln!(w)?;
        writeln!(w, "#[inline(always)]")?;
        writeln!(
            w,
            "fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {{",
        )?;
        {
            let mut w = PadAdapter::wrap(&mut w);
            writeln!(
                w,
                r"if payload.len() != {} {{ return Err(CanError::InvalidPayloadSize); }}",
                msg.size,
            )?;
            writeln!(w, "let mut raw = [0u8; {}];", msg.size)?;
            writeln!(w, "raw.copy_from_slice(&payload[..{}]);", msg.size)?;
            writeln!(w, "Ok(Self {{ raw }})")?;
        }
        writeln!(w, "}}")?;
    }
    writeln!(w, "}}")?;
    writeln!(w)?;

    render_embedded_can_frame(w, config, msg)?;

    config
        .impl_debug
        .fmt_cfg(&mut *w, |w| render_debug_impl(w, msg))?;
    config
        .impl_defmt
        .fmt_cfg(&mut *w, |w| render_defmt_impl(w, msg))?;
    config
        .impl_arbitrary
        .fmt_cfg(&mut *w, |w| render_arbitrary(w, config, msg))?;

    let enums_for_this_message = dbc.value_descriptions.iter().filter_map(|x| {
        if let ValueDescription::Signal {
            message_id,
            name,
            value_descriptions,
        } = x
        {
            if *message_id != msg.id {
                return None;
            }
            dbc.signal_by_name(*message_id, name)
                .map(|v| (v, value_descriptions))
        } else {
            None
        }
    });
    for (signal, variants) in enums_for_this_message {
        write_enum(w, config, signal, msg, variants.as_slice())?;
    }

    let multiplexor_signal = msg
        .signals
        .iter()
        .find(|s| s.multiplexer_indicator == Multiplexor);

    if let Some(multiplexor_signal) = multiplexor_signal {
        render_multiplexor_enums(w, config, dbc, msg, multiplexor_signal)?;
    }

    Ok(())
}

fn render_signal(
    w: &mut impl Write,
    config: &Config<'_>,
    signal: &Signal,
    dbc: &Dbc,
    msg: &Message,
) -> Result<()> {
    writeln!(w, "/// {}", signal.name)?;
    if let Some(comment) = dbc.signal_comment(msg.id, &signal.name) {
        writeln!(w, "///")?;
        for line in comment.trim().lines() {
            writeln!(w, "/// {line}")?;
        }
    }
    writeln!(w, "///")?;
    writeln!(w, "/// - Min: {}", signal.min)?;
    writeln!(w, "/// - Max: {}", signal.max)?;
    writeln!(w, "/// - Unit: {:?}", signal.unit)?;
    writeln!(w, "/// - Receivers: {}", signal.receivers.join(", "))?;
    writeln!(w, "#[inline(always)]")?;
    let fn_name = signal.field_name();
    if let Some(variants) = dbc.value_descriptions_for_signal(msg.id, &signal.name) {
        let type_name = enum_name(msg, signal);
        let signal_rust_type = signal_to_rust_type(signal);
        let variant_infos = generate_variant_info(variants, &signal_rust_type);

        writeln!(w, "pub fn {fn_name}(&self) -> {type_name} {{")?;
        {
            let mut w = PadAdapter::wrap(w);

            // Use signed type for loading when signal is signed and has negative values
            let has_negative_values = variants.iter().any(|v| v.id < 0);
            let load_type = if signal.value_type == Signed && has_negative_values {
                signal_rust_type
            } else {
                signal_to_rust_uint(signal)
            };

            let read = read_fn_with_type(signal, msg, &load_type)?;
            writeln!(w, r"let signal = {read};")?;
            writeln!(w)?;
            writeln!(w, "match signal {{")?;
            {
                let mut w = PadAdapter::wrap(&mut w);
                for info in &variant_infos {
                    let literal = info.value;
                    let variant = &info.base_name;
                    match info.dup_type {
                        DuplicateType::Unique => {
                            writeln!(w, "{literal} => {type_name}::{variant},")?;
                        }
                        DuplicateType::FirstDuplicate | DuplicateType::Duplicate => {
                            writeln!(w, "{literal} => {type_name}::{variant}({literal}),")?;
                        }
                    }
                }
                writeln!(w, "_ => {type_name}::_Other(self.{fn_name}_raw()),")?;
            }
            writeln!(w, "}}")?;
        }
        writeln!(w, "}}")?;
        writeln!(w)?;
    } else {
        let typ = signal_to_rust_type(signal);
        writeln!(w, "pub fn {fn_name}(&self) -> {typ} {{")?;
        {
            let mut w = PadAdapter::wrap(w);
            writeln!(w, "self.{fn_name}_raw()")?;
        }
        writeln!(w, "}}")?;
        writeln!(w)?;
    }

    writeln!(w, "/// Get raw value of {}", signal.name)?;
    writeln!(w, "///")?;
    writeln!(w, "/// - Start bit: {}", signal.start_bit)?;
    writeln!(w, "/// - Signal size: {} bits", signal.size)?;
    writeln!(w, "/// - Factor: {}", signal.factor)?;
    writeln!(w, "/// - Offset: {}", signal.offset)?;
    writeln!(w, "/// - Byte order: {:?}", signal.byte_order)?;
    writeln!(w, "/// - Value type: {:?}", signal.value_type)?;
    writeln!(w, "#[inline(always)]")?;
    let typ = signal_to_rust_type(signal);
    writeln!(w, "pub fn {fn_name}_raw(&self) -> {typ} {{")?;
    {
        let mut w = PadAdapter::wrap(w);
        signal_from_payload(&mut w, signal, msg).context("signal from payload")?;
    }
    writeln!(w, "}}")?;
    writeln!(w)?;

    render_set_signal(w, config, signal, msg)?;

    Ok(())
}

fn render_set_signal(
    w: &mut impl Write,
    config: &Config<'_>,
    signal: &Signal,
    msg: &Message,
) -> Result<()> {
    writeln!(w, "/// Set value of {}", signal.name)?;
    writeln!(w, "#[inline(always)]")?;

    // To avoid accidentally changing the multiplexor value without changing
    // the signals accordingly this fn is kept private for multiplexors.
    let visibility = if signal.multiplexer_indicator == Multiplexor {
        ""
    } else {
        "pub "
    };

    let field = signal.field_name();
    let typ = signal_to_rust_type(signal);
    writeln!(
        w,
        "{visibility}fn set_{field}(&mut self, value: {typ}) -> Result<(), CanError> {{",
    )?;

    {
        let mut w = PadAdapter::wrap(w);

        if signal.size != 1 {
            if let FeatureConfig::Gated(gate) = config.check_ranges {
                writeln!(w, r"#[cfg(feature = {gate:?})]")?;
            }

            if let FeatureConfig::Gated(..) | FeatureConfig::Always = config.check_ranges {
                let typ = signal_to_rust_type(signal);
                let min = signal.min;
                let max = signal.max;
                writeln!(w, r"if value < {min}_{typ} || {max}_{typ} < value {{")?;

                {
                    let mut w = PadAdapter::wrap(&mut w);
                    let typ = msg.type_name();
                    writeln!(
                        w,
                        r"return Err(CanError::ParameterOutOfRange {{ message_id: {typ}::MESSAGE_ID }});",
                    )?;
                }

                writeln!(w, r"}}")?;
            }
        }
        signal_to_payload(&mut w, signal, msg).context("signal to payload")?;
    }

    writeln!(w, "}}")?;
    writeln!(w)?;

    Ok(())
}

fn render_set_signal_multiplexer(
    w: &mut impl Write,
    multiplexor: &Signal,
    msg: &Message,
    switch_index: u64,
) -> Result<()> {
    writeln!(w, "/// Set value of {}", multiplexor.name)?;
    writeln!(w, "#[inline(always)]")?;
    writeln!(
        w,
        "pub fn set_{enum_variant_wrapper}(&mut self, value: {enum_variant}) -> Result<(), CanError> {{",
        enum_variant_wrapper = multiplexed_enum_variant_wrapper_name(switch_index).to_snake_case(),
        enum_variant = multiplexed_enum_variant_name(msg, multiplexor, switch_index)?,
    )?;

    {
        let mut w = PadAdapter::wrap(w);

        writeln!(w, "let b0 = BitArray::<_, LocalBits>::new(self.raw);")?;
        writeln!(w, "let b1 = BitArray::<_, LocalBits>::new(value.raw);")?;
        writeln!(w, "self.raw = b0.bitor(b1).into_inner();")?;
        writeln!(w, "self.set_{}({switch_index})?;", multiplexor.field_name())?;
        writeln!(w, "Ok(())")?;
    }

    writeln!(w, "}}")?;
    writeln!(w)?;

    Ok(())
}

fn render_multiplexor_signal(
    w: &mut impl Write,
    config: &Config<'_>,
    signal: &Signal,
    msg: &Message,
) -> Result<()> {
    writeln!(w, "/// Get raw value of {}", signal.name)?;
    writeln!(w, "///")?;
    writeln!(w, "/// - Start bit: {}", signal.start_bit)?;
    writeln!(w, "/// - Signal size: {} bits", signal.size)?;
    writeln!(w, "/// - Factor: {}", signal.factor)?;
    writeln!(w, "/// - Offset: {}", signal.offset)?;
    writeln!(w, "/// - Byte order: {:?}", signal.byte_order)?;
    writeln!(w, "/// - Value type: {:?}", signal.value_type)?;
    writeln!(w, "#[inline(always)]")?;
    let field = signal.field_name();
    let typ = signal_to_rust_type(signal);
    writeln!(w, "pub fn {field}_raw(&self) -> {typ} {{")?;
    {
        let mut w = PadAdapter::wrap(w);
        signal_from_payload(&mut w, signal, msg).context("signal from payload")?;
    }
    writeln!(w, "}}")?;
    writeln!(w)?;

    let field = signal.field_name();
    let typ = multiplex_enum_name(msg, signal)?;
    writeln!(w, "pub fn {field}(&mut self) -> Result<{typ}, CanError> {{")?;

    let multiplexer_indexes: BTreeSet<u64> = msg
        .signals
        .iter()
        .filter_map(|s| {
            if let MultiplexedSignal(index) = &s.multiplexer_indicator {
                Some(index)
            } else {
                None
            }
        })
        .copied()
        .collect();

    {
        let mut w = PadAdapter::wrap(w);
        writeln!(w, "match self.{}_raw() {{", signal.field_name())?;

        {
            let mut w = PadAdapter::wrap(&mut w);
            for multiplexer_index in &multiplexer_indexes {
                writeln!(
                    w,
                    "{idx} => Ok({enum_name}::{multiplexed_wrapper_name}({multiplexed_name}{{ raw: self.raw }})),",
                    idx = multiplexer_index,
                    enum_name = multiplex_enum_name(msg, signal)?,
                    multiplexed_wrapper_name = multiplexed_enum_variant_wrapper_name(*multiplexer_index),
                    multiplexed_name =
                        multiplexed_enum_variant_name(msg, signal, *multiplexer_index)?,
                )?;
            }
            writeln!(
                w,
                "multiplexor => Err(CanError::InvalidMultiplexor {{ message_id: {}::MESSAGE_ID, multiplexor: multiplexor.into() }}),",
                msg.type_name(),
            )?;
        }

        writeln!(w, "}}")?;
    }
    writeln!(w, "}}")?;

    render_set_signal(w, config, signal, msg)?;

    for switch_index in multiplexer_indexes {
        render_set_signal_multiplexer(w, signal, msg, switch_index)?;
    }

    Ok(())
}

fn be_start_end_bit(signal: &Signal, msg: &Message) -> Result<(u64, u64)> {
    let err = "calculating start bit";

    let x = signal.start_bit.checked_div(8).context(err)?;
    let x = x.checked_mul(8).context(err)?;

    let y = signal.start_bit.checked_rem(8).context(err)?;
    let y = 7u64.checked_sub(y).context(err)?;

    let start_bit = x.checked_add(y).context(err)?;
    let end_bit = start_bit
        .checked_add(signal.size)
        .context("calculating last bit position")?;

    let msg_bits = msg.size.checked_mul(8).unwrap();

    ensure!(
        start_bit <= msg_bits,
        "signal starts at {start_bit}, but message is only {msg_bits} bits",
    );
    ensure!(
        end_bit <= msg_bits,
        "signal ends at {end_bit}, but message is only {msg_bits} bits",
    );
    Ok((start_bit, end_bit))
}

fn le_start_end_bit(signal: &Signal, msg: &Message) -> Result<(u64, u64)> {
    let msg_bits = msg.size.checked_mul(8).unwrap();
    let start_bit = signal.start_bit;
    ensure!(
        start_bit <= msg_bits,
        "signal starts at {start_bit}, but message is only {msg_bits} bits",
    );

    let end_bit = signal
        .start_bit
        .checked_add(signal.size)
        .context("overflow calculating last bit position")?;
    ensure!(
        end_bit <= msg_bits,
        "signal ends at {end_bit}, but message is only {msg_bits} bits",
    );
    Ok((start_bit, end_bit))
}

fn signal_from_payload(w: &mut impl Write, signal: &Signal, msg: &Message) -> Result<()> {
    writeln!(w, r"let signal = {};", read_fn(signal, msg)?)?;
    writeln!(w)?;

    if signal.size == 1 {
        writeln!(w, "signal == 1")?;
    } else if signal_is_float_in_rust(signal) {
        // Scaling is always done on floats
        writeln!(w, "let factor = {}_f32;", signal.factor)?;
        writeln!(w, "let offset = {}_f32;", signal.offset)?;
        writeln!(w, "(signal as f32) * factor + offset")?;
    } else {
        writeln!(w, "let factor = {};", signal.factor)?;
        let scaled_type = scaled_signal_to_rust_int(signal);

        if scaled_type == signal_to_rust_uint(signal).replace('u', "i") {
            // Can't do iNN::from(uNN) if they both fit in the same integer type,
            // so cast first
            writeln!(w, "let signal = signal as {scaled_type};")?;
        }

        if signal.offset >= 0.0 {
            writeln!(
                w,
                "{scaled_type}::from(signal).saturating_mul(factor).saturating_add({})",
                signal.offset,
            )?;
        } else {
            writeln!(
                w,
                "{scaled_type}::from(signal).saturating_mul(factor).saturating_sub({})",
                signal.offset.abs(),
            )?;
        }
    }
    Ok(())
}

fn read_fn(signal: &Signal, msg: &Message) -> Result<String> {
    read_fn_with_type(signal, msg, &signal_to_rust_int(signal))
}

fn read_fn_with_type(signal: &Signal, msg: &Message, typ: &str) -> Result<String> {
    Ok(match signal.byte_order {
        LittleEndian => {
            let (start, end) = le_start_end_bit(signal, msg)?;
            format!("self.raw.view_bits::<Lsb0>()[{start}..{end}].load_le::<{typ}>()")
        }
        BigEndian => {
            let (start, end) = be_start_end_bit(signal, msg)?;
            format!("self.raw.view_bits::<Msb0>()[{start}..{end}].load_be::<{typ}>()")
        }
    })
}

fn signal_to_payload(w: &mut impl Write, signal: &Signal, msg: &Message) -> Result<()> {
    if signal.size == 1 {
        // Map boolean to byte so we can pack it
        writeln!(w, "let value = value as u8;")?;
    } else if signal_is_float_in_rust(signal) {
        // Massage value into an int
        writeln!(w, "let factor = {}_f32;", signal.factor)?;
        writeln!(w, "let offset = {}_f32;", signal.offset)?;
        let typ = signal_to_rust_int(signal);
        writeln!(w, "let value = ((value - offset) / factor) as {typ};")?;
        writeln!(w)?;
    } else {
        writeln!(w, "let factor = {};", signal.factor)?;
        if signal.offset >= 0.0 {
            writeln!(w, "let value = value.checked_sub({})", signal.offset)?;
        } else {
            writeln!(w, "let value = value.checked_add({})", signal.offset.abs())?;
        }
        writeln!(
            w,
            "    .ok_or(CanError::ParameterOutOfRange {{ message_id: {}::MESSAGE_ID }})?;",
            msg.type_name(),
        )?;
        let typ = signal_to_rust_int(signal);
        writeln!(w, "let value = (value / factor) as {typ};")?;
        writeln!(w)?;
    }

    if signal.value_type == Signed {
        let typ = signal_to_rust_uint(signal);
        writeln!(w, "let value = {typ}::from_ne_bytes(value.to_ne_bytes());")?;
    }

    match signal.byte_order {
        LittleEndian => {
            let (start, end) = le_start_end_bit(signal, msg)?;
            writeln!(
                w,
                r"self.raw.view_bits_mut::<Lsb0>()[{start}..{end}].store_le(value);",
            )?;
        }
        BigEndian => {
            let (start, end) = be_start_end_bit(signal, msg)?;
            writeln!(
                w,
                r"self.raw.view_bits_mut::<Msb0>()[{start}..{end}].store_be(value);",
            )?;
        }
    }

    writeln!(w, "Ok(())")?;
    Ok(())
}

fn write_enum(
    w: &mut impl Write,
    config: &Config<'_>,
    signal: &Signal,
    msg: &Message,
    variants: &[ValDescription],
) -> Result<()> {
    let type_name = enum_name(msg, signal);
    let signal_rust_type = signal_to_rust_type(signal);

    // Generate variant info to handle duplicates with tuple variants
    let variant_infos = generate_variant_info(variants, &signal_rust_type);

    writeln!(w, "/// Defined values for {}", signal.name)?;
    writeln!(w, "{ALLOW_LINTS}")?;
    config.write_allow_dead_code(w)?;
    writeln!(w, "#[derive(Clone, Copy, PartialEq)]")?;
    config.impl_debug.fmt_attr(w, "derive(Debug)")?;
    config.impl_defmt.fmt_attr(w, "derive(defmt::Format)")?;
    config.impl_serde.fmt_attr(w, "derive(Serialize)")?;
    config.impl_serde.fmt_attr(w, "derive(Deserialize)")?;
    writeln!(w, "pub enum {type_name} {{")?;
    {
        let mut w = PadAdapter::wrap(w);
        for info in &variant_infos {
            let variant = &info.base_name;
            match info.dup_type {
                DuplicateType::Unique => writeln!(w, "{variant},")?,
                DuplicateType::FirstDuplicate => writeln!(w, "{variant}({}),", info.value_type)?,
                DuplicateType::Duplicate => {}
            }
        }
        writeln!(w, "_Other({signal_rust_type}),")?;
    }
    writeln!(w, "}}")?;
    writeln!(w)?;

    writeln!(w, "impl From<{type_name}> for {signal_rust_type} {{")?;
    {
        let match_on_raw_type = match signal_to_rust_type(signal).as_str() {
            "bool" => |x: i64| format!("{}", x == 1),
            "f32" => |x: i64| format!("{x}_f32"),
            _ => |x: i64| format!("{x}"),
        };

        let mut w = PadAdapter::wrap(w);
        writeln!(w, "fn from(val: {type_name}) -> {signal_rust_type} {{")?;
        {
            let mut w = PadAdapter::wrap(&mut w);
            writeln!(w, "match val {{")?;
            {
                let mut w = PadAdapter::wrap(&mut w);
                for info in &variant_infos {
                    match info.dup_type {
                        DuplicateType::Unique => {
                            let literal = match_on_raw_type(info.value);
                            writeln!(w, "{type_name}::{} => {literal},", info.base_name)?;
                        }
                        DuplicateType::FirstDuplicate => {
                            writeln!(w, "{type_name}::{}(v) => v,", info.base_name)?;
                        }
                        DuplicateType::Duplicate => {}
                    }
                }
                writeln!(w, "{type_name}::_Other(x) => x,")?;
            }
            writeln!(w, "}}")?;
        }
        writeln!(w, "}}")?;
    }
    writeln!(w, "}}")?;
    writeln!(w)?;
    Ok(())
}

/// Determine the smallest rust integer that can fit the actual signal values,
/// i.e. accounting for factor and offset.
///
/// NOTE: Factor and offset must be whole integers.
fn scaled_signal_to_rust_int(signal: &Signal) -> String {
    assert!(
        signal.factor.fract().abs() <= f64::EPSILON,
        "Signal Factor ({}) should be an integer",
        signal.factor,
    );
    assert!(
        signal.offset.fract().abs() <= f64::EPSILON,
        "Signal Offset ({}) should be an integer",
        signal.offset,
    );

    signal_to_rust_int_typ(signal).unwrap_or_else(|| {
        panic!(
            "Signal {} could not be represented as a Rust integer",
            signal.name,
        );
    })
}

/// Convert the relevant parameters of a [`Signal`] into a Rust type.
fn signal_to_rust_int_typ(signal: &Signal) -> Option<String> {
    if signal.size > 64 {
        return None;
    }
    get_range_of_values(signal).map(|(low, high)| range_to_rust_int(low, high))
}

/// Using the signal's parameters, find the range of values that it spans.
fn get_range_of_values(signal: &Signal) -> Option<(i128, i128)> {
    let sign: ValueType = signal.value_type;
    let signal_size: u32 = signal.size as u32;
    let factor: i64 = signal.factor as i64;
    let offset: i64 = signal.offset as i64;

    if signal_size == 0 {
        return None;
    }
    let (low, high) = match sign {
        Signed => (
            1i128
                .checked_shl(signal_size.saturating_sub(1))
                .and_then(|n| n.checked_mul(-1)),
            1i128
                .checked_shl(signal_size.saturating_sub(1))
                .and_then(|n| n.checked_sub(1)),
        ),
        Unsigned => (
            Some(0),
            1i128
                .checked_shl(signal_size)
                .and_then(|n| n.checked_sub(1)),
        ),
    };
    let range1 = apply_factor_and_offset(low, factor, offset);
    let range2 = apply_factor_and_offset(high, factor, offset);
    match (range1, range2) {
        (Some(a), Some(b)) => Some((min(a, b), max(a, b))),
        _ => None,
    }
}

fn apply_factor_and_offset(input: Option<i128>, factor: i64, offset: i64) -> Option<i128> {
    input
        .and_then(|n| n.checked_mul(factor.into()))
        .and_then(|n| n.checked_add(offset.into()))
}

/// Determine the smallest Rust integer type that can fit the range of values
/// Only values derived from 64-bit integers are supported, i.e. the range [-2^64-1, 2^64-1]
fn range_to_rust_int(low: i128, high: i128) -> String {
    let lower_bound: u8;
    let upper_bound: u8;
    let sign: &str;

    if low < 0 {
        // signed case
        sign = "i";
        lower_bound = match low {
            n if n >= i8::MIN.into() => 8,
            n if n >= i16::MIN.into() => 16,
            n if n >= i32::MIN.into() => 32,
            n if n >= i64::MIN.into() => 64,
            _ => 128,
        };
        upper_bound = match high {
            n if n <= i8::MAX.into() => 8,
            n if n <= i16::MAX.into() => 16,
            n if n <= i32::MAX.into() => 32,
            n if n <= i64::MAX.into() => 64,
            _ => 128,
        };
    } else {
        sign = "u";
        lower_bound = 8;
        upper_bound = match high {
            n if n <= u8::MAX.into() => 8,
            n if n <= u16::MAX.into() => 16,
            n if n <= u32::MAX.into() => 32,
            n if n <= u64::MAX.into() => 64,
            _ => 128,
        };
    }

    let size = max(lower_bound, upper_bound);
    format!("{sign}{size}")
}

/// Bit-width suffix for Rust int types (8, 16, 32, 64).
fn signal_bit_size_suffix(size: u32) -> &'static str {
    match size {
        n if n <= 8 => "8",
        n if n <= 16 => "16",
        n if n <= 32 => "32",
        _ => "64",
    }
}

/// Determine the smallest rust integer that can fit the raw signal values.
fn signal_to_rust_int(signal: &Signal) -> String {
    let sign = match signal.value_type {
        Signed => "i",
        Unsigned => "u",
    };
    format!("{sign}{}", signal_bit_size_suffix(signal.size as u32))
}

/// Determine the smallest unsigned rust integer with no fewer bits than the signal.
fn signal_to_rust_uint(signal: &Signal) -> String {
    format!("u{}", signal_bit_size_suffix(signal.size as u32))
}

#[allow(clippy::float_cmp)]
fn signal_is_float_in_rust(signal: &Signal) -> bool {
    signal.offset.fract() != 0.0 || signal.factor.fract() != 0.0
}

/// Get the Rust type for a signal
fn signal_to_rust_type(signal: &Signal) -> String {
    if signal.size == 1 {
        String::from("bool")
    } else if signal_is_float_in_rust(signal) {
        // If there is any scaling needed, go for float
        String::from("f32")
    } else {
        scaled_signal_to_rust_int(signal)
    }
}

enum DuplicateType {
    Unique,
    FirstDuplicate,
    Duplicate,
}

/// Variant info for enum generation
struct VariantInfo {
    base_name: String,
    value: i64,
    dup_type: DuplicateType,
    value_type: String,
}

/// Generate variant info for enum generation.
/// For duplicates, uses tuple variants like `Reserved(u8)` instead of separate variants.
fn generate_variant_info(variants: &[ValDescription], signal_rust_type: &str) -> Vec<VariantInfo> {
    // First pass: count occurrences of each base name
    let mut name_counts: HashMap<String, usize> = HashMap::new();
    for variant in variants {
        let base_name = enum_variant_name(&variant.description);
        name_counts
            .entry(base_name)
            .and_modify(|c| *c = c.saturating_add(1))
            .or_insert(1);
    }

    // Second pass: generate variant info
    let mut variant_infos = Vec::new();
    let mut seen_names: HashMap<String, usize> = HashMap::new();
    for variant in variants {
        let base_name = enum_variant_name(&variant.description);
        let count = name_counts.get(&base_name).copied().unwrap_or(0);
        let seen_count = seen_names.entry(base_name.clone()).or_insert(0);
        *seen_count = (*seen_count).saturating_add(1);

        let dup_type = if count == 1 {
            DuplicateType::Unique
        } else if *seen_count == 1 {
            DuplicateType::FirstDuplicate
        } else {
            DuplicateType::Duplicate
        };

        variant_infos.push(VariantInfo {
            base_name,
            value: variant.id,
            dup_type,
            value_type: signal_rust_type.to_string(),
        });
    }
    variant_infos
}

fn enum_name(msg: &Message, signal: &Signal) -> String {
    // this turns signal `_4DRIVE` into `4drive`
    let signal_name = signal
        .name
        .trim_start_matches(|c: char| c.is_ascii_punctuation())
        .to_pascal_case();
    let msg_name = enum_variant_name(&msg.name);

    format!("{msg_name}{signal_name}")
}

fn multiplexed_enum_variant_wrapper_name(switch_index: u64) -> String {
    format!("M{switch_index}")
}

fn multiplex_enum_name(msg: &Message, multiplexor: &Signal) -> Result<String> {
    ensure!(
        matches!(multiplexor.multiplexer_indicator, Multiplexor),
        "signal {multiplexor:?} is not the multiplexor",
    );
    Ok(format!(
        "{}{}Index",
        msg.name.to_pascal_case(),
        multiplexor.name.to_pascal_case(),
    ))
}

fn multiplexed_enum_variant_name(
    msg: &Message,
    multiplexor: &Signal,
    switch_index: u64,
) -> Result<String> {
    ensure!(
        matches!(multiplexor.multiplexer_indicator, Multiplexor),
        "signal {multiplexor:?} is not the multiplexor",
    );

    Ok(format!(
        "{}{}M{switch_index}",
        msg.name.to_pascal_case(),
        multiplexor.name.to_pascal_case(),
    ))
}

fn render_embedded_can_frame(w: &mut impl Write, config: &Config<'_>, msg: &Message) -> Result<()> {
    config.impl_embedded_can_frame.fmt_cfg(w, |w| {
        writeln!(
            w,
            "\
impl embedded_can::Frame for {0} {{
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {{
        if id.into() != Self::MESSAGE_ID {{
            None
        }} else {{
            data.try_into().ok()
        }}
    }}

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {{
        unimplemented!()
    }}

    fn is_extended(&self) -> bool {{
        match self.id() {{
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }}
    }}

    fn is_remote_frame(&self) -> bool {{
        false
    }}

    fn id(&self) -> Id {{
        Self::MESSAGE_ID
    }}

    fn dlc(&self) -> usize {{
        self.raw.len()
    }}

    fn data(&self) -> &[u8] {{
        &self.raw
    }}
}}",
            msg.type_name(),
        )
    })
}

fn render_debug_impl(w: &mut impl Write, msg: &Message) -> Result<()> {
    let typ = msg.type_name();
    writeln!(w, r"impl core::fmt::Debug for {typ} {{")?;
    {
        let mut w = PadAdapter::wrap(w);
        writeln!(
            w,
            "fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{",
        )?;
        {
            let mut w = PadAdapter::wrap(&mut w);
            writeln!(w, r"if f.alternate() {{")?;
            {
                let mut w = PadAdapter::wrap(&mut w);
                writeln!(w, r#"f.debug_struct("{typ}")"#)?;
                {
                    let mut w = PadAdapter::wrap(&mut w);
                    for signal in &msg.signals {
                        if signal.multiplexer_indicator == Plain {
                            writeln!(w, r#".field("{0}", &self.{0}())"#, signal.field_name())?;
                        }
                    }
                }
                writeln!(w, r".finish()")?;
            }
            writeln!(w, r"}} else {{")?;
            {
                let mut w = PadAdapter::wrap(&mut w);
                writeln!(w, r#"f.debug_tuple("{typ}").field(&self.raw).finish()"#)?;
            }
            writeln!(w, "}}")?;
        }
        writeln!(w, "}}")?;
    }
    writeln!(w, "}}")?;
    writeln!(w)?;
    Ok(())
}

fn render_defmt_impl(w: &mut impl Write, msg: &Message) -> Result<()> {
    let typ = msg.type_name();
    writeln!(w, r"impl defmt::Format for {typ} {{")?;
    {
        let mut w = PadAdapter::wrap(w);
        writeln!(w, "fn format(&self, f: defmt::Formatter) {{")?;
        {
            let mut w = PadAdapter::wrap(&mut w);
            writeln!(w, r"defmt::write!(f,")?;
            {
                let mut w = PadAdapter::wrap(&mut w);
                write!(w, r#""{typ} {{{{"#)?;
                {
                    for signal in &msg.signals {
                        if signal.multiplexer_indicator == Plain {
                            write!(w, r" {}={{:?}}", signal.name)?;
                        }
                    }
                }
                writeln!(w, r#" }}}}","#)?;

                for signal in &msg.signals {
                    if signal.multiplexer_indicator == Plain {
                        writeln!(w, "self.{}(),", signal.field_name())?;
                    }
                }
                writeln!(w, r");")?;
            }
            writeln!(w, "}}")?;
        }
    }
    writeln!(w, "}}")?;
    writeln!(w)?;
    Ok(())
}

fn render_multiplexor_enums(
    w: &mut impl Write,
    config: &Config<'_>,
    dbc: &Dbc,
    msg: &Message,
    multiplexor_signal: &Signal,
) -> Result<()> {
    ensure!(
        multiplexor_signal.multiplexer_indicator == Multiplexor,
        "signal {} is not the multiplexor",
        multiplexor_signal.name,
    );

    let mut multiplexed_signals = BTreeMap::new();
    for signal in &msg.signals {
        if let MultiplexedSignal(switch_index) = signal.multiplexer_indicator {
            multiplexed_signals
                .entry(switch_index)
                .and_modify(|v: &mut Vec<&Signal>| v.push(signal))
                .or_insert_with(|| vec![&signal]);
        }
    }

    writeln!(w, "/// Defined values for multiplexed signal {}", msg.name)?;
    writeln!(w, "{ALLOW_LINTS}")?;
    config.write_allow_dead_code(w)?;

    config.impl_debug.fmt_attr(w, "derive(Debug)")?;
    config.impl_defmt.fmt_attr(w, "derive(defmt::Format)")?;
    config.impl_serde.fmt_attr(w, "derive(Serialize)")?;
    config.impl_serde.fmt_attr(w, "derive(Deserialize)")?;

    let enum_name = multiplex_enum_name(msg, multiplexor_signal)?;
    writeln!(w, "pub enum {enum_name} {{")?;

    {
        let mut w = PadAdapter::wrap(w);
        for switch_index in multiplexed_signals.keys() {
            writeln!(
                w,
                "{multiplexed_wrapper_name}({multiplexed_name}),",
                multiplexed_wrapper_name = multiplexed_enum_variant_wrapper_name(*switch_index),
                multiplexed_name =
                    multiplexed_enum_variant_name(msg, multiplexor_signal, *switch_index)?,
            )?;
        }
    }
    writeln!(w, "}}")?;
    writeln!(w)?;

    for (switch_index, multiplexed_signals) in &multiplexed_signals {
        let struct_name = multiplexed_enum_variant_name(msg, multiplexor_signal, *switch_index)?;

        writeln!(w, "{ALLOW_LINTS}")?;
        config.write_allow_dead_code(w)?;
        config.impl_debug.fmt_attr(w, "derive(Debug)")?;
        config.impl_defmt.fmt_attr(w, "derive(defmt::Format)")?;
        config.impl_serde.fmt_attr(w, "derive(Serialize)")?;
        config.impl_serde.fmt_attr(w, "derive(Deserialize)")?;
        writeln!(w, r"#[derive(Default)]")?;
        writeln!(w, "pub struct {struct_name} {{ raw: [u8; {}] }}", msg.size)?;
        writeln!(w)?;

        writeln!(w, "{ALLOW_LINTS}")?;
        config.write_allow_dead_code(w)?;
        writeln!(w, "impl {struct_name} {{")?;

        writeln!(
            w,
            "pub fn new() -> Self {{ Self {{ raw: [0u8; {}] }} }}",
            msg.size
        )?;

        for signal in multiplexed_signals {
            render_signal(w, config, signal, dbc, msg)?;
        }

        writeln!(w, "}}")?;
        writeln!(w)?;
    }

    Ok(())
}

fn render_arbitrary(w: &mut impl Write, config: &Config<'_>, msg: &Message) -> Result<()> {
    writeln!(w, "{ALLOW_LINTS}")?;
    config.write_allow_dead_code(w)?;
    let typ = msg.type_name();
    writeln!(w, "impl<'a> Arbitrary<'a> for {typ} {{")?;
    {
        let filtered_signals: Vec<&Signal> = msg
            .signals
            .iter()
            .filter(|v| matches!(v.multiplexer_indicator, Plain | Multiplexor))
            .collect();
        let mut w = PadAdapter::wrap(w);
        writeln!(
            w,
            "fn arbitrary({}u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {{",
            if filtered_signals.is_empty() { "_" } else { "" },
        )?;
        {
            let mut w = PadAdapter::wrap(&mut w);

            for signal in &filtered_signals {
                writeln!(
                    w,
                    "let {field_name} = {arbitrary_value};",
                    field_name = signal.field_name(),
                    arbitrary_value = signal_to_arbitrary(signal),
                )?;
            }

            let args: Vec<String> = filtered_signals
                .iter()
                .map(|signal| signal.field_name())
                .collect();

            writeln!(
                w,
                "{typ}::new({args}).map_err(|_| arbitrary::Error::IncorrectFormat)",
                typ = msg.type_name(),
                args = args.join(","),
            )?;
        }
        writeln!(w, "}}")?;
    }
    writeln!(w, "}}")?;

    Ok(())
}

fn render_error(w: &mut impl Write, config: &Config<'_>) -> Result<()> {
    w.write_all(include_bytes!("./includes/errors.rs"))?;

    config.impl_error.fmt_cfg(w, |w| {
        writeln!(w, "impl core::error::Error for CanError {{}}")
    })
}

fn render_arbitrary_helpers(w: &mut impl Write, config: &Config<'_>) -> Result<()> {
    config.impl_arbitrary.fmt_cfg(&mut *w, |w| {
        config.write_allow_dead_code(w)?;
        writeln!(w, "trait UnstructuredFloatExt {{")?;
        writeln!(w, "    fn float_in_range(&mut self, range: core::ops::RangeInclusive<f32>) -> arbitrary::Result<f32>;")?;
        writeln!(w, "}}")?;
        writeln!(w)?;
        Ok::<_, Error>(())
    })?;

    config.impl_arbitrary.fmt_cfg(w, |w| {
        writeln!(w, "impl UnstructuredFloatExt for arbitrary::Unstructured<'_> {{")?;
        writeln!(w, "    fn float_in_range(&mut self, range: core::ops::RangeInclusive<f32>) -> arbitrary::Result<f32> {{")?;
        writeln!(w, "        let min = range.start();")?;
        writeln!(w, "        let max = range.end();")?;
        writeln!(w, "        let steps = u32::MAX;")?;
        writeln!(w, "        let factor = (max - min) / (steps as f32);")?;
        writeln!(w, "        let random_int: u32 = self.int_in_range(0..=steps)?;")?;
        writeln!(w, "        let random = min + factor * (random_int as f32);")?;
        writeln!(w, "        Ok(random)")?;
        writeln!(w, "    }}")?;
        writeln!(w, "}}")?;
        writeln!(w)?;
        Ok::<_, Error>(())
    })
}

fn signal_to_arbitrary(signal: &Signal) -> String {
    if signal.size == 1 {
        "u.int_in_range(0..=1)? == 1".to_string()
    } else if signal_is_float_in_rust(signal) {
        let min = signal.min;
        let max = signal.max;
        format!("u.float_in_range({min}_f32..={max}_f32)?")
    } else {
        let min = signal.min;
        let max = signal.max;
        format!("u.int_in_range({min}..={max})?")
    }
}

fn get_relevant_messages(dbc: &Dbc) -> impl Iterator<Item = &Message> {
    dbc.messages.iter().filter(|v| !message_ignored(v))
}

fn message_ignored(message: &Message) -> bool {
    // DBC internal message containing signals unassigned to any real message
    message.name == "VECTOR__INDEPENDENT_SIG_MSG"
}

impl Config<'_> {
    /// Generate Rust structs matching DBC input description and return as String
    pub fn generate(self) -> Result<String> {
        let mut out = Vec::new();
        self.write(&mut out)?;
        Ok(String::from_utf8(out)?)
    }

    /// Generate Rust structs matching DBC input description and write to `out`
    pub fn write(self, out: impl Write) -> Result<()> {
        codegen(&self, out)
    }

    /// Generate Rust structs matching DBC input description and write to file at `path`
    pub fn write_to_file<P: AsRef<Path>>(self, path: P) -> Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path.as_ref())?;

        self.write(file)
    }

    fn write_allow_dead_code(&self, w: &mut impl Write) -> Result<()> {
        if self.allow_dead_code {
            writeln!(w, "{ALLOW_DEADCODE}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use can_dbc::{Signal, ValueType};

    use super::*;

    fn signal(sign: ValueType, signal_size: u32, factor: i64, offset: i64) -> Signal {
        Signal {
            name: String::new(),
            start_bit: 0,
            size: u64::from(signal_size),
            byte_order: LittleEndian,
            value_type: sign,
            factor: factor as f64,
            offset: offset as f64,
            min: 0.0,
            max: 0.0,
            unit: String::new(),
            receivers: vec![],
            multiplexer_indicator: Plain,
        }
    }

    #[test]
    fn test_range_of_values() {
        assert_eq!(
            get_range_of_values(&signal(Unsigned, 4, 1, 0)),
            Some((0, 15))
        );
        assert_eq!(
            get_range_of_values(&signal(Unsigned, 32, -1, 0)),
            Some((-i128::from(u32::MAX), 0))
        );
        assert_eq!(
            get_range_of_values(&signal(Unsigned, 12, 1, -1000)),
            Some((-1000, 3095))
        );
    }

    #[test]
    fn test_range_0_signal_size() {
        assert_eq!(
            get_range_of_values(&signal(Signed, 0, 1, 0)),
            None,
            "0 bit signal should be invalid",
        );
    }

    #[test]
    fn test_range_to_rust_int() {
        assert_eq!(range_to_rust_int(0, 255), "u8");
        assert_eq!(range_to_rust_int(-1, 127), "i8");
        assert_eq!(range_to_rust_int(-1, 128), "i16");
        assert_eq!(range_to_rust_int(-1, 255), "i16");
        assert_eq!(range_to_rust_int(-65535, 0), "i32");
        assert_eq!(range_to_rust_int(-129, -127), "i16");
        assert_eq!(range_to_rust_int(0, 1i128 << 65), "u128");
        assert_eq!(range_to_rust_int(-(1i128 << 65), 0), "i128");
    }

    #[test]
    fn test_convert_signal_params_to_rust_int() {
        let typ = signal_to_rust_int_typ(&signal(Signed, 8, 1, 0)).unwrap();
        assert_eq!(typ, "i8");
        let typ = signal_to_rust_int_typ(&signal(Signed, 8, 2, 0)).unwrap();
        assert_eq!(typ, "i16");
        let typ = signal_to_rust_int_typ(&signal(Signed, 63, 1, 0)).unwrap();
        assert_eq!(typ, "i64");
        let typ = signal_to_rust_int_typ(&signal(Unsigned, 64, -1, 0)).unwrap();
        assert_eq!(typ, "i128");
        let typ = signal_to_rust_int_typ(&signal(Unsigned, 65, 1, 0));
        assert_eq!(typ, None, "Not valid DBC, it's more than 64 bits");
    }
}
