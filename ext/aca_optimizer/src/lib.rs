//! A Ruby extension providing faster and more memory efficient algorithms.

use magnus::{Error, Ruby};

mod rate_lookup;
use crate::rate_lookup::*;

/// The initialization method for the Ruby extension.
///
/// Will be dynamically generated into an extern "C" function.
#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("AcaOptimizer")?;
    initialize_rate_cache_classes(ruby, &module)?;
    Ok(())
}
