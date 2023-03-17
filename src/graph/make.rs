use itertools::Itertools;
use ndarray::arr2;
use rayon::prelude::*;

use super::{
    shrink,
    types::{
        Adjacency, EdgeAdjacency, Edges, Idx, Node, Nodes, Point, VIMap, Verts, VertsVec, Weights,
        ZOrder,
    },
    utils::{
        check::valid_edge,
        info::{absumv, absumv_v3d, get_max_xyz, get_order_from_n},
        modify::{orient, shift_xyz},
    },
};

pub fn make_graph(
    n: u32,
) -> (
    u32,
    u32,
    VertsVec,
    VIMap,
    Adjacency,
    EdgeAdjacency,
    Adjacency,
    ZOrder,
    i32,
) {
    let order = get_order_from_n(n);
    let max_xyz = get_max_xyz(order as i32);
    let verts: VertsVec = vertices(max_xyz);
    let vi_map: VIMap = vi_map(&verts);
    let adj: Adjacency = adjacency_map(&verts, max_xyz, &vi_map);
    let edge_adj: EdgeAdjacency = edges_adjacency_map_from_adjacency(&adj, &verts);
    let (z_adj, z_order) = shrink::adjacency(&verts, &adj);
    (
        n, order, verts, vi_map, adj, edge_adj, z_adj, z_order, max_xyz,
    )
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

fn get_adjacent_edges(adj: &Adjacency, m_node: Node, n_node: Node, verts: &Verts) -> Edges {
    adj[&m_node]
        .iter()
        .flat_map(|m| adj[&n_node].iter().map(move |n| (*m, *n)))
        .filter(|(m, n)| adj[m].contains(n) && valid_edge(verts[*m as Idx], verts[*n as Idx]))
        .map(|(m, n)| orient(m, n))
        .collect()
}

fn edges_adjacency_map_from_adjacency(adj: &Adjacency, verts: &Verts) -> EdgeAdjacency {
    adj.iter()
        .flat_map(|(k, v)| v.iter().map(move |&i| (*k, i)))
        .filter_map(|(m, n)| {
            if valid_edge(verts[m as usize], verts[n as usize]) {
                Some((orient(m, n), get_adjacent_edges(adj, m, n, verts)))
            } else {
                None
            }
        })
        .collect()
}

fn _edges_adjacency_map_from_edges(adj: &Adjacency, edges: &Edges, verts: &Verts) -> EdgeAdjacency {
    edges
        .par_iter()
        .filter(|&(a, b)| valid_edge(verts[*a as Idx], verts[*b as Idx]))
        .map(|&(m, n)| (orient(m, n), get_adjacent_edges(adj, m, n, verts)))
        .collect()
}

fn _edges_from_adjacency(adj: &Adjacency) -> Edges {
    adj.iter()
        .flat_map(|(k, v)| v.iter().map(move |&i| (*k, i)))
        .collect()
}

fn _weights_map(adj: &Adjacency, verts: &Verts) -> Weights {
    adj.par_iter()
        .map(|(&n, _)| (n, absumv(verts[n as usize])))
        .collect()
}
