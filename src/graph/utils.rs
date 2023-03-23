use itertools;
use itertools::Itertools;
use ndarray::{arr2, Array2};
use rayon;
use std::fmt;

use super::defs::{
    Adjacency, Edge, Edges, Neighbors, Node, Nodes, Point, Points, SignedIdx, Solution, VIMap,
    VecVert, Vert, Verts, ZAdjacency, ZOrder, ZlevelNodesMap,
};

pub mod make {
    use super::{
        arr2,
        info::{absumv, get_max_xyz, get_order_from_n},
        itertools::{iproduct, Itertools},
        modify::shift_xyz,
        rayon::prelude::*,
        shrink::shrink_adjacency,
        Adjacency, Neighbors, Node, Point, VIMap, VecVert, Verts, ZAdjacency, ZOrder,
    };

    pub fn make_graph(n: u32) -> (u32, u32, VecVert, VIMap, Adjacency, ZAdjacency, ZOrder, i16) {
        let order = get_order_from_n(n);
        let max_xyz = get_max_xyz(order) as i16;
        let verts: VecVert = vertices(max_xyz);
        let vi_map: VIMap = vi_map(&verts);
        let adj: Adjacency = adjacency_map(&verts, max_xyz + 2, &vi_map);
        let (z_adj, z_order) = shrink_adjacency(&verts, &adj);
        (n, order, verts, vi_map, adj, z_adj, z_order, max_xyz - 4)
    }

    pub fn vertices(max_xyz: Point) -> VecVert {
        let max_xyz_plus_4 = max_xyz + 4;
        iproduct!(
            (-max_xyz..=max_xyz).step_by(2),
            (-max_xyz..=max_xyz).step_by(2),
            (-max_xyz..=max_xyz).step_by(2)
        )
        .filter(|&vert| absumv(vert) < max_xyz_plus_4)
        .sorted_by_key(|&vert| (absumv(vert), vert.0, vert.1))
        .collect::<VecVert>()
    }

    fn vi_map(verts: &Verts) -> VIMap {
        verts
            .par_iter()
            .enumerate()
            .map(|(idx, vert)| (*vert, idx as Node))
            .collect()
    }

    fn adjacency_map(verts: &Verts, max_xyz_plus_2: Point, vi_map: &VIMap) -> Adjacency {
        verts
            .par_iter()
            .enumerate()
            .map(|(idx, (x, y, z))| {
                let ix = idx as Node;
                (
                    ix,
                    shift_xyz(arr2(&[[*x, *y, *z]]))
                        .into_iter()
                        .filter_map(|new_neighbor_vert| match vi_map.get(&new_neighbor_vert) {
                            Some(&node)
                                if node != ix && absumv(new_neighbor_vert) <= max_xyz_plus_2 =>
                            {
                                Some(node)
                            }
                            _ => None,
                        })
                        .collect::<Neighbors>(),
                )
            })
            .collect()
    }
}

pub mod shrink {
    use crate::graph::defs::ZAdjacency;

    use super::{Adjacency, Itertools, Nodes, Point, Points, Verts, ZOrder, ZlevelNodesMap};

    pub fn shrink_adjacency(verts: &Verts, adj: &Adjacency) -> (ZAdjacency, ZOrder) {
        let stratified: ZlevelNodesMap = stratify_nodes(verts);
        (
            filter_adjacency(adj, verts, stratified[&(-1 as Point)].clone()),
            get_zlevel_order(&stratified),
        )
    }

    fn stratify_nodes(verts: &Verts) -> ZlevelNodesMap {
        verts
            .iter()
            .filter_map(|&(_, _, z)| match z < 0 {
                true => Some(z),
                false => None,
            })
            .collect::<Points>()
            .into_iter()
            .map(|z| {
                (
                    z,
                    verts
                        .iter()
                        .enumerate()
                        .filter_map(|(i, v)| match v.2 as Point == z {
                            true => Some(i as u32),
                            false => None,
                        })
                        .collect::<Nodes>(),
                )
            })
            .collect()
    }

    fn filter_adjacency(adj: &Adjacency, verts: &Verts, nodes: Nodes) -> ZAdjacency {
        adj.iter()
            .filter_map(|(k, v)| match nodes.contains(k) {
                true => Some((
                    {
                        let (x, y, _) = verts[*k as usize];
                        [x, y]
                    },
                    v.intersection(&nodes)
                        .map(|node| {
                            let (x, y, _) = verts[*node as usize];
                            [x, y]
                        })
                        .collect(),
                )),
                false => None,
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

pub mod modify {
    use super::{arr2, Array2, Point, VecVert};

    pub fn orient(m: u32, n: u32) -> (u32, u32) {
        match m < n {
            true => (m, n),
            false => (n, m),
        }
    }

    pub fn shift_xyz(vert: Array2<Point>) -> VecVert {
        (vert
            + arr2(&[
                [2, 0, 0],
                [-2, 0, 0],
                [0, 2, 0],
                [0, -2, 0],
                [0, 0, 2],
                [0, 0, -2],
            ]))
        .outer_iter()
        .map(|point| (point[0], point[1], point[2]))
        .collect()
    }
}

pub mod info {
    use super::{Point, SignedIdx, Vert};

    pub fn axis2d((x, y, _): &Vert, (a, b, _): &Vert) -> usize {
        (0..2)
            .find(|&i| [x, y][i] != [a, b][i])
            .expect("Something's wrong, the same verts are being compared.")
    }

    pub fn absumv2d((x, y, _): Vert) -> i16 {
        let abs_sum = [x, y].iter().fold(0, |acc, x| {
            let mask = x >> 15;
            acc + (x ^ mask) - mask
        });
        let sign_bit = abs_sum >> 15;
        (abs_sum ^ sign_bit) - sign_bit
    }

    pub fn absumv2dc(vert: [i16; 2]) -> i16 {
        let abs_sum = vert.iter().fold(0, |acc, x| {
            let mask = x >> 15;
            acc + (x ^ mask) - mask
        });
        let sign_bit = abs_sum >> 15;
        (abs_sum ^ sign_bit) - sign_bit
    }

    pub fn absumv((x, y, z): Vert) -> Point {
        let abs_sum = [x, y, z].iter().fold(0, |acc, x| {
            let mask = x >> 15;
            acc + (x ^ mask) - mask
        });
        let sign_bit = abs_sum >> 15;
        (abs_sum ^ sign_bit) - sign_bit
    }

    pub fn get_max_xyz(order: u32) -> SignedIdx {
        (get_n_from_order(order) * 2 - 1) as i32
    }

    pub fn get_order_from_n(n: u32) -> u32 {
        ((4.0 / 3.0) * ((n + 2) * (n + 1) * n) as f64).round() as u32
    }

    pub fn get_n_from_order(order: u32) -> u32 {
        (((3.0 / 4.0) * order as f64).powf(1.0 / 3.0) - 2.0 / 3.0).round() as u32
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

pub mod check_edge {
    use super::{Point, Vert};

    pub fn is_valid_edge(v1: Vert, v2: Vert, min_xyz: Point, order: u32, lead: bool) -> bool {
        if order < 160 {
            return valid_edge(v1, v2);
        }
        match lead {
            true => valid_main_edge(v1, v2, min_xyz),
            false => valid_other_edge(v1, v2, min_xyz),
        }
    }

    pub fn valid_edge((x1, y1, _): Vert, (x2, y2, _): Vert) -> bool {
        matches!(x1 + y1 + x2 + y2, 4..=10)
    }

    pub fn valid_main_edge((x, y, z): Vert, (x2, y2, z2): Vert, min_xyz: Point) -> bool {
        if z.abs() == min_xyz && min_xyz == z2.abs() {
            (x == 1 || x == 3) && y == y2 && y2 == 1 && (x2 == 1 || x2 == 3)
        } else {
            x == x2 && x2 == 1 && y == y2 && y2 == 1
        }
    }

    pub fn valid_other_edge((x, y, z): Vert, (x2, y2, z2): Vert, min_xyz: Point) -> bool {
        if z.abs() == min_xyz && min_xyz == z2.abs() {
            (x == 1 || x == 3) && y == y2 && y2 == 3 && (x2 == 1 || x2 == 3)
        } else {
            x == x2 && x2 == 3 && y == y2 && y2 == 1
        }
    }
}

pub mod make_edges_eadjs {
    use super::{Edge, Edges, VIMap, Vert};
    use rayon::prelude::*;

    pub fn make_eadjs((a, b, c): Vert, (x, y, z): Vert, min_xyz: i16, vi_map: &VIMap) -> Edges {
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
                min_xyz,
                vi_map,
            )
        })
        .collect()
    }

    pub fn make_edges((a, b, c): Vert, (x, y, z): Vert, min_xyz: i16, vi_map: &VIMap) -> Edges {
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
                min_xyz,
                vi_map,
            )
        })
        .collect()
    }

    pub fn get_valid_edge(
        (x, y, z): Vert,
        (a, b, c): Vert,
        min_xyz: i16,
        vi_map: &VIMap,
    ) -> Option<Edge> {
        match z.abs() == min_xyz
            && min_xyz == c.abs()
            && (x == 1 || x == 3)
            && y == b
            && b == 1
            && (a == 1 || a == 3)
            || x == a && a == 1 && y == b && b == 1
        {
            true => Some((vi_map[&(x, y, z)], vi_map[&(a, b, c)])),
            false => None,
        }
    }

    pub fn get_valid_eadj(
        (x, y, z): Vert,
        (a, b, c): Vert,
        min_xyz: i16,
        vi_map: &VIMap,
    ) -> Option<Edge> {
        match z.abs() == min_xyz
            && min_xyz == c.abs()
            && (x == 1 || x == 3)
            && y == b
            && b == 3
            && (a == 1 || a == 3)
            || x == a && a == 3 && y == b && b == 1
        {
            true => Some((vi_map[&(x, y, z)], vi_map[&(a, b, c)])),
            false => None,
        }
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

pub mod translate {
    use super::{Adjacency, Verts, ZAdjacency};

    pub fn adj_to_adjvc(adj: &Adjacency, verts: &Verts) -> ZAdjacency {
        // turn every key and every value in value into a [Point;2]
        adj.iter()
            .map(|(k, val)| {
                (
                    {
                        let (x, y, _) = verts[*k as usize];
                        [x, y]
                    },
                    val.iter()
                        .map(|node| {
                            let (x, y, _) = verts[*node as usize];
                            [x, y]
                        })
                        .collect::<Vec<[i16; 2]>>(),
                )
            })
            .collect()
    }
}
