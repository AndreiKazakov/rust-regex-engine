use std::collections::HashMap;
use std::fmt;

#[derive(Clone, PartialEq, Eq)]
/// Basic domain-specific implementation of a graph.
/// It is assumed that there is only one Initial node (at index 0) and one Final node.
pub struct Graph {
    edges: HashMap<Node, Vec<Edge>>,
    node_count: usize,
    pub final_node: usize,
}

impl Graph {
    pub fn new(final_node: Node) -> Self {
        Self {
            edges: HashMap::new(),
            node_count: final_node + 1,
            final_node,
        }
    }

    pub fn add_edge(mut self, from: Node, ch: Option<char>, to: Node) -> Self {
        self.edges
            .entry(from)
            .or_insert(vec![])
            .push(Edge { ch, to });

        if from + 1 > self.node_count {
            self.node_count = from + 1;
        }

        if to + 1 > self.node_count {
            self.node_count = to + 1;
        }

        self
    }

    pub fn concat(self, other: Self) -> Self {
        let offset = self.node_count - 1;
        let mut graph = self;

        for (from, edges) in other.edges {
            for e in edges {
                let edge_from = match from {
                    0 => graph.final_node,
                    f => f + offset,
                };
                let edge_to = match e.to {
                    0 => graph.final_node,
                    t => t + offset,
                };
                graph = graph.add_edge(edge_from, e.ch, edge_to);
            }
        }

        graph.final_node = other.final_node + offset;
        graph
    }

    pub fn attach_parallel(self, other: Self, from: Node, to: Node) -> Self {
        let offset = self.node_count - 1;
        let mut graph = self;

        for (other_from, edges) in other.edges {
            for e in edges {
                let edge_from = match other_from {
                    0 => from,
                    f if other.final_node == f => to,
                    f => f + offset,
                };
                let edge_to = match e.to {
                    0 => from,
                    t if other.final_node == t => to,
                    t => t + offset,
                };
                graph = graph.add_edge(edge_from, e.ch, edge_to);
            }
        }

        graph
    }

    pub fn insert(self, other: Self, at: Node) -> Self {
        let offset = self.node_count;
        let mut graph = self;

        for (_, edges) in graph.edges.iter_mut() {
            for e in edges.iter_mut() {
                if e.to == at {
                    e.to = offset;
                }
            }
        }

        for (other_from, edges) in other.edges {
            for e in edges {
                let edge_to = match e.to {
                    f if f == other.final_node => at,
                    f => f + offset,
                };
                graph = graph.add_edge(other_from + offset, e.ch, edge_to);
            }
        }

        graph
    }
}

impl fmt::Debug for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printed_edges = String::new();

        for (from, edges) in self.edges.iter() {
            for e in edges {
                printed_edges.push_str(&format!("\n\t{:?} {:?}", from, e));
            }
        }

        write!(
            f,
            "Graph({}\n\tfinal node: {}\n)",
            printed_edges, self.final_node
        )
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Edge {
    pub ch: Option<char>,
    pub to: Node,
}

impl fmt::Debug for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "-{}-> {}", self.ch.unwrap_or('ϵ'), self.to)
    }
}

type Node = usize;

#[cfg(test)]
mod test {
    use super::Graph;
    use std::collections::HashMap;

    #[test]
    fn add_edge() {
        let mut g = Graph::new(2);
        g = g.add_edge(0, Some('a'), 1);
        assert_eq!(g.edges, hash(vec![(0, vec![edge('a', 1)])]));
        g = g.add_edge(0, Some('b'), 2);
        assert_eq!(g.edges, hash(vec![(0, vec![edge('a', 1), edge('b', 2)])]));
    }

    #[test]
    fn test_concat() {
        let g1 = Graph::new(2)
            .add_edge(0, Some('a'), 1)
            .add_edge(1, Some('b'), 2);
        let g2 = Graph::new(2)
            .add_edge(0, Some('c'), 1)
            .add_edge(1, Some('d'), 2);

        let expected = Graph {
            node_count: 5,
            final_node: 4,
            edges: hash(vec![
                (0, vec![edge('a', 1)]),
                (1, vec![edge('b', 2)]),
                (2, vec![edge('c', 3)]),
                (3, vec![edge('d', 4)]),
            ]),
        };
        assert_eq!(g1.concat(g2), expected);
    }

    #[test]
    fn test_insert() {
        let g1 = Graph::new(3)
            .add_edge(0, Some('a'), 1)
            .add_edge(1, Some('b'), 2)
            .add_edge(2, Some('c'), 3);
        let g2 = Graph::new(2)
            .add_edge(0, Some('d'), 1)
            .add_edge(1, Some('e'), 2);

        let expected = Graph {
            node_count: 6,
            final_node: 3,
            edges: hash(vec![
                (0, vec![edge('a', 1)]),
                (1, vec![edge('b', 4)]),
                (5, vec![edge('e', 2)]),
                (2, vec![edge('c', 3)]),
                (4, vec![edge('d', 5)]),
            ]),
        };
        assert_eq!(g1.insert(g2, 2), expected);
    }

    #[test]
    fn test_attach_parallel() {
        let g1 = Graph::new(4)
            .add_edge(0, Some('a'), 1)
            .add_edge(1, Some('b'), 2)
            .add_edge(2, Some('c'), 3)
            .add_edge(3, Some('d'), 4);
        let g2 = Graph::new(3)
            .add_edge(0, Some('e'), 1)
            .add_edge(1, Some('f'), 2)
            .add_edge(2, Some('g'), 3);

        let expected = Graph {
            node_count: 7,
            final_node: 4,
            edges: hash(vec![
                (0, vec![edge('a', 1)]),
                (1, vec![edge('b', 2), edge('e', 5)]),
                (2, vec![edge('c', 3)]),
                (3, vec![edge('d', 4)]),
                (5, vec![edge('f', 6)]),
                (6, vec![edge('g', 3)]),
            ]),
        };
        assert_eq!(g1.attach_parallel(g2, 1, 3), expected);
    }

    fn edge(ch: char, to: usize) -> super::Edge {
        super::Edge { ch: Some(ch), to }
    }

    fn hash(entries: Vec<(usize, Vec<super::Edge>)>) -> HashMap<usize, Vec<super::Edge>> {
        entries.iter().cloned().collect()
    }
}
