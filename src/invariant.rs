use clap::Parser;
use petgraph::{algo::is_isomorphic, graph::DiGraph};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[arg(short = 'n', long, default_value = "0")]
    domain: u32,
    #[arg(short, long)]
    degree: u32,
}

use std::{
    cmp::{max, min},
    collections::HashMap,
    fmt, u32,
};

use symmetric_polynomials::{
    polynom::{Index, Monoid},
    weisfeiler_leman::invariants,
};

#[derive(Debug, Clone, Default)]
pub struct Invariant2d {
    indices: Vec<(u32, u32)>,
}

impl From<Monoid> for Invariant2d {
    fn from(value: Monoid) -> Self {
        let mut indices: Vec<(u32, u32)> = Default::default();
        for var in value.variables.iter() {
            indices.push((
                match var.indices[0] {
                    Index::Named(_) => panic!(),
                    Index::Constant(idx) => idx,
                },
                match var.indices[1] {
                    Index::Named(_) => panic!(),
                    Index::Constant(idx) => idx,
                },
            ));
        }

        indices.into_iter().collect()
    }
}

impl fmt::Display for Invariant2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;
        for &(i, j) in self.indices.iter() {
            write!(f, "({}, {}), ", i, j)?;
        }
        write!(f, "}}")
    }
}

impl FromIterator<(u32, u32)> for Invariant2d {
    fn from_iter<T: IntoIterator<Item = (u32, u32)>>(iter: T) -> Self {
        let mut cannon: Invariant2d = Invariant2d {
            indices: Default::default(),
        };

        let mut num_indices: u32 = 0;
        let mut mapping: Vec<u32> = Default::default();

        for (i, j) in iter.into_iter() {
            let i = i as usize;
            let j = j as usize;

            while mapping.len() < i {
                mapping.push(u32::MAX)
            }
            if mapping.len() == i {
                mapping.push(num_indices);
                num_indices += 1;
            }

            if mapping[i] == u32::MAX {
                mapping[i] = num_indices;
                num_indices += 1;
            }

            while mapping.len() < j {
                mapping.push(u32::MAX)
            }
            if mapping.len() == j {
                mapping.push(num_indices);
                num_indices += 1;
            }

            if mapping[j] == u32::MAX {
                mapping[j] = num_indices;
                num_indices += 1;
            }

            cannon.indices.push((mapping[i], mapping[j]));
        }

        cannon
    }
}

impl Invariant2d {
    pub fn add(&mut self, i: u32, j: u32) {
        let mut max_size = 0;
        for &(i, j) in self.indices.iter() {
            max_size = max(max_size, max(i, j));
        }
        max_size += 1;

        let mut added: (u32, u32) = (if i < max_size { i } else { max_size }, j);
        if i == max_size {
            max_size += 1;
        }
        if j > max_size {
            added.1 = max_size;
        }
        self.indices.push(added);
    }
}

impl PartialEq for Invariant2d {
    fn eq(&self, other: &Self) -> bool {
        let self_graph = DiGraph::<u32, ()>::from_edges(&self.indices);
        let other_graph = DiGraph::<u32, ()>::from_edges(&other.indices);

        is_isomorphic(&self_graph, &other_graph)
    }
}

impl Invariant2d {
    fn colours(&self) -> HashMap<u64, u64> {
        let graph = DiGraph::<(), ()>::from_edges(&self.indices);

        invariants(&graph)
    }
}

impl Eq for Invariant2d {}

fn invarient2d(degree: u32, domain_size: u32) -> Vec<Invariant2d> {
    let mut invarients: Vec<Invariant2d> = Default::default();

    if degree == 0 || domain_size == 0 {
        return invarients;
    }
    if domain_size == 1 {
        invarients.push(Invariant2d {
            indices: vec![(0, 0)],
        });
        return invarients;
    }
    if degree == 1 {
        invarients.push(Invariant2d {
            indices: vec![(0, 0)],
        });
        invarients.push(Invariant2d {
            indices: vec![(0, 1)],
        });
        return invarients;
    }

    let limit = min(domain_size, degree * 2);

    for invariant in invarient2d(degree - 1, domain_size).into_iter() {
        let mut max_size = 0;
        for &(i, j) in invariant.indices.iter() {
            max_size = max(max_size, max(i, j));
        }
        max_size = min(max_size + 3, limit);

        for i in 0..max_size {
            for j in 0..max_size {
                if invariant.indices.contains(&(i, j)) {
                    continue;
                }
                let mut current_invariant = invariant.clone();
                current_invariant.add(i, j);

                if !invarients.contains(&current_invariant) {
                    invarients.push(current_invariant);
                }
            }
        }
    }

    invarients
}

fn main() {
    let cli = Args::parse();

    let domain_size = if cli.domain == 0 {
        cli.degree * 2
    } else {
        cli.domain
    };

    let invarients = invarient2d(cli.degree, domain_size);

    let colours: Vec<HashMap<u64, u64>> = invarients.iter().map(|inv| inv.colours()).collect();

    let mut num_indistinguishable = 0;
    for (i, invarient) in invarients.iter().enumerate() {
        if colours[..i].iter().all(|current| current != &colours[i]) {
            num_indistinguishable += 1;
            println!("indistinguishable {invarient}");

            for (j, pair) in invarients.iter().enumerate() {
                if i != j && colours[i] == colours[j] {
                    println!("- {pair}");
                }
            }
        }
    }

    println!("num invarient: {}", invarients.len());
    println!("num indistinguishable: {}", num_indistinguishable);
}
