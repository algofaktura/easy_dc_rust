use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct Cycle<'a> {
    data: Vec<u32>,
    joined: bool,
    adj: &'a HashMap<u32, HashSet<u32>>,
}

impl Cycle<'_> {
    pub fn new<'a>(data: Vec<u32>, adj: &'a HashMap<u32, HashSet<u32>>) -> Cycle {
        Cycle {
            data,
            joined: false,
            adj
        }
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
            .zip([&self.data[1..], &[self.data[0]]].concat().iter())
            .map(|(&a, &b)| if a < b { (a, b) } else { (b, a) })
            .collect()
    }

    pub fn eadjs(&self) -> HashMap<(u32, u32), HashSet<(u32, u32)>> {
        let e_adjs: HashMap<(u32, u32), HashSet<(u32, u32)>> = HashMap::new();
        e_adjs
    }
}