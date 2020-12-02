use super::graph::Graph;
use NfaArrow::*;

pub type ParseResult = (Graph<NfaArrow>, usize);

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum NfaArrow {
    Epsilon,
    Char(char),
    Dot,
    LineStart,
    LineEnd,
}

pub fn parse(pattern: &str, stop_at: Option<char>) -> Result<ParseResult, String> {
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
            Some(c) if c.is_alphanumeric() => {
                graph = graph.add_edge(final_node, Char(c), final_node + 1);
                graph.final_node += 1;
                previous_node = final_node;
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
                graph = graph.attach_parallel(right.0, 0, final_node);
                step += right.1;
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
                        Some(']') => break,
                        Some(option) => {
                            graph = graph.add_edge(final_node, Char(option), final_node + 1);
                            j += 1;
                        }
                    }
                }
                step = j;
                graph.final_node += 1;
                previous_node = final_node;
            }
            Some(c) => return Err(format!("unexpected character: {}", c)),
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_test() {
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
}
