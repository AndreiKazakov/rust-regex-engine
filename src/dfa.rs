use std::collections::HashMap;

use crate::nfa::{follow_char, initial_state, NFAState, NFA};

#[derive(Debug, Eq, PartialEq)]
pub struct DState {
    nfa_states: NFAState,
    next_states: HashMap<char, usize>,
}

#[derive(Debug)]
pub struct DFA {
    nfa: NFA,
    pub states: Vec<DState>,
    pub current_state: usize,
}
impl DFA {
    pub fn new(nfa: NFA) -> Self {
        let init = DState {
            nfa_states: initial_state(&nfa),
            next_states: HashMap::new(),
        };
        Self {
            nfa,
            states: vec![init],
            current_state: 0,
        }
    }

    pub fn walk(&mut self, text: String) -> bool {
        for (i, c) in text.chars().enumerate() {
            if self.get_current_states().contains(&self.nfa.final_node) {
                return true;
            }

            self.next(c, i == text.len() - 1);

            if self.get_current_states().is_empty() {
                return false;
            }
        }

        self.get_current_states().contains(&self.nfa.final_node)
    }

    fn next(&mut self, c: char, is_last: bool) {
        let next_index = self.states.len();
        let current_state = self.states.get_mut(self.current_state).unwrap();
        match current_state.next_states.get(&c) {
            None => {
                let next_states = follow_char(&self.nfa, &current_state.nfa_states, c, is_last);
                let state = DState {
                    nfa_states: next_states,
                    next_states: HashMap::new(),
                };
                current_state.next_states.insert(c, next_index);
                self.states.push(state);
                self.current_state = next_index;
            }
            Some(&d) => self.current_state = d,
        }
    }

    fn get_current_states(&self) -> &NFAState {
        &self.states.get(self.current_state).unwrap().nfa_states
    }
}

#[cfg(test)]
mod dfa_test {
    use crate::{parser, regex_tests};

    use super::*;

    fn check_for_pattern(pattern: &str, string: &str) -> Result<bool, String> {
        let nfa = parser::parse(pattern)?;
        Ok(DFA::new(nfa).walk(string.to_string()))
    }

    regex_tests!(check_for_pattern);
}
