// extern crate serde_json;
// use std::collections::HashMap;
// use std::collections::HashSet;

// use crate::find_solution_var;

// use super::convert;
// use super::make;
// use super::shrink;
// use super::types::*;
// use super::utils;

// pub type VIMapping<K, V> = HashMap<K, V>;
// pub type Vectors<T> = Vec<T>;
// pub type Graph<T, V> = (
//     u32,
//     u32,
//     Vectors<T>,
//     VIMapping<T, V>,
//     Adjacency,
//     EdgeAdjacency,
//     Adjacency,
//     ZOrder,
// );

// pub trait GraphMaker<T, V> {
//     fn make_graph(&self, order: u32, repeats: u32) -> Graph<T, V>;
// }

// struct GraphMakerTuple;
// impl GraphMaker<(i32, i32, i32), u32> for GraphMakerTuple {
//     fn make_graph(&self, order: u32, repeats: u32) -> Graph<(i32, i32, i32), u32> {
//         let max_xyz = utils::get_max_xyz(order as i32);
//         let verts: Vec<(i32, i32, i32)> = make::vertices(max_xyz);
//         let vi_map: HashMap<(i32, i32, i32), u32> = make::vi_map(&verts);
//         let adj: HashMap<u32, HashSet<u32>> = make::adjacency_map(&verts, max_xyz, &vi_map);
//         let edge_adj: HashMap<(u32, u32), HashSet<(u32, u32)>> =
//             make::edges_adjacency_mapping(&adj, &verts);
//         let (z_adj, z_order) = shrink::adjacency(&verts, &adj);
//         (order, repeats, verts, vi_map, adj, edge_adj, z_adj, z_order)
// }}

// struct GraphMakerArray;
// impl GraphMaker<[i32; 3], u32> for GraphMakerArray {
//     fn make_graph(&self, order: u32, repeats: u32) -> Graph<[i32; 3], u32> {
//         let max_xyz = utils::get_max_xyz(order as i32);
//         let verts: Vec<(i32, i32, i32)> = make::vertices(max_xyz);
//         let verts: Varr = convert::from_verts_to_vertsc(&verts);
//         let vi_map: VIMapVar = make::vi_map_var(&verts);
//         let adj: HashMap<u32, HashSet<u32>> = make::adjacency_map_var(&verts, max_xyz, &vi_map);
//         let edge_adj: HashMap<(u32, u32), HashSet<(u32, u32)>> =
//             make::edges_adjacency_mapping_var(&adj, &verts);
//         let (z_adj, z_order) = shrink::adjacency_var(&verts, &adj);
//         (order, repeats, verts, vi_map, adj, edge_adj, z_adj, z_order)
//     }
// }

// struct Solver<T, V> {
//     graph_maker: Box<dyn GraphMaker<T, V>>,
//     find_solution_fn: fn(Graph<T, V>),
// }

// impl<T, V> Solver<T, V> {
//     fn new(
//         graph_maker: Box<dyn GraphMaker<T, V>>,
//         find_solution_fn: fn(Graph<T, V>),
//     ) -> Solver<T, V> {
//         Solver {
//             graph_maker,
//             find_solution_fn,
//         }
//     }

//     fn solve(&self) {
//         let graph = self.graph_maker.make_graph(79040, 100);
//         (self.find_solution_fn)(graph);
//     }
// }

// fn main() {
//     let do_var = true;
//     let solver: Solver<[i32;3], u32> = if do_var {
//         Solver::new(Box::new(GraphMakerArray), find_solution_var)
//     } else {
//         Solver::new(Box::new(GraphMakerTuple), find_solution)
//     };
//     solver.solve();
// }
