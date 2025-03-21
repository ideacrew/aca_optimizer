//! Functions and types to get ruby objects into our objects.

use chrono::NaiveDate;
use magnus::{Error, Ruby, Value};

use super::types::{ProductId, RatingAreaTag, TobaccoUse};

#[doc(hidden)]
fn parse_maybe_tobacco_use(str_val: Option<String>) -> Result<TobaccoUse, String> {
    match str_val {
        None => Ok(TobaccoUse::Unknown),
        Some(x) => parse_tobacco_use(&x),
    }
}

#[doc(hidden)]
fn parse_tobacco_use(str_val: &str) -> Result<TobaccoUse, String> {
    match str_val {
        "Y" => Ok(TobaccoUse::Yes),
        "N" => Ok(TobaccoUse::No),
        "U" => Ok(TobaccoUse::Unknown),
        "NA" => Ok(TobaccoUse::Unknown),
        _ => Err(str_val.to_string()),
    }
}

/// Arguments expected to be parsed out of a call to `RateCache#lookup_rate`.
type LookupRateRubyArgs = (ProductId, RatingAreaTag, NaiveDate, i16, TobaccoUse);

/// Given a list of Ruby arguments for the `RateCache#lookup_rate` method, parse
/// them into values we can use.
pub(crate) fn parse_lookup_rate_args(
    ruby: &Ruby,
    args: &[Value],
) -> Result<LookupRateRubyArgs, Error> {
    let m_args = magnus::scan_args::scan_args::<
        (String, String, i32, u32, u32, i16, Option<String>),
        (),
        (),
        (),
        (),
        (),
    >(args)?;
    let (product_id_string, rating_area_tag, y_val, m_val, d_val, coverage_age, tobacco_use_str) =
        m_args.required;
    let mut product_id: [u8; 24] = Default::default();
    product_id[..product_id_string.len()].copy_from_slice(product_id_string.as_bytes());
    match NaiveDate::from_ymd_opt(y_val, m_val, d_val) {
        None => Err(Error::new(
            ruby.exception_arg_error(),
            format!("invalid rate schedule date: {:?}", (y_val, m_val, d_val)),
        )),
        Some(d) => match parse_maybe_tobacco_use(tobacco_use_str) {
            Err(tus) => Err(Error::new(
                ruby.exception_arg_error(),
                format!("{:?} is not a valid value for tobacco_use", tus.clone()),
            )),
            Ok(tobacco_use) => {
                let mut ra_tag_array: [u8; 10] = Default::default();
                ra_tag_array[..rating_area_tag.len()].copy_from_slice(rating_area_tag.as_bytes());
                Ok((product_id, ra_tag_array, d, coverage_age, tobacco_use))
            }
        },
    }
}

/// Arguments expected to be parsed out of a call to `RateCache#add_rate`.
type AddRateToCacheRubyArgs = (
    [u8; 24],
    i16,
    i16,
    [u8; 10],
    NaiveDate,
    NaiveDate,
    i16,
    TobaccoUse,
    f64,
);

/// Given a list of Ruby arguments for the `RateCache#add_rate` method, parse
/// them into values we can use.
pub(crate) fn parse_cache_rate_args(
    ruby: &Ruby,
    args: &[Value],
) -> Result<AddRateToCacheRubyArgs, Error> {
    let m_args = magnus::scan_args::scan_args::<
        (String, i16, i16, String, String, String, i16, String, f64),
        (),
        (),
        (),
        (),
        (),
    >(args)?;
    let (
        product_id_string,
        min_age,
        max_age,
        rating_area_tag,
        rate_date_start_str,
        rate_date_end_str,
        coverage_age,
        tobacco_use_str,
        cost,
    ) = m_args.required;
    let mut product_id: [u8; 24] = Default::default();
    product_id[..product_id_string.len()].copy_from_slice(product_id_string.as_bytes());
    match NaiveDate::parse_from_str(&rate_date_start_str, "%Y-%m-%d") {
        Err(srd_e) => Err(Error::new(
            ruby.exception_arg_error(),
            format!("{:?}: invalid rate start date", srd_e),
        )),
        Ok(srd) => match NaiveDate::parse_from_str(&rate_date_end_str, "%Y-%m-%d") {
            Err(erd_e) => Err(Error::new(
                ruby.exception_arg_error(),
                format!("{:?}: invalid rate end date", erd_e),
            )),
            Ok(erd) => match parse_tobacco_use(&tobacco_use_str) {
                Err(tus_e) => Err(Error::new(
                    ruby.exception_arg_error(),
                    format!("{:?} is not a valid value for tobacco_use", tus_e),
                )),
                Ok(tus) => {
                    let mut ra_tag_array: [u8; 10] = Default::default();
                    ra_tag_array[..rating_area_tag.len()]
                        .copy_from_slice(rating_area_tag.as_bytes());
                    Ok((
                        product_id,
                        min_age,
                        max_age,
                        ra_tag_array,
                        srd,
                        erd,
                        coverage_age,
                        tus,
                        cost,
                    ))
                }
            },
        },
    }
}
