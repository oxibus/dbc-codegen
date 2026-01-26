use std::fmt::Display;

/// Configuration for including features in the code generator.
///
/// e.g. [Debug] impls for generated types.
#[derive(Default)]
pub enum FeatureConfig<'a> {
    /// Generate code for this feature.
    Always,

    /// Generate code behind `#[cfg(feature = ...)]`
    Gated(&'a str),

    /// Don't generate code for this feature.
    #[default]
    Never,
}

impl FeatureConfig<'_> {
    /// Generate an attribute token stream (like `#[derive(Debug)]`)
    pub(crate) fn to_attr_tokens(&self, attr: impl Display) -> Option<proc_macro2::TokenStream> {
        use quote::quote;

        match self {
            FeatureConfig::Always => {
                let attr_str = attr.to_string();
                let tokens: proc_macro2::TokenStream = attr_str.parse().ok()?;
                Some(quote! { #[#tokens] })
            }
            FeatureConfig::Gated(gate) => {
                let attr_str = attr.to_string();
                let tokens: proc_macro2::TokenStream = attr_str.parse().ok()?;
                Some(quote! { #[cfg_attr(feature = #gate, #tokens)] })
            }
            FeatureConfig::Never => None,
        }
    }

    /// Generate a token stream optionally wrapped in a cfg attribute
    pub(crate) fn to_tokens_opt(
        &self,
        tokens: proc_macro2::TokenStream,
    ) -> Option<proc_macro2::TokenStream> {
        use quote::quote;

        match self {
            FeatureConfig::Never => None,
            FeatureConfig::Gated(gate) => Some(quote! {
                #[cfg(feature = #gate)]
                #tokens
            }),
            FeatureConfig::Always => Some(tokens),
        }
    }

    /// Generate allow(dead_code) attribute if needed
    pub(crate) fn allow_dead_code_tokens(allow: bool) -> Option<proc_macro2::TokenStream> {
        if allow {
            use quote::quote;
            Some(quote! { #[allow(dead_code)] })
        } else {
            None
        }
    }
}
