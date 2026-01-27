use proc_macro2::TokenStream;
use quote::quote;

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
    pub(crate) fn attr(&self, tokens: &TokenStream) -> TokenStream {
        match self {
            FeatureConfig::Always => quote! { #[#tokens] },
            FeatureConfig::Gated(gate) => quote! { #[cfg_attr(feature = #gate, #tokens)] },
            FeatureConfig::Never => quote! {},
        }
    }

    /// Generate a token stream optionally wrapped in a cfg attribute
    pub(crate) fn if_cfg(&self, tokens: TokenStream) -> TokenStream {
        match self {
            FeatureConfig::Always => tokens,
            FeatureConfig::Gated(gate) => quote! {
                #[cfg(feature = #gate)]
                #tokens
            },
            FeatureConfig::Never => quote! {},
        }
    }
}
