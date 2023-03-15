use itertools::Itertools;
use ndarray::{arr2, Array2};
use rayon::prelude::*;

use super::{
    shrink,
    types::{
        Adjacency, EdgeAdjacency, Edges, Idx, Node, Nodes, Point, V3d, VIMap, Vert, Verts, Weights,
        ZOrder,
    },
    utils::{absumv, orient},
};

pub fn make_graph(
    n: u32,
) -> (
    u32,
    u32,
    Verts,
    VIMap,
    Adjacency,
    EdgeAdjacency,
    Adjacency,
    ZOrder,
) {
    let order = get_order_from_n(n);
    let max_xyz = get_max_xyz(order as i32);
    let verts: Verts = vertices(max_xyz);
    let vi_map: VIMap = vi_map(&verts);
    let adj: Adjacency = adjacency_map(&verts, max_xyz, &vi_map);
    let edge_adj: EdgeAdjacency = edges_adjacency_map_from_adjacency(&adj, &verts);
    let (z_adj, z_order) = shrink::adjacency(&verts, &adj);
    (n, order, verts, vi_map, adj, edge_adj, z_adj, z_order)
}

fn get_max_xyz(order: i32) -> Point {
    (0..order)
        .map(|n| (n, get_order_from_n(n as u32)))
        .filter(|(_, sum)| *sum == order as u32)
        .map(|(n, _)| n)
        .next()
        .unwrap()
        * 2
        - 1
}

fn get_order_from_n(n: u32) -> u32 {
    ((4.0 / 3.0) * ((n + 2) * (n + 1) * n) as f64).round() as u32
}

pub fn vertices(max_xyz: Point) -> Verts {
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
                        .collect::<Verts>()
                })
                .collect::<Verts>()
        })
        .sorted_by_key(|(x, y, z)| (absumv_v3d([*x, *y, *z]), *x, *y))
        .collect()
}

pub fn absumv_v3d(v: V3d) -> Point {
    let abs_sum = v.iter().fold(0, |acc, x| {
        let mask = x >> 31;
        acc + (x ^ mask) - mask
    });
    let sign_bit = abs_sum >> 31;
    (abs_sum ^ sign_bit) - sign_bit
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

fn shift_xyz(vert: Array2<Point>) -> Array2<Point> {
    vert + arr2(&[
        [2, 0, 0],
        [-2, 0, 0],
        [0, 2, 0],
        [0, -2, 0],
        [0, 0, 2],
        [0, 0, -2],
    ])
}

fn get_adjacent_edges(adj: &Adjacency, m_node: Node, n_node: Node, verts: &Verts) -> Edges {
    adj[&m_node]
        .iter()
        .flat_map(|m| adj[&n_node].iter().map(move |n| (*m, *n)))
        .filter(|(m, n)| adj[m].contains(n) && is_valid_edge(verts[*m as Idx], verts[*n as Idx]))
        .map(|(m, n)| orient(m, n))
        .collect()
}

fn edges_adjacency_map_from_adjacency(adj: &Adjacency, verts: &Verts) -> EdgeAdjacency {
    adj.iter()
        .flat_map(|(k, v)| v.iter().map(move |&i| (*k, i)))
        .filter_map(|(m, n)| {
            if is_valid_edge(verts[m as usize], verts[n as usize]) {
                Some((orient(m, n), get_adjacent_edges(adj, m, n, verts)))
            } else {
                None
            }
        })
        .collect()
}

pub fn is_valid_edge((x1, y1, _): Vert, (x2, y2, _): Vert) -> bool {
    matches!(
        (x1 & 0xFFFF) + (y1 & 0xFFFF) + (x2 & 0xFFFF) + (y2 & 0xFFFF),
        4..=10
    )
}

pub fn is_valid_edge2((x1, y1, _): Vert, (x2, y2, _): Vert) -> bool {
    matches!(
        (
            (x1 & 0xFFFF) + (y1 & 0xFFFF) + (x2 & 0xFFFF) + (y2 & 0xFFFF),
            (x1 >> 31) + (y1 >> 31) + (x2 >> 31) + (y2 >> 31)
        ),
        (4..=10, 0)
    )
}

fn _edges_adjacency_map_from_edges(adj: &Adjacency, edges: &Edges, verts: &Verts) -> EdgeAdjacency {
    edges
        .par_iter()
        .filter(|&(a, b)| is_valid_edge(verts[*a as Idx], verts[*b as Idx]))
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
