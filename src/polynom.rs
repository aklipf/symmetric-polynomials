use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use itertools::Itertools;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Index {
    Named(String),
    Constant(u32),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Variable {
    pub name: String,
    pub indices: Vec<Index>,
}

#[derive(Debug, Clone, Hash)]
pub struct Monoid {
    pub variables: Vec<Variable>,
}

impl PartialEq for Monoid {
    fn eq(&self, other: &Self) -> bool {
        for var in self.variables.iter() {
            if !other.variables.contains(var) {
                return false;
            }
        }
        true
    }
}

impl Eq for Monoid {}

impl FromIterator<Variable> for Monoid {
    fn from_iter<T: IntoIterator<Item = Variable>>(iter: T) -> Self {
        let mut variables: Vec<Variable> = iter.into_iter().collect();
        variables.sort();
        Monoid { variables }
    }
}

impl Variable {
    pub fn new<T: Into<Index>, U: Into<String>, I: IntoIterator<Item = T>>(
        name: U,
        indices: I,
    ) -> Self {
        Variable {
            name: name.into(),
            indices: indices.into_iter().map(|x| x.into()).collect(),
        }
    }

    pub fn eval(&self, assignement: &HashMap<String, u32>) -> Self {
        Variable {
            name: self.name.clone(),
            indices: self
                .indices
                .iter()
                .map(|x| match x {
                    Index::Named(name) => Index::Constant(*assignement.get(name).unwrap()),
                    Index::Constant(idx) => Index::Constant(*idx),
                })
                .collect(),
        }
    }
}

impl From<String> for Index {
    fn from(name: String) -> Self {
        Index::Named(name)
    }
}

impl From<&str> for Index {
    fn from(name: &str) -> Self {
        Index::Named(name.to_string())
    }
}

impl From<u32> for Index {
    fn from(value: u32) -> Self {
        Index::Constant(value)
    }
}

impl Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Index::Named(name) => write!(f, "{}", name),
            Index::Constant(value) => write!(f, "{}", value),
        }
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_{{", self.name)?;
        for index in self.indices.iter() {
            write!(f, "{index}")?;
        }
        write!(f, "}}")
    }
}

impl Display for Monoid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for var in self.variables.iter() {
            write!(f, "{var}")?;
        }
        std::fmt::Result::Ok(())
    }
}

impl Monoid {
    pub fn is_multilinear(&self) -> bool {
        for (i, var) in self.variables.iter().enumerate() {
            if self.variables[i + 1..].contains(var) {
                return false;
            }
        }
        true
    }

    pub fn eval(&self, assignement: &HashMap<String, u32>) -> Self {
        let mut variables: Vec<Variable> = self
            .variables
            .iter()
            .map(|var| var.eval(assignement))
            .collect();
        variables.sort();
        Monoid { variables }
    }

    pub fn orbital(&self, domain_size: u32) -> HashSet<Monoid> {
        let mut orbit: HashSet<Monoid> = Default::default();
        let indices = self.collect_indices();

        for values in (0..domain_size).permutations(indices.len()) {
            let assignement: HashMap<String, u32> =
                indices.iter().cloned().zip(values.into_iter()).collect();
            orbit.insert(self.eval(&assignement));
        }

        orbit
    }

    fn collect_indices(&self) -> Vec<String> {
        let mut indices: Vec<String> = Default::default();
        for var in self.variables.iter() {
            for idx in var.indices.iter() {
                match idx {
                    Index::Named(name) => {
                        if !indices.contains(name) {
                            indices.push(name.clone());
                        }
                    }
                    _ => {}
                }
            }
        }
        indices
    }
}
