use ndarray::Array2;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
};

use super::structs::Cycle;

pub type Adjacency = HashMap<Node, Nodes>;
pub type AdjC<'a> = [(Node, &'a [Node])];
pub type Bobbins = Vec<Node>;
pub type Count = usize;
pub type Done = HashSet<usize>;
pub type Edge = (Node, Node);
pub type Edges = HashSet<Edge>;
pub type EdgeAdjacency = HashMap<Edge, HashSet<Edge>>;
pub type Idx = Count;
pub type Idxs = Vec<Idx>;
pub type Loom = Vec<Thread>;
pub type Neighbors = HashSet<Node>;
pub type Node = u32;
pub type Nodes = HashSet<Node>;
pub type Order = u32;
pub type Point = i32;
pub type Points = HashSet<Point>;
pub type Solution = Tour;
pub type Spool = HashMap<u32, Yarn>;
pub type Subtours = Vec<Tour>;
pub type Tour = Vec<Node>;
pub type TourSlice<'a> = &'a [Node];
pub type Thread = VecDeque<Node>;
pub type V2d = [Point; 2];
pub type V2Slice<'a> = &'a [V2d];
pub type V3d = [i32; 3];
pub type Varr = Vec<[i32; 2]>;
pub type Vert = (i32, i32, i32);
pub type Verts = Vec<Vert>;
pub type Vert3d = (Point, Point, Point);
pub type Vert3dd = Vec<Vert3d>;
pub type VIMap = HashMap<Vert, Node>;
pub type WarpedLoom<'a> = HashMap<usize, RefCell<&'a mut Cycle<'a>>>;
pub type Warps = Subtours;
pub type Weights = HashMap<Node, i32>;
pub type Woven = Vec<usize>;
pub type Yarn = Array2<Point>;
pub type ZlevelNodesMap = HashMap<Point, Nodes>;
pub type ZOrder = Vec<(Point, usize)>;
