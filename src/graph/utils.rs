use itertools::Itertools;
use ndarray::{arr2, Array2};
use std::fmt;

use super::types::{
    Adjacency, Edge, Edges, Idx, Nodes, Point, Points, SignedIdx, Solution, V2d, V3d, Vert, Verts,
    ZOrder, ZlevelNodesMap,
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
    use super::{Idx, V2d, Vert};

    pub fn axis((x, y, _): &Vert, (x1, y1, _): &Vert) -> usize {
        (0..2)
            .find(|&i| [x, y][i] != [x1, y1][i])
            .expect("Something's wrong, the same verts are being compared.")
    }

    pub fn absumv((x, y, z): (i16, i16, i16)) -> i16 {
        let abs_sum = [x, y, z].iter().fold(0, |acc, x| {
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
}

pub mod maker {
    use super::Itertools;
    use super::{Adjacency, Edge, Edges, Idx, Verts};

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
}
