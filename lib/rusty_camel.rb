require "helix_runtime"

RubyString = String

begin
  require "rusty_camel/native"
rescue LoadError
  warn "Unable to load rusty_camel/native. Please run `rake build`"
end
