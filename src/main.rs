mod graph;
mod nfa;

#[cfg(test)]
mod test;

use std::env;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if let [_, pattern, string] = args.as_slice() {
        println!("{} - {}", pattern, string);
        println!("{:?}", nfa::check(pattern.to_string(), string.to_string()));
    } else {
        println!("Usage: re <pattern> <string>")
    }

    Ok(())
}
