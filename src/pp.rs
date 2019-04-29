// Pretty print a single tree.

mod tree;

use std::fs;
use std::str::FromStr;
use tree::Node;

fn main() {
  let args = std::env::args().collect::<Vec<_>>();
  if args.len() != 2 {
    println!("Expected single filename arg");
    std::process::exit(1);
  }

  let string = fs::read_to_string(&args[1]).expect("failed to read file");
  let node = Node::from_str(&string).expect("failed to parse");
  println!("{:?}", node);
}
