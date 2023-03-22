use itertools;
use itertools::Itertools;
use ndarray::{arr2, Array2};
use rayon;
use std::fmt;

use super::defs::{
    Adjacency, Edge, Edges, Node, Nodes, Point, Points, SignedIdx, Solution, V3d, VIMap, Vert,
    Verts, VertsVec, ZOrder, ZlevelNodesMap,
};

pub mod make {
    use super::{
        arr2,
        info::{absumv, get_max_xyz, get_order_from_n},
        itertools::{iproduct, Itertools},
        modify::shift_xyz,
        rayon::prelude::*,
        shrink::shrink_adjacency,
        Adjacency, Node, Nodes, Point, VIMap, Verts, VertsVec, ZOrder,
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

    pub fn vertices(max_xyz: Point) -> VertsVec {
        iproduct!(
            (-max_xyz..=max_xyz).step_by(2),
            (-max_xyz..=max_xyz).step_by(2),
            (-max_xyz..=max_xyz).step_by(2)
        )
        .filter(|&(x, y, z)| absumv([x, y, z]) < (max_xyz + 4))
        .sorted_by_key(|&(x, y, z)| (absumv([x, y, z]), x, y))
        .collect::<VertsVec>()
    }

    fn vi_map(verts: &Verts) -> VIMap {
        verts
            .par_iter()
            .enumerate()
            .map(|(idx, vert)| (*vert, idx as Node))
            .collect()
    }

    fn adjacency_map(verts: &Verts, max_xyz: Point, vi_map: &VIMap) -> Adjacency {
        verts
            .par_iter()
            .enumerate()
            .map(|(idx, (x, y, z))| {
                let ix = idx as Node;
                (
                    ix,
                    shift_xyz(arr2(&[[*x, *y, *z]]))
                        .outer_iter()
                        .filter_map(|new_vert| {
                            match vi_map.get(&(new_vert[0], new_vert[1], new_vert[2])) {
                                Some(&node)
                                    if node != ix
                                        && absumv([new_vert[0], new_vert[1], new_vert[2]])
                                            <= max_xyz + 2 =>
                                {
                                    Some(node)
                                }
                                _ => None,
                            }
                        })
                        .collect::<Nodes>(),
                )
            })
            .collect()
    }
}

pub mod shrink {
    use super::{Adjacency, Itertools, Nodes, Point, Points, Verts, ZOrder, ZlevelNodesMap};

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
                (
                    z,
                    verts
                        .iter()
                        .enumerate()
                        .filter_map(|(i, v)| {
                            if v.2 as Point == z {
                                Some(i as u32)
                            } else {
                                None
                            }
                        })
                        .collect::<Nodes>(),
                )
            })
            .collect()
    }

    fn filter_adjacency(adj: &Adjacency, nodes: Nodes) -> Adjacency {
        adj.iter()
            .filter_map(|(k, v)| {
                if nodes.contains(k) {
                    Some((*k, v.intersection(&nodes).copied().collect()))
                } else {
                    None
                }
            })
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

pub mod info {
    use super::{Point, SignedIdx, V3d, Vert};

    pub fn absumv(v: V3d) -> Point {
        let abs_sum = v.iter().fold(0, |acc, x| {
            let mask = x >> 15;
            acc + (x ^ mask) - mask
        });
        let sign_bit = abs_sum >> 15;
        (abs_sum ^ sign_bit) - sign_bit
    }

    pub fn get_axis((a, b, c): Vert, (x, y, z): Vert) -> Result<usize, &'static str> {
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
}

pub mod modify {
    use super::{arr2, Array2, Point};

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

pub mod xy {
    use super::Vert;

    pub fn axis((x, y, _): &Vert, (x1, y1, _): &Vert) -> usize {
        (0..2)
            .find(|&i| [x, y][i] != [x1, y1][i])
            .expect("Something's wrong, the same verts are being compared.")
    }

    pub fn absumv((x, y, _): Vert) -> i16 {
        let abs_sum = [x, y].iter().fold(0, |acc, x| {
            let mask = x >> 15;
            acc + (x ^ mask) - mask
        });
        let sign_bit = abs_sum >> 15;
        (abs_sum ^ sign_bit) - sign_bit
    }
}

pub mod check_edge {
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
    use super::{fmt, Adjacency, Itertools, Solution};

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

pub mod make_edges_eadjs {
    use rayon::prelude::*;
    use super::{Edge, Edges, VIMap, Vert};

    pub fn create_eadjs(
        (a, b, c): Vert,
        (x, y, z): Vert,
        max_xyz: i16,
        vi_map: &VIMap,
    ) -> Edges {
        match (a != x, b != y, c != z) {
            (true, false, false) => [[0, 2, 0], [0, -2, 0], [0, 0, 2], [0, 0, -2]],
            (false, true, false) => [[2, 0, 0], [-2, 0, 0], [0, 0, 2], [0, 0, -2]],
            (false, false, true) => [[2, 0, 0], [-2, 0, 0], [0, 2, 0], [0, -2, 0]],
            _ => panic!("NOT A VALID EDGE"),
        }
        .par_iter()
        .filter_map(|[i, j, k]| {
            get_valid_eadj(
                (a + i, b + j, c + k),
                (x + i, y + j, z + k),
                max_xyz,
                vi_map,
            )
        })
        .collect()
    }

    pub fn create_edges(
        (a, b, c): Vert,
        (x, y, z): Vert,
        max_xyz: i16,
        vi_map: &VIMap,
    ) -> Edges {
        match (a != x, b != y, c != z) {
            (true, false, false) => [[0, 2, 0], [0, -2, 0], [0, 0, 2], [0, 0, -2]],
            (false, true, false) => [[2, 0, 0], [-2, 0, 0], [0, 0, 2], [0, 0, -2]],
            (false, false, true) => [[2, 0, 0], [-2, 0, 0], [0, 2, 0], [0, -2, 0]],
            _ => panic!("NOT A VALID EDGE"),
        }
        .par_iter()
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
        (x, y, z): Vert,
        (a, b, c): Vert,
        max_xyz: i16,
        vi_map: &VIMap,
    ) -> Option<Edge> {
        let lowest = max_xyz - 4;
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
        (x, y, z): Vert,
        (a, b, c): Vert,
        max_xyz: i16,
        vi_map: &VIMap,
    ) -> Option<Edge> {
        let lowest = max_xyz - 4;
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
}
