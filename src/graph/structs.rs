use itertools::Itertools;

use super::{
    types::{Adjacency, Edge, Edges, Point, Solution, Tour, Verts, VertsVec, YarnEnds, Vix},
    utils::{check::is_valid_edge, modify::orient, self},
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
            .filter(
                |&(m, n)| 
                is_valid_edge(
                    self.verts[m as usize],
                    self.verts[n as usize],
                    self.max_xyz,
                    self.order,
                    self.lead,
                )
            )
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


#[derive(Clone, Debug)]
pub struct Cyclex<'a> {
    pub data: Tour,
    vertx: &'a Vix,
    lead: bool,
    max_xyz: Point,
    order: u32,
}

impl<'a> Cyclex<'a> {
    pub fn new(
        data: YarnEnds,
        vertx: &'a Vix,
        lead: bool,
        max_xyz: Point,
    ) -> Cyclex<'a> {
        Cyclex {
            data: data.into_iter().collect::<Tour>(),
            vertx,
            lead,
            max_xyz,
            order: vertx.len() as u32,
        }
    }

    pub fn make_edges(&mut self) -> Edges {
        self.data
            .iter()
            .circular_tuple_windows()
            .map(|(a, b)| orient(*a, *b))
            .filter(
                |&(m, n)| 
                utils::check_v3::is_valid_edge(
                    *self.vertx.get_index(m as usize).unwrap().0,
                    *self.vertx.get_index(n as usize).unwrap().0,
                    self.max_xyz,
                    self.order,
                    self.lead,
                )
            )
            .collect()
    }

    pub fn join(&mut self, edge: Edge, oedge: Edge, other: &mut Cyclex) {
        self.rotate_to_edge(edge.0, edge.1);
        let reversed = !self.vertx[edge.1 as usize].contains(&oedge.0);
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

    pub fn retrieve_vectors(&self) -> Vec<&[i16; 3]> {
        self.data
            .to_vec()
            .iter()
            .map(|node| self.vertx.get_index(*node as usize).unwrap().0)
            .collect()
    }
}
