extern crate itertools;

use itertools::Itertools;

use crate::utils::operators::{color, get_upper_nodes, get_next, get_next_xyz, get_node_yarn};
use crate::graph::translate::{self, from_verts_to_vertsc};
use crate::graph::types::{
    Adjacency, Bobbins, Count, Done, EdgeAdjacency,  
    Idx, Loom, Node, Point, 
    Solution, Spool, Subtours, Thread, Tour,
    VIMap, Vert2dd, Verts, Yarn, VertsC3,
    WarpedLoom, Warps, Wefts, Weights, Woven, ZOrder
};

use crate::graph::cycle::Cycle;

pub fn weave(
    adj: &Adjacency,
    vert_idx: &VIMap,
    edge_adj: &EdgeAdjacency,
    verts: &Verts,
    weights: &Weights,
    z_adj: &Adjacency,
    z_length: &ZOrder,
) -> Solution {
    let mut warp_wefts: Loom = warp_loom(vert_idx, verts, weights, z_adj, z_length);
    let (warp, wefts) = warp_wefts.split_first_mut().unwrap();
    let warp: &mut Cycle = Cycle::new(warp, &adj, &edge_adj, verts);
    join_loops(warp, wefts, adj, verts, edge_adj);
    warp.retrieve()
}

pub fn warp_loom(
    vert_idx: &VIMap,
    verts: &Verts,
    weights: &Weights,
    z_adj: &Adjacency,
    z_length: &ZOrder,
) -> Loom {
    let spool: Spool = yarn(&z_adj, verts, weights);
    let mut bobbins: Bobbins = Vec::new();
    let mut loom: Loom = Loom::new();
    for (zlevel, order) in z_length {
        let warps: Warps = get_warps(*zlevel, *order, &bobbins, &spool, vert_idx);
        let woven: Woven = join_threads(&mut loom, &warps);
        affix_loose_threads(&mut loom, warps, woven);
        if *zlevel != -1 {
            bobbins = wind(&mut loom, verts, &vert_idx);
        }
    }
    reflect_solution(&mut loom, verts, vert_idx);
    loom.sort_by_key(|w| w.len());
    loom
}

pub fn yarn(z_adj: &Adjacency, verts: &Verts, weights: &Weights) -> Spool {
    let verts2dd: &Vert2dd = &translate::from_v3c_to_v2c(verts);
    let path: Tour = spin(&z_adj, &weights, verts);
    let natural: Yarn = translate::from_nodes_to_yarn(path, verts2dd);
    let colored: Yarn = color(&natural);
    Spool::from([(3, natural), (1, colored)])
}

pub fn spin(adj: &Adjacency, weights: &Weights, verts: &Verts) -> Tour {
    let var = from_verts_to_vertsc(verts);
    let path: &mut Tour = &mut vec![*adj.keys().max().unwrap() as Node];
    let order: Count = adj.len();
    for idx in 1..order {
        path.push(if idx < order - 5 {
            get_next(&path, adj, weights)
        } else {
            get_next_xyz(&path, adj, weights, &var)
        })
    }
    path.to_vec()
}

pub fn wind(loom: &mut Loom, verts: &Verts, vert_idx: &VIMap) -> Bobbins {
    loom.iter_mut()
        .map(|thread| {
            let (left, right) = get_upper_nodes(
                verts[thread[0] as usize],
                verts[thread[thread.len() - 1] as usize],
                vert_idx,
            );
            thread.push_front(left);
            thread.push_back(right);
            vec![left, right]
        })
        .flatten()
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

pub fn get_warps(
    zlevel: Point,
    order: Count,
    bobbins: &Bobbins,
    spool: &Spool,
    vert_idx: &VIMap,
) -> Warps {
    let node_yarn: Tour = get_node_yarn(
        spool[&(zlevel % 4 + 4).try_into().unwrap()].clone(),
        zlevel,
        order,
        vert_idx,
    );
    if bobbins.is_empty() {
        vec![node_yarn]
    } else {
        cut(node_yarn, &bobbins)
    }
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

pub fn reflect_solution(loom: &mut Loom, verts: &Verts, vert_idx: &VIMap) {
    for thread in loom {
        thread.extend(
            thread
                .iter()
                .rev()
                .map(|&node| verts[node as usize])
                .map(|(x, y, z)| vert_idx[&(x, y, -z)])
                .collect::<Tour>(),
        )
    }
}

pub fn join_loops(
    warp: &mut Cycle,
    wefts: &mut Wefts,
    adj: &Adjacency,
    verts: &VertsC3,
    edge_adj: &EdgeAdjacency,
) {
    let loom: WarpedLoom = wefts
        .iter()
        .enumerate()
        .map(|(idx, seq)| (idx, Cycle::new(&seq, &adj, &edge_adj, verts)))
        .collect();
    let mut done: Done = Done::new();
    let loom_order = loom.keys().len();
    if loom_order > 0 {
        loop {
            for idx in loom.keys() {
                if done.len() != loom_order {
                    if !done.contains(idx) {
                        let mut other: Cycle = loom[&*idx].clone();
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
