use std::ops::Deref;

use anyhow::{ensure, Result};
use can_dbc::MultiplexIndicator::Multiplexor;
use can_dbc::{Dbc, Message, MessageId, Signal};
use heck::{ToPascalCase, ToShoutySnakeCase, ToSnakeCase};
use proc_macro2::Ident;
use quote::format_ident;

use crate::signal_type::ValType;
use crate::utils::{enum_variant_name, sanitize_name, ToIdent};

/// Wrapper around [`Dbc`] used during code generation. Holds a pre-built vec of [`MessageGen`] for the DBC's messages.
#[derive(Clone)]
pub(crate) struct DbcGen<'a> {
    inner: &'a Dbc,
    messages: Vec<MessageGen<'a>>,
}

impl Deref for DbcGen<'_> {
    type Target = Dbc;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'a> DbcGen<'a> {
    /// Creates a wrapper and builds the cached messages list.
    pub fn new(inner: &'a Dbc) -> DbcGen<'a> {
        let messages = inner
            .messages
            .iter()
            .filter(|v| v.name != "VECTOR__INDEPENDENT_SIG_MSG")
            .map(MessageGen::new)
            .collect();

        DbcGen { inner, messages }
    }

    #[must_use]
    pub fn messages(&self) -> &[MessageGen<'_>] {
        &self.messages
    }

    /// Looks up a signal by message id and name. Returns a [`SignalGen`] wrapper if found.
    pub fn signal_by_name(&self, message_id: MessageId, name: &str) -> Option<SignalGen<'_>> {
        self.inner
            .signal_by_name(message_id, name)
            .map(SignalGen::new)
    }
}

/// Wrapper around [`Message`] used during code generation. Holds a pre-built vec of [`SignalGen`] for the message's signals.
#[derive(Clone)]
pub(crate) struct MessageGen<'a> {
    inner: &'a Message,
    signals: Vec<SignalGen<'a>>,
    type_name: Ident,
}

impl<'a> MessageGen<'a> {
    /// Creates a wrapper and builds the cached signals list and type name.
    pub fn new(inner: &'a Message) -> MessageGen<'a> {
        MessageGen {
            inner,
            signals: inner.signals.iter().map(SignalGen::new).collect(),
            type_name: format_ident!(
                "{}",
                sanitize_name(&inner.name, ToPascalCase::to_pascal_case)
            ),
        }
    }

    #[must_use]
    pub fn signals(&self) -> &[SignalGen<'_>] {
        &self.signals
    }

    /// Type name (`PascalCase`) for the message, safe for use as a Rust type.
    #[must_use]
    pub fn type_name(&self) -> Ident {
        self.type_name.clone()
    }

    pub fn type_name_str(&self) -> String {
        self.type_name.to_string()
    }

    pub fn size_lit(&self) -> proc_macro2::Literal {
        proc_macro2::Literal::u64_unsuffixed(self.size)
    }

    /// Enum type name for a signal's value descriptions (message name + signal name in `PascalCase`).
    #[must_use]
    pub fn enum_name(&self, signal: &SignalGen<'_>) -> Ident {
        let signal_name = signal
            .name
            .trim_start_matches(|c: char| c.is_ascii_punctuation())
            .to_pascal_case();
        let msg_name = enum_variant_name(&self.name);
        format_ident!("{msg_name}{signal_name}")
    }

    /// Enum type name for a multiplexor's index (message + multiplexor name + `Index`).
    ///
    /// # Errors
    /// Returns an error if `multiplexor` is not the multiplexor signal.
    pub fn multiplex_enum_name(&self, multiplexor: &SignalGen<'_>) -> Result<Ident> {
        ensure!(
            matches!(multiplexor.multiplexer_indicator, Multiplexor),
            "signal is not the multiplexor",
        );
        Ok(format!(
            "{}{}Index",
            self.name.as_str().to_pascal_case(),
            multiplexor.name.as_str().to_pascal_case(),
        )
        .ident())
    }

    /// Enum variant type name for a multiplexed switch value (message + multiplexor name + `M` + `switch_index`).
    ///
    /// # Errors
    /// Returns an error if `multiplexor` is not the multiplexor signal.
    pub fn multiplexed_enum_variant_name(
        &self,
        multiplexor: &SignalGen<'_>,
        switch_index: u64,
    ) -> Result<Ident> {
        ensure!(
            matches!(multiplexor.multiplexer_indicator, Multiplexor),
            "signal is not the multiplexor",
        );
        Ok(format!(
            "{}{}M{switch_index}",
            self.name.as_str().to_pascal_case(),
            multiplexor.name.as_str().to_pascal_case(),
        )
        .ident())
    }
}

impl Deref for MessageGen<'_> {
    type Target = Message;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

/// Wrapper around [`Signal`] used during code generation. Caches [`ValType`] and name-derived idents to avoid repeated computation.
#[derive(Clone)]
pub(crate) struct SignalGen<'a> {
    inner: &'a Signal,
    typ: ValType,
    typ_int: ValType,
    typ_uint: ValType,
    field_name: Ident,
}

impl Deref for SignalGen<'_> {
    type Target = Signal;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'a> SignalGen<'a> {
    /// Creates a wrapper and caches the signal's value types and name-derived idents.
    pub fn new(inner: &'a Signal) -> SignalGen<'a> {
        SignalGen {
            inner,
            typ: ValType::from_signal(inner),
            typ_int: ValType::from_signal_int(inner),
            typ_uint: ValType::from_signal_uint(inner),
            field_name: format_ident!("{}", sanitize_name(&inner.name, ToSnakeCase::to_snake_case)),
        }
    }

    pub fn typ(&self) -> ValType {
        self.typ
    }
    pub fn typ_ident(&self) -> Ident {
        self.typ.ident()
    }
    pub fn typ_int(&self) -> ValType {
        self.typ_int
    }
    pub fn typ_uint(&self) -> ValType {
        self.typ_uint
    }
    pub fn typ_uint_to_int(&self) -> Option<ValType> {
        self.typ_uint.unsigned_to_signed()
    }

    /// Field name (`snake_case`) for the signal, safe for use as a Rust field.
    #[must_use]
    pub fn field_name(&self) -> Ident {
        self.field_name.clone()
    }

    pub fn field_name_str(&self) -> String {
        self.field_name.to_string()
    }

    pub fn field_name_raw(&self) -> Ident {
        format_ident!(
            "{}_raw",
            sanitize_name(&self.name, ToSnakeCase::to_snake_case)
        )
    }

    pub fn field_name_setter(&self) -> Ident {
        format_ident!(
            "set_{}",
            sanitize_name(&self.name, ToSnakeCase::to_snake_case)
        )
    }

    #[must_use]
    pub fn const_name_min(&self) -> Ident {
        format_ident!(
            "{}",
            sanitize_name(
                &format!("{}{}", self.name, "_MIN"),
                ToShoutySnakeCase::to_shouty_snake_case
            )
        )
    }

    #[must_use]
    pub fn const_name_max(&self) -> Ident {
        format_ident!(
            "{}",
            sanitize_name(
                &format!("{}{}", self.name, "_MAX"),
                ToShoutySnakeCase::to_shouty_snake_case
            )
        )
    }
}
