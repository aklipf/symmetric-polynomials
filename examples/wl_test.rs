use petgraph::graph::DiGraph;
use symmetric_polynomials::weisfeiler_leman::invariants;

fn main() {
    let g1 = DiGraph::<(), ()>::from_edges([
        (0, 1),
        (1, 0),
        (1, 2),
        (2, 1),
        (2, 3),
        (3, 2),
        (3, 4),
        (4, 3),
        (4, 5),
        (5, 4),
        (5, 0),
        (0, 5),
    ]);
    let g2 = DiGraph::<(), ()>::from_edges([
        (0, 1),
        (1, 0),
        (1, 2),
        (2, 1),
        (2, 0),
        (0, 2),
        (3, 4),
        (4, 3),
        (4, 5),
        (5, 4),
        (5, 3),
        (3, 5),
    ]);

    println!("first graph");
    println!("{:?}", invariants(&g1));
    println!("second graph");
    println!("{:?}", invariants(&g2));
}
