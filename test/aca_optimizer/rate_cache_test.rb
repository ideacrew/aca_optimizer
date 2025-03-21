# frozen_string_literal: true

require "minitest/autorun"
require "aca_optimizer"
require "csv"

# rubocop:disable Metrics/ParameterLists
class RubyLookupCopy
  def initialize
    @product_rate_age_bounding_cache = {}
    @product_rate_calculation_cache = Hash.new do |h, k|
      h[k] = (Hash.new do |h2, k2|
        h2[k2] = (Hash.new do |h3, k3|
          h3[k3] = (Hash.new do |h4, k4|
            h4[k4] = []
          end)
        end)
      end)
    end
  end

  def add_rate(
    product_id,
    min_age,
    max_age,
    rating_area_tag,
    rate_start_date,
    rate_end_date,
    coverage_age,
    tobacco_value,
    cost
  )
    @product_rate_age_bounding_cache[product_id] = {
      minimum: min_age,
      maximum: max_age
    }
    @product_rate_calculation_cache[product_id][rating_area_tag][coverage_age][tobacco_value] = (
      @product_rate_calculation_cache[product_id][rating_area_tag][coverage_age][tobacco_value] +
      [{
        start_on: rate_start_date,
        end_on: rate_end_date,
        cost: cost
      }]
    )
  end

  def age_bounding(plan_id, coverage_age)
    plan_age = @product_rate_age_bounding_cache[plan_id]
    return plan_age[:minimum] if coverage_age < plan_age[:minimum]
    return plan_age[:maximum] if coverage_age > plan_age[:maximum]

    coverage_age
  end

  def lookup_rate(
    product_id,
    rating_area,
    rate_schedule_date,
    coverage_age,
    tobacco_use = "NA"
  )
    calc_age = age_bounding(product_id, coverage_age)
    age_record = @product_rate_calculation_cache[product_id][rating_area][calc_age][tobacco_use].detect do |pt|
      (pt[:start_on] <= rate_schedule_date) && (pt[:end_on] >= rate_schedule_date)
    end
    age_record[:cost]
  end
end
# rubocop:enable Metrics/ParameterLists

# rubocop:disable Style/ClassVars
class RateCacheTest < Minitest::Test
  def test_lookup_empty_tobacco
    @@loaded_instance.lookup_rate(
      "60f5d9f22a6d430254ae7bc6",
      "R-ME002",
      2021,
      6,
      24,
      32,
      nil
    )
  end

  def test_lookup_bogus_rate
    instance = AcaOptimizer::RateCache.instance
    assert_raises(
      ArgumentError
    ) do
      instance.lookup_rate("1", "2", 2024, 9, 3, 4, "Y")
    end
  end

  def test_lookup_loaded_rates
    assert_equal(
      @@loaded_instance.lookup_rate(
        "60f5d9f22a6d430254ae7bc6",
        "R-ME002",
        2021,
        6,
        24,
        32,
        "Y"
      ),
      @@r_cache.lookup_rate(
        "60f5d9f22a6d430254ae7bc6",
        "R-ME002",
        "2021-05-24",
        32,
        "Y"
      )
    )
  end

  # rubocop:disable Metrics/MethodLength
  # rubocop:disable Metrics/AbcSize
  def self.prepare
    @@loaded_instance = AcaOptimizer::RateCache.new
    @@r_cache = RubyLookupCopy.new
    CSV.foreach(File.join(File.dirname(__FILE__), "rates.csv"), headers: true) do |row|
      @@loaded_instance.add_rate(
        row["product_id"],
        row["min_age"].to_i,
        row["max_age"].to_i,
        row["rating_area_tag"],
        row["rate_start_date"],
        row["rate_end_date"],
        row["coverage_age"].to_i,
        row["tobacco_use"],
        row["cost"].to_f
      )
      @@r_cache.add_rate(
        row["product_id"],
        row["min_age"].to_i,
        row["max_age"].to_i,
        row["rating_area_tag"],
        row["rate_start_date"],
        row["rate_end_date"],
        row["coverage_age"].to_i,
        row["tobacco_use"],
        row["cost"].to_f
      )
    end
  end
  # rubocop:enable Metrics/MethodLength
  # rubocop:enable Metrics/AbcSize

  prepare
end

# rubocop:enable Style/ClassVars
