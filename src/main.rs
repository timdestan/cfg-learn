mod tree;

use std::fs;
use std::str::FromStr;
use tree::Node;

fn parse_nodes_from_file(fname: &str) -> Result<Vec<Node>, String> {
  let mut trees = vec![];
  for line in fs::read_to_string(fname)
    .expect("Failed to read file")
    .lines()
  {
    trees.push(Node::from_str(line)?);
  }
  Ok(trees)
}

fn main() {
  println!("Parsing training trees");
  let training_trees = parse_nodes_from_file("f2-21.train.parse.noLEX").expect("Failed to parse");
  println!("Done parsing {} trees", training_trees.len());
  println!("Parsing test trees");
  let testing_trees = parse_nodes_from_file("f2-21.test.parse.noLEX").expect("Failed to parse");
  println!("Done parsing {} trees", testing_trees.len());
}
