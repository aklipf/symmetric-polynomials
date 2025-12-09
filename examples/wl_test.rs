use petgraph::{
    Graph,
    graph::{DiGraph, UnGraph},
};
use symmetric_polynomials::weisfeiler_leman::invariants;

fn main() {
    let g1 = DiGraph::<u64, ()>::from_edges([(0,0), (0,1)]);
    let g2 = DiGraph::<u64, ()>::from_edges([(0,1), (0,0)]);

    println!("first graph");
    println!("{:?}",invariants(&g1));
    println!("second graph");
    println!("{:?}",invariants(&g2));
}
