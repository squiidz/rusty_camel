# encoding: utf-8

Gem::Specification.new do |s|
  s.name         = 'rusty_camel'
  s.version      = '1.0.3'
  s.authors      = ['Squiidz']
  s.summary      = "Snake to Camel case"
  s.files        = Dir['{lib/**/*,[A-Z]*}']

  s.platform     = Gem::Platform::RUBY
  s.require_path = 'lib'

  s.add_dependency 'helix_runtime', '~> 0.5.0'
end
