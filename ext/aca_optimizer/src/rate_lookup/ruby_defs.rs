//! Trait implementations needed for Magnus to treat our types as ruby types.
//!
//! This module defines:
//! * the [`DataTypeFunctions`](https://docs.rs/magnus/latest/magnus/typed_data/trait.DataTypeFunctions.html) impl for [`RateCache`]
//! * the [`TypedData`](https://docs.rs/magnus/latest/magnus/derive.TypedData.html) impl for [`RateCache`]
use magnus::{data_type_builder, value::{Lazy, ReprValue}, Class, DataTypeFunctions, RClass};

use deepsize::DeepSizeOf;

use crate::rate_lookup::types::RateCache;

impl DataTypeFunctions for RateCache {
  fn size(&self) -> usize {
    (*self.product_rates.borrow()).deep_size_of()
  }
}

unsafe impl magnus::TypedData for RateCache {
  fn class(ruby: &magnus::Ruby) -> RClass {
      static CLASS: Lazy<RClass> = Lazy::new(|ruby| {
          let class: RClass = ruby.class_object().funcall("const_get", ("AcaOptimizer::RateCache",)).unwrap();
          class.undef_default_alloc_func();
          class
      });
      ruby.get_inner(&CLASS)
  }

  fn data_type() -> &'static magnus::DataType {
      static DATA_TYPE: magnus::DataType = data_type_builder!(RateCache, "AcaOptimizer::RateCache").free_immediately().size().build();
      &DATA_TYPE
  }
}