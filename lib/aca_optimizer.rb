# frozen_string_literal: true

require_relative "aca_optimizer/version"
require_relative "aca_optimizer/aca_optimizer"

# Structures and calculations optimized for efficient RAM and speed.
#
# Documentation of the associated extensions is provided as {Rust Docs}[./crate_docs/aca_optimizer/index.html]
module AcaOptimizer
  # @!parse ruby
  #
  #  # Ruby facade for the Rust-provided RateCache class.
  #  #
  #  # The implementation of this object is actually provided by a Ruby extension
  #  # written in rust.  This is merely the documentation presented to show Yard
  #  # an API.
  #  class RateCache
  #    # Create a new instance.
  #    def initialize
  #    end
  #
  #    # Add a rate entry to the cache.
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
  #    # Look up a rate.
  #    # @param product_id          [String] the id of the product
  #    # @param rating_area         [String] the rating area, as a string
  #    # @param rate_schedule_year  [Integer] the year of the rate schedule date
  #    # @param rate_schedule_month [Integer] the month of the rate schedule date
  #    # @param rate_schedule_day   [Integer] the day of the rate schedule date
  #    # @param coverage_age        [Integer] the age on coverage start for the person
  #    # @param tobacco_use         [String | nil] tobacco usage for the person
  #    # @return [Double] the found cost
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
  #    # Finish the loading of rates and compact memory.
  #    def finish_caching!
  #    end
  #
  #    # Return the cached class instance.  Populated at class load.
  #    # @return [RateCache] the cached instance
  #    def self.instance
  #    end
  #  end
end
