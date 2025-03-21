//! Optimized Rate Lookup types and functions.

use chrono::NaiveDate;
use foldhash::fast::RandomState;
use std::collections::HashMap;

use magnus::{method, Class, Error, Module, Object, RClass, RModule, Ruby, Value};

mod types;
use crate::rate_lookup::types::*;

mod ruby_defs;

mod argument_conversions;
use crate::rate_lookup::argument_conversions::*;

#[doc(hidden)]
fn lookup_rate(
    rate_dictionary: &ProductRateDirectory,
    rate_lookup: &RateLookupKey,
    rate_schedule_date: &NaiveDate,
) -> Result<f64, RateLookupError> {
    let rate_hash = &rate_dictionary.rate_collection;
    let age_bounds = &rate_dictionary.age_bounds;
    match age_bounds.get(&rate_lookup.product_id) {
        None => Err(RateLookupError::NoSuchProduct(rate_lookup.product_id)),
        Some(ab) => {
            let lookup_age = if rate_lookup.coverage_age < ab.0 {
                ab.0
            } else if rate_lookup.coverage_age > ab.1 {
                ab.1
            } else {
                rate_lookup.coverage_age
            };
            let corrected_lookup = RateLookupKey {
                coverage_age: lookup_age,
                product_id: rate_lookup.product_id,
                tobacco_use: rate_lookup.tobacco_use.clone(),
                rating_area_tag: rate_lookup.rating_area_tag,
            };
            match rate_hash.get(&corrected_lookup) {
                None => Err(RateLookupError::NoMatchingRate(
                    rate_lookup.product_id,
                    *rate_schedule_date,
                    rate_lookup.rating_area_tag,
                    lookup_age,
                    rate_lookup.tobacco_use.clone(),
                )),
                Some(rt) => {
                    match rt.iter().find(|e| {
                        e.start_on <= *rate_schedule_date && e.end_on >= *rate_schedule_date
                    }) {
                        None => Err(RateLookupError::NoProductDataForDateRange(
                            rate_lookup.product_id,
                            *rate_schedule_date,
                        )),
                        Some(sc) => Ok(sc.cost),
                    }
                }
            }
        }
    }
}

#[doc(hidden)]
#[allow(clippy::too_many_arguments)]
fn insert_rate_value(
    rate_hash: &mut ProductRateDirectory,
    product_id: &ProductId,
    ra_tag: &RatingAreaTag,
    min_age: &i16,
    max_age: &i16,
    rate_start_date: &NaiveDate,
    rate_end_date: &NaiveDate,
    tobacco_value: &TobaccoUse,
    coverage_age: i16,
    cost: &f64,
) {
    match rate_hash.age_bounds.get_mut(product_id) {
        None => {
            let ab = &mut rate_hash.age_bounds;
            ab.insert(
                product_id.to_owned(),
                (min_age.to_owned(), max_age.to_owned()),
            )
        }
        Some(_) => None,
    };
    let lookup = RateLookupKey {
        product_id: product_id.to_owned(),
        rating_area_tag: ra_tag.to_owned(),
        tobacco_use: tobacco_value.to_owned(),
        coverage_age,
    };
    let rate_collection = &mut rate_hash.rate_collection;
    match rate_collection.get_mut(&lookup) {
        None => {
            let sc = ScheduledCost {
                start_on: rate_start_date.to_owned(),
                end_on: rate_end_date.to_owned(),
                cost: *cost,
            };
            let entries = vec![sc];

            rate_collection.insert(lookup, entries);
        }
        Some(entries) => {
            let sc = ScheduledCost {
                start_on: rate_start_date.to_owned(),
                end_on: rate_end_date.to_owned(),
                cost: *cost,
            };
            entries.push(sc);
            entries.sort_by(|a, b| (a.start_on, a.end_on).cmp(&(b.start_on, b.end_on)));
        }
    }
}

impl RateCache {
    /// Rust implementation for the `initialize` method on the
    ///   `AcaOptimizer::RateCache` Ruby class.
    fn initialize(_ruby: &Ruby, rb_self: magnus::typed_data::Obj<Self>) -> Result<(), Error> {
        let age_bounds = HashMap::with_hasher(RandomState::default());
        let rate_dictionary = HashMap::with_hasher(RandomState::default());
        *rb_self.product_rates.borrow_mut() = ProductRateDirectory {
            rate_collection: rate_dictionary,
            age_bounds,
        };
        Ok(())
    }

    /// Rust implementation for the `finish_caching!` method on the
    ///   `AcaOptimizer::RateCache` Ruby class.
    fn finish_caching(_ruby: &Ruby, rb_self: magnus::typed_data::Obj<Self>) -> Result<(), Error> {
        let mut product_dictionary = rb_self.product_rates.borrow_mut();
        let ab = &mut product_dictionary.age_bounds;
        ab.shrink_to_fit();
        let rd = &mut product_dictionary.rate_collection;
        rd.values_mut().for_each(|vm| vm.shrink_to_fit());
        rd.shrink_to_fit();
        Ok(())
    }

    /// Rust implementation for the `add_rate` method on the
    ///   `AcaOptimizer::RateCache` Ruby class.
    fn cache_rate(
        ruby: &Ruby,
        rb_self: magnus::typed_data::Obj<Self>,
        args: &[Value],
    ) -> Result<(), Error> {
        let (product_id, min_age, max_age, ra_tag_array, srd, erd, coverage_age, tus, cost) =
            parse_cache_rate_args(ruby, args)?;
        let rate_hash = &mut rb_self.product_rates.borrow_mut();
        insert_rate_value(
            rate_hash,
            &product_id,
            &ra_tag_array,
            &min_age,
            &max_age,
            &srd,
            &erd,
            &tus,
            coverage_age,
            &cost,
        );
        Ok(())
    }

    /// Rust implementation for the `lookup_rate` method on the
    ///   `AcaOptimizer::RateCache` Ruby class.
    fn lookup_rate(
        ruby: &Ruby,
        rb_self: magnus::typed_data::Obj<Self>,
        args: &[Value],
    ) -> Result<f64, Error> {
        let (product_id, rating_area_tag, d, coverage_age, tobacco_use) =
            parse_lookup_rate_args(ruby, args)?;
        let lookup = RateLookupKey {
            product_id,
            tobacco_use,
            coverage_age,
            rating_area_tag
        };
        let rate_hash = rb_self.product_rates.borrow();
        match lookup_rate(&rate_hash, &lookup, &d) {
            Err(rle) => Err(Error::new(ruby.exception_arg_error(), format!("{}", rle))),
            Ok(r) => Ok(r),
        }
    }
}

/// Return the rate cache instance.
///
/// Bound to the class method ".instance" in the
///   `AcaOptimizer::RateCache` Ruby class.
fn rate_cache_instance(
    rate_cache_class: RClass,
) -> Result<magnus::typed_data::Obj<RateCache>, Error> {
    rate_cache_class.ivar_get("@__cache_instance")
}

/// Register the `RateCache` class into the `AcaOptimizer` module and
///   and map its methods to the Rust object.
pub(crate) fn initialize_rate_cache_classes(ruby: &Ruby, module: &RModule) -> Result<(), Error> {
    let rate_cache_class = module.define_class("RateCache", ruby.class_object())?;
    rate_cache_class.define_alloc_func::<RateCache>();
    rate_cache_class.define_method("initialize", method!(RateCache::initialize, 0))?;
    rate_cache_class.define_method("lookup_rate", method!(RateCache::lookup_rate, -1))?;
    rate_cache_class.define_method("add_rate", method!(RateCache::cache_rate, -1))?;
    rate_cache_class.define_method("finish_caching!", method!(RateCache::finish_caching, 0))?;
    let r_cache_instance = rate_cache_class.new_instance(())?;
    rate_cache_class.ivar_set("@__cache_instance", r_cache_instance)?;
    rate_cache_class.define_singleton_method("instance", method!(rate_cache_instance, 0))?;
    Ok(())
}
