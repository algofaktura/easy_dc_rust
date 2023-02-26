extern crate ndarray;
use ndarray::Array1;
use std::collections::HashMap;
use common_macros::hash_map;

pub type Adjacency<'a> = [(i32, &'a [i32]); 32];
pub type AdjacencyVects<'a> = [((i32, i32), &'a [(i32, i32)]); 32];
pub type Spool = HashMap<i32, Array1<i32>>;
pub type Vector = (i32, i32);
pub type AdjacencyMapV = HashMap<Vector, Vec<Vector>>;
pub type AdjacencyMap = HashMap<u32, Vec<u32>>;
pub type Edge = (i32, i32);

#[derive(Debug)]
pub enum Neighbors {
    Three([u32; 3]),
    Six([u32; 6])
}

pub type Adj = HashMap<u32, Neighbors>;


pub fn graph32() -> Adj {
    let graph = hash_map! {
        1 => Neighbors::Six([1, 2, 3, 4, 5, 6]),
        2 => Neighbors::Three([1, 2, 3]),
        3 => Neighbors::Six([1, 2, 3, 4, 5, 6]),
        4 => Neighbors::Three([1, 5, 6]),
    };
    graph
}