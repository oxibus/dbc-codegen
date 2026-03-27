use proc_macro2::TokenStream;
use template_quote::quote;

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
    /// Generate an attribute token stream (like `#[derive(Debug)]`).
    /// The closure is only called when the feature is enabled, so expensive codegen can be skipped when `Never`.
    pub(crate) fn attr<F>(&self, f: F) -> TokenStream
    where
        F: FnOnce() -> TokenStream,
    {
        match self {
            FeatureConfig::Always => {
                let tokens = f();
                quote! { #[#tokens] }
            }
            FeatureConfig::Gated(gate) => {
                let tokens = f();
                quote! { #[cfg_attr(feature = #gate, #tokens)] }
            }
            FeatureConfig::Never => quote! {},
        }
    }

    /// Generate a token stream optionally wrapped in a cfg attribute.
    /// The closure is only called when the feature is enabled, so expensive codegen can be skipped when `Never`.
    pub(crate) fn if_cfg<F>(&self, f: F) -> TokenStream
    where
        F: FnOnce() -> TokenStream,
    {
        match self {
            FeatureConfig::Always => f(),
            FeatureConfig::Gated(gate) => {
                let tokens = f();
                quote! {
                    #[cfg(feature = #gate)]
                    #tokens
                }
            }
            FeatureConfig::Never => quote! {},
        }
    }
}
