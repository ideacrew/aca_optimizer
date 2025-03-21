# frozen_string_literal: true

require "bundler/gem_tasks"
require "rubocop/rake_task"

RuboCop::RakeTask.new

require "rb_sys/extensiontask"
require "yard"

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

crate_doc_sources = Dir.glob("ext/**/*.*")

cargo_script = (ENV["GITHUB_CI"] == "true") ? "./cargoci" : "./cargo"

file "target/doc" => crate_doc_sources do
  require "open3"

  FileUtils.rm_f "target/doc"

  cmd = "#{cargo_script} doc --no-deps --document-private-items"
  Open3.popen3(cmd) do |_stdin, stdout, stderr, thread|
    [stdout, stderr].each do |stream|
      Thread.new do
        until (raw_line = stream.gets).nil?
          puts raw_line
        end
      end
    end

    thread.join # don't exit until the external process is done
  end
end

YARD::Rake::YardocTask.new(yard: "target/doc")

Rake::Task["yard"].enhance do
  FileUtils.rm_f "doc/crate_docs"
  FileUtils.cp_r "target/doc", "doc/crate_docs"
end
