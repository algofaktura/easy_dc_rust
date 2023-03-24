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
pub type Neighbors = HashSet<[i16; 3]>;
pub type Node = [i16; 3];
pub type Nodes = HashSet<Node>;
pub type Order = u32;
pub type Point = i16;
pub type Points = HashSet<Point>;
pub type Solution = Tour;
pub type Spool = HashMap<u32, Yarn>;
pub type Subtours = Vec<Tour>;
pub type TourSlice<'a> = &'a [[i16; 2]];
pub type YarnEnds = VecDeque<Node>;
pub type Vert = (Point, Point, Point);
pub type Verts = [[i16; 3]];
pub type VecVert = Vec<Vert>;
pub type VIMap = HashMap<Vert, Node>;
pub type Weights = HashMap<Node, Point>;
pub type SignedIdx = i32;
pub type Yarn = Array2<Point>;
pub type ZlevelNodesMap = HashMap<Point, Nodes>;
pub type ZOrder = Vec<(Point, usize)>;

pub type Loom = Vec<VecDeque<[i16; 3]>>;
pub type Tour = Vec<[i16; 3]>;
pub type Warps = Vec<Vec<[i16; 3]>>;

#[derive(Clone, Debug)]
pub struct Weaver {
    pub data: Tour,
    lead: bool,
    min_xyz: Point,
    order: u32,
}

impl Weaver {
    pub fn new(mut data: YarnEnds, lead: bool, min_xyz: Point, order: u32) -> Weaver {
        Weaver {
            data: data.drain(..).collect(),
            lead,
            min_xyz,
            order,
        }
    }

    pub fn make_edges_for(&self, other_data: &Tour) -> Edges {
        other_data
            .iter()
            .circular_tuple_windows()
            .map(|(a, b)| orient(*a, *b))
            .filter(|&(m, n)| is_valid_edge(m, n, self.min_xyz, self.order, false))
            .collect()
    }

    pub fn join(&mut self, edge: Edge, wedge: Edge, warp: &mut Tour) {
        self.rotated_to_edge(edge);
        Weaver::rotate_to_edge(warp, wedge);
        self.data.append(warp);
    }

    pub fn rotated_to_edge(&mut self, (lhs, rhs): ([i16; 3], [i16; 3])) {
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

    pub fn rotate_to_edge(other: &mut Tour, (lhs, rhs): ([i16; 3], [i16; 3])) {
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
            .filter(|&(m, n)| is_valid_edge(m, n, self.min_xyz, self.order, self.lead))
            .collect()
    }

    pub fn get_nodes(&self) -> Solution {
        self.data.to_vec()
    }
}
