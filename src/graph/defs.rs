use itertools::Itertools;
use ndarray::Array2;
use std::collections::{HashMap, HashSet, VecDeque};

use super::utils::{check_edge::is_valid_edge, modify::orient};

pub type Adjacency = HashMap<Node, Neighbors>;
pub type ZAdjacency = HashMap<[Point; 2], Vec<[Point; 2]>>;
pub type Bobbins = Vec<Node>;
pub type Count = usize;
pub type Edge = (Node, Node);
pub type Edges = HashSet<Edge>;
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
pub type TourSliceThick<'a> = &'a [[i16; 2]];
pub type YarnEnds = VecDeque<Node>;
pub type Vert = (Point, Point, Point);
pub type Verts = [Vert];
pub type VecVert = Vec<Vert>;
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
    lead: bool,
    min_xyz: Point,
    order: u32,
}

impl<'a> Weaver<'a> {
    pub fn new(
        mut data: YarnEnds,
        verts: &'a Verts,
        lead: bool,
        min_xyz: Point,
    ) -> Weaver<'a> {
        Weaver {
            data: data.drain(..).collect(),
            verts,
            lead,
            min_xyz,
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
                    self.min_xyz,
                    self.order,
                    false,
                )
            })
            .collect()
    }

    pub fn join(&mut self, edge: Edge, wedge: Edge, warp: &mut Tour) {
        self.rotated_to_edge(edge);
        Weaver::rotate_to_edge(warp, wedge);
        self.data.append(warp);
    }

    pub fn rotated_to_edge(&mut self, (lhs, rhs): (u32, u32)) {
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

    pub fn rotate_to_edge(other: &mut Tour, (lhs, rhs): (u32, u32)) {
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

    pub fn edges(&mut self) -> Edges {
        self.data
            .iter()
            .circular_tuple_windows()
            .map(|(a, b)| orient(*a, *b))
            .filter(|&(m, n)| {
                is_valid_edge(
                    self.verts[m as usize],
                    self.verts[n as usize],
                    self.min_xyz,
                    self.order,
                    self.lead,
                )
            })
            .collect()
    }

    pub fn get_nodes(&self) -> Solution {
        self.data.to_vec()
    }

    pub fn get_vectors(&self) -> VecVert {
        self.data
            .to_vec()
            .iter()
            .map(|node| self.verts[*node as usize])
            .collect()
    }
}
