# frozen_string_literal: true

require "aca_optimizer"

require "csv"
require "set"

require "benchmark"

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
    start_date = Date.strptime(rate_start_date, "%Y-%m-%d")
    end_date = Date.strptime(rate_end_date, "%Y-%m-%d")
    @product_rate_calculation_cache[product_id][rating_area_tag][coverage_age][tobacco_value] = (
      @product_rate_calculation_cache[product_id][rating_area_tag][coverage_age][tobacco_value] +
      [{
        start_on: start_date,
        end_on: end_date,
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

product_ids = Set.new
rating_areas = Set.new

instance = AcaOptimizer::RateCache.instance
r_cache = RubyLookupCopy.new
CSV.foreach(File.join(File.dirname(__FILE__), "test/aca_optimizer/rates.csv"), headers: true) do |row|
  product_id = row["product_id"]
  rating_area = row["rating_area_tag"]
  product_ids << product_id
  rating_areas << rating_area

  instance.add_rate(
    product_id,
    row["min_age"].to_i,
    row["max_age"].to_i,
    rating_area,
    row["rate_start_date"],
    row["rate_end_date"],
    row["coverage_age"].to_i,
    row["tobacco_use"],
    row["cost"].to_f
  )

  r_cache.add_rate(
    product_id,
    row["min_age"].to_i,
    row["max_age"].to_i,
    rating_area,
    row["rate_start_date"],
    row["rate_end_date"],
    row["coverage_age"].to_i,
    row["tobacco_use"],
    row["cost"].to_f
  )
end

def build_sample_set(p_ids, ras)
  product_list = p_ids.to_a
  rating_area_list = ras.to_a
  age_list = (1..120).to_a
  tobacco_list = %w[Y N NA]
  date_list = %w[2022-11-02 2025-06-23 2024-02-05]

  (1..20_000).to_a.map do |_i|
    [
      product_list.sample,
      rating_area_list.sample,
      Date.strptime(date_list.sample, "%Y-%m-%d"),
      age_list.sample,
      tobacco_list.sample
    ]
  end
end

instance.finish_caching!

samples = build_sample_set(product_ids, rating_areas)

samples_ru_input = samples.map do |s|
  [
    s[0],
    s[1],
    s[2].year,
    s[2].month,
    s[2].day,
    s[3],
    s[4]
  ]
end

samples_ru = []
samples_rb = []
require "objspace"

samples = nil
samples_ru = nil

GC.compact
GC.start

a_size = ObjectSpace.memsize_of_all.inspect

i_size = ObjectSpace.memsize_of(instance)
#
instance = nil
#
AcaOptimizer::RateCache.instance_variable_set(:@__cache_instance, nil)
#
GC.start
GC.compact
#
a_after_ru_cache_drop = ObjectSpace.memsize_of_all.inspect
#
r_cache = nil
#
GC.start
GC.compact
#
a_after_ruby_cache_drop = ObjectSpace.memsize_of_all.inspect
#
puts "Full Size: #{a_size}"
puts "Rust Cache Size: #{i_size}"
puts "Full Size after rust drop: #{a_after_ru_cache_drop}"
puts "Full Size after all drop: #{a_after_ruby_cache_drop}"
#

__END__
Benchmark.bm do |x|
  x.report do
    samples_ru_input.each do |sample|
      samples_ru << begin
        instance.lookup_rate(
          *sample
        )
      rescue StandardError
        nil
      end
    end
  end

  x.report do
    samples.each do |sample|
      samples_rb << begin
        r_cache.lookup_rate(
          *sample
        )
      rescue StandardError
        nil
      end
    end
  end
end

puts (samples_ru == samples_rb).inspect
