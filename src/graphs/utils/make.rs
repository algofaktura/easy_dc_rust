extern crate itertools;

use itertools::Itertools;

use crate::{graphs::info::info::is_valid_edge, types::types::*};

pub fn make_weights(adj: &Adjacency, verts: &VertsC3) -> Weights {
    adj.iter()
        .map(|(&n, _)| {
            let (x, y, z): Vert3d = verts[n as usize];
            let weight: i32 = x.abs() + y.abs() + z.abs();
            (n, weight)
        })
        .collect()
}

pub fn make_vi_mapping(verts: &Vectors3d) -> VertIdx {
    verts
        .iter()
        .enumerate()
        .map(|(idx, vert)| (vert, idx as u32))
        .collect::<VertIdx>()
}

fn get_adj_edges2(adj: &Adjacency, m_node: Node, n_node: Node) -> Edges {
    adj.get(&m_node)
        .unwrap()
        .iter()
        .flat_map(|m| adj.get(&n_node).unwrap().iter().map(move |n| (*m, *n)))
        .filter(|(m, n)| adj.get(m).unwrap().contains(n))
        .map(|(m, n)| if m < n { (m, n) } else { (n, m) })
        .collect()
}

pub fn make_edges_adj2(adj: &Adjacency, edges: &Edges) -> EdgeAdjacency {
    edges
        .iter()
        .map(|&(m, n)| ((m, n), get_adj_edges2(adj, m, n)))
        .collect()
}

pub fn make_edges_adj(adj: &Adjacency, edges: &Edges, verts: &VertsC3) -> EdgeAdjacency {
    edges
        .iter()
        .filter(|&(a, b)| is_valid_edge(verts[*a as usize], verts[*b as usize]))
        .map(|&(m, n)| ((m, n), get_adj_edges(adj, m, n, verts)))
        .collect()
}

pub fn get_adj_edges(adj: &Adjacency, m_node: Node, n_node: Node, verts: &VertsC3) -> Edges {
    adj.get(&m_node)
        .unwrap()
        .iter()
        .flat_map(|m| adj.get(&n_node).unwrap().iter().map(move |n| (*m, *n)))
        .filter(|(m, n)| {
            adj.get(m).unwrap().contains(n) && is_valid_edge(verts[*m as usize], verts[*n as usize])
        })
        .map(|(m, n)| if m < n { (m, n) } else { (n, m) })
        .collect()
}

pub fn make_edges_adj1(a: &Adjacency, edges: &Edges) -> EdgeAdjacency {
    edges
        .iter()
        .map(|&(u, p)| {
            let lhs = a
                .get(&u)
                .unwrap()
                .difference(&Neighbors::from([p]))
                .cloned()
                .collect::<Neighbors>();
            let rhs = a
                .get(&p)
                .unwrap()
                .difference(&Neighbors::from([u]))
                .cloned()
                .collect::<Neighbors>();
            let prod = lhs
                .iter()
                .flat_map(|&m| {
                    rhs.iter()
                        .map(move |&n| if m < n { (m, n) } else { (n, m) })
                })
                .collect::<Edges>();
            ((u, p), prod.intersection(edges).cloned().collect::<Edges>())
        })
        .filter(|(_, s)| !s.is_empty())
        .collect()
}

type Vert = (i32, i32, i32);
type Verts = Vec<(i32, i32, i32)>;

pub fn make_vertices(order: i32) -> Verts {
    let max_xyz = (0..)
        .map(|n| (n, (4 / 3) * (n + 2) * (n + 1) * n))
        .filter(|(_, sum)| *sum == order)
        .map(|(n, _)| n)
        .next()
        .unwrap()
        * 2
        - 1;
    (-(max_xyz as i32)..=(max_xyz as i32))
        .step_by(2)
        .flat_map(|x| {
            (-(max_xyz as i32)..=(max_xyz as i32))
                .step_by(2)
                .flat_map(move |y| {
                    (-(max_xyz as i32)..=(max_xyz as i32))
                        .step_by(2)
                        .map(move |z| (x, y, z))
                        .filter(|&v| absumv(v) < (max_xyz + 4) as u32)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .into_iter()
        .sorted_by_key(|v| (edist2(*v), v.0, v.1, v.2))
        .collect()
}

pub fn make_edges(vertices: &[(i32, i32, i32)], edgelength: u32) -> Edges {
    vertices
        .iter()
        .enumerate()
        .combinations(2)
        .map(|pairs| (pairs[0], pairs[1]))
        .filter_map(|((i, &p), (j, &q))| {
            let (dx, dy, dz) = (p.0 - q.0, p.1 - q.1, p.2 - q.2);
            if (dx * dx + dy * dy + dz * dz) as u32 == edgelength * edgelength {
                Some(if i < j {
                    (i as u32, j as u32)
                } else {
                    (j as u32, i as u32)
                })
            } else {
                None
            }
        })
        .collect()
}

#[macro_export]
macro_rules! hashcomp {
    // hashcomp!(hm = i+i => i*i; for i in 1..5);
    ($name:ident = $k:expr => $v:expr; for $i:ident in $itr:expr) => {
        let mut $name = HashMap::new();
        for $i in $itr {
            $name.insert($k, $v);
        }
    };
}

pub fn make_adj(_edges: Edges) -> Adjacency {
    Adjacency::new()
}

pub fn absumv(v: Vert) -> u32 {
    (v.0.abs() + v.1.abs() + v.2.abs()) as u32
}

pub fn edist((x, y, z): (i32, i32, i32)) -> f32 {
    ((x.pow(2) + y.pow(2) + z.pow(2)) as f32).sqrt()
}

pub fn edist2((x, y, z): (i32, i32, i32)) -> i32 {
    ((x.pow(2) + y.pow(2) + z.pow(2)) as f32).sqrt().round() as i32
}
