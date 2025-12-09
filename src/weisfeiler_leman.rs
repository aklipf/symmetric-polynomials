use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

use itertools::Itertools;
use petgraph::{Direction, Graph};

fn get_hash<I: Iterator<Item = T>, T: Hash>(it: I) -> u64 {
    let mut s = DefaultHasher::new();
    for value in it {
        value.hash(&mut s);
    }
    s.finish()
}

pub fn propagate(graph: &Graph<u64, ()>) -> Graph<u64, ()> {
    let mut result = graph.clone();
    for node in graph.node_indices() {
        result[node] = get_hash(
            graph
                .neighbors_directed(node, Direction::Incoming)
                .map(|n| graph.node_weight(n).unwrap())
                .sorted(),
        );
    }
    result
}

pub fn invariants<N, E>(graph: &Graph<N, E>) -> HashMap<u64, u64> {
    let n = graph.node_count();

    let mut node_hash = Graph::<u64, ()>::from_edges(
        graph
            .edge_indices()
            .map(|e| graph.edge_endpoints(e).unwrap()),
    );

    for _ in 1..n {
        node_hash = propagate(&node_hash);
    }

    let mut frequency: HashMap<u64, u64> = Default::default();

    for &hash in node_hash.node_weights() {
        frequency
            .entry(hash)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    frequency
}
