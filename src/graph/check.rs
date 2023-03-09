use itertools::Itertools;
use std::fmt;

use super::types::{Adjacency, Solution, Verti16};

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

pub fn is_valid_edge((x1, y1, _): (i32, i32, i32), (x2, y2, _): (i32, i32, i32)) -> bool {
    let total = (x1 & 0xFFFF) + (y1 & 0xFFFF) + (x2 & 0xFFFF) + (y2 & 0xFFFF);
    (4 <= total) && (total <= 10)
}

pub fn is_valid_edge_i16((x1, y1, _): Verti16, (x2, y2, _): Verti16) -> bool {
    let total = x1 + y1 + x2 + y2;
    (4 <= total) && (total <= 10)
}
