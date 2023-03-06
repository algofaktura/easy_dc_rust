use crate::{
    graphs::utils::{make::make_weights, map::convert_from_nodes},
    types::types::{Adjacency, Spool, Tour, Vert2dd, VertsC3, Weights, Yarn},
};

use super::{color::color, spin::spin};

pub fn spool_yarn(z_adj: &Adjacency, verts: &VertsC3, var: &[[i32; 3]]) -> Spool {
    let verts2dd: &Vert2dd = &verts
        .iter()
        .clone()
        .map(|&(x, y, _)| (x, y))
        .collect::<Vert2dd>();
    let weights: Weights = make_weights(z_adj, verts);
    let path: Tour = spin(&z_adj, &weights, var);
    let natural: Yarn = convert_from_nodes(path, &verts2dd);
    let colored: Yarn = color(&natural);
    Spool::from([(3, natural), (1, colored)])
}
