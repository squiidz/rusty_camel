require 'helix_runtime/build_task'

HelixRuntime::BuildTask.new("rusty_camel")

task :default => :build
