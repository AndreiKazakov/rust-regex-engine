use crate::graph::Graph;
use crate::nfa::NfaArrow::*;
use crate::nfa::{NfaArrow, NFA};

pub fn parse(pattern: &str) -> Result<NFA, String> {
    Ok(parse_inner(pattern, None)?.0)
}

fn parse_inner(pattern: &str, stop_at: Option<char>) -> Result<(NFA, usize), String> {
    let mut i = 0;
    let mut graph = Graph::new(0);
    if stop_at == None {
        graph = graph.add_edge(0, Dot, 0);
    }
    let mut previous_node = 0;

    loop {
        let mut step = 1;
        let final_node = graph.final_node;

        match pattern.chars().nth(i) {
            None => break,
            Some(c) if stop_at == Some(c) => {
                i += step;
                return Ok((graph, i));
            }
            Some('.') => {
                graph = graph.add_edge(final_node, Dot, final_node + 1);
                graph.final_node += 1;
                previous_node = final_node;
            }
            Some('^') => {
                graph = graph.add_edge(final_node, LineStart, final_node + 1);
                graph.final_node += 1;
                previous_node = final_node;
            }
            Some('$') => {
                graph = graph.add_edge(final_node, LineEnd, final_node + 1);
                graph.final_node += 1;
                previous_node = final_node;
            }
            Some('\\') => match pattern.chars().nth(i + 1) {
                None => return Err("escape character at EOL".to_string()),
                Some(c) if ['\\', '+', '*', '(', ')', '[', ']', '.', '?'].contains(&c) => {
                    graph = graph.add_edge(final_node, Char(c), final_node + 1);
                    graph.final_node += 1;
                    previous_node = final_node;
                    step += 1;
                }
                Some(c) => return Err(format!("Unexpected character escaped: {}", c)),
            },
            Some('|') => {
                let right = parse_inner(&pattern[i + 1..], stop_at)?;
                if right.0.edges.is_empty() {
                    graph = graph.add_edge(0, Epsilon, final_node)
                } else {
                    graph = graph.attach_parallel(right.0, 0, final_node);
                }
                return Ok((graph, i + step + right.1));
            }
            Some('?') => {
                if i == 0 || !can_apply_metacharacter(pattern.chars().nth(i - 1)) {
                    return Err("Can not apply '?'".to_string());
                }
                graph = graph.add_edge(previous_node, Epsilon, final_node);
            }
            Some('+') => {
                if i == 0 || !can_apply_metacharacter(pattern.chars().nth(i - 1)) {
                    return Err("Can not apply '+'".to_string());
                }
                graph = graph.add_edge(final_node, Epsilon, previous_node);
            }
            Some('*') => {
                if i == 0 || !can_apply_metacharacter(pattern.chars().nth(i - 1)) {
                    return Err("Can not apply '*'".to_string());
                }
                graph = graph.add_edge(previous_node, Epsilon, final_node);
                graph = graph.add_edge(final_node, Epsilon, previous_node);
            }
            Some('(') => {
                let inner = parse_inner(&pattern[i + 1..], Some(')'))?;
                step += inner.1;
                graph = graph.concat(inner.0);
                previous_node = final_node;
            }
            Some('[') => {
                let (char_class, len) = parse_character_class(&pattern[i + 1..])?;
                graph = graph.add_edge(final_node, char_class, final_node + 1);
                step += len;
                graph.final_node += 1;
                previous_node = final_node;
            }
            Some(')') => return Err("unexpected character: )".to_string()),
            Some(c) => {
                graph = graph.add_edge(final_node, Char(c), final_node + 1);
                graph.final_node += 1;
                previous_node = final_node;
            }
        }

        i += step;
    }

    match stop_at {
        None => Ok((graph, i)),
        Some(c) => Err(format!("Expected {} got end of line", c)),
    }
}

fn parse_character_class(char_class: &str) -> Result<(NfaArrow, usize), String> {
    let mut j = 0;
    let mut chars = Vec::new();
    let mut exclusive = false;

    loop {
        match char_class.chars().nth(j) {
            None => return Err("Unexpected EOL".to_string()),
            Some('\\') => {
                let c = char_class
                    .chars()
                    .nth(j + 1)
                    .ok_or_else(|| "Unexpected EOL".to_string())?;
                chars.push(c);
                j += 1;
            }
            Some('^') if j == 0 => exclusive = true,
            Some(']') if j == 0 || (exclusive && j == 1) => chars.push(']'),
            Some(']') => break,
            Some(option) => chars.push(option),
        }
        j += 1;
    }

    if chars.is_empty() {
        return Err("Empty character class".to_string());
    }
    Ok((
        if exclusive {
            NotOneOf(chars)
        } else {
            OneOf(chars)
        },
        j + 1,
    ))
}

fn can_apply_metacharacter(ch: Option<char>) -> bool {
    match ch {
        None | Some('*') | Some('+') | Some('?') => false,
        _ => true,
    }
}

#[cfg(test)]
mod nfa_test {
    use super::*;

    #[test]
    fn test_parse_inner() {
        let graph = Graph::new(3)
            .add_edge(0, Dot, 0)
            .add_edge(1, OneOf(vec!['b', 'c', 'd']), 2)
            .add_edge(2, Epsilon, 1)
            .add_edge(2, Char('e'), 3)
            .add_edge(5, Char('\\'), 3)
            .add_edge(4, Epsilon, 0)
            .add_edge(4, Dot, 5)
            .add_edge(0, Char('a'), 1)
            .add_edge(0, Char('q'), 4)
            .add_edge(0, Epsilon, 4)
            .add_edge(0, Epsilon, 3);

        match parse_inner(r"a[bcd]+e|(q*.\\)?", None) {
            Err(e) => panic!("Failed to parse: {}", e),
            Ok(res) => assert_eq!(res, (graph, 17)),
        }
    }

    #[test]
    fn test_parse_brackets() {
        let graph = Graph::new(1)
            .add_edge(0, Dot, 0)
            .add_edge(0, OneOf(vec!['b', 'c']), 1);

        match parse_inner(r"[bc]", None) {
            Err(e) => panic!("Failed to parse: {}", e),
            Ok(res) => assert_eq!(res, (graph, 4)),
        }
    }
}
