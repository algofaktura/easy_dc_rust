use std::iter::zip;

use crate::types::types::{Adjacency, Edge, EdgeAdjacency, Edges, Solution, Thread, Tour, VertsC3};

#[derive(Clone, Debug)]
pub struct Cycle<'a> {
    data: Tour,
    prev: Tour,
    _eadjs: Edges,
    _edges: Edges,
    verts: &'a VertsC3,
    adj: &'a Adjacency,
    edge_adj: &'a EdgeAdjacency,
}

impl<'a> Cycle<'a> {
    pub fn new(
        data: &Thread,
        adj: &'a Adjacency,
        edge_adj: &'a EdgeAdjacency,
        verts: &'a VertsC3,
    ) -> &'a mut Cycle<'a> {
        let cycle = Cycle {
            data: data.iter().cloned().collect::<Tour>(),
            prev: Tour::new(),
            _eadjs: Edges::new(),
            _edges: Edges::new(),
            verts,
            adj,
            edge_adj,
        };
        Box::leak(Box::new(cycle))
    }

    pub fn new_from_vec(
        data: &Tour,
        adj: &'a Adjacency,
        edge_adj: &'a EdgeAdjacency,
        verts: &'a VertsC3,
    ) -> &'a mut Cycle<'a> {
        let cycle = Cycle {
            data: data.iter().cloned().collect::<Tour>(),
            prev: Tour::new(),
            _eadjs: Edges::new(),
            _edges: Edges::new(),
            verts,
            adj,
            edge_adj,
        };
        Box::leak(Box::new(cycle))
    }

    pub fn retrieve(&self) -> Solution {
        self.data.iter().cloned().collect::<Vec<u32>>()
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

    pub fn join(&mut self, edge: Edge, oedge: Edge, other: &mut Cycle) {
        self.rotate_to_edge(edge.0, edge.1);
        let reversed = !self.adj.get(&edge.1).unwrap().contains(&oedge.0);
        other.rotate_to_edge(
            if reversed { oedge.1 } else { oedge.0 },
            if reversed { oedge.0 } else { oedge.1 },
        );
        self.data.extend(&other.data);
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
            .flat_map(|edge| self.edge_adj.get(edge).unwrap().iter())
            .map(|&ea| ea)
            .filter(|&(a, b)| is_valid_edge(self.verts[a as usize], self.verts[b as usize]))
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

    pub fn from<'b>(
        vecdata: Thread,
        adj: &'a Adjacency,
        edge_adj: &'a EdgeAdjacency,
        verts: &'a VertsC3,
    ) -> Cycle<'a> {
        Cycle {
            data: vecdata.into_iter().collect::<Tour>(),
            prev: Tour::new(),
            _eadjs: Edges::new(),
            _edges: Edges::new(),
            verts,
            adj,
            edge_adj,
        }
    }
}

fn is_valid_edge((x1, y1, _): (i32, i32, i32), (x2, y2, _): (i32, i32, i32)) -> bool {
    let total = (x1 & 0xFFFF) + (y1 & 0xFFFF) + (x2 & 0xFFFF) + (y2 & 0xFFFF);
    (4 <= total) && (total <= 10)
}
