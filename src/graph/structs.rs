use itertools::Itertools;

use super::{
    make::is_valid_edge,
    types::{Adjacency, Edge, EdgeAdjacency, Edges, Solution, Thread, Tour, Verts},
    utils::orient,
};

#[derive(Clone, Debug)]
pub struct Cycle<'a> {
    pub data: Tour,
    verts: &'a Verts,
    adj: &'a Adjacency,
    edge_adj: &'a EdgeAdjacency,
    _eadjs: Option<Edges>
}

impl<'a> Cycle<'a> {
    pub fn new(
        data: &Thread,
        adj: &'a Adjacency,
        edge_adj: &'a EdgeAdjacency,
        verts: &'a Verts,
    ) -> Box<Cycle<'a>> {
        let cycle = Cycle {
            data: data.iter().cloned().collect::<Tour>(),
            verts,
            adj,
            edge_adj,
            _eadjs: None
        };
        Box::new(cycle)
    }

    pub fn eadjs(&mut self) -> Edges {
        if self._eadjs.is_none() {
            self._eadjs = Some(self.make_eadjs());
        }
        self._eadjs.clone().unwrap()
    }

    pub fn make_eadjs(&mut self) -> Edges {
        self.edges()
            .iter()
            .flat_map(|edge| self.edge_adj[edge].iter())
            .copied()
            .collect()
    }

    pub fn edges(&mut self) -> Edges {
        self.data
            .iter()
            .circular_tuple_windows()
            .map(|(a, b)| orient(*a, *b))
            .filter(|&(a, b)| is_valid_edge(self.verts[a as usize], self.verts[b as usize]))
            .collect()
    }

    pub fn join(&mut self, edge: Edge, oedge: Edge, other: &mut Cycle) {
        self.rotate_to_edge(edge.0, edge.1);
        let reversed = !self.adj[&edge.1].contains(&oedge.0);
        other.rotate_to_edge(
            if reversed { oedge.1 } else { oedge.0 },
            if reversed { oedge.0 } else { oedge.1 },
        );
        self.data.extend(&other.data);
    }

    pub fn rotate_to_edge(&mut self, lhs: u32, rhs: u32) {
        if lhs == self.data[self.data.len() - 1] && rhs == self.data[0] {
            self.data.reverse();
        } else {
            match (
                self.data.iter().position(|&x| x == lhs).unwrap(),
                self.data.iter().position(|&x| x == rhs).unwrap(),
            ) {
                (idx_lhs, idx_rhs) if idx_lhs < idx_rhs => {
                    self.data.rotate_left(idx_rhs);
                    self.data.reverse()
                }
                (idx_lhs, _) => self.data.rotate_left(idx_lhs),
            }
        }
    }

    pub fn retrieve_nodes(&self) -> Solution {
        self.data.to_vec()
    }

    pub fn retrieve_vectors(&self) -> Verts {
        self.data
            .to_vec()
            .iter()
            .map(|node| self.verts[*node as usize])
            .collect()
    }
}
