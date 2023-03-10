use itertools::Itertools;
use std::fmt;

use super::types::{Adjacency, Solution, Vert};

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
    if seq.iter().duplicates().count() > 0 || seq.len() != adj.len() {
        return SequenceID::Broken;
    }
    match seq
        .windows(2)
        .all(|window| adj[&window[0]].contains(&window[1]))
    {
        true if adj[&seq[seq.len() - 1]].contains(&seq[0]) => SequenceID::HamCycle,
        true => SequenceID::HamChain,
        false => SequenceID::Broken,
    }
}

pub fn is_valid_edge((x1, y1, _): Vert, (x2, y2, _): Vert) -> bool {
    match (x1 & 0xFFFF) + (y1 & 0xFFFF) + (x2 & 0xFFFF) + (y2 & 0xFFFF) {
        4..=10 => true,
        _ => false,
    }
}
