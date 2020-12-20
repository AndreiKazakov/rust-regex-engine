use super::graph::{Edge, Graph};

use std::collections::HashSet;
use NfaArrow::*;

pub type NFA = Graph<NfaArrow>;
pub type NFAState = HashSet<usize>;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum NfaArrow {
    Epsilon,
    Char(char),
    OneOf(Vec<char>),
    NotOneOf(Vec<char>),
    Dot,
    LineStart,
    LineEnd,
}

pub fn initial_state(nfa: &NFA) -> NFAState {
    let mut state = HashSet::new();
    state.insert(0);
    state.extend(step(&nfa, &state, |e| e.ch == LineStart));
    follow_empty(&nfa, state)
}

pub fn walk(nfa: NFA, text: String) -> bool {
    let mut state = initial_state(&nfa);

    for (i, c) in text.chars().enumerate() {
        if state.contains(&nfa.final_node) {
            return true;
        }

        state = follow_char(&nfa, &state, c, i == text.len() - 1);

        if state.is_empty() {
            return false;
        }
    }

    state.contains(&nfa.final_node)
}

pub fn follow_char(nfa: &NFA, state: &NFAState, c: char, is_last: bool) -> NFAState {
    let mut new_state = step_with_char(&nfa, &state, c);
    if is_last {
        new_state.extend(step_with_line_end(&nfa, &new_state));
    }
    follow_empty(&nfa, new_state)
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

fn step_with_line_end(nfa: &NFA, state: &NFAState) -> NFAState {
    step(&nfa, &state, |e| e.ch == LineEnd)
}

fn step_with_char(nfa: &NFA, state: &NFAState, c: char) -> NFAState {
    step(nfa, state, |e| match &e.ch {
        Char(ch) => c == *ch,
        Dot => true,
        OneOf(chars) => chars.contains(&c),
        NotOneOf(chars) => !chars.contains(&c),
        _ => false,
    })
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
    use super::*;
    use crate::parser::parse;
    use crate::regex_tests;

    fn check_for_pattern(pattern: &str, string: &str) -> Result<bool, String> {
        let graph = parse(pattern)?;
        Ok(walk(graph, string.to_string()))
    }

    regex_tests!(check_for_pattern);
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
