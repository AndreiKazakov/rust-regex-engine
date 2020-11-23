mod graph;
mod nfa;

use std::env;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if let [_, pattern, string] = args.as_slice() {
        println!("{} - {}", pattern, string);
        println!("{:?}", nfa::parse(pattern.as_str(), None)?);
    } else {
        println!("Usage: re <pattern> <string>")
    }

    Ok(())
}
