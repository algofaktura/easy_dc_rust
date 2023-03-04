use crate::{types::types::{Adjacency, Path, Spool, Vert2dd, VertsC3, Weights, Yarn}, graphs::utils::{map::convert_from_nodes, make::make_weights}};

use super::{spin::spin, color::color};

pub fn spool_yarn(z_adj: &Adjacency, verts: &VertsC3, var: &[[i32; 3]]) -> Spool {
    let verts2dd: &Vert2dd = &verts.iter().clone().map(|&(x, y, _)| (x, y)).collect::<Vert2dd>();
    let weights: Weights = make_weights(z_adj, verts);
    let path: Path = spin(&z_adj, &weights, var);
    let natural: Yarn = convert_from_nodes(path, &verts2dd);
    let colored: Yarn = color(&natural);
    Spool::from([(3, natural), (1, colored)])
}