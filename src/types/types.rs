extern crate ndarray;
use ndarray::Array1;
use std::collections::HashMap;

pub type Adjacency<'a> = [(i32, &'a [i32]); 32];
pub type AdjacencyVects<'a> = [((i32, i32), &'a [(i32, i32)]); 32];
pub type Spool = HashMap<i32, Array1<i32>>;
pub type Vector = (i32, i32);
pub type AdjacencyMapV = HashMap<Vector, Vec<Vector>>;
pub type AdjacencyMap = HashMap<u32, Vec<u32>>;
pub type Edge = (i32, i32);

trait Degrees {
    type Three;
    type Six;
}

struct Neighbors;

impl Degrees for Neighbors {
    type Three = [i32; 3];
    type Six = [i32; 6];
}

pub fn tryit() {
    let degree3: <Neighbors as Degrees>::Three = [1, 2, 3];
    let degree6: <Neighbors as Degrees>::Six = [1, 2, 3, 1, 2, 3];
    println!("{:?} {:?}", degree3, degree6)
}