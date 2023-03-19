use itertools::Itertools;

use super::{
    types::{Adjacency, Edge, Edges, Point, Solution, Tour, VertN, Verts, VertsVec, YarnEnds},
    utils::{check::is_valid_edge, modify::orient},
};

#[derive(Clone, Debug)]
pub struct Cycle<'a> {
    pub data: Tour,
    verts: &'a Verts,
    adj: &'a Adjacency,
    lead: bool,
    max_xyz: Point,
    order: u32,
}

impl<'a> Cycle<'a> {
    pub fn new(
        data: YarnEnds,
        adj: &'a Adjacency,
        verts: &'a Verts,
        lead: bool,
        max_xyz: Point,
    ) -> Cycle<'a> {
        Cycle {
            data: data.into_iter().collect::<Tour>(),
            verts,
            adj,
            lead,
            max_xyz,
            order: verts.len() as u32,
        }
    }

    pub fn make_edges(&mut self) -> Edges {
        self.data
            .iter()
            .circular_tuple_windows()
            .map(|(a, b)| orient(*a, *b))
            .filter(|&(m, n)| {
                is_valid_edge(
                    self.verts[m as usize],
                    self.verts[n as usize],
                    self.max_xyz,
                    self.order,
                    self.lead,
                )
            })
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
        other.data = Vec::with_capacity(1);
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

    pub fn retrieve_vectors(&self) -> VertsVec {
        self.data
            .to_vec()
            .iter()
            .map(|node| self.verts[*node as usize])
            .collect()
    }
}

pub struct Cyclex<'a> {
    pub data: Tour,
    vertn: &'a VertN,
    lead: bool,
    max_xyz: Point,
    order: u32,
}

impl<'a> Cyclex<'a> {
    pub fn new(data: YarnEnds, vertn: &'a VertN, lead: bool, max_xyz: Point) -> Cyclex<'a> {
        Cyclex {
            data: data.into_iter().collect::<Tour>(),
            vertn,
            lead,
            max_xyz,
            order: vertn.len() as u32,
        }
    }

    pub fn make_edges(&mut self) -> Edges {
        self.data
            .iter()
            .circular_tuple_windows()
            .map(|(a, b)| orient(*a, *b))
            .filter(|&(m, n)| {
                is_valid_edge(
                    self.vertn[m as usize].0,
                    self.vertn[n as usize].0,
                    self.max_xyz,
                    self.order,
                    self.lead,
                )
            })
            .collect()
    }

    pub fn join(&mut self, edge: Edge, oedge: Edge, other: &mut Cyclex) {
        self.rotate_to_edge(edge.0, edge.1);
        let reversed = !self.vertn[edge.1 as usize].1.contains(&oedge.0);
        other.rotate_to_edge(
            if reversed { oedge.1 } else { oedge.0 },
            if reversed { oedge.0 } else { oedge.1 },
        );
        self.data.extend(&other.data);
        other.data = Vec::with_capacity(1);
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

    pub fn retrieve_vectors(&self) -> VertsVec {
        self.data
            .to_vec()
            .iter()
            .map(|node| self.vertn[*node as usize].0)
            .collect()
    }
}
