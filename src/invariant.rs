use clap::Parser;

#[derive(Parser,Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[arg(short='n', long, default_value="0")]
    domain: u32,
    #[arg(short, long)]
    degree: u32,
}

use std::collections::HashSet;

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

        if !monoid.is_multilinear(){
            continue;
        }

        let orbital = monoid.orbital(domain_size);

        if orbital.len()>0 && (!invarients.contains(&orbital)) {
            invarients.push(orbital);
        }
    }
    invarients
}

/*
function to check if two monoid are on the same orbital
cannonical orderging of a monomial?

*/

fn main() {
    let cli = Args::parse();

    let domain_size = if cli.domain==0{
        cli.degree*2
    }else{
        cli.domain
    };

    let invarients = invarient(cli.degree, 2, domain_size);

    let mut sum:usize=0;
    for invarient in invarients.iter(){
        println!("Invarients (len: {})",invarient.len());
        sum+=invarient.len();
        println!("{}",invarient.iter().join(" + "));
    }
    println!("total size: {sum}");
    println!("expected size: {}",choose((domain_size * domain_size) as u64, cli.degree as u64));
    println!("num invarient: {}",invarients.len());

}
