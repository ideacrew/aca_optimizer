# frozen_string_literal: true

require_relative "aca_optimizer/version"
require_relative "aca_optimizer/aca_optimizer"

# Structures and calculations optimized for efficient RAM and speed.
#
# Documentation of the associated extensions is provided as {Rust Docs}[./crate_docs/aca_optimizer/index.html]
module AcaOptimizer
  # @!parse ruby
  #
  #  # Ruby facade for the Rust-provided {RateCache}[../crate_docs/aca_optimizer/rate_lookup/index.html] class.
  #  #
  #  # This object is provided by the Ruby extension.
  #  class RateCache
  #    # Create a new instance - maps to
  #    # {initialize}[../crate_docs/aca_optimizer/rate_lookup/types/struct.RateCache.html#method.initialize].
  #    def initialize
  #    end
  #
  #    # Add a rate entry to the cache - maps to
  #    # {cache_rate}[../crate_docs/aca_optimizer/rate_lookup/types/struct.RateCache.html#method.cache_rate].
  #    # @param product_id      [String] the id of the product
  #    # @param min_age         [Integer] the lowest age supported by this
  #    #                                  product's rate set
  #    # @param max_age         [Integer] the highest age supported by this
  #    #                                  product's rate set
  #    # @param rating_area     [String] the rating area, as a string
  #    # @param rate_date_start [String] the date the rate becomes effective,
  #    #                                 in "YYYY-MM-DD" format
  #    # @param rate_date_end   [String] the date the rate becomes invalid,
  #    #                                 in "YYYY-MM-DD" format
  #    # @param coverage_age    [Integer] the age for this entry
  #    # @param tobacco_use     [String | nil] tobacco usage value for the rate
  #    # @param cost            [Numeric] the cost of the rate with these
  #    #                                 parameters
  #    # @return [void]
  #    # @raise [ArgumentError] invalid arguments provided
  #    def add_rate(
  #      product_id,
  #      min_age,
  #     max_age,
  #      rating_area,
  #      rate_date_start,
  #      rate_date_end,
  #      coverage_age,
  #      tobacco_use,
  #      cost
  #    )
  #    end
  #
  #    # Look up a rate - maps to
  #    # {lookup_rate}[../crate_docs/aca_optimizer/rate_lookup/types/struct.RateCache.html#method.lookup_rate].
  #    # @param product_id          [String] the id of the product
  #    # @param rating_area         [String] the rating area, as a string
  #    # @param rate_schedule_year  [Integer] the year of the rate schedule date
  #    # @param rate_schedule_month [Integer] the month of the rate schedule date
  #    # @param rate_schedule_day   [Integer] the day of the rate schedule date
  #    # @param coverage_age        [Integer] the age on coverage start for the person
  #    # @param tobacco_use         [String | nil] tobacco usage for the person
  #    # @return [Double] the found cost
  #    # @raise [ArgumentError] invalid arguments provided, or no
  #    #   corresponding data found
  #    def lookup_rate(
  #      product_id,
  #      rating_area,
  #      rate_schedule_year,
  #      rate_schedule_month,
  #      rate_schedule_day,
  #      coverage_age,
  #      tobacco_use
  #    )
  #    end
  #
  #    # Finish the loading of rates and compact memory - maps to
  #    # {finish_caching}[../crate_docs/aca_optimizer/rate_lookup/types/struct.RateCache.html#method.finish_caching]
  #    # @return [void]
  #    def finish_caching!
  #    end
  #
  #    # Return the cached class instance - maps to
  #    # {rate_cache_instance}[../crate_docs/aca_optimizer/rate_lookup/fn.rate_cache_instance.html].
  #    # Populated at class load.
  #    # @return [RateCache] the cached instance
  #    def self.instance
  #    end
  #  end
end
