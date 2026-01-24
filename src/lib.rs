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
mod signal_type;
mod utils;

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;

use anyhow::{anyhow, ensure, Context, Error, Result};
use can_dbc::ByteOrder::{BigEndian, LittleEndian};
use can_dbc::MultiplexIndicator::{
    MultiplexedSignal, Multiplexor, MultiplexorAndMultiplexedSignal, Plain,
};
use can_dbc::ValueType::Signed;
use can_dbc::{Dbc, Message, MessageId, Signal, Transmitter, ValDescription, ValueDescription};
use heck::{ToPascalCase, ToSnakeCase};
use typed_builder::TypedBuilder;

pub use crate::feature_config::FeatureConfig;
use crate::pad::PadAdapter;
use crate::signal_type::ValType;
use crate::utils::{enum_variant_name, MessageExt as _, SignalExt as _};

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
            let typ = ValType::from_signal(signal);
            if typ != ValType::Bool {
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
                    let typ = ValType::from_signal(signal);
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
        let signal_ty = ValType::from_signal(signal);
        let variant_infos = generate_variant_info(variants, signal_ty);

        writeln!(w, "pub fn {fn_name}(&self) -> {type_name} {{")?;
        {
            let mut w = PadAdapter::wrap(w);

            // Use signed type for loading when signal is signed and has negative values
            let has_negative_values = variants.iter().any(|v| v.id < 0);
            let load_type = if signal.value_type == Signed && has_negative_values {
                signal_ty
            } else {
                ValType::from_signal_uint(signal)
            };

            let read = read_fn_with_type(signal, msg, load_type)?;
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
        let typ = ValType::from_signal(signal);
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
    let typ = ValType::from_signal(signal);
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
    let typ = ValType::from_signal(signal);
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
                let typ = ValType::from_signal(signal);
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
    let typ = ValType::from_signal(signal);
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

    let typ = ValType::from_signal(signal);
    match typ {
        ValType::Bool => {
            writeln!(w, "signal == 1")?;
        }
        ValType::F32 => {
            // Scaling is always done on floats
            writeln!(w, "let factor = {}_f32;", signal.factor)?;
            writeln!(w, "let offset = {}_f32;", signal.offset)?;
            writeln!(w, "(signal as f32) * factor + offset")?;
        }
        _ => {
            writeln!(w, "let factor = {};", signal.factor)?;
            if Some(typ) == ValType::from_signal_uint(signal).unsigned_to_signed() {
                // Can't do iNN::from(uNN) if they both fit in the same integer type,
                // so cast first
                writeln!(w, "let signal = signal as {typ};")?;
            }

            if signal.offset >= 0.0 {
                writeln!(
                    w,
                    "{typ}::from(signal).saturating_mul(factor).saturating_add({})",
                    signal.offset,
                )?;
            } else {
                writeln!(
                    w,
                    "{typ}::from(signal).saturating_mul(factor).saturating_sub({})",
                    signal.offset.abs(),
                )?;
            }
        }
    }
    Ok(())
}

fn read_fn(signal: &Signal, msg: &Message) -> Result<String> {
    read_fn_with_type(signal, msg, ValType::from_signal_int(signal))
}

fn read_fn_with_type(signal: &Signal, msg: &Message, typ: ValType) -> Result<String> {
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
    let typ = ValType::from_signal(signal);
    match typ {
        ValType::Bool => {
            // Map boolean to byte so we can pack it
            writeln!(w, "let value = value as u8;")?;
        }
        ValType::F32 => {
            // Massage value into an int
            writeln!(w, "let factor = {}_f32;", signal.factor)?;
            writeln!(w, "let offset = {}_f32;", signal.offset)?;
            let typ = ValType::from_signal_int(signal);
            writeln!(w, "let value = ((value - offset) / factor) as {typ};")?;
            writeln!(w)?;
        }
        _ => {
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
            let typ = ValType::from_signal_int(signal);
            writeln!(w, "let value = (value / factor) as {typ};")?;
            writeln!(w)?;
        }
    }

    if signal.value_type == Signed {
        let typ = ValType::from_signal_uint(signal);
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
    let signal_ty = ValType::from_signal(signal);

    // Generate variant info to handle duplicates with tuple variants
    let variant_infos = generate_variant_info(variants, signal_ty);

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
        writeln!(w, "_Other({signal_ty}),")?;
    }
    writeln!(w, "}}")?;
    writeln!(w)?;

    writeln!(w, "impl From<{type_name}> for {signal_ty} {{")?;
    {
        let match_on_raw_type = match ValType::from_signal(signal) {
            ValType::Bool => |x: i64| format!("{}", x == 1),
            ValType::F32 => |x: i64| format!("{x}_f32"),
            _ => |x: i64| format!("{x}"),
        };

        let mut w = PadAdapter::wrap(w);
        writeln!(w, "fn from(val: {type_name}) -> {signal_ty} {{")?;
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
fn generate_variant_info(variants: &[ValDescription], signal_ty: ValType) -> Vec<VariantInfo> {
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
            value_type: signal_ty.to_string(),
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
    let typ = ValType::from_signal(signal);
    match typ {
        ValType::Bool => "u.int_in_range(0..=1)? == 1".to_string(),
        ValType::F32 => {
            let min = signal.min;
            let max = signal.max;
            format!("u.float_in_range({min}_f32..={max}_f32)?")
        }
        _ => {
            let min = signal.min;
            let max = signal.max;
            format!("u.int_in_range({min}..={max})?")
        }
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
