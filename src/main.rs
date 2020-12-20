mod dfa;
mod graph;
mod nfa;
mod parser;

#[cfg(test)]
mod test;

use crate::dfa::DFA;
use std::env;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if let [_, pattern, string] = args.as_slice() {
        let graph = parser::parse(pattern.as_str())?;
        println!("{:?}", DFA::new(graph).walk(string.to_owned()));
    } else {
        println!("Usage: re <pattern> <string>")
    }

    Ok(())
}
