extern crate itertools;

mod tree;

use itertools::{join, sorted};
use std::collections::HashMap;
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

fn get_all_trees<'a>(trees: &'a Vec<Node>) -> Vec<&'a Node> {
  trees
    .iter()
    .flat_map(|t| t.preorder())
    // Exclude POS -> wPOS rules
    .filter(|t| !t.kids.is_empty() && (!(t.kids.len() == 1 && t.kids[0].kids.is_empty())))
    .collect()
}

fn main() {
  println!("Parsing trees");
  let mut trees = parse_nodes_from_file("f2-21.train.parse.noLEX").expect("Failed to parse");
  println!("Done parsing {} trees (top level)", trees.len());
  let all_trees = get_all_trees(&trees);
  println!("Expanded to {} trees", all_trees.len());

  // Rules to show probabilities for.
  let to_show = ["ADJP", "NAC"];

  {
    let mut lhs_counts = HashMap::new();
    let mut lhs_rhs_counts = HashMap::new();

    for tree in &all_trees {
      let lhs_counter = lhs_counts.entry(tree.lhs()).or_insert(0);
      *lhs_counter += 1;
      let lhs_rhs_counter = lhs_rhs_counts.entry((tree.lhs(), tree.rhs())).or_insert(0);
      *lhs_rhs_counter += 1;
    }

    println!(
      "There are {} rules in the grammar.",
      lhs_rhs_counts.keys().len()
    );

    let mut out_str = join(
      sorted(lhs_rhs_counts.into_iter().filter_map(|kv| {
        let ((lhs, rhs), value) = kv;
        let lhs_count = lhs_counts.get(lhs).expect("missing lhs");
        let prob = (value as f64) / (*lhs_count as f64);
        if to_show.contains(&lhs) {
          Some(format!("{} -> {} = {:.6}", lhs, rhs, prob))
        } else {
          None
        }
      })),
      "\n",
    );
    out_str += "\n"; // for consistency with ruby version
    fs::write("unfactored.txt", out_str).expect("error writing file");
  }

  println!("Left factoring trees.");
  for tree in &mut trees {
    tree.left_factor();
  }

  let all_lf_trees = get_all_trees(&trees);

  {
    let mut lhs_counts = HashMap::new();
    let mut lhs_rhs_counts = HashMap::new();

    for tree in &all_lf_trees {
      let lhs_counter = lhs_counts.entry(tree.lhs()).or_insert(0);
      *lhs_counter += 1;
      let lhs_rhs_counter = lhs_rhs_counts.entry((tree.lhs(), tree.rhs())).or_insert(0);
      *lhs_rhs_counter += 1;
    }

    println!(
      "There are {} rules in the left-factored grammar.",
      lhs_rhs_counts.keys().len()
    );

    let mut out_str = join(
      sorted(lhs_rhs_counts.into_iter().filter_map(|kv| {
        let ((lhs, rhs), value) = kv;
        let lhs_count = lhs_counts.get(lhs).expect("missing lhs");
        let prob = (value as f64) / (*lhs_count as f64);
        if to_show.contains(&lhs) || to_show.iter().any(|sym| lhs.starts_with(&format!("{}~", sym))) {
          Some(format!("{} -> {} = {:.6}", lhs, rhs, prob))
        } else {
          None
        }
      })),
      "\n",
    );
    out_str += "\n"; // for consistency with ruby version
    fs::write("leftfactored.txt", out_str).expect("error writing file");
  }
}
