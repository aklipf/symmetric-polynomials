use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[arg(short = 'n', long, default_value = "0")]
    domain: u32,
    #[arg(short, long)]
    degree: u32,
}

use std::{collections::HashSet, ffi::os_str::Display, fmt, u32};

use itertools::Itertools;
use symmetric_polynomials::polynom::{Index, Monoid, Variable};

fn choose(n: u64, k: u64) -> u64 {
    if k == 0 {
        return 1;
    }
    (n * choose(n - 1, k - 1)) / k
}

fn invarient(degree: u32, num_variables: u32, domain_size: u32) -> Vec<HashSet<Monoid>> {
    let mut invarients: Vec<HashSet<Monoid>> = Default::default();

    let num_indices = degree * num_variables;
    let indices: Vec<Index> = (0..num_indices)
        .map(|idx| Index::Named(format!("i_{idx}")))
        .collect();

    for x in (0..num_indices)
        .map(|_| indices.iter())
        .multi_cartesian_product()
    {
        let monoid: Monoid = (0..degree)
            .map(|i| {
                let begin: usize = (i * num_variables) as usize;
                let end: usize = ((i + 1) * num_variables) as usize;
                Variable::new("x", x[begin..end].iter().cloned().cloned())
            })
            .collect();

        if !monoid.is_multilinear() {
            continue;
        }

        let orbital = monoid.orbital(domain_size);

        if orbital.len() > 0 && (!invarients.contains(&orbital)) {
            invarients.push(orbital);
        }
    }
    invarients
}

/*
function to check if two monoid are on the same orbital
cannonical orderging of a monomial?


# Cannonical ordering

| diag elements (x_{ii}) | off diag elements (x_{ij}) |

from 1 variable to 2d variables (if monomials of degree d)

Degree d+1 can be built recusivly from degree d.

*/

#[derive(Debug, Clone, Default)]
pub struct Monomial2d {
    indices: Vec<(u32, u32)>,
}

impl fmt::Display for Monomial2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;
        for &(i, j) in self.indices.iter() {
            write!(f, "({}, {}), ", i, j)?;
        }
        write!(f, "}}")
    }
}

impl FromIterator<(u32, u32)> for Monomial2d {
    fn from_iter<T: IntoIterator<Item = (u32, u32)>>(iter: T) -> Self {
        Monomial2d {
            indices: iter.into_iter().collect(),
        }
    }
}

impl Monomial2d {
    pub fn cannonical(self) -> Self {
        let mut cannon: Monomial2d = Monomial2d {
            indices: Vec::with_capacity(self.indices.len()),
        };

        let mut num_indices: u32 = 0;
        let mut mapping: Vec<u32> = vec![u32::MAX; self.indices.len() * 2];

        for &(i, j) in self.indices.iter() {
            if i != j {
                continue;
            }

            if mapping[i as usize] == u32::MAX {
                mapping[i as usize] = num_indices;
                num_indices += 1;
            }

            let index = mapping[i as usize];
            cannon.indices.push((index, index));
        }

        let num_diag = cannon.indices.len();

        for &(i, j) in self.indices.iter() {
            if i == j {
                continue;
            }

            if mapping[i as usize] == u32::MAX {
                mapping[i as usize] = num_indices;
                num_indices += 1;
            }

            if mapping[j as usize] == u32::MAX {
                mapping[j as usize] = num_indices;
                num_indices += 1;
            }

            cannon
                .indices
                .push((mapping[i as usize], mapping[j as usize]));
        }

        cannon.indices[..num_diag].sort_by_key(|&(i, j)| i * num_indices + j);

        cannon
    }
}

fn main() {
    let cli = Args::parse();

    let domain_size = if cli.domain == 0 {
        cli.degree * 2
    } else {
        cli.domain
    };

    let invarients = invarient(cli.degree, 2, domain_size);

    for invarient in invarients.iter() {
        println!("invariant {}",invarient.iter().nth(0).unwrap().clone());
        for monomial in invarient.iter().take(10) {
            let mut indices: Vec<(u32, u32)> = Default::default();
            for var in monomial.variables.iter() {
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

            let monomial: Monomial2d = indices.into_iter().collect();
            let cannonical = monomial.clone().cannonical();
            println!("{monomial} {cannonical}");
        }
    }

    /*
    let mut sum: usize = 0;
    for invarient in invarients.iter() {
        println!("Invarients (len: {})", invarient.len());
        sum += invarient.len();
        println!("{}", invarient.iter().join(" + "));
    }
    println!("total size: {sum}");
    println!(
        "expected size: {}",
        choose((domain_size * domain_size) as u64, cli.degree as u64)
    );
    println!("num invarient: {}", invarients.len());
    */
}
