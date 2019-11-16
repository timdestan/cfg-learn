// Pretty print a single tree.

use cfg_learn::Node;
use std::fs;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        println!("Expected single filename arg");
        std::process::exit(1);
    }

    let string = fs::read_to_string(&args[1])?;
    let node = Node::from_str(&string)?;
    println!("{:?}", node);
    Ok(())
}
