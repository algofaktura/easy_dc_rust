use std::collections::{HashMap, HashSet, VecDeque};

use crate::types::types::*;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Cycle<'a> {
    data: Vec<u32>,
    joined: bool,
    last: bool,
    lead: bool, 
    prev: Path,
    prev2: Path,
    _eadjs: Edges,
    _edges: Edges,
    adj: &'a Adjacency,
    edge_adj: &'a EdgeAdjacency
}

impl<'a> Cycle<'a> {
    pub fn new(data: &Thread, adj: &'a Adjacency, edge_adj: &'a EdgeAdjacency, lead: bool) -> &'a mut Cycle<'a> {
        let cycle = Cycle {
            data: data.iter().cloned().collect::<Path>(),
            joined: false,
            last: false,
            lead,
            prev: Vec::new(),
            prev2: Vec::new(),
            _eadjs: HashSet::new(),
            _edges: HashSet::new(),
            adj,
            edge_adj
        };
        Box::leak(Box::new(cycle))
    }

    pub fn new_from_vec(data: &Path, adj: &'a Adjacency, edge_adj: &'a EdgeAdjacency, lead: bool) -> &'a mut Cycle<'a> {
        let cycle = Cycle {
            data: data.iter().cloned().collect::<Path>(),
            joined: false,
            last: false,
            lead,
            prev: Vec::new(),
            prev2: Vec::new(), 
            _eadjs: HashSet::new(),
            _edges: HashSet::new(),
            adj,
            edge_adj
        };
        Box::leak(Box::new(cycle))
    }

    pub fn retrieve(&self) -> Vec<u32> {
        self.data.iter().cloned().collect::<Vec<u32>>()
    }

    pub fn set_last(&mut self) {
        self.last = true;
    }

    pub fn set_lead(&mut self) {
        self.lead = true;
    }

    pub fn rotate_to_edge(&mut self, left: u32, right: u32) {
        if left == self.data[self.data.len() - 1] && right == self.data[0] {
            self.data.reverse();
        } else {
            let idx_left = self.data.iter().position(|&x| x == left).unwrap();
            let idx_right = self.data.iter().position(|&x| x == right).unwrap();
            if idx_left > idx_right {
                self.data.rotate_left(idx_left);
            } else {
                self.data.rotate_left(idx_right);
                self.data.reverse()
            }
        }
    }

    pub fn join(&mut self, edge: (u32, u32), oedge: (u32, u32), other: &mut Cycle) {
        self.rotate_to_edge(edge.0, edge.1);
        let neighs = self.adj.get(&edge.1).unwrap();
        let mut o_edge = (oedge.0, oedge.1);
        if !neighs.contains(&oedge.0) {
            o_edge = (oedge.1, oedge.0);
        }
        other.rotate_to_edge(o_edge.0, o_edge.1);
        self.data.extend(&other.data);
        self.joined = true;
    }
        
    pub fn edges(&self) -> HashSet<(u32, u32)> {
        self.data
            .iter()
            .zip([&self.data[1..], &self.data[..1]].concat().iter())
            .map(|(&a, &b)| if a < b { (a, b) } else { (b, a) })
            .collect()
    }
    
    pub fn eadjs(&self) -> HashSet<(u32, u32)> {
        self.edges()
            .iter()
            .flat_map(|edge| self.edge_adj.get(edge).unwrap().iter())
            .map(|&ea| ea)
            .collect()
    }    
    
    // pub fn edges(&mut self) -> &HashSet<(u32, u32)> {
    //     if self.prev2 != self.data {
    //         self.prev2 = self.data.clone();
    //         self._edges = self.data
    //             .iter()
    //             .zip([&self.data[1..], &self.data[..1]].concat().iter())
    //             .map(|(&a, &b)| if a < b { (a, b) } else { (b, a) })
    //             .collect()
    //     }
    //     &self._edges
    // }
    
    // pub fn eadjs(&mut self) -> &HashSet<(u32, u32)> {
    //     if self.prev != self.data {
    //         self.prev = self.data.clone();
    //         self._eadjs = self.edges().clone()
    //             .iter()
    //             .flat_map(|edge| self.edge_adj.get(edge).unwrap())
    //             .cloned()
    //             .collect()
    //     }
    //     &self._eadjs
    // }

    pub fn from<'b>(vecdata: VecDeque<u32>, adj: &'a HashMap<u32, HashSet<u32>>, edge_adj: &'a HashMap<(u32, u32), HashSet<(u32, u32)>>) -> Cycle<'a> {
        Cycle {
            data: vecdata.into_iter().collect::<Vec<u32>>(),
            joined: false,
            last: false,
            lead: false,
            prev: Vec::new(),
            prev2: Vec::new(),
            _eadjs: HashSet::new(),
            _edges: HashSet::new(),
            adj,
            edge_adj
        }
    }
}