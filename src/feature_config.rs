use std::fmt::Display;
use std::io::Write;

use anyhow::{Error, Result};

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
    pub(crate) fn fmt_attr(&self, w: &mut impl Write, attr: impl Display) -> Result<()> {
        match self {
            FeatureConfig::Always => writeln!(w, "#[{attr}]")?,
            FeatureConfig::Gated(gate) => writeln!(w, "#[cfg_attr(feature = {gate:?}, {attr})]")?,
            FeatureConfig::Never => {}
        }
        Ok(())
    }

    pub(crate) fn fmt_cfg<W: Write, E: Into<Error>>(
        &self,
        mut w: W,
        f: impl FnOnce(W) -> Result<(), E>,
    ) -> Result<()> {
        match self {
            // If config is Never, return immediately without calling `f`
            FeatureConfig::Never => return Ok(()),
            // If config is Gated, prepend `f` with a cfg guard
            FeatureConfig::Gated(gate) => {
                writeln!(w, "#[cfg(feature = {gate:?})]")?;
            }
            // Otherwise, just call `f`
            FeatureConfig::Always => {}
        }

        f(w).map_err(Into::into)
    }
}
