//! Type definitions for our data structures.

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;

use chrono::NaiveDate;
use deepsize::DeepSizeOf;
use foldhash::fast::RandomState;

/// Type alias for a Product ID.  A UTF-8 string limited to 24 bytes
///   to aid in data packing and speed.
pub(crate) type ProductId = [u8; 24];

/// Type alias for a Rating Area.  A UTF-8 string limited to 10 bytes
///   to aid in data packing and speed.
pub(crate) type RatingAreaTag = [u8; 10];

/// Potential values for Tobacco Use.  Unknown is the same as 'NA' and Ruby's
///   `nil`.
#[derive(PartialEq, Eq, Hash, Debug, Clone, DeepSizeOf)]
pub(crate) enum TobaccoUse {
    Yes,
    No,
    Unknown
}


/// Various reasons we could fail to find the rate.
#[allow(clippy::enum_variant_names)]
pub enum RateLookupError {
    /// I have never heard of this Product ID.
    NoSuchProduct(ProductId),
    // I found the product, but no data in that date range.
    NoProductDataForDateRange(ProductId, NaiveDate),
    /// I found the product, but I couldn't find match data than that.
    NoMatchingRate(ProductId, NaiveDate, RatingAreaTag, i16, TobaccoUse)
}

/// Turns our error kinds into a nicely-formatted string.
impl Display for RateLookupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::NoSuchProduct(s) => {
                let product_id_string = String::from_utf8_lossy(s);
                write!(f, "No Such Product ID: {}", product_id_string)
            }
            Self::NoProductDataForDateRange(s, d) => {
                let product_id_string = String::from_utf8_lossy(s);
                write!(f, "NoProductDataForDateRange: Product ID: {}", product_id_string)?;
                write!(f, ", Date: {}", d)
            }
            Self::NoMatchingRate(s, d, ra, age, tu) => {
                let product_id_string = String::from_utf8_lossy(s);
                let ra_string = String::from_utf8_lossy(ra);
                write!(f, "No Matching Rate: Product ID: {}", product_id_string)?;
                write!(f, ", Date: {}", d)?;
                write!(f, ", Rating Area: {}", ra_string)?;
                write!(f, ", Age: {}", age)?;
                write!(f, ", Tobacco Use: {:?}", tu)
            }
        }
    }
}

/// A key composing all the parameters used to both store and retrieve a list
///   of rates.
#[derive(PartialEq, Eq, Hash, Clone, DeepSizeOf)]
pub(crate) struct RateLookupKey {
    /// Age of the individual when coverage begins.
    pub(crate) coverage_age: i16,
    /// ID of the product under which coverage will be provided.
    pub(crate) product_id: ProductId,
    /// The rating area of the coverage.
    pub(crate) rating_area_tag: RatingAreaTag,
    /// Tobacco use of the individual.
    pub(crate) tobacco_use: TobaccoUse
}

/// A rate schedule.  Represents a cost valid during a time period.
#[derive(Default, Clone)]
pub(crate) struct ScheduledCost {
    /// The date when this rate starts to be valid.
    pub(crate) start_on: NaiveDate,
    /// The date after which this rate is invalid. 
    pub(crate) end_on: NaiveDate,
    /// The cost, currently in USD, of the plan coverage.
    pub(crate) cost: f64
}

impl DeepSizeOf for ScheduledCost {
    fn deep_size_of_children(&self, context: &mut deepsize::Context) -> usize {
        self.cost.deep_size_of_children(context) + 8
    }
}

/// Encapsulates our rate data.
#[derive(Default, Clone, DeepSizeOf)]
pub(crate) struct ProductRateDirectory {
    /// A list of Min/Max ages for a given product.  Used to pick a high/low
    ///   limit before looking up the actual rate by age.
    pub(crate) age_bounds: HashMap<ProductId, (i16, i16), RandomState>,
    /// The actual rate schedules.
    /// Keyed by a combination of:
    ///   * Product ID
    ///   * Coverage Age
    ///   * Rating Area Name
    ///   * Tobacco Use
    pub(crate) rate_collection: HashMap<RateLookupKey, Vec<ScheduledCost>, RandomState>
}

/// Wrapper for our data structure.
///
/// It will be turned into a ruby object by Magnus.
/// We need to wrap the data content we actually care about into a `RefCell` so
///   Rust can handle the memory management without confusing Ruby:
/// Ruby is allowed to move our RateCache around as long as the pointer
///   **inside** isn't moved by Ruby.
#[derive(Clone, Default)]
pub(crate) struct RateCache {
    /// Our wrapped, protected, reference to our own data, to prevent Rust and
    ///   Ruby from fighting over data ownership.
    pub(crate) product_rates: RefCell<ProductRateDirectory>,
}