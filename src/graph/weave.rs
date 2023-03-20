use itertools::Itertools;
use ndarray;
use rayon::prelude::*;

use super::{
    types::{
        Adjacency, Bobbins, Count, Idx, Loom, Node, Point, Solution, Spool, Subtours, Tour,
        TourSlice, VIMap, Vert, Verts, Warps, Weaver, Woven, Yarn, ZOrder,
    },
    utils::{
        get_adj_edges::{create_eadjs, create_edges},
        xy::{absumv, axis},
    },
};

pub fn weave(
    adj: &Adjacency,
    vi_map: &VIMap,
    verts: &Verts,
    z_adj: &Adjacency,
    z_order: &ZOrder,
    max_xyz: Point,
) -> Solution {
    let mut loom = prepare_loom(vi_map, verts, z_adj, z_order);
    let weaver: Weaver = Weaver::new(loom[0].split_off(0), adj, verts, true, max_xyz);
    let loom = loom
        .split_off(1)
        .into_iter()
        .map(|mut data| data.drain(..).collect())
        .collect::<Vec<Vec<_>>>();
    weave_loom(weaver, loom, verts, vi_map, max_xyz)
}

fn prepare_loom(vi_map: &VIMap, verts: &Verts, z_adj: &Adjacency, z_order: &ZOrder) -> Loom {
    let spool: Spool = spin_and_color_yarn(z_adj, verts);
    let mut bobbins: Bobbins = Vec::new();
    let mut loom: Loom = Loom::new();
    for (zlevel, order) in z_order {
        let warps: Warps = get_warps(*zlevel, *order, &bobbins, &spool, vi_map);
        let woven: Woven = attach_warps_to_loom(&mut loom, &warps);
        affix_unwoven_to_loom(&mut loom, warps, woven);
        if *zlevel != -1 {
            bobbins = wind_threads(&mut loom, verts, vi_map);
        }
    }
    reflect_loom(&mut loom, verts, vi_map);
    loom
}

fn spin_and_color_yarn(z_adj: &Adjacency, verts: &Verts) -> Spool {
    let natural: Yarn = spin_yarn(z_adj, verts);
    let colored: Yarn = color_yarn(&natural);
    Spool::from([(3, natural), (1, colored)])
}

fn spin_yarn(z_adj: &Adjacency, verts: &Verts) -> Yarn {
    let tour: &mut Tour = &mut vec![*z_adj.keys().max().unwrap() as Node];
    let order: Count = z_adj.len();
    (1..order).for_each(|idx| tour.push(get_next_node(tour, z_adj, verts, idx, order)));
    make_yarn_from(tour, verts)
}

fn get_next_node(
    path: TourSlice,
    adj: &Adjacency,
    verts: &Verts,
    idx: usize,
    order: usize,
) -> Node {
    let curr = *path.last().unwrap();
    adj[&curr]
        .iter()
        .filter(|n| !path.contains(*n))
        .filter_map(|&n| {
            if idx < order - 5 {
                Some((n, absumv(verts[n as usize])))
            } else {
                let curr_vert = &verts[curr as usize];
                if axis(&verts[path[path.len() - 2] as usize], curr_vert)
                    == axis(curr_vert, &verts[n as usize])
                {
                    None
                } else {
                    Some((n, absumv(verts[n as usize])))
                }
            }
        })
        .max_by_key(|&(_, absumv)| absumv)
        .unwrap()
        .0
}

fn make_yarn_from(tour: &mut Tour, verts: &Verts) -> Yarn {
    Yarn::from(
        tour.iter()
            .map(|&n| [verts[n as usize].0, verts[n as usize].1])
            .collect::<Vec<[Point; 2]>>(),
    )
}

fn color_yarn(a: &Yarn) -> Yarn {
    a.clone().dot(&ndarray::arr2(&[[-1, 0], [0, -1]])) + ndarray::arr2(&[[0, 2]])
}

fn wind_threads(loom: &mut Loom, verts: &Verts, vi_map: &VIMap) -> Bobbins {
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
    let node_yarn: Tour = prepare_yarn(
        spool[&(zlevel % 4 + 4).try_into().unwrap()].clone(),
        zlevel,
        order,
        vi_map,
    );
    if bobbins.is_empty() {
        vec![node_yarn]
    } else {
        cut_yarn(node_yarn, bobbins)
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

pub fn cut_yarn(tour: Tour, subset: &Bobbins) -> Subtours {
    let mut subtours: Subtours = Vec::new();
    let last_ix: Idx = tour.len() - 1;
    let last_idx: Idx = subset.len() - 1;
    let mut prev: i32 = -1_i32;
    for (e, idx) in subset
        .iter()
        .filter_map(|node| tour.iter().position(|&x| x == *node))
        .sorted()
        .enumerate()
    {
        if e == last_idx && idx != last_ix {
            if let Some(first_slice) = tour.get(prev as usize + 1..idx) {
                if !first_slice.is_empty() {
                    subtours.push(if subset.contains(&first_slice[0]) {
                        first_slice.to_vec()
                    } else {
                        first_slice.iter().rev().cloned().collect()
                    });
                }
            }
            if let Some(first_slice) = tour.get(idx..) {
                if !first_slice.is_empty() {
                    subtours.push(if subset.contains(&first_slice[0]) {
                        first_slice.to_vec()
                    } else {
                        first_slice.iter().rev().cloned().collect()
                    });
                }
            }
        } else {
            if let Some(first_slice) = tour.get(prev as usize + 1..=idx) {
                if !first_slice.is_empty() {
                    subtours.push(if subset.contains(&first_slice[0]) {
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

fn attach_warps_to_loom(loom: &mut Loom, warps: &Warps) -> Woven {
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

pub fn affix_unwoven_to_loom(loom: &mut Loom, warps: Warps, woven: Woven) {
    warps
        .iter()
        .enumerate()
        .filter_map(|(idx, seq)| {
            if woven.contains(&idx) {
                None
            } else {
                Some(seq)
            }
        })
        .for_each(|seq| loom.extend(vec![seq.clone().into_iter().collect()]));
}

pub fn reflect_loom(loom: &mut Loom, verts: &Verts, vi_map: &VIMap) {
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

pub fn weave_loom<'a>(
    mut weaver: Weaver,
    mut loom: Vec<Vec<u32>>,
    verts: &'a Verts,
    vi_map: &VIMap,
    max_xyz: Point,
) -> Solution {
    loom.iter_mut().for_each(|other| {
        let other_edges = weaver.make_edges_for(other);
        if let Some((m, n)) = (&weaver.get_edges()
            & &other_edges
                .iter()
                .flat_map(|(m, n)| {
                    create_edges(verts[*m as usize], verts[*n as usize], max_xyz, vi_map)
                })
                .collect())
            .into_iter()
            .next()
        {
            if let Some(weft_e) =
                (&create_eadjs(verts[m as usize], verts[n as usize], max_xyz, vi_map)
                    & &other_edges)
                    .into_iter()
                    .next()
            {
                weaver.join((m, n), weft_e, other);
            }
        }
    });
    weaver.get_nodes()
}
