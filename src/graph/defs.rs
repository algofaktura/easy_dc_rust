use fxhash::FxBuildHasher;
use itertools::Itertools;
use ndarray::Array2;
use std::collections::{HashMap, HashSet, VecDeque};

use super::utils::{check_edge::is_valid_edge, modify::orient};

pub type FxHashMap<K, V> = HashMap<K, V, FxBuildHasher>;
pub type Adjacency = HashMap<Node, Nodes>;
pub type Bobbins = Vec<Node>;
pub type Count = usize;
pub type Edge = (Node, Node);
pub type Edges = HashSet<Edge>;
pub type EdgeAdjacency = HashMap<Edge, HashSet<Edge>>;
pub type Idx = Count;
pub type Loom = Vec<YarnEnds>;
pub type Neighbors = HashSet<Node>;
pub type Node = u32;
pub type Nodes = HashSet<Node>;
pub type Order = u32;
pub type Point = i16;
pub type Points = HashSet<Point>;
pub type Solution = Tour;
pub type Spool = HashMap<u32, Yarn>;
pub type Subtours = Vec<Tour>;
pub type Tour = Vec<Node>;
pub type TourSlice<'a> = &'a [Node];
pub type YarnEnds = VecDeque<Node>;
pub type V2d = [Point; 2];
pub type V2Slice<'a> = &'a [V2d];
pub type V3d = [Point; 3];
pub type Varr = Vec<[Point; 2]>;
pub type Vert = (Point, Point, Point);
pub type Verts = [Vert];
pub type VertsVec = Vec<Vert>;
pub type VIMap = HashMap<Vert, Node>;
pub type Warps = Subtours;
pub type Weights = HashMap<Node, Point>;
pub type SignedIdx = i32;
pub type Yarn = Array2<Point>;
pub type ZlevelNodesMap = HashMap<Point, Nodes>;
pub type ZOrder = Vec<(Point, usize)>;

#[derive(Clone, Debug)]
pub struct Weaver<'a> {
    pub data: Tour,
    verts: &'a Verts,
    adj: &'a Adjacency,
    lead: bool,
    max_xyz: Point,
    order: u32,
}

impl<'a> Weaver<'a> {
    pub fn new(
        mut data: YarnEnds,
        adj: &'a Adjacency,
        verts: &'a Verts,
        lead: bool,
        max_xyz: Point,
    ) -> Weaver<'a> {
        Weaver {
            data: data.drain(..).collect(),
            verts,
            adj,
            lead,
            max_xyz,
            order: verts.len() as u32,
        }
    }

    pub fn make_edges_for(&self, other_data: &Tour) -> Edges {
        other_data
            .iter()
            .circular_tuple_windows()
            .map(|(a, b)| orient(*a, *b))
            .filter(|&(m, n)| {
                is_valid_edge(
                    self.verts[m as usize],
                    self.verts[n as usize],
                    self.max_xyz,
                    self.order,
                    false,
                )
            })
            .collect()
    }

    pub fn join(&mut self, edge: Edge, oedge: Edge, other: &mut Tour) {
        self.rotated_to_edge(edge.0, edge.1);
        let reversed = !self.adj[&edge.1].contains(&oedge.0);
        Weaver::rotate_to_edge(
            other,
            if reversed { oedge.1 } else { oedge.0 },
            if reversed { oedge.0 } else { oedge.1 },
        );
        self.data.append(other);
    }

    pub fn rotated_to_edge(&mut self, lhs: u32, rhs: u32) {
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

    pub fn rotate_to_edge(other: &mut Tour, lhs: u32, rhs: u32) {
        if lhs == other[other.len() - 1] && rhs == other[0] {
            other.reverse();
        } else {
            match (
                other.iter().position(|&x| x == lhs).unwrap(),
                other.iter().position(|&x| x == rhs).unwrap(),
            ) {
                (idx_lhs, idx_rhs) if idx_lhs < idx_rhs => {
                    other.rotate_left(idx_rhs);
                    other.reverse()
                }
                (idx_lhs, _) => other.rotate_left(idx_lhs),
            }
        }
    }
    
    pub fn get_edges(&mut self) -> Edges {
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

    pub fn get_nodes(&self) -> Solution {
        self.data.to_vec()
    }

    pub fn get_vectors(&self) -> VertsVec {
        self.data
            .to_vec()
            .iter()
            .map(|node| self.verts[*node as usize])
            .collect()
    }
}
