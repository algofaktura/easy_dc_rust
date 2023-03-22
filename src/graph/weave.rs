use std::collections::VecDeque;

use itertools::Itertools;
use ndarray::arr2;
use rayon::prelude::*;

use super::{
    defs::{
        Adjacency, Bobbins, Count, Loom, Node, Point, Solution, Spool, Subtours, Tour, TourSlice,
        VIMap, Vert, Verts, Warps, Weaver, Yarn, ZOrder,
    },
    utils::{
        info::{absumv2d, axis2d},
        make_edges_eadjs::{make_eadjs, make_edges},
    },
};

pub fn weave(
    adj: &Adjacency,
    vi_map: VIMap,
    verts: &Verts,
    z_adj: Adjacency,
    z_order: ZOrder,
    min_xyz: Point,
) -> Solution {
    let mut loom = prepare_loom(&vi_map, &verts, z_adj, z_order);
    let mut weaver: Weaver = Weaver::new(loom[0].split_off(0), adj, &verts, true, min_xyz);
    let mut loom = loom
        .split_off(1)
        .into_iter()
        .map(|mut data| data.drain(..).collect())
        .collect::<Vec<Vec<_>>>();
    loom.iter_mut().for_each(|warp| {
        let warp_edges = weaver.make_edges_for(warp);
        if let Some((m, n)) = (&weaver.get_edges()
            & &warp_edges
                .iter()
                .flat_map(|(m, n)| {
                    make_edges(verts[*m as usize], verts[*n as usize], min_xyz, &vi_map)
                })
                .collect())
            .into_iter()
            .next()
        {
            if let Some(warp_e) =
                (&make_eadjs(verts[m as usize], verts[n as usize], min_xyz, &vi_map) & &warp_edges)
                    .into_iter()
                    .next()
            {
                weaver.join((m, n), warp_e, warp);
            }
        }
    });
    weaver.get_nodes()
}

fn prepare_loom(vi_map: &VIMap, verts: &Verts, z_adj: Adjacency, z_order: ZOrder) -> Loom {
    let spool: Spool = spin_and_color_yarn(z_adj, verts);
    let mut bobbins: Bobbins = Vec::new();
    let mut loom: Loom = Loom::new();
    for (z, length) in z_order {
        wrap_warps_onto_loom(get_warps(z, length, &bobbins, &spool, vi_map), &mut loom);
        if z == -1 {
            loom.par_iter_mut().for_each(|thread| {
                thread.extend(
                    thread
                        .iter()
                        .rev()
                        .map(|&node| {
                            let (x, y, z) = verts[node as usize];
                            vi_map[&(x, y, -z)]
                        })
                        .collect::<Tour>(),
                )
            });
            break;
        }
        bobbins = pin_ends(&mut loom, verts, vi_map)
    }
    loom
}

fn spin_and_color_yarn(z_adj: Adjacency, verts: &Verts) -> Spool {
    let natur: Yarn = spin_yarn(z_adj.len(), z_adj, verts);
    let color: Yarn = natur.clone().dot(&arr2(&[[-1, 0], [0, -1]])) + arr2(&[[0, 2]]);
    Spool::from([(3, natur), (1, color)])
}

fn spin_yarn(order_z: Count, z_adj: Adjacency, verts: &Verts) -> Yarn {
    let spindle: &mut Tour = &mut Vec::with_capacity(order_z);
    spindle.push(*z_adj.keys().max().unwrap());
    let tail = order_z - 5;
    (1..order_z).for_each(|idx| spindle.push(get_unspun(spindle, &z_adj, verts, idx, tail)));
    Yarn::from(
        spindle
            .iter()
            .map(|&n| {
                let (x, y, _) = verts[n as usize];
                [x, y]
            })
            .collect::<Vec<_>>(),
    )
}

fn get_unspun(
    spindle: TourSlice,
    z_adj: &Adjacency,
    verts: &Verts,
    idx: usize,
    tail: usize,
) -> Node {
    let curr = *spindle.last().unwrap();
    z_adj[&curr]
        .iter()
        .filter_map(|&n| match (spindle.contains(&n), verts.get(n as usize)) {
            (true, _) => None,
            (false, Some(next_vert))
                if idx < tail
                    || axis2d(
                        &verts[spindle[spindle.len() - 2] as usize],
                        &verts[curr as usize],
                    ) != axis2d(&verts[curr as usize], next_vert) =>
            {
                Some((n, absumv2d(*next_vert)))
            }
            _ => None,
        })
        .max_by_key(|&(_, absumv)| absumv)
        .unwrap()
        .0
}

fn get_warps(
    zlevel: Point,
    length: Count,
    bobbins: &Bobbins,
    spool: &Spool,
    vi_map: &VIMap,
) -> Warps {
    let mut yarn = spool[&(zlevel % 4 + 4).try_into().unwrap()].clone();
    yarn.slice_axis_inplace(
        ndarray::Axis(0),
        ndarray::Slice::new(
            (yarn.len_of(ndarray::Axis(0)) - length).try_into().unwrap(),
            None,
            1,
        ),
    );
    match yarn
        .outer_iter()
        .map(|row| vi_map[&(row[0], row[1], zlevel)])
        .collect()
    {
        node_yarn if bobbins.is_empty() => vec![node_yarn],
        node_yarn => cut_yarn(node_yarn, bobbins),
    }
}

fn cut_yarn(yarn: Tour, cuts: &Bobbins) -> Subtours {
    let mut subtours: Subtours = Vec::new();
    let last_ix: usize = yarn.len() - 1;
    let last_idx: usize = cuts.len() - 1;
    let mut prev: i32 = -1_i32;
    for (e, idx) in cuts
        .iter()
        .filter_map(|node| yarn.iter().position(|&x| x == *node))
        .sorted()
        .enumerate()
    {
        if e == last_idx && idx != last_ix {
            if let Some(first_slice) = yarn.get(prev as usize + 1..idx) {
                if !first_slice.is_empty() {
                    subtours.push(if cuts.contains(&first_slice[0]) {
                        first_slice.to_vec()
                    } else {
                        first_slice.iter().rev().cloned().collect()
                    });
                }
            }
            if let Some(first_slice) = yarn.get(idx..) {
                if !first_slice.is_empty() {
                    subtours.push(if cuts.contains(&first_slice[0]) {
                        first_slice.to_vec()
                    } else {
                        first_slice.iter().rev().cloned().collect()
                    });
                }
            }
        } else {
            if let Some(first_slice) = yarn.get(prev as usize + 1..=idx) {
                if !first_slice.is_empty() {
                    subtours.push(if cuts.contains(&first_slice[0]) {
                        first_slice.to_vec()
                    } else {
                        first_slice.iter().rev().cloned().collect()
                    });
                }
            }
            prev = idx as i32;
        }
    }
    subtours
}

fn pin_ends(loom: &mut Loom, verts: &Verts, vi_map: &VIMap) -> Bobbins {
    loom.iter_mut()
        .flat_map(|thread| {
            let (left, right) = get_upper_pins(
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

fn get_upper_pins((x, y, z): Vert, (x1, y1, z1): Vert, vi_map: &VIMap) -> (u32, u32) {
    (vi_map[&(x, y, z + 2)], vi_map[&(x1, y1, z1 + 2)])
}

fn wrap_warps_onto_loom(mut warps: Warps, loom: &mut Loom) {
    for thread in &mut *loom {
        for warp in warps.iter_mut().filter(|w| !w.is_empty()) {
            match (thread.front(), thread.back()) {
                (Some(front), _) if *front == warp[0] => {
                    *thread = warp
                        .drain(..)
                        .rev()
                        .chain(std::mem::take(thread).drain(1..))
                        .collect();
                }
                (_, Some(back)) if *back == warp[0] => {
                    thread.extend(warp.drain(..).skip(1));
                }
                _ => continue,
            }
        }
    }
    warps.iter_mut().filter(|s| !s.is_empty()).for_each(|seq| {
        loom.append(&mut vec![seq.drain(..).collect::<VecDeque<_>>()]);
    });
}
