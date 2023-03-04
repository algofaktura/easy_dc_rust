use quote;
use syn;

use crate::operators::weave::weave;
use crate::types::types::*;

pub fn solve(_order: u32, adj: &Adjacency, v3verts: &Vectors3d, vert_idx: &VertIdx, edge_adj: &EdgeAdjacency, verts: &[(i32, i32, i32)], var: &[[i32; 3]]) -> Solution {
    // preface: loads appropriate graph based on _order. right now looks empty. 
    weave(&v3verts, &adj, &vert_idx, &edge_adj, verts, var) 
}

use quote::{quote, format_ident};
use syn::parse_str;

#[macro_export]
macro_rules! eval_path {
    ($order:expr, $field:ident) => {{
        let path_str = format!("crate::graphs::data::g_{}::{}", $order, stringify!($field));
        let path: syn::Path = parse_str(&path_str).unwrap();
        quote! { #path }
    }};
}

// #[macro_export]
// macro_rules! eval_path {
//     ($order:expr, $field:ident, $graph_module_path:ident) => {{
//         let graph_mod_path = stringify!($graph_module_path);
//         let graph_mod = module_path!();
//         let path_str = if graph_mod == graph_mod_path {
//             concat!("crate::graphs::data::g_", stringify!($order), "::", stringify!($field))
//         } else {
//             concat!(stringify!($graph_module_path), "::g_", stringify!($order), "::", stringify!($field))
//         };
//         syn::parse_str::<syn::Path>(path_str).unwrap()
//     }};
// }

pub fn solve2(order: u32, repeats: u32) {
    // Construct the path to the appropriate graph module based on the order input
    let graph_module_path = format!("crate::graphs::data::g_{}::", order);

    let adj: Adjacency = map_graph(&eval_path!(order, ADJ));
    let v3verts: Vectors3d = vectorize(&eval_path!(order, VERTS));
    let vert_idx: VertIdx = make_vi_mapping(&v3verts);
    let edge_adj: EdgeAdjacency = make_edges_adj(&adj, &eval_path!(graph_module_path, EDGES).iter().cloned().collect::<Edges>());

    let mut solution: Solution = Solution::new();
    let start: Instant = Instant::now();
    for _ in 0..repeats { solution = weave(&v3verts, &adj, &vert_idx, &edge_adj, eval_path!(order, VERTS), &eval_path!(order, VAR)) }
    elapsed_ms(start, Instant::now(), repeats, "WEAVE");

    let id: SequenceID = id_seq(&solution, &adj);
    assert_eq!(HamCycle, id);
    println!("{:?}", id);
    println!("⭕️ ORDER: {:?} | ID: {:?} | {:?}", order, id, solution.len());
}