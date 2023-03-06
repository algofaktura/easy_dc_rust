use itertools::Itertools;
use std::{collections::HashSet, fmt};

use crate::types::types::{Adjacency, Solution};

#[derive(Debug, PartialEq)]
pub enum SequenceID {
    Broken,
    HamChain,
    HamCycle,
}

impl fmt::Display for SequenceID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SequenceID::Broken => write!(f, "Broken"),
            SequenceID::HamChain => write!(f, "HamChain"),
            SequenceID::HamCycle => write!(f, "HamCycle"),
        }
    }
}

pub fn id_seq(seq: &Solution, adj: &Adjacency) -> SequenceID {
    if seq.iter().duplicates().count() > 0 {
        return SequenceID::Broken;
    }
    match seq
        .windows(2)
        .all(|window| adj.get(&window[0]).unwrap().contains(&window[1]))
    {
        true if adj.get(&seq[seq.len() - 1]).unwrap().contains(&seq[0]) => SequenceID::HamCycle,
        true => SequenceID::HamChain,
        false => SequenceID::Broken,
    }
}

pub fn id_seq1(seq: &Solution, adj: &Adjacency) -> SequenceID {
    assert!(seq.len() == seq.iter().cloned().collect::<HashSet<_>>().len());
    for i in 1..seq.len() {
        if !adj.get(&seq[i - 1]).unwrap().contains(&seq[i]) {
            return SequenceID::Broken;
        }
    }
    if adj.get(&seq[seq.len() - 1]).unwrap().contains(&seq[0]) {
        return SequenceID::HamCycle;
    }
    SequenceID::HamChain
}
