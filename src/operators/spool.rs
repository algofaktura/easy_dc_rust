use ndarray::arr2;

use super::spin::spin;

use crate::{
    graphs::utils::{make::make_weights, map::convert_from_nodes},
    types::types::{Adjacency, Spool, Tour, Vert2dd, VertsC3, Weights, Yarn},
};

pub fn spool_yarn(z_adj: &Adjacency, verts: &VertsC3, var: &[[i32; 3]]) -> Spool {
    let verts2dd: &Vert2dd = &make_verts2dd(verts);
    let weights: Weights = make_weights(z_adj, verts);
    let path: Tour = spin(&z_adj, &weights, var);
    let natural: Yarn = convert_from_nodes(path, verts2dd);
    let colored: Yarn = color(&natural);
    Spool::from([(3, natural), (1, colored)])
}

pub fn make_verts2dd(verts: &VertsC3) -> Vert2dd {
    verts.iter().clone().map(|&(x, y, _)| (x, y)).collect()
}

pub fn color(a: &Yarn) -> Yarn {
    a.clone().dot(&arr2(&[[-1, 0], [0, -1]])) + arr2(&[[0, 2]])
}

pub fn reflect(a: &Yarn) -> Yarn {
    a.clone().dot(&arr2(&[[-1, 0], [0, -1]]))
}

pub fn shift(a: Yarn) -> Yarn {
    a + arr2(&[[0, 2]])
}