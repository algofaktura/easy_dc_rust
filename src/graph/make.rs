use itertools::Itertools;
use ndarray::arr2;
use rayon::prelude::*;

use super::{
    types::{Adjacency, Node, Nodes, Point, VIMap, Verts, VertsVec, ZOrder, Vix},
    utils::{
        info::{absumv_v3d, get_max_xyz, get_order_from_n},
        modify::shift_xyz,
        shrink::shrink_adjacency,
    },
};

pub fn make_graph(n: u32) -> (u32, u32, VertsVec, VIMap, Adjacency, Adjacency, ZOrder, i16) {
    let order = get_order_from_n(n);
    let max_xyz = get_max_xyz(order) as i16;
    let verts: VertsVec = vertices(max_xyz);
    let vi_map: VIMap = vi_map(&verts);
    let adj: Adjacency = adjacency_map(&verts, max_xyz, &vi_map);
    let (z_adj, z_order) = shrink_adjacency(&verts, &adj);
    (n, order, verts, vi_map, adj, z_adj, z_order, max_xyz)
}

pub fn make_graphx(n: u32) -> (u32, u32, Vix, Adjacency, ZOrder, i16) {
    let order = get_order_from_n(n);
    let max_xyz = get_max_xyz(order) as i16;
    let verts: VertsVec = vertices(max_xyz);
    let vi_map: VIMap = vi_map(&verts);
    let adj: Adjacency = adjacency_map(&verts, max_xyz, &vi_map);
    let vertx = vi_map_mix(&verts, &adj);
    let (z_adj, z_order) = shrink_adjacency(&verts, &adj);
    (n, order, vertx, z_adj, z_order, max_xyz)
}

pub fn vertices(max_xyz: Point) -> VertsVec {
    (-(max_xyz)..=(max_xyz))
        .step_by(2)
        .flat_map(|x| {
            (-max_xyz..=max_xyz)
                .step_by(2)
                .flat_map(move |y| {
                    (-max_xyz..=max_xyz)
                        .step_by(2)
                        .map(move |z| (x, y, z))
                        .filter(|&v| absumv_v3d([v.0, v.1, v.2]) < (max_xyz + 4))
                        .collect::<VertsVec>()
                })
                .collect::<VertsVec>()
        })
        .sorted_by_key(|(x, y, z)| (absumv_v3d([*x, *y, *z]), *x, *y))
        .collect()
}

fn vi_map(verts: &Verts) -> VIMap {
    verts
        .par_iter()
        .enumerate()
        .map(|(idx, vert)| (*vert, idx as Node))
        .collect()
}

pub fn vi_map_mix(verts: &Verts, adj: &Adjacency) -> Vix {
    // a hybrid of a vi map and a vertices list. 
    verts
        .iter()
        .enumerate()
        .map(|(idx, (x, y, z))| ([*x, *y, *z], adj[&(idx as u32)].clone()))
        .collect()
}

fn adjacency_map(verts: &Verts, max_xyz: Point, vi: &VIMap) -> Adjacency {
    verts
        .par_iter()
        .enumerate()
        .map(|(idx, vert)| {
            (
                idx as Node,
                shift_xyz(arr2(&[[vert.0, vert.1, vert.2]]))
                    .outer_iter()
                    .filter(|new_vert| {
                        absumv_v3d([new_vert[0], new_vert[1], new_vert[2]]) <= max_xyz + 2
                    })
                    .map(|new_vert| vi[&(new_vert[0], new_vert[1], new_vert[2])])
                    .filter(|&m| m != (idx as Node))
                    .collect::<Nodes>(),
            )
        })
        .collect()
}
