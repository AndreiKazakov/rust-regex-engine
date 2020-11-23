use super::graph::Graph;

pub type ParseResult = (Graph, usize);

pub fn parse(pattern: &str, stop_at: Option<char>) -> Result<ParseResult, String> {
    let mut i = 0;
    let mut graph = Graph::new(0);
    let mut previous_node = 0;

    loop {
        let mut step = 1;
        let final_node = graph.final_node;

        match pattern.chars().nth(i) {
            None => break,
            Some(c) if stop_at == Some(c) => {
                i += step;
                break;
            }
            Some(c) if c.is_alphanumeric() => {
                graph = graph.add_edge(final_node, Some(c), final_node + 1);
                graph.final_node += 1;
                previous_node = final_node;
            }
            Some('\\') => match pattern.chars().nth(i + 1) {
                None => return Err("escape character at EOL".to_string()),
                Some(c) if ['\\', '+', '*', '(', ')', '[', ']', '.', '?'].contains(&c) => {
                    graph = graph.add_edge(final_node, Some(c), final_node + 1);
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
                graph = graph.add_edge(previous_node, None, final_node);
            }
            Some('+') => {
                graph = graph.add_edge(final_node, None, previous_node);
            }
            Some('*') => {
                graph = graph.add_edge(previous_node, None, final_node);
                graph = graph.add_edge(final_node, None, previous_node);
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
                            graph = graph.add_edge(final_node, Some(option), final_node + 1);
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

    Ok((graph, i))
}
