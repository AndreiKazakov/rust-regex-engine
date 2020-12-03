use super::graph::{Edge, Graph};
use std::collections::HashSet;
use NfaArrow::*;

pub fn check(pattern: String, string: String) -> Result<bool, String> {
    let graph = parse(pattern.as_str(), None)?.0;
    Ok(walk(graph, string))
}

type NFA = Graph<NfaArrow>;
type NFAState = HashSet<usize>;
type ParseResult = (NFA, usize);

#[derive(Debug, Eq, PartialEq, Clone)]
enum NfaArrow {
    Epsilon,
    Char(char),
    OneOf(Vec<char>),
    NotOneOf(Vec<char>),
    Dot,
    LineStart,
    LineEnd,
}

fn parse(pattern: &str, stop_at: Option<char>) -> Result<ParseResult, String> {
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
                let right = parse(&pattern[i + 1..], stop_at)?;
                if right.0.edges.is_empty() {
                    graph = graph.add_edge(0, Epsilon, final_node)
                } else {
                    graph = graph.attach_parallel(right.0, 0, final_node);
                }
                step += right.1 - 1;
                previous_node = 0;
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
                let inner = parse(&pattern[i + 1..], Some(')'))?;
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

fn walk(nfa: NFA, text: String) -> bool {
    let mut state = HashSet::new();
    state.insert(0);
    state.extend(step(&nfa, &state, |e| e.ch == LineStart));
    state = follow_empty(&nfa, state);

    for (i, c) in text.chars().enumerate() {
        if state.contains(&nfa.final_node) {
            return true;
        }

        state = step(&nfa, &state, |e| match &e.ch {
            Char(ch) => c == *ch,
            Dot => true,
            OneOf(chars) => chars.contains(&c),
            NotOneOf(chars) => !chars.contains(&c),
            _ => false,
        });
        if state.is_empty() {
            return false;
        }

        if i == text.len() - 1 {
            state.extend(step(&nfa, &state, |e| e.ch == LineEnd));
        }

        state = follow_empty(&nfa, state);
    }

    state.contains(&nfa.final_node)
}

fn step<F: Fn(&&Edge<NfaArrow>) -> bool>(nfa: &NFA, states: &NFAState, predicate: F) -> NFAState {
    let mut relevant_edges = HashSet::new();

    for s in states {
        if let Some(edges) = nfa.edges.get(&s) {
            relevant_edges.extend(edges.iter().filter(&predicate).map(|e| e.to));
        }
    }

    relevant_edges
}

fn follow_empty(nfa: &NFA, mut state: NFAState) -> NFAState {
    loop {
        let empty = step(&nfa, &state, |e| e.ch == Epsilon);
        let diff: HashSet<_> = empty.difference(&state).collect();
        if diff.is_empty() {
            break;
        }
        state.extend(empty)
    }

    state
}

#[cfg(test)]
mod nfa_test {
    use super::super::test::TEST_CASES;
    use super::*;

    #[test]
    fn test_parse() {
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

        match parse(r"a[bcd]+e|(q*.\\)?", None) {
            Err(e) => panic!("Failed to parse: {}", e),
            Ok(res) => assert_eq!(res, (graph, 17)),
        }
    }

    #[test]
    fn test_parse_brackets() {
        let graph = Graph::new(1)
            .add_edge(0, Dot, 0)
            .add_edge(0, OneOf(vec!['b', 'c']), 1);

        match parse(r"[bc]", None) {
            Err(e) => panic!("Failed to parse: {}", e),
            Ok(res) => assert_eq!(res, (graph, 4)),
        }
    }

    #[test]
    fn test_check() {
        for (pattern, string, expected) in TEST_CASES.iter() {
            let res = check(pattern.to_string(), string.to_string());
            match (expected, &res) {
                (Ok(e), Ok(r)) => assert_eq!(
                    r, e,
                    "Testing that pattern {} tested for string {} should be {}",
                    pattern, string, e
                ),
                (Err(_), Err(_)) => (),
                _ => panic!(
                    "Expectation failed: {:?} is not equal to {:?} for pattern {} and string {}",
                    res, expected, pattern, string
                ),
            }
        }
    }

    #[test]
    fn test_follow_empty() {
        let graph = Graph::new(3)
            .add_edge(0, Char('b'), 1)
            .add_edge(1, Char('c'), 4)
            .add_edge(2, Char('d'), 3)
            .add_edge(1, Epsilon, 2)
            .add_edge(1, Epsilon, 5)
            .add_edge(2, Epsilon, 1)
            .add_edge(2, Char('e'), 3)
            .add_edge(2, Epsilon, 0);

        let mut expected = HashSet::new();
        expected.extend(vec![0, 2, 5, 1]);

        let mut state = HashSet::new();
        state.insert(1);
        assert_eq!(expected, follow_empty(&graph, state));
    }

    #[test]
    fn test_step() {
        let graph = Graph::new(3)
            .add_edge(0, Dot, 0)
            .add_edge(0, Char('b'), 1)
            .add_edge(1, Char('c'), 4)
            .add_edge(2, Char('d'), 3)
            .add_edge(1, Epsilon, 2)
            .add_edge(2, Epsilon, 0)
            .add_edge(3, Epsilon, 0)
            .add_edge(4, Char('z'), 5)
            .add_edge(5, Char('z'), 6)
            .add_edge(7, Char('u'), 6)
            .add_edge(7, Epsilon, 9)
            .add_edge(9, Char('z'), 10)
            .add_edge(10, Char('u'), 6)
            .add_edge(5, Epsilon, 8);

        let mut expected = HashSet::new();
        expected.extend(vec![5]);

        let mut initial_states = HashSet::new();
        initial_states.extend(vec![1, 3, 4, 7]);
        assert_eq!(
            expected,
            step(&graph, &initial_states, |e| e.ch == Char('z'))
        );
    }
}
