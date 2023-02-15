#![allow(non_snake_case)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Node(usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Edge(usize, usize);

#[derive(Debug)]
pub(crate) struct Graph {
    pub V: Vec<Node>,
    pub E: Vec<Edge>,
}

impl Graph {
    pub(crate) fn new(V: Vec<Node>, E: Vec<Edge>) -> Self {
        Graph { V, E }
    }

    pub(crate) fn populate(V: Vec<usize>, E: Vec<(usize, usize)>) -> Graph {
        Graph::new(
            V.into_iter().map(|v| v.into()).collect(),
            E.into_iter().map(|e| e.into()).collect(),
        )
    }
}

impl Node {
    /// `neighbours` collects a `Ǹode`'s (outgoing) neighbours.
    fn neighbours(&self, g: &Graph) -> Vec<Node> {
        g.E.iter()
            .filter(|e| e.0 == self.0)
            .map(|e| e.1.into())
            .collect()
    }

    /// `deg` counts a `Ǹode`'s (outgoing) neighbours.
    fn deg(&self, g: &Graph) -> usize {
        self.neighbours(g).len()
    }
}

impl From<(usize, usize)> for Edge {
    fn from(pair: (usize, usize)) -> Self {
        Edge(pair.0, pair.1)
    }
}

impl From<usize> for Node {
    fn from(name: usize) -> Self {
        Node(name)
    }
}

use crate::data::{Queue, Stack};
use std::collections::HashSet;
/// `breadth_first_search` traverses the given `Graph` using an intermediate `Queue`.
pub(crate) fn breadth_first_search(g: &Graph, start: Node, end: Node) -> bool {
    let mut seen: HashSet<Node> = HashSet::new();
    let mut q = Queue::new();
    q.enq(start);

    println!("\nBFS(G(V,E),{},{})", start.0, end.0);
    while let Some(v) = q.deq() {
        println!("Visited {:?}", v);
        if v == end {
            return true;
        }
        for n in v.neighbours(g).into_iter() {
            println!("Seen {:?}", n);
            if seen.insert(n) {
                q.enq(n);
            }
        }
    }
    false
}

/// `depth_first_search` traverses the given `Graph` using an intermediate `Stack`.
pub(crate) fn depth_first_search(g: &Graph, start: Node, end: Node) -> bool {
    let mut seen: HashSet<Node> = HashSet::new();
    let mut s = Stack::new();
    s.push(start);
    println!("\nDFS(G(V,E),{},{})", start.0, end.0);

    while let Some(v) = s.pop() {
        println!("Visited {:?}", v);
        if v == end {
            return true;
        }
        for n in v.neighbours(g).into_iter() {
            println!("Seen {:?}", n);
            if seen.insert(n) {
                s.push(n);
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_one_node() {
        let V = vec![1];
        let E = vec![];
        let G = Graph::populate(V, E);
        assert!(breadth_first_search(&G, 1.into(), 1.into()));
        assert!(depth_first_search(&G, 1.into(), 1.into()));
    }

    #[test]
    fn tree_with_5_nodes_and_5_edges() {
        //  /1-2-3 Salzburg
        // 0 Wien
        //  \ 4 Eisenstadt
        let V = vec![0, 1, 2, 3, 4];
        let E = vec![(0, 1), (0, 4), (1, 2), (2, 3), (3, 4)];
        let G = Graph::populate(V, E);
        // Visited (and Seen) Nodes BFS: 0, (1), (4), 1, (2), 4 <- visited, 2, (3), 3 <- visited
        assert!(breadth_first_search(&G, 0.into(), 3.into()));
        assert!(!breadth_first_search(&G, 0.into(), 5.into()));
        // Visited (and Seen) Nodes DFS: 0, (1), (4), 4 <- visited, 1, (2), 2, (3), 3 <- visited
        assert!(depth_first_search(&G, 0.into(), 3.into()));
        assert!(!depth_first_search(&G, 0.into(), 5.into()));
    }
}
