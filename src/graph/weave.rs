use itertools::Itertools;
use ndarray;
use rayon::prelude::*;
use std::cell::RefCell;

use super::{
    structs::Cycle,
    types::{
        Adjacency, Bobbins, Count, EdgeAdjacency, Idx, Loom, Node, Point, Solution, Spool,
        Subtours, YarnEnds, Tour, TourSlice, VIMap, Vert, Verts, WarpedLoom, Warps, Woven, Yarn,
        ZOrder,
    },
};

pub fn weave(
    adj: &Adjacency,
    vi_map: &VIMap,
    edge_adj: &EdgeAdjacency,
    verts: &Verts,
    z_adj: &Adjacency,
    z_order: &ZOrder,
) -> Solution {
    join_loops(
        prepare_loom(vi_map, verts, z_adj, z_order),
        adj,
        verts,
        edge_adj,
    )
}

fn prepare_loom(vi_map: &VIMap, verts: &Verts, z_adj: &Adjacency, z_order: &ZOrder) -> Loom {
    let spool: Spool = spin_and_color_yarn(z_adj, verts);
    let mut bobbins: Bobbins = Vec::new();
    let mut loom: Loom = Loom::new();
    for (zlevel, order) in z_order {
        let warps: Warps = get_warps(*zlevel, *order, &bobbins, &spool, vi_map);
        let woven: Woven = join_threads(&mut loom, &warps);
        affix_loose_threads(&mut loom, warps, woven);
        if *zlevel != -1 {
            bobbins = wind(&mut loom, verts, vi_map);
        }
    }
    reflect_loom(&mut loom, verts, vi_map);
    loom.sort_by_key(|w| w.len());
    loom
}

fn spin_and_color_yarn(z_adj: &Adjacency, verts: &Verts) -> Spool {
    let natural: Yarn = spin(z_adj, verts);
    let colored: Yarn = color_yarn(&natural);
    Spool::from([(3, natural), (1, colored)])
}

fn spin(z_adj: &Adjacency, verts: &Verts) -> Yarn {
    let path: &mut Tour = &mut vec![*z_adj.keys().max().unwrap() as Node];
    let order: Count = z_adj.len();
    (1..order).for_each(|idx| path.push(next_node(path, z_adj, verts, idx, order)));
    convert_nodes_to_yarn(path, verts)
}

fn next_node(path: TourSlice, adj: &Adjacency, verts: &Verts, idx: usize, order: usize) -> Node {
    if idx < order - 5 {
        adj[path.last().unwrap()]
            .iter()
            .filter(|n| !path.contains(*n))
            .copied()
            .max_by_key(|&n| absumv_2d(verts[n as usize]))
            .unwrap()
    } else {
        let curr: &Node = &path[path.len() - 1];
        let curr_vert: &Vert = &verts[*curr as usize];
        adj[curr]
            .iter()
            .filter(|n| !path.contains(*n))
            .map(|&n| (n, axis_2d(curr_vert, &verts[n as usize])))
            .filter(|(_, next_axis)| {
                *next_axis != axis_2d(&verts[path[path.len() - 2] as usize], curr_vert)
            })
            .max_by_key(|&(n, _)| absumv_2d(verts[n as usize]))
            .unwrap()
            .0
    }
}

fn axis_2d((x, y, _): &Vert, (x1, y1, _): &Vert) -> Idx {
    (0..2)
        .find(|&i| [x, y][i] != [x1, y1][i])
        .expect("Something's wrong, the same verts are being compared.")
}

fn absumv_2d((x, y, _): Vert) -> i32 {
    let abs_sum = [x, y].iter().fold(0, |acc, x| {
        let mask = x >> 31;
        acc + (x ^ mask) - mask
    });
    let sign_bit = abs_sum >> 31;
    (abs_sum ^ sign_bit) - sign_bit
}

fn convert_nodes_to_yarn(path: &mut Tour, verts: &Verts) -> Yarn {
    Yarn::from(
        path.iter()
            .map(|&n| [verts[n as usize].0, verts[n as usize].1])
            .collect::<Vec<[Point; 2]>>(),
    )
}

fn color_yarn(a: &Yarn) -> Yarn {
    a.clone().dot(&ndarray::arr2(&[[-1, 0], [0, -1]])) + ndarray::arr2(&[[0, 2]])
}

fn wind(loom: &mut Loom, verts: &Verts, vi_map: &VIMap) -> Bobbins {
    loom.iter_mut()
        .flat_map(|thread| {
            let (left, right) = get_upper_nodes(
                verts[thread[0] as usize],
                verts[thread[thread.len() - 1] as usize],
                vi_map,
            );
            thread.push_front(left);
            thread.push_back(right);
            vec![left, right]
        })
        .collect()
}

fn get_upper_nodes((x, y, z): Vert, (x1, y1, z1): Vert, vi_map: &VIMap) -> (u32, u32) {
    (vi_map[&(x, y, z + 2)], vi_map[&(x1, y1, z1 + 2)])
}

fn get_warps(
    zlevel: Point,
    order: Count,
    bobbins: &Bobbins,
    spool: &Spool,
    vi_map: &VIMap,
) -> Warps {
    let node_yarn: Tour = convert_yarn_to_nodes(
        spool[&(zlevel % 4 + 4).try_into().unwrap()].clone(),
        zlevel,
        order,
        vi_map,
    );
    if bobbins.is_empty() {
        vec![node_yarn]
    } else {
        cut(node_yarn, bobbins)
    }
}

fn convert_yarn_to_nodes(mut yarn: Yarn, zlevel: Point, order: Count, vi_map: &VIMap) -> Tour {
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

fn cut(tour: Tour, subset: &Bobbins) -> Subtours {
    let mut subtours: Subtours = Vec::new();
    let last_ix: Idx = tour.len() - 1;
    let last_idx: Idx = subset.len() - 1;
    let mut prev: i32 = -1_i32;
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
            for subtour in vec![tour[prev as usize + 1..idx].to_vec(), tour[idx..].to_vec()] {
                if !subtour.is_empty() {
                    subtours.push(if subset.contains(&subtour[0]) {
                        subtour
                    } else {
                        subtour.iter().rev().cloned().collect()
                    });
                }
            }
        } else {
            let subtour = tour[prev as usize + 1..=idx].to_vec();
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

fn join_threads(loom: &mut Loom, warps: &Warps) -> Woven {
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

fn affix_loose_threads(loom: &mut Loom, warps: Warps, woven: Woven) {
    for (_, seq) in warps
        .iter()
        .enumerate()
        .filter(|(idx, _)| !woven.contains(idx))
    {
        loom.extend(vec![seq.iter().cloned().collect::<YarnEnds>()])
    }
}

fn reflect_loom(loom: &mut Loom, verts: &Verts, vi_map: &VIMap) {
    loom.par_iter_mut().for_each(|thread| {
        thread.extend(
            thread
                .iter()
                .rev()
                .map(|&node| verts[node as usize])
                .map(|(x, y, z)| vi_map[&(x, y, -z)])
                .collect::<Tour>(),
        )
    });
}

pub fn join_loops<'a>(
    mut warp_wefts: Loom,
    adj: &'a Adjacency,
    verts: &'a Verts,
    edge_adj: &'a EdgeAdjacency,
) -> Solution {
    let (warp, wefts) = warp_wefts.split_first_mut().unwrap();
    let mut key_to_remove: Vec<usize> = Vec::with_capacity(1);
    let mut core_cord: Cycle = Cycle::new(warp, adj, edge_adj, verts);
    let mut loom: WarpedLoom = wefts
        .iter()
        .enumerate()
        .map(|(idx, seq)| (idx, RefCell::new(Cycle::new(seq, adj, edge_adj, verts))))
        .collect();
    loop {
        for key in loom.keys() {
            let other = &mut loom[key].borrow_mut();
            if let Some(warp_e) = (&core_cord.edges() & &other.eadjs()).into_iter().next() {
                if let Some(weft_e) = edge_adj[(&warp_e)].intersection(&other.edges()).next() {
                    core_cord.join(warp_e, *weft_e, other);
                    key_to_remove.push(*key);
                    break;
                }
            }
        }
        for key in key_to_remove.iter() {
            loom.remove(key);
        }
        if loom.is_empty() {
            return core_cord.retrieve_nodes();
        }
        key_to_remove.clear();
    }
}
