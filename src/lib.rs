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
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write as _;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use anyhow::{anyhow, ensure, Context, Result};
use can_dbc::ByteOrder::{BigEndian, LittleEndian};
use can_dbc::MultiplexIndicator::{MultiplexedSignal, Multiplexor, Plain};
use can_dbc::ValueType::Signed;
use can_dbc::{Dbc, Message, MessageId, Signal, Transmitter, ValDescription, ValueDescription};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse2, LitFloat, LitInt};
use typed_builder::TypedBuilder;

pub use crate::feature_config::FeatureConfig;
use crate::signal_type::ValType;
use crate::utils::{
    enum_name, enum_variant_name, is_integer, multiplex_enum_name, multiplexed_enum_variant_name,
    multiplexed_enum_variant_wrapper_name, multiplexed_enum_variant_wrapper_setter_name,
    MessageExt as _, SignalExt as _, ToIdent as _, Tokens as _,
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

        let dbc_name = &self.dbc_name;
        let dbc_version = &dbc.version.0;
        let header = quote! {
            /// The name of the DBC file this code was generated from
            #[allow(dead_code)]
            pub const DBC_FILE_NAME: &str = #dbc_name;
            /// The version of the DBC file this code was generated from
            #[allow(dead_code)]
            pub const DBC_FILE_VERSION: &str = #dbc_version;

        };

        let mut use_statements = quote! {
            #[allow(unused_imports)]
            use core::ops::BitOr;
            #[allow(unused_imports)]
            use bitvec::prelude::*;
            #[allow(unused_imports)]
            use embedded_can::{Id, StandardId, ExtendedId};
        };
        use_statements.extend(
            self.impl_arbitrary
                .if_cfg(quote! { use arbitrary::Arbitrary; }),
        );
        use_statements.extend(self.impl_serde.if_cfg(quote! {
            use serde::{Serialize, Deserialize};
        }));

        let dbc_content = self
            .render_dbc(&dbc)
            .context("could not generate Rust code")?;
        let error_content = self.render_error();
        let arbitrary_helpers = self.render_arbitrary_helpers();

        Ok(quote! {
            #header
            #use_statements
            #dbc_content

            /// This is just to make testing easier
            #[allow(dead_code)]
            fn main() {}

            #error_content
            #arbitrary_helpers
        })
    }

    fn render_dbc(&self, dbc: &Dbc) -> Result<TokenStream> {
        let root_enum = self.render_root_enum(dbc);

        let messages = get_relevant_messages(dbc)
            .map(|msg| {
                self.render_message(msg, dbc)
                    .with_context(|| format!("write message `{}`", msg.name))
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(quote! {
            #root_enum
            #(#messages)*
        })
    }

    fn render_root_enum(&self, dbc: &Dbc) -> TokenStream {
        let allow_dead_code = allow_dead_code_tokens(self.allow_dead_code);
        let debug_derive = self.impl_debug.attr(&quote! { derive(Debug) });
        let defmt_derive = self.impl_defmt.attr(&quote! { derive(defmt::Format) });
        let serde_derives = self
            .impl_serde
            .attr(&quote! { derive(Serialize, Deserialize) });
        let allow_lints = allow_lints();

        let variants: Vec<_> = get_relevant_messages(dbc)
            .map(|msg| {
                let msg_type = msg.type_name();
                let doc_str = format!(" {}", msg.name); // Use message name, not type_name
                quote! {
                    #[doc = #doc_str]
                    #msg_type(#msg_type)
                }
            })
            .collect();

        let from_can_arms: Vec<_> = get_relevant_messages(dbc)
            .map(|msg| {
                let msg_type = msg.type_name();
                quote! { #msg_type::MESSAGE_ID => Messages::#msg_type(#msg_type::try_from(payload)?) }
            })
            .collect();

        let from_can_body = if from_can_arms.is_empty() {
            quote! { Err(CanError::UnknownMessageId(id)) }
        } else {
            quote! {
                let res = match id {
                    #(#from_can_arms,)*
                    id => return Err(CanError::UnknownMessageId(id)),
                };
                Ok(res)
            }
        };

        quote! {
            /// All messages
            #allow_lints
            #allow_dead_code
            #[derive(Clone)]
            #debug_derive
            #defmt_derive
            #serde_derives
            pub enum Messages {
                #(#variants,)*
            }

            #allow_lints
            #allow_dead_code
            impl Messages {
                /// Read message from CAN frame
                #[inline(never)]
                pub fn from_can_message(id: Id, payload: &[u8]) -> Result<Self, CanError> {
                    #from_can_body
                }
            }
        }
    }

    fn render_message(&self, msg: &Message, dbc: &Dbc) -> Result<TokenStream> {
        let mut doc = String::new();
        writeln!(doc, "/// {}", msg.name)?;
        writeln!(doc, "///")?;
        let message_id = match msg.id {
            MessageId::Standard(id) => {
                writeln!(doc, "/// - Standard ID: {id} (0x{id:x})")?;
                let id_lit = LitInt::new(&format!("{id:#x}"), Span::call_site());
                quote! { Id::Standard(unsafe { StandardId::new_unchecked(#id_lit) }) }
            }
            MessageId::Extended(id) => {
                writeln!(doc, "/// - Extended ID: {id} (0x{id:x})")?;
                let id_lit = LitInt::new(&format!("{id:#x}"), Span::call_site());
                quote! { Id::Extended(unsafe { ExtendedId::new_unchecked(#id_lit) }) }
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

        let msg_type = msg.type_name();
        let msg_size = msg.size as usize;
        let msg_size_lit = LitInt::new(&msg_size.to_string(), Span::call_site());

        // Struct attributes
        let serde_serialize = self.impl_serde.attr(&quote! { derive(Serialize) });
        let serde_deserialize = self.impl_serde.attr(&quote! { derive(Deserialize) });
        let serde_with = self
            .impl_serde
            .attr(&quote! { serde(with = "serde_bytes") });

        // Signal min/max constants
        let signal_constants: Vec<_> = msg
            .signals
            .iter()
            .filter_map(|signal| {
                let typ = ValType::from_signal(signal);
                if typ == ValType::Bool {
                    None
                } else {
                    let min_name = signal.const_name("_MIN");
                    let max_name = signal.const_name("_MAX");
                    let typ_ident = typ.ident();
                    let min_lit = generate_value_literal(signal.min, typ);
                    let max_lit = generate_value_literal(signal.max, typ);

                    Some(quote! {
                        pub const #min_name: #typ_ident = #min_lit;
                        pub const #max_name: #typ_ident = #max_lit;
                    })
                }
            })
            .collect();

        // New function arguments and setter calls
        let new_fn_args: Vec<_> = msg
            .signals
            .iter()
            .filter_map(|signal| {
                if matches!(signal.multiplexer_indicator, Plain | Multiplexor) {
                    let field_name = signal.field_name();
                    let typ = ValType::from_signal(signal);
                    let typ_ident = typ.ident();
                    Some(quote! { #field_name: #typ_ident })
                } else {
                    None
                }
            })
            .collect();

        let new_fn_setters: Vec<_> = msg
            .signals
            .iter()
            .filter_map(|signal| {
                if matches!(signal.multiplexer_indicator, Plain | Multiplexor) {
                    let field_name = signal.field_name();
                    let setter_name = signal.field_name2("set_", "");
                    Some(quote! { res.#setter_name(#field_name)?; })
                } else {
                    None
                }
            })
            .collect();

        let new_fn_mutability = if msg.signals.is_empty() {
            quote! {}
        } else {
            quote! { mut }
        };

        // Render signals
        let signal_impls: Result<Vec<_>> = msg
            .signals
            .iter()
            .map(|signal| {
                if signal.multiplexer_indicator == Multiplexor {
                    self.render_multiplexor_signal(signal, msg)
                        .with_context(|| format!("write signal impl `{}`", signal.name))
                } else if signal.multiplexer_indicator == Plain {
                    self.render_signal(signal, dbc, msg)
                        .with_context(|| format!("write signal impl `{}`", signal.name))
                } else {
                    Ok(quote! {})
                }
            })
            .collect();
        let signal_impls = signal_impls?;

        // Render embedded can frame impl
        let embedded_can_impl = self.render_embedded_can_frame(msg);

        // Render debug/defmt/arbitrary impls
        let debug_impl = self.impl_debug.if_cfg(render_debug_impl(msg));
        let defmt_impl = self.impl_defmt.if_cfg(render_defmt_impl(msg));
        let arbitrary_impl = self.impl_arbitrary.if_cfg(self.render_arbitrary(msg));

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
                        .map(|v| self.write_enum(v, msg, value_descriptions.as_slice()))
                } else {
                    None
                }
            })
            .collect();

        // Render multiplexor enums
        let multiplexor_enums = msg
            .signals
            .iter()
            .find(|signal| signal.multiplexer_indicator == Multiplexor)
            .map(|multiplexor_signal| self.render_multiplexor_enums(dbc, msg, multiplexor_signal))
            .transpose()?;

        let allow_lints = allow_lints();
        let allow_dead_code = allow_dead_code_tokens(self.allow_dead_code);

        let new_fn_doc = format!("/// Construct new {} from values", msg.name).tokens()?;

        Ok(quote! {
            #struct_doc
            #[derive(Clone, Copy)]
            #serde_serialize
            #serde_deserialize
            pub struct #msg_type {
                #serde_with
                raw: [u8; #msg_size_lit],
            }

            #allow_lints
            #allow_dead_code
            impl #msg_type {
                pub const MESSAGE_ID: embedded_can::Id = #message_id;

                #(#signal_constants)*

                #new_fn_doc
                pub fn new(#(#new_fn_args),*) -> Result<Self, CanError> {
                    let #new_fn_mutability res = Self { raw: [0u8; #msg_size_lit] };
                    #(#new_fn_setters)*
                    Ok(res)
                }

                /// Access message payload raw value
                pub fn raw(&self) -> &[u8; #msg_size_lit] {
                    &self.raw
                }

                #(#signal_impls)*
            }

            impl core::convert::TryFrom<&[u8]> for #msg_type {
                type Error = CanError;

                #[inline(always)]
                fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
                    if payload.len() != #msg_size_lit {
                        return Err(CanError::InvalidPayloadSize);
                    }
                    let mut raw = [0u8; #msg_size_lit];
                    raw.copy_from_slice(&payload[..#msg_size_lit]);
                    Ok(Self { raw })
                }
            }

            #embedded_can_impl
            #debug_impl
            #defmt_impl
            #arbitrary_impl
            #(#enums_for_this_message)*
            #multiplexor_enums
        })
    }

    fn render_signal(&self, signal: &Signal, dbc: &Dbc, msg: &Message) -> Result<TokenStream> {
        let signal_name = &signal.name;
        let fn_name = signal.field_name();
        let fn_name_raw = signal.field_name2("", "_raw");

        // Build signal getter doc as doc comment and parse into tokens
        let mut doc = format!("/// {signal_name}\n");
        if let Some(comment) = dbc.signal_comment(msg.id, &signal.name) {
            doc.push_str("///\n");
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
        let signal_doc = doc.tokens()?;

        let typ = ValType::from_signal(signal);
        let typ = typ.ident();

        // Generate getter function
        let getter = if let Some(variants) = dbc.value_descriptions_for_signal(msg.id, &signal.name)
        {
            let type_name = enum_name(msg, signal);
            let signal_ty = ValType::from_signal(signal);
            let variant_infos = generate_variant_info(variants, signal_ty);

            // Use signed type for loading when signal is signed and has negative values
            let has_negative_values = variants.iter().any(|v| v.id < 0);
            let load_type = if signal.value_type == Signed && has_negative_values {
                signal_ty
            } else {
                ValType::from_signal_uint(signal)
            };

            let read_expr = read_fn_with_type(signal, msg, load_type)?;

            let match_arms: Vec<_> = variant_infos
                .iter()
                .map(|info| {
                    let literal = LitInt::new(&info.value.to_string(), Span::call_site());
                    let variant = info.base_name.ident();
                    match info.dup_type {
                        DuplicateType::Unique => {
                            quote! { #literal => #type_name::#variant }
                        }
                        DuplicateType::FirstDuplicate | DuplicateType::Duplicate => {
                            quote! { #literal => #type_name::#variant(#literal) }
                        }
                    }
                })
                .collect();

            quote! {
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
            quote! {
                #[inline(always)]
                pub fn #fn_name(&self) -> #typ {
                    self.#fn_name_raw()
                }
            }
        };

        let raw_doc_text = format!(
            "\
/// Get raw value of {signal_name}
///
/// - Start bit: {}
/// - Signal size: {} bits
/// - Factor: {}
/// - Offset: {}
/// - Byte order: {:?}
/// - Value type: {:?}",
            signal.start_bit,
            signal.size,
            signal.factor,
            signal.offset,
            signal.byte_order,
            signal.value_type
        );
        let fn_name_doc = raw_doc_text.tokens()?;
        let fn_body = signal_from_payload(signal, msg).context("signal from payload")?;
        let setter = self.render_set_signal(signal, msg)?;

        Ok(quote! {
            #signal_doc
            #getter

            #fn_name_doc
            #[inline(always)]
            pub fn #fn_name_raw(&self) -> #typ {
                #fn_body
            }

            #setter
        })
    }

    fn render_set_signal(&self, signal: &Signal, msg: &Message) -> Result<TokenStream> {
        let setter_name = signal.field_name2("set_", "");

        // To avoid accidentally changing the multiplexor value without changing
        // the signals accordingly this fn is kept private for multiplexors.
        let visibility = if signal.multiplexer_indicator == Multiplexor {
            quote! {}
        } else {
            quote! { pub }
        };

        let setter_doc = format!(" Set value of {}", signal.name);
        let typ = ValType::from_signal(signal);
        let typ_ident = typ.ident();
        let msg_type = msg.type_name();

        // Range check logic
        let range_check = if signal.size == 1 {
            quote! {}
        } else {
            let min_lit = generate_value_literal(signal.min, typ);
            let max_lit = generate_value_literal(signal.max, typ);

            let check_code = quote! {
                if value < #min_lit || #max_lit < value {
                    return Err(CanError::ParameterOutOfRange { message_id: #msg_type::MESSAGE_ID });
                }
            };

            self.check_ranges.if_cfg(check_code)
        };

        let signal_to_payload_body = signal_to_payload(signal, msg)?;

        Ok(quote! {
            #[doc = #setter_doc]
            #[inline(always)]
            #visibility fn #setter_name(&mut self, value: #typ_ident) -> Result<(), CanError> {
                #range_check
                #signal_to_payload_body
            }
        })
    }
}

fn render_set_signal_multiplexer(
    multiplexor: &Signal,
    msg: &Message,
    switch_index: u64,
) -> Result<TokenStream> {
    let enum_variant = multiplexed_enum_variant_name(msg, multiplexor, switch_index)?;
    let setter_name = multiplexed_enum_variant_wrapper_setter_name(switch_index);
    let multiplexor_setter = multiplexor.field_name2("set_", "");
    let switch_index_lit = LitInt::new(&switch_index.to_string(), Span::call_site());

    let doc = format!("/// Set value of {}", multiplexor.name).tokens()?;

    Ok(quote! {
        #doc
        #[inline(always)]
        pub fn #setter_name(&mut self, value: #enum_variant) -> Result<(), CanError> {
            let b0 = BitArray::<_, LocalBits>::new(self.raw);
            let b1 = BitArray::<_, LocalBits>::new(value.raw);
            self.raw = b0.bitor(b1).into_inner();
            self.#multiplexor_setter(#switch_index_lit)?;
            Ok(())
        }
    })
}

impl Config<'_> {
    fn render_multiplexor_signal(&self, signal: &Signal, msg: &Message) -> Result<TokenStream> {
        let field = signal.field_name();
        let field_raw = signal.field_name2("", "_raw");
        let typ = ValType::from_signal(signal);
        let typ_ident = typ.ident();
        let enum_type = multiplex_enum_name(msg, signal)?;

        // Build raw doc as doc comment string and parse into tokens
        let raw_doc_text = format!(
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
        );
        let field_raw_doc = raw_doc_text.tokens()?;

        let signal_from_payload_body = signal_from_payload(signal, msg)?;

        let multiplexer_indexes: BTreeSet<u64> = msg
            .signals
            .iter()
            .filter_map(|s| {
                if let MultiplexedSignal(index) = &s.multiplexer_indicator {
                    Some(*index)
                } else {
                    None
                }
            })
            .collect();

        let match_arms: Vec<_> = multiplexer_indexes.iter().map(|idx| {
            let multiplexed_wrapper_name = multiplexed_enum_variant_wrapper_name(*idx);
            let multiplexed_name = multiplexed_enum_variant_name(msg, signal, *idx).unwrap();
            let idx_lit = LitInt::new(&idx.to_string(), Span::call_site());
            quote! {
                #idx_lit => Ok(#enum_type::#multiplexed_wrapper_name(#multiplexed_name { raw: self.raw }))
            }
        }).collect();

        let msg_type = msg.type_name();

        let setter = self.render_set_signal(signal, msg)?;

        let set_multiplexer_fns: Result<Vec<_>> = multiplexer_indexes
            .iter()
            .map(|switch_index| render_set_signal_multiplexer(signal, msg, *switch_index))
            .collect();
        let set_multiplexer_fns = set_multiplexer_fns?;

        Ok(quote! {
            #field_raw_doc
            #[inline(always)]
            pub fn #field_raw(&self) -> #typ_ident {
                #signal_from_payload_body
            }

            pub fn #field(&mut self) -> Result<#enum_type, CanError> {
                match self.#field_raw() {
                    #(#match_arms,)*
                    multiplexor => Err(CanError::InvalidMultiplexor {
                        message_id: #msg_type::MESSAGE_ID,
                        multiplexor: multiplexor.into()
                    }),
                }
            }

            #setter
            #(#set_multiplexer_fns)*
        })
    }
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

fn read_fn_with_type(signal: &Signal, msg: &Message, typ: ValType) -> Result<TokenStream> {
    let typ_ident = typ.ident();
    Ok(match signal.byte_order {
        LittleEndian => {
            let (start, end) = le_start_end_bit(signal, msg)?;
            let start_lit = LitInt::new(&start.to_string(), Span::call_site());
            let end_lit = LitInt::new(&end.to_string(), Span::call_site());
            quote! { self.raw.view_bits::<Lsb0>()[#start_lit..#end_lit].load_le::<#typ_ident>() }
        }
        BigEndian => {
            let (start, end) = be_start_end_bit(signal, msg)?;
            let start_lit = LitInt::new(&start.to_string(), Span::call_site());
            let end_lit = LitInt::new(&end.to_string(), Span::call_site());
            quote! { self.raw.view_bits::<Msb0>()[#start_lit..#end_lit].load_be::<#typ_ident>() }
        }
    })
}

fn signal_from_payload(signal: &Signal, msg: &Message) -> Result<TokenStream> {
    let read_expr = read_fn_with_type(signal, msg, ValType::from_signal_int(signal))?;

    let typ = ValType::from_signal(signal);
    let typ_ident = typ.ident();

    Ok(match typ {
        ValType::Bool => {
            quote! {
                let signal = #read_expr;
                signal == 1
            }
        }
        ValType::F32 => {
            let factor = signal.factor;
            let offset = signal.offset;
            let factor_lit = LitFloat::new(&format!("{factor}_f32"), Span::call_site());
            let offset_lit = LitFloat::new(&format!("{offset}_f32"), Span::call_site());
            quote! {
                let signal = #read_expr;
                let factor = #factor_lit;
                let offset = #offset_lit;
                (signal as f32) * factor + offset
            }
        }
        _ => {
            let factor = signal.factor;
            let factor_lit = LitFloat::new(&factor.to_string(), Span::call_site());

            let signal_cast = if Some(typ) == ValType::from_signal_uint(signal).unsigned_to_signed()
            {
                quote! { let signal = signal as #typ_ident; }
            } else {
                quote! {}
            };

            if signal.offset >= 0.0 {
                let offset = signal.offset;
                let offset_lit = LitFloat::new(&offset.to_string(), Span::call_site());
                quote! {
                    let signal = #read_expr;
                    let factor = #factor_lit;
                    #signal_cast
                    #typ_ident::from(signal).saturating_mul(factor).saturating_add(#offset_lit)
                }
            } else {
                let offset_abs = signal.offset.abs();
                let offset_lit = LitFloat::new(&offset_abs.to_string(), Span::call_site());
                quote! {
                    let signal = #read_expr;
                    let factor = #factor_lit;
                    #signal_cast
                    #typ_ident::from(signal).saturating_mul(factor).saturating_sub(#offset_lit)
                }
            }
        }
    })
}

fn signal_to_payload(signal: &Signal, msg: &Message) -> Result<TokenStream> {
    let typ = ValType::from_signal(signal);
    let msg_type = msg.type_name();

    let value_conversion = match typ {
        ValType::Bool => {
            quote! { let value = value as u8; }
        }
        ValType::F32 => {
            let factor = signal.factor;
            let offset = signal.offset;
            let factor_lit = LitFloat::new(&format!("{factor}_f32"), Span::call_site());
            let offset_lit = LitFloat::new(&format!("{offset}_f32"), Span::call_site());
            let int_typ = ValType::from_signal_int(signal);
            let int_typ_ident = int_typ.ident();
            quote! {
                let factor = #factor_lit;
                let offset = #offset_lit;
                let value = ((value - offset) / factor) as #int_typ_ident;
            }
        }
        _ => {
            let factor = signal.factor;
            let factor_lit = LitFloat::new(&factor.to_string(), Span::call_site());
            let int_typ = ValType::from_signal_int(signal);
            let int_typ_ident = int_typ.ident();

            if signal.offset >= 0.0 {
                let offset = signal.offset;
                let offset_lit = LitFloat::new(&offset.to_string(), Span::call_site());
                quote! {
                    let factor = #factor_lit;
                    let value = value.checked_sub(#offset_lit)
                        .ok_or(CanError::ParameterOutOfRange { message_id: #msg_type::MESSAGE_ID })?;
                    let value = (value / factor) as #int_typ_ident;
                }
            } else {
                let offset_abs = signal.offset.abs();
                let offset_lit = LitFloat::new(&offset_abs.to_string(), Span::call_site());
                quote! {
                    let factor = #factor_lit;
                    let value = value.checked_add(#offset_lit)
                        .ok_or(CanError::ParameterOutOfRange { message_id: #msg_type::MESSAGE_ID })?;
                    let value = (value / factor) as #int_typ_ident;
                }
            }
        }
    };

    let signed_conversion = if signal.value_type == Signed {
        let uint_typ = ValType::from_signal_uint(signal);
        let uint_typ_ident = uint_typ.ident();
        quote! { let value = #uint_typ_ident::from_ne_bytes(value.to_ne_bytes()); }
    } else {
        quote! {}
    };

    let store_expr = match signal.byte_order {
        LittleEndian => {
            let (start, end) = le_start_end_bit(signal, msg)?;
            let start_lit = LitInt::new(&start.to_string(), Span::call_site());
            let end_lit = LitInt::new(&end.to_string(), Span::call_site());
            quote! { self.raw.view_bits_mut::<Lsb0>()[#start_lit..#end_lit].store_le(value); }
        }
        BigEndian => {
            let (start, end) = be_start_end_bit(signal, msg)?;
            let start_lit = LitInt::new(&start.to_string(), Span::call_site());
            let end_lit = LitInt::new(&end.to_string(), Span::call_site());
            quote! { self.raw.view_bits_mut::<Msb0>()[#start_lit..#end_lit].store_be(value); }
        }
    };

    Ok(quote! {
        #value_conversion
        #signed_conversion
        #store_expr
        Ok(())
    })
}

impl Config<'_> {
    fn write_enum(
        &self,
        signal: &Signal,
        msg: &Message,
        variants: &[ValDescription],
    ) -> TokenStream {
        let type_name = enum_name(msg, signal);
        let signal_ty = ValType::from_signal(signal);
        let signal_ty_ident = signal_ty.ident();

        // Generate variant info to handle duplicates with tuple variants
        let variant_infos = generate_variant_info(variants, signal_ty);

        let signal_name = &signal.name;
        let doc = format!(" Defined values for {signal_name}");

        let allow_lints = allow_lints();
        let allow_dead_code = allow_dead_code_tokens(self.allow_dead_code);
        let debug_derive = self.impl_debug.attr(&quote! { derive(Debug) });
        let defmt_derive = self.impl_defmt.attr(&quote! { derive(defmt::Format) });
        let serde_serialize = self.impl_serde.attr(&quote! { derive(Serialize) });
        let serde_deserialize = self.impl_serde.attr(&quote! { derive(Deserialize) });

        // Generate enum variants
        let enum_variants: Vec<_> = variant_infos
            .iter()
            .filter_map(|info| {
                let variant = info.base_name.ident();
                match info.dup_type {
                    DuplicateType::Unique => Some(quote! { #variant }),
                    DuplicateType::FirstDuplicate => {
                        let value_type = info.value_type.ident();
                        Some(quote! { #variant(#value_type) })
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
                        let literal_value = match signal_ty {
                            ValType::Bool => {
                                if info.value == 1 {
                                    quote! { true }
                                } else {
                                    quote! { false }
                                }
                            }
                            ValType::F32 => {
                                let val = LitFloat::new(
                                    &format!("{}_f32", info.value),
                                    Span::call_site(),
                                );
                                quote! { #val }
                            }
                            _ => {
                                let val = LitInt::new(&info.value.to_string(), Span::call_site());
                                quote! { #val }
                            }
                        };
                        Some(quote! { #type_name::#variant => #literal_value })
                    }
                    DuplicateType::FirstDuplicate => Some(quote! { #type_name::#variant(v) => v }),
                    DuplicateType::Duplicate => None,
                }
            })
            .collect();

        quote! {
            #[doc = #doc]
            #allow_lints
            #allow_dead_code
            #[derive(Clone, Copy, PartialEq)]
            #debug_derive
            #defmt_derive
            #serde_serialize
            #serde_deserialize
            pub enum #type_name {
                #(#enum_variants,)*
                _Other(#signal_ty_ident),
            }

            impl From<#type_name> for #signal_ty_ident {
                fn from(val: #type_name) -> #signal_ty_ident {
                    match val {
                        #(#from_match_arms,)*
                        #type_name::_Other(x) => x,
                    }
                }
            }
        }
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

impl Config<'_> {
    fn render_embedded_can_frame(&self, msg: &Message) -> TokenStream {
        let msg_type = msg.type_name();

        let impl_tokens = quote! {
            impl embedded_can::Frame for #msg_type {
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
        };

        self.impl_embedded_can_frame.if_cfg(impl_tokens)
    }
}

fn render_debug_impl(msg: &Message) -> TokenStream {
    let msg_type = msg.type_name();
    let typ_name = msg_type.to_string();

    let debug_fields: Vec<_> = msg
        .signals
        .iter()
        .filter(|signal| signal.multiplexer_indicator == Plain)
        .map(|signal| {
            let field_ident = signal.field_name();
            let field_name = field_ident.to_string();
            quote! { .field(#field_name, &self.#field_ident()) }
        })
        .collect();

    quote! {
        impl core::fmt::Debug for #msg_type {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                if f.alternate() {
                    f.debug_struct(#typ_name)
                        #(#debug_fields)*
                        .finish()
                } else {
                    f.debug_tuple(#typ_name).field(&self.raw).finish()
                }
            }
        }
    }
}

fn render_defmt_impl(msg: &Message) -> TokenStream {
    let msg_type = msg.type_name();
    let typ_name = msg_type.to_string();

    let plain_signals: Vec<_> = msg
        .signals
        .iter()
        .filter(|signal| signal.multiplexer_indicator == Plain)
        .collect();

    // Build format string
    let mut format_str = format!("{typ_name} {{{{");
    for signal in &plain_signals {
        let _ = write!(format_str, " {}={{:?}}", signal.name);
    }
    format_str.push_str(" }}}}");

    // Build field accessors
    let field_accessors: Vec<_> = plain_signals
        .iter()
        .map(|signal| {
            let field_ident = signal.field_name();
            quote! { self.#field_ident() }
        })
        .collect();

    quote! {
        impl defmt::Format for #msg_type {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    #format_str,
                    #(#field_accessors,)*
                );
            }
        }
    }
}

impl Config<'_> {
    fn render_multiplexor_enums(
        &self,
        dbc: &Dbc,
        msg: &Message,
        multiplexor_signal: &Signal,
    ) -> Result<TokenStream> {
        let mut multiplexed_signals = BTreeMap::new();
        for signal in &msg.signals {
            if let MultiplexedSignal(switch_index) = signal.multiplexer_indicator {
                multiplexed_signals
                    .entry(switch_index)
                    .and_modify(|v: &mut Vec<&Signal>| v.push(signal))
                    .or_insert_with(|| vec![signal]);
            }
        }

        let doc = format!(" Defined values for multiplexed signal {}", msg.name);

        let enum_name = multiplex_enum_name(msg, multiplexor_signal)?;

        // Generate enum variants
        let enum_variants = multiplexed_signals
            .keys()
            .map(|switch_index| {
                let wrapper_name = multiplexed_enum_variant_wrapper_name(*switch_index);
                let variant_name =
                    multiplexed_enum_variant_name(msg, multiplexor_signal, *switch_index)?;

                Ok(quote! { #wrapper_name(#variant_name) })
            })
            .collect::<Result<Vec<_>>>()?;

        // Generate structs for each multiplexed signal
        let allow_lints_outer = allow_lints();
        let allow_dead_code_outer = allow_dead_code_tokens(self.allow_dead_code);

        let struct_defs: Result<Vec<_>> = multiplexed_signals
            .iter()
            .map(|(switch_index, signals)| {
                let struct_name =
                    multiplexed_enum_variant_name(msg, multiplexor_signal, *switch_index)?;
                let msg_size = msg.size as usize;
                let msg_size_lit = LitInt::new(&msg_size.to_string(), Span::call_site());

                let signal_impls: Result<Vec<_>> = signals
                    .iter()
                    .map(|signal| {
                        self.render_signal(signal, dbc, msg)
                            .with_context(|| format!("write signal impl `{}`", signal.name))
                    })
                    .collect();
                let signal_impls = signal_impls?;

                let allow_lints_inner = allow_lints_outer.clone();
                let allow_dead_code_inner = allow_dead_code_outer.clone();

                let serde_serialize_inner = self.impl_serde.attr(&quote! { derive(Serialize) });
                let serde_deserialize_inner = self.impl_serde.attr(&quote! { derive(Deserialize) });

                let allow_lints_inner2 = allow_lints_outer.clone();
                let allow_dead_code_inner2 = allow_dead_code_outer.clone();

                Ok(quote! {
                    #allow_lints_inner
                    #allow_dead_code_inner
                    #[derive(Default)]
                    #serde_serialize_inner
                    #serde_deserialize_inner
                    pub struct #struct_name {
                        raw: [u8; #msg_size_lit]
                    }

                    #allow_lints_inner2
                    #allow_dead_code_inner2
                    impl #struct_name {
                        pub fn new() -> Self {
                            Self { raw: [0u8; #msg_size_lit] }
                        }

                        #(#signal_impls)*
                    }
                })
            })
            .collect();
        let struct_defs = struct_defs?;

        Ok(quote! {
            #[doc = #doc]
            #allow_lints_outer
            #allow_dead_code_outer
            pub enum #enum_name {
                #(#enum_variants,)*
            }

            #(#struct_defs)*
        })
    }

    fn render_arbitrary(&self, msg: &Message) -> TokenStream {
        let allow_lints = allow_lints();
        let allow_dead_code = allow_dead_code_tokens(self.allow_dead_code);
        let msg_type = msg.type_name();

        let filtered_signals: Vec<&Signal> = msg
            .signals
            .iter()
            .filter(|v| matches!(v.multiplexer_indicator, Plain | Multiplexor))
            .collect();

        let u_param = if filtered_signals.is_empty() {
            quote! { _u }
        } else {
            quote! { u }
        };

        // Generate signal bindings
        let signal_bindings: Vec<_> = filtered_signals
            .iter()
            .map(|signal| {
                let field_name = signal.field_name();
                let value_expr = signal_to_arbitrary(signal);
                quote! { let #field_name = #value_expr; }
            })
            .collect();

        // Generate function arguments
        let args: Vec<_> = filtered_signals
            .iter()
            .map(|signal| signal.field_name())
            .collect();

        quote! {
            #allow_lints
            #allow_dead_code
            impl arbitrary::Arbitrary<'_> for #msg_type {
                fn arbitrary(#u_param: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
                    #(#signal_bindings)*
                    #msg_type::new(#(#args),*)
                        .map_err(|_| arbitrary::Error::IncorrectFormat)
                }
            }
        }
    }

    fn render_error(&self) -> TokenStream {
        let error_impl = self.impl_error.if_cfg(quote! {
            impl core::error::Error for CanError {}
        });

        quote! {
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

            #error_impl
        }
    }

    fn render_arbitrary_helpers(&self) -> TokenStream {
        let allow_dead_code = allow_dead_code_tokens(self.allow_dead_code);

        let trait_def = self.impl_arbitrary.if_cfg(quote! {
            #allow_dead_code
            trait UnstructuredFloatExt {
                fn arbitrary_f32(&mut self) -> arbitrary::Result<f32>;
            }
        });

        let trait_impl = self.impl_arbitrary.if_cfg(quote! {
            impl UnstructuredFloatExt for arbitrary::Unstructured<'_> {
                fn arbitrary_f32(&mut self) -> arbitrary::Result<f32> {
                    Ok(f32::from_bits(u32::arbitrary(self)?))
                }
            }
        });

        quote! {
            #trait_def
            #trait_impl
        }
    }
}

fn signal_to_arbitrary(signal: &Signal) -> TokenStream {
    let typ = ValType::from_signal(signal);
    match typ {
        ValType::Bool => quote! { u.int_in_range(0..=1)? == 1 },
        ValType::F32 => {
            quote! { u.arbitrary_f32()? }
        }
        _ => {
            let min = signal.min as i64;
            let max = signal.max as i64;
            let typ_ident = typ.ident();
            quote! { u.int_in_range(#min..=#max)? as #typ_ident }
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
        let tokens = self.codegen()?;
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

fn allow_lints() -> TokenStream {
    quote! {
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
        let lit = LitFloat::new(&format!("{value}_f32"), Span::call_site());
        quote! { #lit }
    } else {
        let typ_str = typ.to_string().to_lowercase();
        // Check if value is an integer and fits in i64 range
        if is_integer(value) && value >= i64::MIN as f64 && value <= i64::MAX as f64 {
            let val = value as i64;
            let lit = LitInt::new(&format!("{val}_{typ_str}"), Span::call_site());
            quote! { #lit }
        } else {
            // Use float literal with integer type suffix for fractional/overflow values
            let lit = LitFloat::new(&format!("{value}_{typ_str}"), Span::call_site());
            quote! { #lit }
        }
    }
}

/// Generate `[allow(dead_code)]` attribute if needed
fn allow_dead_code_tokens(allow: bool) -> Option<TokenStream> {
    if allow {
        use quote::quote;
        Some(quote! { #[allow(dead_code)] })
    } else {
        None
    }
}
