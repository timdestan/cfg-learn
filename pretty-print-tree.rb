#!/usr/bin/env ruby -wKU

require_relative 'tree'

(puts "usage: #{$0} FILENAME"; exit) if ARGV.empty?

File.open(ARGV[0], "r") do |f|
  t = Tree.from_string(f.read)
  puts t
end