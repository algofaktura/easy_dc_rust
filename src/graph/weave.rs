use std::collections::VecDeque;

use itertools::Itertools;
use ndarray;
use rayon::prelude::*;

use super::{
    defs::{
        Adjacency, Bobbins, Count, Idx, Loom, Node, Point, Solution, Spool, Subtours, Tour,
        TourSlice, VIMap, Vert, Verts, Warps, Weaver, Yarn, ZOrder,
    },
    utils::{
        make_edges_eadjs::{create_eadjs, create_edges},
        xy::{absumv, axis},
    },
};

pub fn weave(
    adj: &Adjacency,
    vi_map: VIMap,
    verts: &Verts,
    z_adj: Adjacency,
    z_order: ZOrder,
    max_xyz: Point,
) -> Solution {
    let mut loom = prepare_loom(&vi_map, verts, z_adj, z_order);
    let mut weaver: Weaver = Weaver::new(loom[0].split_off(0), adj, verts, true, max_xyz);
    let mut loom = loom
        .split_off(1)
        .into_iter()
        .map(|mut data| data.drain(..).collect())
        .collect::<Vec<Vec<_>>>();
    loom.iter_mut().for_each(|other| {
        let other_edges = weaver.make_edges_for(other);
        if let Some((m, n)) = (&weaver.get_edges()
            & &other_edges
                .iter()
                .flat_map(|(m, n)| {
                    create_edges(verts[*m as usize], verts[*n as usize], max_xyz, &vi_map)
                })
                .collect())
            .into_iter()
            .next()
        {
            if let Some(warp_e) =
                (&create_eadjs(verts[m as usize], verts[n as usize], max_xyz, &vi_map)
                    & &other_edges)
                    .into_iter()
                    .next()
            {
                weaver.join((m, n), warp_e, other);
            }
        }
    });
    weaver.get_nodes()
}

fn prepare_loom(vi_map: &VIMap, verts: &Verts, z_adj: Adjacency, z_order: ZOrder) -> Loom {
    let spool: Spool = spin_and_color_yarn(z_adj, verts);
    let mut bobbins: Bobbins = Vec::new();
    let mut loom: Loom = Loom::new();
    for (zlevel, order) in z_order {
        wrap_warps_onto_loom(
            get_warps(zlevel, order, &bobbins, &spool, vi_map),
            &mut loom,
        );
        if zlevel != -1 {
            bobbins = prepare_bobbins(&mut loom, verts, vi_map);
        }
    }
    reflect_loom(&mut loom, verts, vi_map);
    loom
}

fn spin_and_color_yarn(z_adj: Adjacency, verts: &Verts) -> Spool {
    let natural: Yarn = spin_yarn(z_adj.len(), z_adj, verts);
    let colored: Yarn = color_yarn(&natural);
    Spool::from([(3, natural), (1, colored)])
}

fn spin_yarn(order_z: Count, z_adj: Adjacency, verts: &Verts) -> Yarn {
    let spindle: &mut Tour = &mut vec![*z_adj.keys().max().unwrap()];
    (1..order_z).for_each(|idx| spindle.push(get_fibre(spindle, &z_adj, verts, idx, order_z)));
    shape_yarn(spindle, verts)
}

fn get_fibre(
    spindle: TourSlice,
    z_adj: &Adjacency,
    verts: &Verts,
    idx: usize,
    order_z: usize,
) -> Node {
    let curr = *spindle.last().unwrap();
    z_adj[&curr]
        .iter()
        .filter_map(|&n| match (spindle.contains(&n), verts.get(n as usize)) {
            (true, _) => None,
            (false, Some(next_vert))
                if idx < order_z - 5
                    || axis(
                        &verts[spindle[spindle.len() - 2] as usize],
                        &verts[curr as usize],
                    ) != axis(&verts[curr as usize], next_vert) =>
            {
                Some((n, absumv(*next_vert)))
            }
            _ => None,
        })
        .max_by_key(|&(_, absumv)| absumv)
        .unwrap()
        .0
}

fn shape_yarn(tour: &mut Tour, verts: &Verts) -> Yarn {
    Yarn::from(
        tour.iter()
            .map(|&n| [verts[n as usize].0, verts[n as usize].1])
            .collect::<Vec<[Point; 2]>>(),
    )
}

fn color_yarn(a: &Yarn) -> Yarn {
    a.clone().dot(&ndarray::arr2(&[[-1, 0], [0, -1]])) + ndarray::arr2(&[[0, 2]])
}

fn get_warps(
    zlevel: Point,
    order: Count,
    bobbins: &Bobbins,
    spool: &Spool,
    vi_map: &VIMap,
) -> Warps {
    let node_yarn: Tour = prepare_yarn(
        spool[&(zlevel % 4 + 4).try_into().unwrap()].clone(),
        zlevel,
        order,
        vi_map,
    );
    match bobbins.is_empty() {
        true => vec![node_yarn],
        false => cut_yarn(node_yarn, bobbins),
    }
}

fn prepare_yarn(mut yarn: Yarn, zlevel: Point, order: Count, vi_map: &VIMap) -> Tour {
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

fn cut_yarn(yarn: Tour, cuts: &Bobbins) -> Subtours {
    let mut subtours: Subtours = Vec::new();
    let last_ix: Idx = yarn.len() - 1;
    let last_idx: Idx = cuts.len() - 1;
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

fn prepare_bobbins(loom: &mut Loom, verts: &Verts, vi_map: &VIMap) -> Bobbins {
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

fn reflect_loom(loom: &mut Loom, verts: &Verts, vi_map: &VIMap) {
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
}
