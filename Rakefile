# frozen_string_literal: true

require "bundler/gem_tasks"
require "rubocop/rake_task"

RuboCop::RakeTask.new

require "rb_sys/extensiontask"

task build: :compile

GEMSPEC = Gem::Specification.load("aca_optimizer.gemspec")

RbSys::ExtensionTask.new("aca_optimizer", GEMSPEC) do |ext|
  ext.lib_dir = "lib/aca_optimizer"
end

task default: %i[test]

require "rake/testtask"

Rake::TestTask.new(test: :compile) do |t|
  t.test_files = FileList["test/**/*_test.rb"]
end
