use std::iter::zip;

use super::{
    check::is_valid_edge,
    types::{Adjacency, Edge, EdgeAdjacency, Edges, Solution, Thread, Tour, Verts},
};

#[derive(Clone, Debug)]
pub struct Cycle<'a> {
    pub data: Tour,
    prev: Tour,
    _eadjs: Edges,
    _edges: Edges,
    verts: &'a Verts,
    adj: &'a Adjacency,
    edge_adj: &'a EdgeAdjacency,
    pub is_empty: bool,
}

impl<'a> Cycle<'a> {
    pub fn new(
        data: &Thread,
        adj: &'a Adjacency,
        edge_adj: &'a EdgeAdjacency,
        verts: &'a Verts,
    ) -> &'a mut Cycle<'a> {
        let cycle = Cycle {
            data: data.iter().cloned().collect::<Tour>(),
            prev: Tour::new(),
            _eadjs: Edges::new(),
            _edges: Edges::new(),
            verts,
            adj,
            edge_adj,
            is_empty: false,
        };
        Box::leak(Box::new(cycle))
    }
    
    pub fn retrieve(&self) -> Solution {
        self.data.iter().cloned().collect()
    }

    pub fn rotate_to_edge(&mut self, left: u32, right: u32) {
        if left == self.data[self.data.len() - 1] && right == self.data[0] {
            self.data.reverse();
        } else {
            match (
                self.data.iter().position(|&x| x == left).unwrap(),
                self.data.iter().position(|&x| x == right).unwrap(),
            ) {
                (ixl, ixr) if ixl < ixr => {
                    self.data.rotate_left(ixr);
                    self.data.reverse()
                }
                (ixl, _) => self.data.rotate_left(ixl),
            }
        }
    }

    pub fn join(&mut self, edge: Edge, oedge: Edge, other: &mut Cycle) {
        self.rotate_to_edge(edge.0, edge.1);
        let reversed = !self.adj[&edge.1].contains(&oedge.0);
        other.rotate_to_edge(
            if reversed { oedge.1 } else { oedge.0 },
            if reversed { oedge.0 } else { oedge.1 },
        );
        self.data.extend(&other.data);
        other.data.clear();
        other.is_empty = true;
    }

    pub fn make_edges(&self) -> Edges {
        zip(
            self.data.clone(),
            [&self.data[1..], &self.data[..1]].concat(),
        )
        .map(|(a, b)| if a < b { (a, b) } else { (b, a) })
        .collect()
    }

    pub fn eadjs(&mut self) -> Edges {
        self.edges()
            .iter()
            .flat_map(|edge| self.edge_adj[edge].iter())
            .map(|&ea| ea)
            .collect()
    }

    pub fn edges(&mut self) -> Edges {
        if self.prev != self.data {
            self._edges = zip(
                self.data.clone(),
                [&self.data[1..], &self.data[..1]].concat(),
            )
            .into_iter()
            .map(|(a, b)| if a < b { (a, b) } else { (b, a) })
            .filter(|&(a, b)| is_valid_edge(self.verts[a as usize], self.verts[b as usize]))
            .collect();
            self.prev = self.data.clone()
        }
        self._edges.clone()
    }
}
