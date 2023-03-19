use itertools::Itertools;
use ndarray::{arr2, Array2};
use std::fmt;

use super::types::{
    Adjacency, Edge, EdgeAdjacency, Edges, Idx, Node, Nodes, Point, Points, SignedIdx, Solution,
    V2d, V3d, VIMap, Vert, Verts, Weights, ZOrder, ZlevelNodesMap,
};

// Output is a primitive type scalar.
pub mod info {
    use super::Itertools;
    use super::{Adjacency, Idx, Point, SignedIdx, Solution, V3d, Vert, Verts};

    pub fn absumv((x, y, z): Vert) -> Point {
        let abs_sum = [x, y, z].iter().fold(0, |acc, x| {
            let mask = x >> 15;
            acc + (x ^ mask) - mask
        });
        let sign_bit = abs_sum >> 15;
        (abs_sum ^ sign_bit) - sign_bit
    }

    pub fn absumv_v3d(v: V3d) -> Point {
        let abs_sum = v.iter().fold(0, |acc, x| {
            let mask = x >> 15;
            acc + (x ^ mask) - mask
        });
        let sign_bit = abs_sum >> 15;
        (abs_sum ^ sign_bit) - sign_bit
    }

    pub fn get_axis_3d(m_vert: &V3d, n_vert: &V3d) -> Idx {
        (0..3)
            .find(|&i| m_vert[i] != n_vert[i])
            .expect("Something's wrong, the same verts are being compared.")
    }

    pub fn get_axis(
        (a, b, c): (i16, i16, i16),
        (x, y, z): (i16, i16, i16),
    ) -> Result<usize, &'static str> {
        match (a != x, b != y, c != z) {
            (true, _, _edges_adjacency_map_from_adjacency) => Ok(0),
            (_, true, _) => Ok(1),
            (_, _, true) => Ok(2),
            _ => Err("The nodes aren't adjacent to each other."),
        }
    }

    pub fn get_max_xyz(order: u32) -> SignedIdx {
        ((0..order)
            .map(|n| (n, get_order_from_n(n)))
            .filter(|(_, sum)| *sum == order)
            .map(|(n, _)| n)
            .next()
            .unwrap()
            * 2
            - 1) as i32
    }

    pub fn get_order_from_n(n: u32) -> u32 {
        ((4.0 / 3.0) * ((n + 2) * (n + 1) * n) as f64).round() as u32
    }

    pub fn sum_neighbors(adj: &Adjacency) -> usize {
        adj.values().map(|value| value.len()).sum()
    }

    pub fn count_axes(solution: &Solution, vert: &Verts) -> [u32; 3] {
        solution
            .iter()
            .circular_tuple_windows()
            .fold([0, 0, 0], |mut axes, (m, n)| {
                let m_vert = vert[*m as usize];
                let n_vert = vert[*n as usize];
                axes[get_axis_3d(
                    &[m_vert.0, m_vert.1, m_vert.2],
                    &[n_vert.0, n_vert.1, n_vert.2],
                )] += 1;
                axes
            })
    }
}

pub mod shrink {
    use super::Itertools;
    use super::{Adjacency, Nodes, Point, Points, Verts, ZOrder, ZlevelNodesMap};

    pub fn shrink_adjacency(verts: &Verts, adj: &Adjacency) -> (Adjacency, ZOrder) {
        let stratified: ZlevelNodesMap = stratify_nodes(verts);
        (
            filter_adjacency(adj, stratified[&(-1 as Point)].clone()),
            get_zlevel_order(&stratified),
        )
    }

    fn stratify_nodes(verts: &Verts) -> ZlevelNodesMap {
        verts
            .iter()
            .map(|v| v.2)
            .filter(|&z| z < 0)
            .collect::<Points>()
            .into_iter()
            .map(|z| {
                let nodes = verts
                    .iter()
                    .enumerate()
                    .filter(|&(_, v)| v.2 as Point == z)
                    .map(|(i, _)| i as u32)
                    .collect::<Nodes>();
                (z, nodes)
            })
            .collect()
    }

    fn filter_adjacency(adj: &Adjacency, nodes: Nodes) -> Adjacency {
        adj.iter()
            .filter(|(k, _)| nodes.contains(k))
            .map(|(k, v)| (*k, v.intersection(&nodes).copied().collect()))
            .collect()
    }

    fn get_zlevel_order(stratified: &ZlevelNodesMap) -> ZOrder {
        stratified
            .iter()
            .map(|(&level, nodes)| (level, nodes.len()))
            .sorted_by_key(|&(level, _)| level)
            .collect()
    }
}

// Input and output are the same.
pub mod modify {

    use super::Point;
    use super::{arr2, Array2};
    pub fn orient(m: u32, n: u32) -> (u32, u32) {
        if m < n {
            (m, n)
        } else {
            (n, m)
        }
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
}

pub mod iters {
    pub fn uon(start: usize, end: usize, max_n: usize) -> impl Iterator<Item = usize> {
        (0..max_n + 2).filter_map(move |i| {
            let _uon = (0..max_n * 2 + 2)
                .step_by(2)
                .take(i)
                .map(|n| n * (n + 2))
                .sum();
            if _uon >= start && _uon <= end {
                Some(_uon)
            } else {
                None
            }
        })
    }
}

// Versions in which only xy where otherwise xyz is considered.
pub mod xy {
    use crate::graph::types::V3d;

    use super::{Idx, V2d, Vert};

    pub fn axis((x, y, _): &Vert, (x1, y1, _): &Vert) -> usize {
        (0..2)
            .find(|&i| [x, y][i] != [x1, y1][i])
            .expect("Something's wrong, the same verts are being compared.")
    }

    pub fn axisx(vert_m: &V3d, vert_n: &V3d) -> usize {
        (0..2)
            .find(|&i| vert_m[i] != vert_n[i])
            .expect("Something's wrong, the same verts are being compared.")
    }

    pub fn absumv((x, y, _): (i16, i16, i16)) -> i16 {
        let abs_sum = [x, y].iter().fold(0, |acc, x| {
            let mask = x >> 15;
            acc + (x ^ mask) - mask
        });
        let sign_bit = abs_sum >> 15;
        (abs_sum ^ sign_bit) - sign_bit
    }

    pub fn absumvx(vert: [i16;3]) -> i16 {
        let abs_sum = vert.iter().fold(0, |acc, x| {
            let mask = x >> 15;
            acc + (x ^ mask) - mask
        });
        let sign_bit = abs_sum >> 15;
        (abs_sum ^ sign_bit) - sign_bit
    }

    pub fn get_axis(m_vert: &V2d, n_vert: &V2d) -> Idx {
        (0..2)
            .find(|&i| m_vert[i] != n_vert[i])
            .expect("Something's wrong, the same verts are being compared.")
    }
}

// Version for eventual changing of type from i16 to i32.
pub mod version_i16 {

    pub fn abs(n: i16) -> i16 {
        let mask = n >> 15;
        (n + mask) ^ mask
    }

    pub fn sum(numbers: &[i16]) -> i16 {
        numbers.iter().fold(0, |acc, &num| {
            let sum = acc ^ num;
            let carry = (acc & num) << 1;
            sum + carry
        })
    }

    pub fn sumbit(numbers: &[i16]) -> i16 {
        numbers
            .iter()
            .fold(0, |acc, num| (acc.wrapping_add(num & 0x7FFF)) & 0x7FFF)
    }
}

pub mod check {
    use super::{Point, Vert};

    pub fn is_valid_edge(v1: Vert, v2: Vert, max_xyz: Point, order: u32, lead: bool) -> bool {
        if order < 160 {
            return valid_edge(v1, v2);
        }
        match lead {
            true => valid_main_edge(v1, v2, max_xyz),
            false => valid_other_edge(v1, v2, max_xyz),
        }
    }

    pub fn valid_edge((x1, y1, _): Vert, (x2, y2, _): Vert) -> bool {
        matches!(x1 + y1 + x2 + y2, 4..=10)
    }

    pub fn valid_main_edge((x, y, z): Vert, (x2, y2, z2): Vert, max_xyz: Point) -> bool {
        let lowest = max_xyz - 4;
        if z.abs() == lowest && lowest == z2.abs() {
            (x == 1 || x == 3) && y == y2 && y2 == 1 && (x2 == 1 || x2 == 3)
        } else {
            x == x2 && x2 == 1 && y == y2 && y2 == 1
        }
    }

    pub fn valid_other_edge((x, y, z): Vert, (x2, y2, z2): Vert, max_xyz: Point) -> bool {
        let lowest = max_xyz - 4;
        if z.abs() == lowest && lowest == z2.abs() {
            (x == 1 || x == 3) && y == y2 && y2 == 3 && (x2 == 1 || x2 == 3)
        } else {
            x == x2 && x2 == 3 && y == y2 && y2 == 1
        }
    }
}

pub mod check_v3 {
    use crate::graph::types::V3d;

    use super::Point;

    pub fn is_valid_edge(v1: V3d, v2: V3d, max_xyz: Point, order: u32, lead: bool) -> bool {
        if order < 160 {
            return valid_edgex(v1, v2);
        }
        match lead {
            true => valid_main_edgex(v1, v2, max_xyz),
            false => valid_other_edgex(v1, v2, max_xyz),
        }
    }

    pub fn valid_edgex([x1, y1, _]: V3d, [x2, y2, _]: V3d) -> bool {
        matches!(x1 + y1 + x2 + y2, 4..=10)
    }

    pub fn valid_main_edgex([x, y, z]: V3d, [x2, y2, z2]: V3d, max_xyz: Point) -> bool {
        let lowest = max_xyz - 4;
        if z.abs() == lowest && lowest == z2.abs() {
            (x == 1 || x == 3) && y == y2 && y2 == 1 && (x2 == 1 || x2 == 3)
        } else {
            x == x2 && x2 == 1 && y == y2 && y2 == 1
        }
    }

    pub fn valid_other_edgex([x, y, z]: V3d, [x2, y2, z2]: V3d, max_xyz: Point) -> bool {
        let lowest = max_xyz - 4;
        if z.abs() == lowest && lowest == z2.abs() {
            (x == 1 || x == 3) && y == y2 && y2 == 3 && (x2 == 1 || x2 == 3)
        } else {
            x == x2 && x2 == 3 && y == y2 && y2 == 1
        }
    }
}

pub mod debug {
    use super::{Edge, Vert};

    pub fn show_edge_vectors(
        (m, n): Edge,
        (o, p): Edge,
        verts: &[Vert],
    ) -> Vec<(String, Vert, Vert)> {
        vec![
            (
                "main_edge".to_string(),
                verts[m as usize],
                verts[n as usize],
            ),
            (
                "other_edge".to_string(),
                verts[o as usize],
                verts[p as usize],
            ),
        ]
    }
}

pub mod certify {
    use crate::graph::types::Vix;

    use super::fmt;
    use super::Itertools;
    use super::{Adjacency, Solution};

    #[derive(Debug, PartialEq)]
    pub enum SequenceID {
        Broken,
        HamChain,
        HamCycle,
    }

    impl fmt::Display for SequenceID {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                SequenceID::Broken => write!(f, "Broken"),
                SequenceID::HamChain => write!(f, "HamChain"),
                SequenceID::HamCycle => write!(f, "HamCycle"),
            }
        }
    }

    pub fn id_seq(seq: &Solution, adj: &Adjacency) -> SequenceID {
        if seq.iter().duplicates().count() > 0 || seq.len() != adj.len() {
            return SequenceID::Broken;
        }
        match seq
            .windows(2)
            .all(|window| adj[&window[0]].contains(&window[1]))
        {
            true if adj[&seq[seq.len() - 1]].contains(&seq[0]) => SequenceID::HamCycle,
            true => SequenceID::HamChain,
            false => SequenceID::Broken,
        }
    }

    pub fn id_seqx(seq: &Solution, vertx: &Vix) -> SequenceID {
        if seq.iter().duplicates().count() > 0 || seq.len() != vertx.len() {
            return SequenceID::Broken;
        }
        match seq
            .windows(2)
            .all(|window| vertx[window[0] as usize].contains(&window[1]))
        {
            true if vertx[seq[seq.len() - 1] as usize].contains(&seq[0]) => SequenceID::HamCycle,
            true => SequenceID::HamChain,
            false => SequenceID::Broken,
        }
    }
}

pub mod maker {
    use super::info::absumv;
    use super::Itertools;
    use super::{Adjacency, Edge, EdgeAdjacency, Edges, Idx, Node, Verts, Weights};
    use rayon::prelude::*;

    use super::{check::valid_edge, modify::orient};

    pub fn get_adjacent_edges(adj: &Adjacency, (m_node, n_node): Edge, verts: &Verts) -> Edges {
        adj[&m_node]
            .iter()
            .flat_map(|m| adj[&n_node].iter().map(move |n| (*m, *n)))
            .filter(|(m, n)| adj[m].contains(n) && valid_edge(verts[*m as Idx], verts[*n as Idx]))
            .map(|(m, n)| orient(m, n))
            .collect()
    }

    pub fn make_edges(data: Vec<u32>, verts: &Verts) -> Edges {
        data.iter()
            .circular_tuple_windows()
            .map(|(a, b)| orient(*a, *b))
            .filter(|&(a, b)| valid_edge(verts[a as usize], verts[b as usize]))
            .collect()
    }

    fn _get_adjacent_edges(adj: &Adjacency, m_node: Node, n_node: Node, verts: &Verts) -> Edges {
        adj[&m_node]
            .iter()
            .flat_map(|m| adj[&n_node].iter().map(move |n| (*m, *n)))
            .filter(|(m, n)| adj[m].contains(n) && valid_edge(verts[*m as Idx], verts[*n as Idx]))
            .map(|(m, n)| orient(m, n))
            .collect()
    }

    fn _edges_adjacency_map_from_adjacency(adj: &Adjacency, verts: &Verts) -> EdgeAdjacency {
        adj.iter()
            .flat_map(|(k, v)| v.iter().map(move |&i| (*k, i)))
            .filter_map(|(m, n)| {
                if valid_edge(verts[m as usize], verts[n as usize]) {
                    Some((orient(m, n), _get_adjacent_edges(adj, m, n, verts)))
                } else {
                    None
                }
            })
            .collect()
    }

    fn _edges_adjacency_map_from_edges(
        adj: &Adjacency,
        edges: &Edges,
        verts: &Verts,
    ) -> EdgeAdjacency {
        edges
            .par_iter()
            .filter(|&(a, b)| valid_edge(verts[*a as Idx], verts[*b as Idx]))
            .map(|&(m, n)| (orient(m, n), _get_adjacent_edges(adj, m, n, verts)))
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
}

pub mod get_adj_edges {
    use super::{Edge, Edges, VIMap};

    pub fn create_edges2(
        (a, b, c): (i16, i16, i16),
        (x, y, z): (i16, i16, i16),
        max_xyz: i16,
        vi_map: &VIMap,
    ) -> Edges {
        // 16.710316
        match (a != x, b != y, c != z) {
            (true, false, false) => &[[0, 2, 0], [0, -2, 0], [0, 0, 2], [0, 0, -2]],
            (false, true, false) => &[[2, 0, 0], [-2, 0, 0], [0, 0, 2], [0, 0, -2]],
            (false, false, true) => &[[2, 0, 0], [-2, 0, 0], [0, 2, 0], [0, -2, 0]],
            _ => panic!("NOT A VALID EDGE"),
        }
        .iter()
        .filter_map(|[i, j, k]| {
            get_valid_edge(
                (a + i, b + j, c + k),
                (x + i, y + j, z + k),
                max_xyz,
                vi_map,
            )
        })
        .collect()
    }

    pub fn get_valid_edge(
        (x, y, z): (i16, i16, i16),
        (a, b, c): (i16, i16, i16),
        max_xyz: i16,
        vi_map: &VIMap,
    ) -> Option<Edge> {
        let lowest = max_xyz - 4; // furthest axis value from origin.
        if z.abs() == lowest
            && lowest == c.abs()
            && (x == 1 || x == 3)
            && y == b
            && b == 1
            && (a == 1 || a == 3)
            || x == a && a == 1 && y == b && b == 1
        {
            Some((vi_map[&(x, y, z)], vi_map[&(a, b, c)]))
        } else {
            None
        }
    }

    pub fn get_valid_eadj(
        (x, y, z): (i16, i16, i16),
        (a, b, c): (i16, i16, i16),
        max_xyz: i16,
        vi_map: &VIMap,
    ) -> Option<Edge> {
        let lowest = max_xyz - 4; // furthest axis value from origin.
        if z.abs() == lowest
            && lowest == c.abs()
            && (x == 1 || x == 3)
            && y == b
            && b == 3
            && (a == 1 || a == 3)
            || x == a && a == 3 && y == b && b == 1
        {
            Some((vi_map[&(x, y, z)], vi_map[&(a, b, c)]))
        } else {
            None
        }
    }

    pub fn create_eadjs(
        (a, b, c): (i16, i16, i16),
        (x, y, z): (i16, i16, i16),
        max_xyz: i16,
        vi_map: &VIMap,
    ) -> Edges {
        // 11.868491
        // writing out the steps definitely yields improvements.
        let mut new_edges = Edges::new();
        match (a != x, b != y, c != z) {
            (true, false, false) => {
                // Y_EDGE and Z_EDGE
                if let Some(edge) = get_valid_eadj((a, b + 2, c), (x, y + 2, z), max_xyz, vi_map) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_eadj((a, b - 2, c), (x, y - 2, z), max_xyz, vi_map) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_eadj((a, b, c + 2), (x, y, z + 2), max_xyz, vi_map) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_eadj((a, b, c - 2), (x, y, z - 2), max_xyz, vi_map) {
                    new_edges.insert(edge);
                }
            }
            (false, true, false) => {
                // X_EDGE and Z_EDGE
                if let Some(edge) = get_valid_eadj((a + 2, b, c), (x + 2, y, z), max_xyz, vi_map) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_eadj((a - 2, b, c), (x - 2, y, z), max_xyz, vi_map) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_eadj((a, b, c + 2), (x, y, z + 2), max_xyz, vi_map) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_eadj((a, b, c - 2), (x, y, z - 2), max_xyz, vi_map) {
                    new_edges.insert(edge);
                }
            }
            (false, false, true) => {
                // X_EDGE and Y_EDGE
                if let Some(edge) = get_valid_eadj((a + 2, b, c), (x + 2, y, z), max_xyz, vi_map) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_eadj((a - 2, b, c), (x - 2, y, z), max_xyz, vi_map) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_eadj((a, b + 2, c), (x, y + 2, z), max_xyz, vi_map) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_eadj((a, b - 2, c), (x, y - 2, z), max_xyz, vi_map) {
                    new_edges.insert(edge);
                }
            }
            _ => {} // The nodes aren't adjacent to each other.
        }
        new_edges
    }

    pub fn create_edges(
        (a, b, c): (i16, i16, i16),
        (x, y, z): (i16, i16, i16),
        max_xyz: i16,
        vi_map: &VIMap,
    ) -> Edges {
        // 11.868491
        // writing out the steps definitely yields improvements.
        let mut new_edges = Edges::new();
        match (a != x, b != y, c != z) {
            (true, false, false) => {
                // Y_EDGE and Z_EDGE
                if let Some(edge) = get_valid_edge((a, b + 2, c), (x, y + 2, z), max_xyz, vi_map) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_edge((a, b - 2, c), (x, y - 2, z), max_xyz, vi_map) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_edge((a, b, c + 2), (x, y, z + 2), max_xyz, vi_map) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_edge((a, b, c - 2), (x, y, z - 2), max_xyz, vi_map) {
                    new_edges.insert(edge);
                }
            }
            (false, true, false) => {
                // X_EDGE and Z_EDGE
                if let Some(edge) = get_valid_edge((a + 2, b, c), (x + 2, y, z), max_xyz, vi_map) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_edge((a - 2, b, c), (x - 2, y, z), max_xyz, vi_map) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_edge((a, b, c + 2), (x, y, z + 2), max_xyz, vi_map) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_edge((a, b, c - 2), (x, y, z - 2), max_xyz, vi_map) {
                    new_edges.insert(edge);
                }
            }
            (false, false, true) => {
                // X_EDGE and Y_EDGE
                if let Some(edge) = get_valid_edge((a + 2, b, c), (x + 2, y, z), max_xyz, vi_map) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_edge((a - 2, b, c), (x - 2, y, z), max_xyz, vi_map) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_edge((a, b + 2, c), (x, y + 2, z), max_xyz, vi_map) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_edge((a, b - 2, c), (x, y - 2, z), max_xyz, vi_map) {
                    new_edges.insert(edge);
                }
            }
            _ => {} // The nodes aren't adjacent to each other.
        }
        new_edges
    }
}

pub mod get_adj_edgesx {
    use crate::graph::types::{V3d, Vix};

    use super::{Edge, Edges, modify::orient};

    pub fn create_edges(
        [a, b, c]: V3d,
        [x, y, z]: V3d,
        max_xyz: i16,
        vertx: &Vix,
    ) -> Edges {
        // 16.710316
        match (a != x, b != y, c != z) {
            (true, false, false) => &[[0, 2, 0], [0, -2, 0], [0, 0, 2], [0, 0, -2]],
            (false, true, false) => &[[2, 0, 0], [-2, 0, 0], [0, 0, 2], [0, 0, -2]],
            (false, false, true) => &[[2, 0, 0], [-2, 0, 0], [0, 2, 0], [0, -2, 0]],
            _ => panic!("NOT A VALID EDGE"),
        }
        .iter()
        .filter_map(|[i, j, k]| {
            get_valid_edgex(
                [a + i, b + j, c + k],
                [x + i, y + j, z + k],
                max_xyz,
                vertx,
            )
        })
        .collect()
    }

    pub fn get_valid_edgex(
        [a, b, c]: V3d,
        [x, y, z]: V3d,
        max_xyz: i16,
        vertx: &Vix,
    ) -> Option<Edge> {
        let lowest = max_xyz - 4; // furthest axis value from origin.
        if z.abs() == lowest
            && lowest == c.abs()
            && (x == 1 || x == 3)
            && y == b
            && b == 1
            && (a == 1 || a == 3)
            || x == a && a == 1 && y == b && b == 1
        {
            Some(orient(vertx.get_index_of(&[a, b, c]).unwrap() as u32, vertx.get_index_of(&[x, y, z]).unwrap() as u32))
        } else {
            None
        }
    }

    pub fn get_valid_eadjx(
        [a, b, c]: V3d,
        [x, y, z]: V3d,
        max_xyz: i16,
        vertx: &Vix,
    ) -> Option<Edge> {
        let lowest = max_xyz - 4; // furthest axis value from origin.
        if z.abs() == lowest
            && lowest == c.abs()
            && (x == 1 || x == 3)
            && y == b
            && b == 3
            && (a == 1 || a == 3)
            || x == a && a == 3 && y == b && b == 1
        {
            Some(orient(vertx.get_index_of(&[a, b, c]).unwrap() as u32, vertx.get_index_of(&[x, y, z]).unwrap() as u32))
        } else {
            None
        }
    }

    pub fn create_eadjsx(
        [a, b, c]: V3d,
        [x, y, z]: V3d,
        max_xyz: i16,
        vertx: &Vix,
    ) -> Edges {
        // 11.868491
        // writing out the steps definitely yields improvements.
        let mut new_edges = Edges::new();
        match (a != x, b != y, c != z) {
            (true, false, false) => {
                // Y_EDGE and Z_EDGE
                if let Some(edge) = get_valid_eadjx([a, b + 2, c], [x, y + 2, z], max_xyz, vertx) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_eadjx([a, b - 2, c], [x, y - 2, z], max_xyz, vertx) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_eadjx([a, b, c + 2], [x, y, z + 2], max_xyz, vertx) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_eadjx([a, b, c - 2], [x, y, z - 2], max_xyz, vertx) {
                    new_edges.insert(edge);
                }
            }
            (false, true, false) => {
                // X_EDGE and Z_EDGE
                if let Some(edge) = get_valid_eadjx([a + 2, b, c], [x + 2, y, z], max_xyz, vertx) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_eadjx([a - 2, b, c], [x - 2, y, z], max_xyz, vertx) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_eadjx([a, b, c + 2], [x, y, z + 2], max_xyz, vertx) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_eadjx([a, b, c - 2], [x, y, z - 2], max_xyz, vertx) {
                    new_edges.insert(edge);
                }
            }
            (false, false, true) => {
                // X_EDGE and Y_EDGE
                if let Some(edge) = get_valid_eadjx([a + 2, b, c], [x + 2, y, z], max_xyz, vertx) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_eadjx([a - 2, b, c], [x - 2, y, z], max_xyz, vertx) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_eadjx([a, b + 2, c], [x, y + 2, z], max_xyz, vertx) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_eadjx([a, b - 2, c], [x, y - 2, z], max_xyz, vertx) {
                    new_edges.insert(edge);
                }
            }
            _ => {} // The nodes aren't adjacent to each other.
        }
        new_edges
    }

    pub fn create_edgesx(
        [a, b, c]: V3d,
        [x, y, z]: V3d,
        max_xyz: i16,
        vertx: &Vix,
    ) -> Edges {
        // 11.868491
        // writing out the steps definitely yields improvements.
        let mut new_edges = Edges::new();
        match (a != x, b != y, c != z) {
            (true, false, false) => {
                // Y_EDGE and Z_EDGE
                if let Some(edge) = get_valid_edgex([a, b + 2, c], [x, y + 2, z], max_xyz, vertx) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_edgex([a, b - 2, c], [x, y - 2, z], max_xyz, vertx) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_edgex([a, b, c + 2], [x, y, z + 2], max_xyz, vertx) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_edgex([a, b, c - 2], [x, y, z - 2], max_xyz, vertx) {
                    new_edges.insert(edge);
                }
            }
            (false, true, false) => {
                // X_EDGE and Z_EDGE
                if let Some(edge) = get_valid_edgex([a + 2, b, c], [x + 2, y, z], max_xyz, vertx) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_edgex([a - 2, b, c], [x - 2, y, z], max_xyz, vertx) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_edgex([a, b, c + 2], [x, y, z + 2], max_xyz, vertx) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_edgex([a, b, c - 2], [x, y, z - 2], max_xyz, vertx) {
                    new_edges.insert(edge);
                }
            }
            (false, false, true) => {
                // X_EDGE and Y_EDGE
                if let Some(edge) = get_valid_edgex([a + 2, b, c], [x + 2, y, z], max_xyz, vertx) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_edgex([a - 2, b, c], [x - 2, y, z], max_xyz, vertx) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_edgex([a, b + 2, c], [x, y + 2, z], max_xyz, vertx) {
                    new_edges.insert(edge);
                }
                if let Some(edge) = get_valid_edgex([a, b - 2, c], [x, y - 2, z], max_xyz, vertx) {
                    new_edges.insert(edge);
                }
            }
            _ => {} // The nodes aren't adjacent to each other.
        }
        new_edges
    }
}
