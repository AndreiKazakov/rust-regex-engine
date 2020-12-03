use super::graph::Graph;
use std::collections::HashSet;
use NfaArrow::*;

type ParseResult = (Graph<NfaArrow>, usize);

#[derive(Debug, Eq, PartialEq, Clone)]
enum NfaArrow {
    Epsilon,
    Char(char),
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
                let mut j = i + 1;
                loop {
                    match pattern.chars().nth(j) {
                        None => return Err("Unexpected EOL".to_string()),
                        Some('\\') => {
                            let c = pattern
                                .chars()
                                .nth(j + 1)
                                .ok_or_else(|| "Unexpected EOL".to_string())?;
                            graph = graph.add_edge(final_node, Char(c), final_node + 1);
                            j += 2;
                        }
                        Some(']') if j == i + 1 => {
                            graph = graph.add_edge(final_node, Char(']'), final_node + 1);
                            j += 1;
                        }
                        Some(']') => break,
                        Some(option) => {
                            graph = graph.add_edge(final_node, Char(option), final_node + 1);
                            j += 1;
                        }
                    }
                }
                if j == i + 1 {
                    return Err("Empty character class".to_string());
                }
                step += j - i;
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

fn can_apply_metacharacter(ch: Option<char>) -> bool {
    match ch {
        None | Some('*') | Some('+') | Some('?') => false,
        _ => true,
    }
}

pub fn check(pattern: String, string: String) -> Result<bool, String> {
    let graph = parse(pattern.as_str(), None)?.0;
    println!("pattern {}: {:?}", pattern, graph);
    Ok(walk(graph, string))
}

fn walk(nfa: Graph<NfaArrow>, text: String) -> bool {
    let mut state = HashSet::new();
    state.insert(0);
    state.extend(follow(&nfa, &state, LineStart));
    state.extend(follow_empty(&nfa, &state));

    for (i, c) in text.chars().enumerate() {
        if state.contains(&nfa.final_node) {
            return true;
        }

        state = step(&nfa, state, c);
        if state.is_empty() {
            return false;
        }

        if i == text.len() - 1 {
            state.extend(follow(&nfa, &state, LineEnd));
        }

        state.extend(follow_empty(&nfa, &state));
    }

    state.contains(&nfa.final_node)
}

fn step(nfa: &Graph<NfaArrow>, states: HashSet<usize>, c: char) -> HashSet<usize> {
    let mut relevant_edges = HashSet::new();

    for s in states {
        if let Some(edges) = nfa.edges.get(&s) {
            relevant_edges.extend(
                edges
                    .iter()
                    .filter(|e| e.ch == Char(c) || e.ch == Dot)
                    .map(|e| e.to),
            );
        }
    }

    relevant_edges
}

fn follow(nfa: &Graph<NfaArrow>, state: &HashSet<usize>, arrow: NfaArrow) -> HashSet<usize> {
    let mut matches = HashSet::new();

    for s in state {
        if let Some(edges) = nfa.edges.get(s) {
            for e in edges {
                if e.ch == arrow {
                    matches.insert(e.to);
                }
            }
        }
    }

    matches
}

fn follow_empty(nfa: &Graph<NfaArrow>, state: &HashSet<usize>) -> HashSet<usize> {
    let mut empty = HashSet::new();

    for &s in state {
        empty.extend(follow_single_empty(&nfa, s, &empty));
    }
    empty
}

fn follow_single_empty(
    nfa: &Graph<NfaArrow>,
    state: usize,
    acc: &HashSet<usize>,
) -> HashSet<usize> {
    let mut empty = HashSet::new();
    let mut new_acc = acc.clone();

    if let Some(edges) = nfa.edges.get(&state) {
        for e in edges {
            if e.ch == Epsilon && !new_acc.contains(&e.to) {
                empty.insert(e.to);
                new_acc.insert(e.to);
                empty.extend(follow_single_empty(nfa, e.to, &new_acc));
            }
        }
    }

    empty
}

#[cfg(test)]
mod nfa_test {
    use super::super::test::TEST_CASES;
    use super::*;

    #[test]
    fn test_parse() {
        let graph = Graph::new(3)
            .add_edge(0, Dot, 0)
            .add_edge(1, Char('b'), 2)
            .add_edge(1, Char('c'), 2)
            .add_edge(1, Char('d'), 2)
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
            .add_edge(0, Char('b'), 1)
            .add_edge(0, Char('c'), 1);

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
    fn test_follow_single_empty() {
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
        expected.extend(vec![0, 2, 5]);

        let mut acc = HashSet::new();
        acc.insert(1);
        assert_eq!(expected, follow_single_empty(&graph, 1, &acc));
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
        assert_eq!(expected, step(&graph, initial_states, 'z'));
    }
}
