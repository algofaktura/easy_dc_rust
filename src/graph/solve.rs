extern crate itertools;
use itertools::Itertools;
use ndarray;

use crate::graph::convert;
use crate::graph::structs;
use crate::graph::types::{
    Adjacency, Bobbins, Count, Done, EdgeAdjacency, Idx, Loom, Node, Point, Solution, Spool,
    Subtours, Thread, Tour, TourSlice, V3d, VIMap, Varr, Vert, Vert2dd, Verts, VertsC3, WarpedLoom,
    Warps, Wefts, Weights, Woven, Yarn, ZOrder,
};

pub fn weave(
    adj: &Adjacency,
    vi_map: &VIMap,
    edge_adj: &EdgeAdjacency,
    verts: &Verts,
    weights: &Weights,
    z_adj: &Adjacency,
    z_length: &ZOrder,
) -> Solution {
    let mut warp_wefts: Loom = warp_loom(vi_map, verts, weights, z_adj, z_length);
    let (warp, wefts) = warp_wefts.split_first_mut().unwrap();
    let warp: &mut structs::Cycle = structs::Cycle::new(warp, &adj, &edge_adj, verts);
    join_loops(warp, wefts, adj, verts, edge_adj);
    warp.retrieve()
}

pub fn warp_loom(
    vi_map: &VIMap,
    verts: &Verts,
    weights: &Weights,
    z_adj: &Adjacency,
    z_length: &ZOrder,
) -> Loom {
    let spool: Spool = yarn(&z_adj, verts, weights);
    let mut bobbins: Bobbins = Vec::new();
    let mut loom: Loom = Loom::new();
    for (zlevel, order) in z_length {
        let warps: Warps = get_warps(*zlevel, *order, &bobbins, &spool, vi_map);
        let woven: Woven = join_threads(&mut loom, &warps);
        affix_loose_threads(&mut loom, warps, woven);
        if *zlevel != -1 {
            bobbins = wind(&mut loom, verts, &vi_map);
        }
    }
    reflect_loom(&mut loom, verts, vi_map);
    loom.sort_by_key(|w| w.len());
    loom
}

pub fn yarn(z_adj: &Adjacency, verts: &Verts, weights: &Weights) -> Spool {
    let verts2dd: &Vert2dd = &convert::from_v3c_to_v2c(verts);
    let path: Tour = spin(&z_adj, &weights, verts);
    let natural: Yarn = convert::from_nodes_to_yarn(path, verts2dd);
    let colored: Yarn = color(&natural);
    Spool::from([(3, natural), (1, colored)])
}

pub fn spin(adj: &Adjacency, weights: &Weights, verts: &Verts) -> Tour {
    let var = convert::from_verts_to_vertsc(verts);
    let path: &mut Tour = &mut vec![*adj.keys().max().unwrap() as Node];
    let order: Count = adj.len();
    for idx in 1..order {
        path.push(get_next(&path, adj, weights, &var, idx, order))
    }
    path.to_vec()
}

pub fn get_next(
    path: TourSlice,
    adj: &Adjacency,
    weights: &Weights,
    verts: &Varr,
    idx: usize,
    order: usize,
) -> Node {
    if idx < order - 5 {
        adj.get(path.last().unwrap())
            .unwrap()
            .iter()
            .filter(|n| !path.contains(*n))
            .copied()
            .max_by_key(|&n| *weights.get(&n).unwrap())
            .unwrap()
    } else {
        let curr: &Node = path.last().unwrap();
        let curr_vert: &V3d = &verts[*curr as usize];
        adj.get(curr)
            .unwrap()
            .iter()
            .filter(|n| !path.contains(*n))
            .map(|&n| (n, get_axis(curr_vert, &verts[n as usize])))
            .filter(|(_, next_axis)| {
                *next_axis != get_axis(&verts[path[path.len() - 2] as usize], curr_vert)
            })
            .max_by_key(|&(n, _)| weights[&n])
            .unwrap()
            .0
    }
}

pub fn get_axis(m_vert: &V3d, n_vert: &V3d) -> Idx {
    (0..2)
        .find(|&i| m_vert[i] != n_vert[i])
        .expect("VERTS ARE SIMILAR")
}

pub fn color(a: &Yarn) -> Yarn {
    a.clone().dot(&ndarray::arr2(&[[-1, 0], [0, -1]])) + ndarray::arr2(&[[0, 2]])
}

pub fn wind(loom: &mut Loom, verts: &Verts, vi_map: &VIMap) -> Bobbins {
    loom.iter_mut()
        .map(|thread| {
            let (left, right) = get_upper_nodes(
                verts[thread[0] as usize],
                verts[thread[thread.len() - 1] as usize],
                vi_map,
            );
            thread.push_front(left);
            thread.push_back(right);
            vec![left, right]
        })
        .flatten()
        .collect()
}

pub fn get_upper_nodes((x, y, z): Vert, (x1, y1, z1): Vert, vi_map: &VIMap) -> (u32, u32) {
    (vi_map[&(x, y, z + 2)], vi_map[&(x1, y1, z1 + 2)])
}

pub fn get_warps(
    zlevel: Point,
    order: Count,
    bobbins: &Bobbins,
    spool: &Spool,
    vi_map: &VIMap,
) -> Warps {
    let node_yarn: Tour = get_node_yarn(
        spool[&(zlevel % 4 + 4).try_into().unwrap()].clone(),
        zlevel,
        order,
        vi_map,
    );
    if bobbins.is_empty() {
        vec![node_yarn]
    } else {
        cut(node_yarn, &bobbins)
    }
}

pub fn get_node_yarn(mut yarn: Yarn, zlevel: Point, order: Count, vi_map: &VIMap) -> Tour {
    yarn.slice_axis_inplace(
        ndarray::Axis(0),
        ndarray::Slice::new(
            (yarn.len_of(ndarray::Axis(0)) - order).try_into().unwrap(),
            None,
            1,
        ),
    );
    yarn.outer_iter()
        .map(|row| vi_map[&(row[0], row[1], zlevel)])
        .collect()
}

pub fn cut(tour: Tour, subset: &Bobbins) -> Subtours {
    let mut subtours: Subtours = Vec::new();
    let last_ix: Idx = tour.len() - 1;
    let last_idx: Idx = subset.len() - 1;
    let mut prev: i32 = -1 as i32;
    for (e, idx) in tour
        .iter()
        .enumerate()
        .filter_map(|(i, &node)| {
            if subset.contains(&node) {
                Some(i)
            } else {
                None
            }
        })
        .sorted()
        .enumerate()
    {
        if e == last_idx && idx != last_ix {
            for subtour in vec![
                tour[(prev + 1) as usize..idx].to_vec(),
                tour[idx..].to_vec(),
            ] {
                if !subtour.is_empty() {
                    subtours.push(if subset.contains(&subtour[0]) {
                        subtour
                    } else {
                        subtour.iter().rev().cloned().collect()
                    });
                }
            }
        } else {
            let subtour = tour[(prev + 1) as usize..=idx].to_vec();
            if !subtour.is_empty() {
                subtours.push(if subset.contains(&subtour[0]) {
                    subtour
                } else {
                    subtour.iter().rev().cloned().collect()
                });
            }
            prev = idx as i32;
        }
    }
    subtours
}

pub fn join_threads(loom: &mut Loom, warps: &Warps) -> Woven {
    let mut woven: Woven = Woven::new();
    for thread in loom {
        for (idx, warp) in warps.iter().enumerate() {
            if !woven.contains(&idx) {
                match (thread.front(), thread.back()) {
                    (Some(front), _) if *front == warp[0] => {
                        *thread = warp[1..]
                            .iter()
                            .rev()
                            .chain(thread.iter())
                            .cloned()
                            .collect()
                    }
                    (_, Some(back)) if *back == warp[0] => thread.extend(warp.iter().skip(1)),
                    _ => continue,
                }
                woven.extend([idx])
            }
        }
    }
    woven
}

pub fn affix_loose_threads(loom: &mut Loom, warps: Warps, woven: Woven) {
    for (_, seq) in warps
        .iter()
        .enumerate()
        .filter(|(idx, _)| !woven.contains(idx))
    {
        loom.extend(vec![Thread::from(seq.iter().cloned().collect::<Thread>())])
    }
}

pub fn reflect_loom(loom: &mut Loom, verts: &Verts, vi_map: &VIMap) {
    for thread in loom {
        thread.extend(
            thread
                .iter()
                .rev()
                .map(|&node| verts[node as usize])
                .map(|(x, y, z)| vi_map[&(x, y, -z)])
                .collect::<Tour>(),
        )
    }
}

pub fn join_loops(
    warp: &mut structs::Cycle,
    wefts: &mut Wefts,
    adj: &Adjacency,
    verts: &VertsC3,
    edge_adj: &EdgeAdjacency,
) {
    let loom: WarpedLoom = wefts
        .iter()
        .enumerate()
        .map(|(idx, seq)| (idx, structs::Cycle::new(&seq, &adj, &edge_adj, verts)))
        .collect();
    let mut done: Done = Done::new();
    let loom_order = loom.keys().len();
    if loom_order > 0 {
        loop {
            for idx in loom.keys() {
                if done.len() != loom_order {
                    if !done.contains(idx) {
                        let mut other: structs::Cycle = loom[&*idx].clone();
                        if let Some(warp_e) = warp.edges().intersection(&other.eadjs()).next() {
                            if let Some(weft_e) = edge_adj
                                .get(&warp_e)
                                .unwrap()
                                .intersection(&other.edges())
                                .next()
                            {
                                warp.join(*warp_e, *weft_e, &mut other);
                                done.extend([idx])
                            }
                        }
                    }
                } else {
                    return;
                }
            }
        }
    }
}
