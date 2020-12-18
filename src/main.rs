mod graph;
mod nfa;
mod parser;

#[cfg(test)]
mod test;

use crate::nfa::walk;
use std::env;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if let [_, pattern, string] = args.as_slice() {
        let graph = parser::parse(pattern.as_str())?;
        println!("{:?}", walk(graph, string.to_owned()));
    } else {
        println!("Usage: re <pattern> <string>")
    }

    Ok(())
}
