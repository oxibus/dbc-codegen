#![deny(missing_docs)]
#![deny(clippy::arithmetic_side_effects)]
#![allow(clippy::needless_doctest_main)]
#![cfg_attr(feature = "std", doc = include_str!("../README.md"))]
#![cfg_attr(
    not(feature = "std"),
    doc = "Documentation is only available with the `std` feature."
)]

mod feature_config;
mod keywords;
mod signal_type;
mod utils;
mod wrappers;

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write as _;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use anyhow::{anyhow, ensure, Context, Result};
use can_dbc::ByteOrder::{BigEndian, LittleEndian};
use can_dbc::MultiplexIndicator::{MultiplexedSignal, Multiplexor, Plain};
use can_dbc::ValueType::Signed;
use can_dbc::{Dbc, MessageId, Transmitter, ValDescription, ValueDescription};
use proc_macro2::TokenStream;
use syn::parse2;
use template_quote::quote as q;
use typed_builder::TypedBuilder;
use wrappers::{DbcGen, MessageGen, SignalGen};

pub use crate::feature_config::FeatureConfig;
use crate::signal_type::ValType;
use crate::utils::{
    enum_variant_name, is_integer, lit_float, lit_int, multiplexed_enum_variant_wrapper_name,
    multiplexed_enum_variant_wrapper_setter_name, ToIdent as _, Tokens as _,
};

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

impl Config<'_> {
    /// Write Rust structs matching DBC input description to `out` buffer
    fn codegen(&self) -> Result<TokenStream> {
        let dbc = Dbc::try_from(self.dbc_content).map_err(|e| {
            if self.debug_prints {
                anyhow!("Could not parse dbc file: {e:#?}")
            } else {
                anyhow!("Could not parse dbc file")
            }
        })?;
        if self.debug_prints {
            eprintln!("{dbc:#?}");
        }

        let dbc_gen = DbcGen::new(&dbc);
        Ok(q! {
            /// The name of the DBC file this code was generated from
            #[allow(dead_code)]
            pub const DBC_FILE_NAME: &str = #{self.dbc_name};
            /// The version of the DBC file this code was generated from
            #[allow(dead_code)]
            pub const DBC_FILE_VERSION: &str = #{&dbc_gen.version.0};

            #[allow(unused_imports)]
            use core::ops::BitOr;
            #[allow(unused_imports)]
            use bitvec::prelude::*;
            #[allow(unused_imports)]
            use embedded_can::{Id, StandardId, ExtendedId};
            #{self.impl_arbitrary.if_cfg(|| q! { use arbitrary::Arbitrary; })}
            #{self.impl_serde.if_cfg(|| q! { use serde::{Serialize, Deserialize}; })}

            #{self.render_dbc(&dbc_gen)?}
            #{self.render_error()}
            #{self.render_arbitrary_helpers()}
        })
    }

    fn render_dbc(&self, dbc: &DbcGen<'_>) -> Result<TokenStream> {
        let messages: Vec<_> = dbc
            .messages()
            .iter()
            .map(|msg| {
                self.render_message(msg, dbc)
                    .with_context(|| format!("write message `{}`", msg.name))
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(q! {
            #{self.render_root_enum(dbc)}
            #(#messages)*
        })
    }

    fn render_root_enum(&self, dbc: &DbcGen<'_>) -> TokenStream {
        let messages = dbc.messages();
        let from_can_body = if messages.is_empty() {
            q! { Err(CanError::UnknownMessageId(id)) }
        } else {
            q! {
                let res = match id {
                    #(for msg in messages) {
                        #{msg.type_name()}::MESSAGE_ID => Messages::#{msg.type_name()}(#{msg.type_name()}::try_from(payload)?),
                    }
                    id => return Err(CanError::UnknownMessageId(id)),
                };
                Ok(res)
            }
        };

        q! {
            /// All messages
            #{allow_lints()}
            #{self.allow_dead_code()}
            #[derive(Clone)]
            #{self.impl_debug.attr(|| q! { derive(Debug) })}
            #{self.impl_defmt.attr(|| q! { derive(defmt::Format) })}
            #{self.impl_serde.attr(|| q! { derive(Serialize, Deserialize) })}
            pub enum Messages {
                #(for msg in messages) {
                    #[doc = #{format!(" {}", msg.name)}]
                    #{msg.type_name()}(#{msg.type_name()}),
                }
            }

            #{allow_lints()}
            #{self.allow_dead_code()}
            impl Messages {
                /// Read message from CAN frame
                #[inline(never)]
                pub fn from_can_message(id: Id, payload: &[u8]) -> Result<Self, CanError> {
                    #from_can_body
                }
            }
        }
    }

    fn render_message(&self, msg: &MessageGen<'_>, dbc: &DbcGen<'_>) -> Result<TokenStream> {
        let mut doc = String::new();
        writeln!(doc, "/// {}", msg.name)?;
        writeln!(doc, "///")?;
        let message_id = match msg.id {
            MessageId::Standard(id) => {
                writeln!(doc, "/// - Standard ID: {id} (0x{id:x})")?;
                q! { Id::Standard(unsafe { StandardId::new_unchecked(#{lit_int(format!("{id:#x}"))}) }) }
            }
            MessageId::Extended(id) => {
                writeln!(doc, "/// - Extended ID: {id} (0x{id:x})")?;
                q! { Id::Extended(unsafe { ExtendedId::new_unchecked(#{lit_int(format!("{id:#x}"))}) }) }
            }
        };
        writeln!(doc, "/// - Size: {} bytes", msg.size)?;
        if let Transmitter::NodeName(transmitter) = &msg.transmitter {
            writeln!(doc, "/// - Transmitter: {transmitter}")?;
        }
        if let Some(comment) = dbc.message_comment(msg.id) {
            writeln!(doc, "///")?;
            for line in comment.trim().lines() {
                writeln!(doc, "/// {line}")?;
            }
        }
        let struct_doc = doc.tokens().context("message doc to tokens")?;

        // New function arguments and setter calls (Vecs needed for comma-separated/joined output)
        let new_fn_args: Vec<_> = msg
            .signals()
            .iter()
            .filter_map(|signal| {
                if matches!(signal.multiplexer_indicator, Plain | Multiplexor) {
                    let field_name = signal.field_name();
                    let typ = signal.typ();
                    let typ_ident = typ.ident();
                    Some(q! { #field_name: #typ_ident })
                } else {
                    None
                }
            })
            .collect();

        let new_fn_setters: Vec<_> = msg
            .signals()
            .iter()
            .filter_map(|signal| {
                if matches!(signal.multiplexer_indicator, Plain | Multiplexor) {
                    Some(q! { res.#{signal.field_name_setter()}(#{signal.field_name()})?; })
                } else {
                    None
                }
            })
            .collect();

        // Render signals
        let signal_impls: Result<Vec<_>> = msg
            .signals()
            .iter()
            .map(|signal| {
                if signal.multiplexer_indicator == Multiplexor {
                    self.render_multiplexor_signal(signal, msg)
                        .with_context(|| format!("write signal impl `{}`", signal.name))
                } else if signal.multiplexer_indicator == Plain {
                    self.render_signal(signal, dbc, msg)
                        .with_context(|| format!("write signal impl `{}`", signal.name))
                } else {
                    Ok(q! {})
                }
            })
            .collect();
        let signal_impls = signal_impls?;

        // Render enums for this message
        let enums_for_this_message: Vec<_> = dbc
            .value_descriptions
            .iter()
            .filter_map(|x| {
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
                        .map(|v| self.write_enum(&v, msg, value_descriptions.as_slice()))
                } else {
                    None
                }
            })
            .collect();

        Ok(q! {
            #struct_doc
            #[derive(Clone, Copy)]
            #{self.impl_serde.attr(|| q! { derive(Serialize) })}
            #{self.impl_serde.attr(|| q! { derive(Deserialize) })}
            pub struct #{msg.type_name()} {
                #{self.impl_serde.attr(|| q! { serde(with = "serde_bytes") })}
                raw: [u8; #{msg.size_lit()}],
            }

            #{allow_lints()}
            #{self.allow_dead_code()}
            impl #{msg.type_name()} {
                pub const MESSAGE_ID: embedded_can::Id = #message_id;

                #(for signal in msg.signals()) {
                    #(if signal.typ() != ValType::Bool) {
                        pub const #{signal.const_name_min()}: #{signal.typ().ident()} = #{generate_value_literal(signal.min, signal.typ())};
                        pub const #{signal.const_name_max()}: #{signal.typ().ident()} = #{generate_value_literal(signal.max, signal.typ())};
                    }
                }

                #{format!("/// Construct new {} from values", msg.name).tokens()?}
                pub fn new(#(#new_fn_args),*) -> Result<Self, CanError> {
                    let #(if !new_fn_args.is_empty()) { mut } res = Self { raw: [0u8; #{msg.size_lit()}] };
                    #(#new_fn_setters)*
                    Ok(res)
                }

                /// Access message payload raw value
                pub fn raw(&self) -> &[u8; #{msg.size_lit()}] {
                    &self.raw
                }

                #(#signal_impls)*
            }

            impl core::convert::TryFrom<&[u8]> for #{msg.type_name()} {
                type Error = CanError;

                #[inline(always)]
                fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
                    if payload.len() != #{msg.size_lit()} {
                        return Err(CanError::InvalidPayloadSize);
                    }
                    let mut raw = [0u8; #{msg.size_lit()}];
                    raw.copy_from_slice(&payload[..#{msg.size_lit()}]);
                    Ok(Self { raw })
                }
            }

            #{self.render_embedded_can_frame(msg)}
            #{self.impl_debug.if_cfg(|| render_debug_impl(msg))}
            #{self.impl_defmt.if_cfg(|| render_defmt_impl(msg))}
            #{self.impl_arbitrary.if_cfg(|| self.render_arbitrary(msg))}

            #(#enums_for_this_message)*
            #{msg
                .signals()
                .iter()
                .find(|signal| signal.multiplexer_indicator == Multiplexor)
                .map(|multiplexor_signal| self.render_multiplexor_enums(dbc, msg, multiplexor_signal))
                .transpose()?
            }
        })
    }

    fn render_signal(
        &self,
        signal: &SignalGen<'_>,
        dbc: &DbcGen<'_>,
        msg: &MessageGen<'_>,
    ) -> Result<TokenStream> {
        let fn_name = signal.field_name();
        let fn_name_raw = signal.field_name_raw();

        // Build signal getter doc as doc comment and parse into tokens
        let mut doc = format!("/// {}\n", signal.name);
        if let Some(comment) = dbc.signal_comment(msg.id, &signal.name) {
            let _ = writeln!(doc, "///");
            for line in comment.trim().lines() {
                let _ = writeln!(doc, "/// {line}");
            }
        }
        let _ = writeln!(
            doc,
            "\
///
/// - Min: {}
/// - Max: {}
/// - Unit: {:?}
/// - Receivers: {}",
            signal.min,
            signal.max,
            signal.unit,
            signal.receivers.join(", ")
        );

        let typ = signal.typ();
        let typ = typ.ident();

        // Generate getter function
        let getter = if let Some(variants) = dbc.value_descriptions_for_signal(msg.id, &signal.name)
        {
            let type_name = msg.enum_name(signal);
            let signal_ty = signal.typ();
            let variant_infos = generate_variant_info(variants, signal_ty);

            // Use signed type for loading when signal is signed and has negative values
            let has_negative_values = variants.iter().any(|v| v.id < 0);
            let load_type = if signal.value_type == Signed && has_negative_values {
                signal_ty
            } else {
                signal.typ_uint()
            };

            let read_expr = read_fn_with_type(signal, msg, load_type)?;

            let match_arms: Vec<_> = variant_infos
                .iter()
                .map(|info| {
                    let variant = info.base_name.ident();
                    match info.dup_type {
                        DuplicateType::Unique => {
                            q! { #{lit_int(info.value)} => #type_name::#variant }
                        }
                        DuplicateType::FirstDuplicate | DuplicateType::Duplicate => {
                            q! { #{lit_int(info.value)} => #type_name::#variant(#{lit_int(info.value)}) }
                        }
                    }
                })
                .collect();

            q! {
                #[inline(always)]
                pub fn #fn_name(&self) -> #type_name {
                    let signal = #read_expr;

                    match signal {
                        #(#match_arms,)*
                        _ => #type_name::_Other(self.#fn_name_raw()),
                    }
                }
            }
        } else {
            q! {
                #[inline(always)]
                pub fn #fn_name(&self) -> #typ {
                    self.#fn_name_raw()
                }
            }
        };

        Ok(q! {
            #{doc.tokens()?}
            #getter

            #{format!(
                "\
/// Get raw value of {}
///
/// - Start bit: {}
/// - Signal size: {} bits
/// - Factor: {}
/// - Offset: {}
/// - Byte order: {:?}
/// - Value type: {:?}",
                signal.name,
                signal.start_bit,
                signal.size,
                signal.factor,
                signal.offset,
                signal.byte_order,
                signal.value_type
            ).tokens()?}
            #[inline(always)]
            pub fn #fn_name_raw(&self) -> #typ {
                #{signal_from_payload(signal, msg).context("signal from payload")?}
            }

            #{self.render_set_signal(signal, msg)?}
        })
    }

    fn render_set_signal(
        &self,
        signal: &SignalGen<'_>,
        msg: &MessageGen<'_>,
    ) -> Result<TokenStream> {
        // To avoid accidentally changing the multiplexor value without changing
        // the signals accordingly, setter is kept private for multiplexors.
        Ok(q! {
            #[doc = #{format!(" Set value of {}", signal.name)}]
            #[inline(always)]
            #{if signal.multiplexer_indicator == Multiplexor { q! {} } else { q! { pub } }}
            fn #{signal.field_name_setter()}(
                &mut self,
                value: #{signal.typ_ident()}
            ) -> Result<(), CanError> {
                #(if signal.size != 1) {
                    #{self.check_ranges.if_cfg(|| q! {
                        if value < #{generate_value_literal(signal.min, signal.typ())}
                           || #{generate_value_literal(signal.max, signal.typ())} < value
                        {
                            return Err(CanError::ParameterOutOfRange {
                                message_id: #{msg.type_name()}::MESSAGE_ID
                            });
                        }
                    })}
                }
                #{signal_to_payload(signal, msg)?}
            }
        })
    }

    fn render_multiplexor_signal(
        &self,
        signal: &SignalGen<'_>,
        msg: &MessageGen<'_>,
    ) -> Result<TokenStream> {
        let multiplexer_indexes: BTreeSet<u64> = msg
            .signals()
            .iter()
            .filter_map(|s| {
                if let MultiplexedSignal(index) = s.multiplexer_indicator {
                    Some(index)
                } else {
                    None
                }
            })
            .collect();
        let multiplexer_indexes_vec: Vec<_> = multiplexer_indexes.iter().copied().collect();

        let set_multiplexer_fns: Result<Vec<_>> = multiplexer_indexes
            .iter()
            .map(|switch_index| render_set_signal_multiplexer(signal, msg, *switch_index))
            .collect();
        let set_multiplexer_fns = set_multiplexer_fns?;

        Ok(q! {
            #{format!(
            "\
/// Get raw value of {}
///
/// - Start bit: {}
/// - Signal size: {} bits
/// - Factor: {}
/// - Offset: {}
/// - Byte order: {:?}
/// - Value type: {:?}",
                signal.name,
                signal.start_bit,
                signal.size,
                signal.factor,
                signal.offset,
                signal.byte_order,
                signal.value_type
            ).tokens()?}
            #[inline(always)]
            pub fn #{signal.field_name_raw()}(&self) -> #{signal.typ().ident()} {
                #{signal_from_payload(signal, msg)?}
            }

            pub fn #{signal.field_name()}(&mut self) -> Result<#{msg.multiplex_enum_name(signal)?}, CanError> {
                match self.#{signal.field_name_raw()}() {
                    #(for idx in &multiplexer_indexes_vec) {
                        #{lit_int(idx)}
                        => Ok(
                            #{msg.multiplex_enum_name(signal)?}::#{
                                multiplexed_enum_variant_wrapper_name(*idx)
                            }(
                                #{msg.multiplexed_enum_variant_name(signal, *idx)?}
                                { raw: self.raw }
                            )
                        ),
                    }
                    multiplexor => Err(CanError::InvalidMultiplexor {
                        message_id: #{msg.type_name()}::MESSAGE_ID,
                        multiplexor: multiplexor.into()
                    }),
                }
            }

            #{self.render_set_signal(signal, msg)?}
            #(#set_multiplexer_fns)*
        })
    }

    /// Generate `[allow(dead_code)]` attribute if needed
    fn allow_dead_code(&self) -> Option<TokenStream> {
        if self.allow_dead_code {
            Some(q! { #[allow(dead_code)] })
        } else {
            None
        }
    }

    fn write_enum(
        &self,
        signal: &SignalGen<'_>,
        msg: &MessageGen<'_>,
        variants: &[ValDescription],
    ) -> TokenStream {
        let type_name = msg.enum_name(signal);

        // Generate variant info to handle duplicates with tuple variants
        let variant_infos = generate_variant_info(variants, signal.typ());

        // Generate enum variants
        let enum_variants: Vec<_> = variant_infos
            .iter()
            .filter_map(|info| {
                let variant = info.base_name.ident();
                match info.dup_type {
                    DuplicateType::Unique => Some(q! { #variant }),
                    DuplicateType::FirstDuplicate => {
                        let value_type = info.value_type.ident();
                        Some(q! { #variant(#value_type) })
                    }
                    DuplicateType::Duplicate => None,
                }
            })
            .collect();

        // Generate From impl match arms
        let from_match_arms: Vec<_> = variant_infos
            .iter()
            .filter_map(|info| {
                let variant = info.base_name.ident();
                match info.dup_type {
                    DuplicateType::Unique => {
                        let literal_value = match signal.typ() {
                            ValType::Bool => {
                                if info.value == 1 {
                                    q! { true }
                                } else {
                                    q! { false }
                                }
                            }
                            ValType::F32 => q! { #{lit_float(format!("{}_f32", info.value))} },
                            _ => q! { #{lit_int(info.value)} },
                        };
                        Some(q! { #type_name::#variant => #literal_value })
                    }
                    DuplicateType::FirstDuplicate => Some(q! { #type_name::#variant(v) => v }),
                    DuplicateType::Duplicate => None,
                }
            })
            .collect();

        q! {
            #[doc = #{format!(" Defined values for {}", signal.name)}]
            #{allow_lints()}
            #{self.allow_dead_code()}
            #[derive(Clone, Copy, PartialEq)]
            #{self.impl_debug.attr(|| q! { derive(Debug) })}
            #{self.impl_defmt.attr(|| q! { derive(defmt::Format) })}
            #{self.impl_serde.attr(|| q! { derive(Serialize) })}
            #{self.impl_serde.attr(|| q! { derive(Deserialize) })}
            pub enum #type_name {
                #(#enum_variants,)*
                _Other(#{signal.typ_ident()}),
            }

            impl From<#type_name> for #{signal.typ_ident()} {
                fn from(val: #type_name) -> #{signal.typ_ident()} {
                    match val {
                        #(#from_match_arms,)*
                        #type_name::_Other(x) => x,
                    }
                }
            }
        }
    }

    fn render_embedded_can_frame(&self, msg: &MessageGen<'_>) -> TokenStream {
        self.impl_embedded_can_frame.if_cfg(|| {
            q! {
                impl embedded_can::Frame for #{msg.type_name()} {
                    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
                        if id.into() != Self::MESSAGE_ID {
                            None
                        } else {
                            data.try_into().ok()
                        }
                    }
                    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
                        unimplemented!()
                    }
                    fn is_extended(&self) -> bool {
                        match self.id() {
                            Id::Standard(_) => false,
                            Id::Extended(_) => true,
                        }
                    }
                    fn is_remote_frame(&self) -> bool {
                        false
                    }
                    fn id(&self) -> Id {
                        Self::MESSAGE_ID
                    }
                    fn dlc(&self) -> usize {
                        self.raw.len()
                    }
                    fn data(&self) -> &[u8] {
                        &self.raw
                    }
                }
            }
        })
    }

    fn render_multiplexor_enums(
        &self,
        dbc: &DbcGen<'_>,
        msg: &MessageGen<'_>,
        multiplexor_signal: &SignalGen<'_>,
    ) -> Result<TokenStream> {
        let mut multiplexed_signals: BTreeMap<u64, Vec<SignalGen<'_>>> = BTreeMap::new();
        for signal in msg.signals() {
            if let MultiplexedSignal(switch_index) = signal.multiplexer_indicator {
                multiplexed_signals
                    .entry(switch_index)
                    .and_modify(|v| v.push(signal.clone()))
                    .or_insert_with(|| vec![signal.clone()]);
            }
        }

        let enum_variant_entries: Vec<_> = multiplexed_signals
            .keys()
            .map(|switch_index| {
                let variant_name =
                    msg.multiplexed_enum_variant_name(multiplexor_signal, *switch_index)?;
                Ok((*switch_index, variant_name))
            })
            .collect::<Result<Vec<_>>>()?;

        let struct_defs = multiplexed_signals
            .iter()
            .map(|(switch_index, signals)| {
                let struct_name =
                    msg.multiplexed_enum_variant_name(multiplexor_signal, *switch_index)?;

                let signal_impls: Result<Vec<_>> = signals
                    .iter()
                    .map(|signal| {
                        self.render_signal(signal, dbc, msg)
                            .with_context(|| format!("write signal impl `{}`", signal.name))
                    })
                    .collect();
                let signal_impls = signal_impls?;

                Ok(q! {
                    #{allow_lints()}
                    #{self.allow_dead_code()}
                    #[derive(Default)]
                    #{self.impl_serde.attr(|| q! { derive(Serialize) })}
                    #{self.impl_serde.attr(|| q! { derive(Deserialize) })}
                    pub struct #struct_name {
                        raw: [u8; #{msg.size_lit()}]
                    }

                    #{allow_lints()}
                    #{self.allow_dead_code()}
                    impl #struct_name {
                        pub fn new() -> Self {
                            Self { raw: [0u8; #{msg.size_lit()}] }
                        }

                        #(#signal_impls)*
                    }
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(q! {
            #[doc = #{format!(" Defined values for multiplexed signal {}", msg.name)}]
            #{allow_lints()}
            #{self.allow_dead_code()}
            pub enum #{msg.multiplex_enum_name(multiplexor_signal)?} {
                #(for (switch_index, variant_name) in &enum_variant_entries) {
                    #{multiplexed_enum_variant_wrapper_name(*switch_index)}(#variant_name),
                }
            }

            #(#struct_defs)*
        })
    }

    fn render_arbitrary(&self, msg: &MessageGen<'_>) -> TokenStream {
        let filtered_signals: Vec<SignalGen<'_>> = msg
            .signals()
            .iter()
            .filter(|v| matches!(v.multiplexer_indicator, Plain | Multiplexor))
            .cloned()
            .collect();

        q! {
            #{allow_lints()}
            #{self.allow_dead_code()}
            impl arbitrary::Arbitrary<'_> for #{msg.type_name()} {
                fn arbitrary(
                    #{if filtered_signals.is_empty() { q! { _u } } else { q! { u } }}
                    : &mut arbitrary::Unstructured<'_>
                ) -> arbitrary::Result<Self> {
                    #(for signal in &filtered_signals) {
                        let #{signal.field_name()} = #{signal_to_arbitrary(signal)};
                    }
                    #{msg.type_name()}::new(
                        #(for signal in &filtered_signals) { #{signal.field_name()}, }
                    ).map_err(|_| arbitrary::Error::IncorrectFormat)
                }
            }
        }
    }

    fn render_error(&self) -> TokenStream {
        q! {
            #[allow(dead_code)]
            #[derive(Clone, Copy, Debug, PartialEq, Eq)]
            pub enum CanError {
                UnknownMessageId(embedded_can::Id),
                /// Signal parameter is not within the range
                /// defined in the dbc
                ParameterOutOfRange {
                    /// dbc message id
                    message_id: embedded_can::Id,
                },
                InvalidPayloadSize,
                /// Multiplexor value not defined in the dbc
                InvalidMultiplexor {
                    /// dbc message id
                    message_id: embedded_can::Id,
                    /// Multiplexor value not defined in the dbc
                    multiplexor: u16,
                },
            }

            impl core::fmt::Display for CanError {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    write!(f, "{self:?}")
                }
            }

            #{self.impl_error.if_cfg(|| q! { impl core::error::Error for CanError {} })}
        }
    }

    fn render_arbitrary_helpers(&self) -> TokenStream {
        q! {
            #{self.impl_arbitrary.if_cfg(|| q! {
                #{self.allow_dead_code()}
                trait UnstructuredFloatExt {
                    fn arbitrary_f32(&mut self) -> arbitrary::Result<f32>;
                }
            })}
            #{self.impl_arbitrary.if_cfg(|| q! {
                impl UnstructuredFloatExt for arbitrary::Unstructured<'_> {
                    fn arbitrary_f32(&mut self) -> arbitrary::Result<f32> {
                        Ok(f32::from_bits(u32::arbitrary(self)?))
                    }
                }
            })}
        }
    }

    /// Generate Rust structs matching DBC input description and return as String
    pub fn generate(self) -> Result<String> {
        let tokens = self.codegen().context("could not generate Rust code")?;
        // Debug: write tokens to stderr for debugging
        if std::env::var("DEBUG_TOKENS").is_ok() {
            eprintln!("=== Generated TokenStream ===");
            eprintln!("{tokens}");
            eprintln!("=== End TokenStream ===");
        }
        let file = parse2(tokens).context("Failed to parse generated TokenStream as Rust code")?;
        Ok(prettyplease::unparse(&file))
    }

    /// Generate Rust structs matching DBC input description and write to `out`
    pub fn write(self, mut out: impl Write) -> Result<()> {
        out.write_all(self.generate()?.as_bytes())?;
        Ok(())
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
}

fn render_debug_impl(msg: &MessageGen<'_>) -> TokenStream {
    let plain_signals: Vec<_> = msg
        .signals()
        .iter()
        .filter(|signal| signal.multiplexer_indicator == Plain)
        .cloned()
        .collect();

    q! {
        impl core::fmt::Debug for #{msg.type_name()} {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                if f.alternate() {
                    f.debug_struct(#{msg.type_name_str()})
                        #(for signal in &plain_signals) {
                            .field(
                                #{signal.field_name_str()},
                                &self.#{signal.field_name()}(),
                            )
                        }
                        .finish()
                } else {
                    f.debug_tuple(#{msg.type_name_str()}).field(&self.raw).finish()
                }
            }
        }
    }
}

fn render_defmt_impl(msg: &MessageGen<'_>) -> TokenStream {
    let plain_signals: Vec<_> = msg
        .signals()
        .iter()
        .filter(|signal| signal.multiplexer_indicator == Plain)
        .cloned()
        .collect();

    let mut format_str = format!("{} {{{{", msg.type_name());
    for signal in &plain_signals {
        let _ = write!(format_str, " {}={{:?}}", signal.name);
    }
    format_str.push_str(" }}}}");

    q! {
        impl defmt::Format for #{msg.type_name()} {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    #format_str,
                    #(for signal in &plain_signals) {
                        self.#{signal.field_name()}(),
                    }
                );
            }
        }
    }
}

fn render_set_signal_multiplexer(
    multiplexor: &SignalGen<'_>,
    msg: &MessageGen<'_>,
    switch_index: u64,
) -> Result<TokenStream> {
    Ok(q! {
        #{format!("/// Set value of {}", multiplexor.name).tokens()?}
        #[inline(always)]
        pub fn #{
            multiplexed_enum_variant_wrapper_setter_name(switch_index)
        }(
            &mut self,
            value: #{msg.multiplexed_enum_variant_name(multiplexor, switch_index)?}
        ) -> Result<(), CanError> {
            let b0 = BitArray::<_, LocalBits>::new(self.raw);
            let b1 = BitArray::<_, LocalBits>::new(value.raw);
            self.raw = b0.bitor(b1).into_inner();
            self.#{multiplexor.field_name_setter()}(#{lit_int(switch_index)})?;
            Ok(())
        }
    })
}

fn be_start_end_bit(signal: &SignalGen<'_>, msg: &MessageGen<'_>) -> Result<(u64, u64)> {
    let err = "calculating start bit";

    let x = signal.start_bit.checked_div(8).context(err)?;
    let x = x.checked_mul(8).context(err)?;

    let y = signal.start_bit.checked_rem(8).context(err)?;
    let y = 7u64.checked_sub(y).context(err)?;

    let start_bit = x.checked_add(y).context(err)?;
    let end_bit = start_bit
        .checked_add(signal.size)
        .context("calculating last bit position")?;

    let msg_bits: u64 = msg.size.checked_mul(8).unwrap();

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

fn le_start_end_bit(signal: &SignalGen<'_>, msg: &MessageGen<'_>) -> Result<(u64, u64)> {
    let msg_bits: u64 = msg.size.checked_mul(8).unwrap();
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

fn read_fn_with_type(
    signal: &SignalGen<'_>,
    msg: &MessageGen<'_>,
    typ: ValType,
) -> Result<TokenStream> {
    Ok(match signal.byte_order {
        LittleEndian => {
            let (start, end) = le_start_end_bit(signal, msg)?;
            q! { self.raw.view_bits::<Lsb0>()[#{lit_int(start)}..#{lit_int(end)}].load_le::<#{typ.ident()}>() }
        }
        BigEndian => {
            let (start, end) = be_start_end_bit(signal, msg)?;
            q! { self.raw.view_bits::<Msb0>()[#{lit_int(start)}..#{lit_int(end)}].load_be::<#{typ.ident()}>() }
        }
    })
}

fn signal_from_payload(signal: &SignalGen<'_>, msg: &MessageGen<'_>) -> Result<TokenStream> {
    let read_expr = read_fn_with_type(signal, msg, signal.typ_int())?;
    let typ = signal.typ();
    let typ_ident = typ.ident();

    Ok(match typ {
        ValType::Bool => {
            q! {
                let signal = #read_expr;
                signal == 1
            }
        }
        ValType::F32 => {
            q! {
                let signal = #read_expr;
                let factor = #{lit_float(format!("{}_f32", signal.factor))};
                let offset = #{lit_float(format!("{}_f32", signal.offset))};
                (signal as f32) * factor + offset
            }
        }
        _ => {
            if signal.offset >= 0.0 {
                q! {
                    let signal = #read_expr;
                    let factor = #{lit_float(signal.factor)};
                    #(if Some(typ) == signal.typ_uint_to_int()) { let signal = signal as #typ_ident; }
                    #typ_ident::from(signal).saturating_mul(factor).saturating_add(#{lit_float(signal.offset)})
                }
            } else {
                q! {
                    let signal = #read_expr;
                    let factor = #{lit_float(signal.factor)};
                    #(if Some(typ) == signal.typ_uint_to_int()) { let signal = signal as #typ_ident; }
                    #typ_ident::from(signal).saturating_mul(factor).saturating_sub(#{lit_float(signal.offset.abs())})
                }
            }
        }
    })
}

fn signal_to_payload(signal: &SignalGen<'_>, msg: &MessageGen<'_>) -> Result<TokenStream> {
    Ok(q! {
        #{match signal.typ() {
            ValType::Bool => q! { let value = value as u8; },
            ValType::F32 => {
                q! {
                    let factor = #{lit_float(format!("{}_f32", signal.factor))};
                    let offset = #{lit_float(format!("{}_f32", signal.offset))};
                    let value = ((value - offset) / factor) as #{signal.typ_int().ident()};
                }
            }
            _ => {
                if signal.offset >= 0.0 {
                    q! {
                        let factor = #{lit_float(signal.factor)};
                        let value = value.checked_sub(#{lit_float(signal.offset)})
                            .ok_or(CanError::ParameterOutOfRange { message_id: #{msg.type_name()}::MESSAGE_ID })?;
                        let value = (value / factor) as #{signal.typ_int().ident()};
                    }
                } else {
                    q! {
                        let factor = #{lit_float(signal.factor)};
                        let value = value.checked_add(#{lit_float(signal.offset.abs())})
                            .ok_or(CanError::ParameterOutOfRange { message_id: #{msg.type_name()}::MESSAGE_ID })?;
                        let value = (value / factor) as #{signal.typ_int().ident()};
                    }
                }
            }
        }}
        #(if signal.value_type == Signed) {
            let value = #{signal.typ_uint().ident()}::from_ne_bytes(value.to_ne_bytes());
        }
        #{match signal.byte_order {
            LittleEndian => {
                let (start, end) = le_start_end_bit(signal, msg)?;
                q! { self.raw.view_bits_mut::<Lsb0>()[#{lit_int(start)}..#{lit_int(end)}].store_le(value); }
            }
            BigEndian => {
                let (start, end) = be_start_end_bit(signal, msg)?;
                q! { self.raw.view_bits_mut::<Msb0>()[#{lit_int(start)}..#{lit_int(end)}].store_be(value); }
            }
        }}
        Ok(())
    })
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

fn signal_to_arbitrary(signal: &SignalGen<'_>) -> TokenStream {
    let typ = signal.typ();
    match typ {
        ValType::Bool => q! { u.int_in_range(0..=1)? == 1 },
        ValType::F32 => q! { u.arbitrary_f32()? },
        _ => q! { u.int_in_range(#{signal.min as i64}..=#{signal.max as i64})? as #{typ.ident()} },
    }
}

fn allow_lints() -> TokenStream {
    q! {
        #[allow(
            clippy::absurd_extreme_comparisons,
            clippy::excessive_precision,
            clippy::manual_range_contains,
            clippy::unnecessary_cast,
            clippy::useless_conversion,
            unused_comparisons,
            unused_variables,
        )]
    }
}

/// Generate a literal token for a signal min/max value.
///
/// For F32 types, always generates a float literal.
/// For other types, generates an integer literal if the value is an integer within i64 range,
/// otherwise generates a float literal with the integer type suffix.
fn generate_value_literal(value: f64, typ: ValType) -> TokenStream {
    if typ == ValType::F32 {
        q! { #{lit_float(format!("{value}_f32"))} }
    } else {
        let typ_str = typ.to_string().to_lowercase();
        // Check if value is an integer and fits in i64 range
        if is_integer(value) && value >= i64::MIN as f64 && value <= i64::MAX as f64 {
            q! { #{lit_int(format!("{}_{typ_str}", value as i64))} }
        } else {
            // Use float literal with integer type suffix for fractional/overflow values
            q! { #{lit_float(format!("{value}_{typ_str}"))} }
        }
    }
}
