use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum SequenceID {
    Broken,
    HamChain,
    HamCycle
}

impl fmt::Display for SequenceID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SequenceID::Broken => write!(f, "Broken"),
            SequenceID::HamChain => write!(f, "Chain"),
            SequenceID::HamCycle => write!(f, "Cycle"),
        }
    }
}

pub fn id_seq(seq: &[u32], adj: &HashMap<u32, HashSet<u32>>) -> SequenceID {
    for i in 1..seq.len() {
        if !adj.get(&seq[i-1]).unwrap().contains(&seq[i]) {
            return SequenceID::Broken;
        }
    }
    if adj.get(&seq[seq.len() - 1]).unwrap().contains(&seq[0]) {
        return SequenceID::HamCycle;
    }
    SequenceID::HamChain
}
