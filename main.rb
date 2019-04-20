#!/usr/bin/env ruby -wKU

require_relative 'tree'

#module Logger
  def log(str)
    $stderr.puts(str) if $DEBUG
  end
#end

TRAINING_FILENAME = 'f2-21.train.parse.noLEX'
# TEST_FILENAME = 'f2-21.test.parse'

UN_FACTORED_OUT = 'unfactored.txt'
LEFT_FACTORED_OUT = 'leftfactored.txt'

# Reads an array of trees from the given filename.
def read_trees(filename)
  log("Reading #{filename}...")
  File.open(filename, "r") do |file|
    file.readlines.collect do |line|
      Tree.from_string(line)
    end
  end
end

# read trees
training_trees = read_trees(TRAINING_FILENAME)
# don't think we care about the test set at all for this homework...
#test_trees = read_trees(TEST_FILENAME)

log("Done reading trees. There are #{training_trees.length} training trees (top level)")

all_training_rules = Array.new(10 * training_trees.length)
i = 0
capacity = 0
training_trees.each do |tree|
  i += 1
  log("Tree number #{i}") if (i % 1000) == 0
  some_trees = tree.get_all_trees()
  all_training_rules[capacity, some_trees.length] = some_trees
  capacity += some_trees.length
end
all_training_rules = all_training_rules.select { |x| not x.nil? }

log("Computing frequencies of LHS's and rules...")

lhs_counts = Hash.new(0)
lhs_rhs_counts = Hash.new(0)

all_training_rules.each do |rule|
  lhs_counts[rule.lhs] += 1
  lhs_rhs_counts[[rule.lhs, rule.rhs]] += 1
end

puts "There are #{lhs_rhs_counts.keys.length} rules in the grammar."

TO_SHOW = ["ADJP","NAC"]

outfile = File.open(UN_FACTORED_OUT, "w+")
lhs_rhs_prob = Hash.new()
lhs_rhs_counts.each_key do |lhs,rhs|
  lhs_rhs_prob[[lhs,rhs]] = Float(lhs_rhs_counts[[lhs,rhs]]) / lhs_counts[lhs]
  case lhs
  when *TO_SHOW
    outfile.puts "#{lhs} -> #{rhs} = #{lhs_rhs_prob[[lhs,rhs]]}"
  end
end
outfile.close

log("Left factoring trees.")
training_trees.each do |tree|
  tree.left_factor!
end

all_training_rules = Array.new(10 * training_trees.length)
i = 0
capacity = 0
training_trees.each do |tree|
  i += 1
  log("Tree number #{i}") if (i % 1000) == 0
  some_trees = tree.get_all_trees()
  all_training_rules[capacity, some_trees.length] = some_trees
  capacity += some_trees.length
end
all_training_rules = all_training_rules.select { |x| not x.nil? }

lhs_counts = Hash.new(0)
lhs_rhs_counts = Hash.new(0)

all_training_rules.each do |rule|
  lhs_counts[rule.lhs] += 1
  lhs_rhs_counts[[rule.lhs, rule.rhs]] += 1
end

puts "There are #{lhs_rhs_counts.keys.length} rules in the left-factored grammar."

outfile = File.open(LEFT_FACTORED_OUT, "w+")
lhs_rhs_prob = Hash.new()
lhs_rhs_counts.each_key do |lhs,rhs|
  lhs_rhs_prob[[lhs,rhs]] = Float(lhs_rhs_counts[[lhs,rhs]]) / lhs_counts[lhs]
  if TO_SHOW.include? lhs or TO_SHOW.any? { |prefix| lhs.index("#{prefix}~") == 0 } 
    outfile.puts "#{lhs} -> #{rhs} = #{lhs_rhs_prob[[lhs,rhs]]}"
  end
end
outfile.close
