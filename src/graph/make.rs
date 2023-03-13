use itertools::Itertools;
use ndarray::{arr2, Array2};
use rayon::prelude::*;

use super::{
    check::is_valid_edge,
    shrink,
    types::{
        Adjacency, EdgeAdjacency, Edges, Idx, Node, Nodes, Point, V3d, VIMap, Vert, Verts,
        Weights, ZOrder,
    },
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
    let edge_adj: EdgeAdjacency = edges_adjacency_mapping(&adj, &verts);
    let (z_adj, z_order) = shrink::adjacency(&verts, &adj);
    (n, order, verts, vi_map, adj, edge_adj, z_adj, z_order)
}

pub fn get_order_from_n(n: u32) -> u32 {
    ((4.0 / 3.0) * (n as f64 + 2.0) * (n as f64 + 1.0) * n as f64).round() as u32
}

pub fn get_max_xyz(order: i32) -> Point {
    (0..order)
        .map(|n| {
            (
                n,
                ((4.0 / 3.0) * (n as f64 + 2.0) * (n as f64 + 1.0) * n as f64).round() as u32,
            )
        })
        .filter(|(_, sum)| *sum == order as u32)
        .map(|(n, _)| n)
        .next()
        .unwrap()
        * 2
        - 1
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
                        .filter(|&v| absumv(v) < (max_xyz + 4))
                        .collect::<Verts>()
                })
                .collect::<Verts>()
        })
        .into_iter()
        .sorted_by_key(|v| (edist(*v), v.0, v.1, v.2))
        .collect()
}

pub fn absumv((x, y, z): Vert) -> Point {
    [x, y, z]
        .iter()
        .map(|&n| ((n >> 31) ^ n).wrapping_sub(n >> 31))
        .sum()
}

pub fn edist((x, y, z): Vert) -> Point {
    ((x.pow(2) + y.pow(2) + z.pow(2)) as f32).sqrt().round() as i32
}

pub fn vi_map(verts: &Verts) -> VIMap {
    verts
        .par_iter()
        .enumerate()
        .map(|(idx, vert)| (*vert, idx as Node))
        .collect()
}

pub fn adjacency_map(verts: &Verts, max_xyz: Point, vi: &VIMap) -> Adjacency {
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

pub fn shift_xyz(vert: Array2<Point>) -> Array2<Point> {
    vert + arr2(&[
        [2, 0, 0],
        [-2, 0, 0],
        [0, 2, 0],
        [0, -2, 0],
        [0, 0, 2],
        [0, 0, -2],
    ])
}

pub fn absumv_v3d(vert: V3d) -> Point {
    vert.iter()
        .map(|&n| ((n >> 31) ^ n).wrapping_sub(n >> 31))
        .sum()
}

pub fn edges_from_adjacency(adj: &Adjacency) -> Edges {
    adj.iter()
        .flat_map(|(k, v)| v.iter().map(move |&i| (*k, i)))
        .collect()
}

pub fn edges_adjacency_map(adj: &Adjacency, edges: &Edges, verts: &Verts) -> EdgeAdjacency {
    edges
        .par_iter()
        .filter(|&(a, b)| is_valid_edge(verts[*a as Idx], verts[*b as Idx]))
        .map(|&(m, n)| (orient(m, n), get_adjacent_edges(adj, m, n, verts)))
        .collect()
}

pub fn edges_adjacency_mapping(adj: &Adjacency, verts: &Verts) -> EdgeAdjacency {
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

fn get_adjacent_edges(adj: &Adjacency, m_node: Node, n_node: Node, verts: &Verts) -> Edges {
    adj[&m_node]
        .iter()
        .flat_map(|m| adj[&n_node].iter().map(move |n| (*m, *n)))
        .filter(|(m, n)| adj[m].contains(n) && is_valid_edge(verts[*m as Idx], verts[*n as Idx]))
        .map(|(m, n)| orient(m, n))
        .collect()
}

pub fn get_adjacent_edgesvec(adj: &Adjacency, m_node: Node, n_node: Node, verts: &Verts) -> Edges {
    adj[&m_node]
        .iter()
        .flat_map(|m| adj[&n_node].iter().map(move |n| (*m, *n)))
        .filter(|(m, n)| adj[m].contains(n) && is_valid_edge(verts[*m as Idx], verts[*n as Idx]))
        .map(|(m, n)| orient(m, n))
        .collect()
}

fn orient<T: std::cmp::PartialOrd>(m: T, n: T) -> (T, T) {
    if m < n {
        (m, n)
    } else {
        (n, m)
    }
}

pub fn weights_map(adj: &Adjacency, verts: &Verts) -> Weights {
    adj.par_iter()
        .map(|(&n, _)| (n, absumv(verts[n as usize])))
        .collect()
}
