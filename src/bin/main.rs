use cfg_learn::Node;
use defaultmap::DefaultHashMap;
use itertools::Itertools;
use std::fs;
use std::str::FromStr;

type GenError = Box<dyn std::error::Error>;

fn parse_nodes_from_file(fname: &str) -> Result<Vec<Node>, GenError> {
    let mut trees = vec![];
    for line in fs::read_to_string(fname)?.lines() {
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

fn main() -> Result<(), GenError> {
    println!("Parsing trees");
    let mut trees = parse_nodes_from_file("data/f2-21.train.parse.noLEX")?;
    println!("Done parsing {} trees (top level)", trees.len());
    let all_trees = get_all_trees(&trees);
    println!("Expanded to {} trees", all_trees.len());

    // Rules to show probabilities for.
    let to_show = ["ADJP", "NAC"];

    {
        let mut lhs_counts = DefaultHashMap::new(0);
        let mut lhs_rhs_counts = DefaultHashMap::new(0);

        for tree in &all_trees {
            lhs_counts[tree.lhs()] += 1;
            lhs_rhs_counts[(tree.lhs(), tree.rhs())] += 1;
        }

        println!(
            "There are {} rules in the grammar.",
            lhs_rhs_counts.keys().len()
        );

        let out_str = lhs_rhs_counts
            .iter()
            .filter_map(|kv| {
                let ((lhs, rhs), value) = kv;
                let lhs_count = lhs_counts[lhs];
                let prob = (*value as f64) / (lhs_count as f64);
                if to_show.contains(&lhs) {
                    Some(format!("{} -> {} = {:.6}", lhs, rhs, prob))
                } else {
                    None
                }
            })
            .sorted()
            .join("\n")
            + "\n";
        fs::write("data/unfactored.txt", out_str)?;
    }

    println!("Left factoring trees.");
    for tree in &mut trees {
        tree.left_factor();
    }

    let all_lf_trees = get_all_trees(&trees);

    {
        let mut lhs_counts = DefaultHashMap::new(0);
        let mut lhs_rhs_counts = DefaultHashMap::new(0);

        for tree in &all_lf_trees {
            lhs_counts[tree.lhs()] += 1;
            lhs_rhs_counts[(tree.lhs(), tree.rhs())] += 1;
        }

        println!(
            "There are {} rules in the left-factored grammar.",
            lhs_rhs_counts.keys().len()
        );

        let out_str = lhs_rhs_counts
            .iter()
            .filter_map(|kv| {
                let ((lhs, rhs), value) = kv;
                let lhs_count = lhs_counts[lhs];
                let prob = (*value as f64) / (lhs_count as f64);
                if to_show.contains(&lhs)
                    || to_show
                        .iter()
                        .any(|sym| lhs.starts_with(&format!("{}~", sym)))
                {
                    Some(format!("{} -> {} = {:.6}", lhs, rhs, prob))
                } else {
                    None
                }
            })
            .sorted()
            .join("\n")
            + "\n";
        fs::write("data/leftfactored.txt", out_str)?;
    }
    Ok(())
}
