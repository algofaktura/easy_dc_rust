use std::iter::zip;

use super::{
    make::is_valid_edge,
    types::{Adjacency, Edge, EdgeAdjacency, Edges, Solution, Thread, Tour, Verts},
    utils::orient,
};

/// Cycle struct provides functionality that allows instances to be joined together.
#[derive(Clone, Debug)]
pub struct Cycle<'a> {
    pub data: Tour,
    verts: &'a Verts,
    adj: &'a Adjacency,
    edge_adj: &'a EdgeAdjacency,
}

impl<'a> Cycle<'a> {
    pub fn new(
        data: &Thread,
        adj: &'a Adjacency,
        edge_adj: &'a EdgeAdjacency,
        verts: &'a Verts,
    ) -> Box<Cycle<'a>> {
        let cycle = Cycle {
            data: data.iter().cloned().collect::<Tour>(),
            verts,
            adj,
            edge_adj,
        };
        Box::new(cycle)
    }

    /// If self.data is represented as a series of edges:
    /// ```
    /// data = [0, 1, 2, 3]
    /// data_as_edges = [(0, 1), (1, 2), (2, 3), (3, 0)]
    /// ```
    /// its adjacent edges are those edges that are adjacent to each edge:
    /// ```
    /// let adjacent_edges = edge_adj[&data_as_edges[0]];
    /// ```
    /// These are the edges that are adjacent to self in other.
    pub fn eadjs(&mut self) -> Edges {
        self.edges()
            .iter()
            .flat_map(|edge| self.edge_adj[edge].iter())
            .copied()
            .collect()
    }

    /// Data represented as a sliding window of two positions incrementing one step and wraps the end to the beginning:
    /// ```
    /// data = [0, 1, 2, 3]
    /// data_as_edges = [(0, 1), (1, 2), (2, 3), (3, 0)]
    /// ```
    /// Contained in a HashSet allowing for quick intersection operations to determine edge adjacency.
    pub fn edges(&mut self) -> Edges {
        zip(
            self.data.clone(),
            [&self.data[1..], &self.data[..1]].concat(),
        )
        .into_iter()
        .map(|(a, b)| orient(a, b))
        .filter(|&(a, b)| is_valid_edge(self.verts[a as usize], self.verts[b as usize]))
        .collect()
    }

    /// Retrieve the data
    pub fn retrieve_nodes(&self) -> Solution {
        self.data.to_vec()
    }

    /// Retrieve the data but as vectors.
    pub fn retrieve_vectors(&self) -> Verts {
        self.data
            .to_vec()
            .iter()
            .map(|node| self.verts[*node as usize])
            .collect()
    }

    /// If any edges of two instances of the Cycle struct are adjacent, join them using a reference edge for each sequence.
    /// Each edge represents a connection between the core cord and other.
    /// The core cord is rotated so that it aligns with its reference edge.
    /// The reference edge of other is flipped so that the end of lead edge is adjacent to the beginning of the other edge.
    /// The other sequence is rotated accordingly and then joined with the lead loop to form one loop.
    ///
    ///
    /// 1: Rotate core_code so that the ends of core_code align to the edge. The edge is assumed to be aligned correctly as is.
    /// ```
    /// let edge = (5, 1);
    /// let cycle_to_rotate = Cycle::new([3, 1, 5, 6, 8, 9, 2], adj, edge_adj, verts);
    /// let result = cycle_to_rotate.rotate_to_edge(2, 3);
    /// assert_eq!(result, [5, 6, 8, 9, 2, 3, 1])
    /// ```
    /// 2: Oedge is rotated so that the rhs side of edge is adjacent to the lhs side of oedge. Using edges as reductions for larger sequences, we chain the two edges together. This can then be used to rotate the sequence accordingly.
    /// ```
    /// // GIVEN THIS SQUARE GRAPH:
    /// // 0 ― 1
    /// // │   │
    /// // 2 ― 3
    ///
    /// let adjacency = hash_map! {
    ///     0 => hash_set!{ 1, 2 }
    ///     1 => hash_set!{ 0, 3 }
    ///     2 => hash_set!{ 0, 3 }
    ///     3 => hash_set!{ 1, 2 }
    /// }
    ///
    /// let edge = (0, 2);
    /// let oedge = (1, 3);
    ///
    /// match edge {
    ///     (edge_left, edge_right) => match oedge {
    ///         (other_left, other_right) => {
    ///             if edge_right == other_left {
    ///                 println!("Edge is already aligned let's return as is");
    ///                 oedge
    ///             } else if edge_right == other_right {
    ///                 (other_left, other_right);
    ///                 println!("edge_right and other_right are adjacent -> reverse_edge");
    ///             } else if edge_left == other_left {
    ///                 (other_left, other_right);
    ///                 println!("edge_left and other_left are adjacent -> reverse_edge");
    ///             } else {
    ///                 panic!("No match found!");
    ///             }
    ///         }
    ///     }
    /// }
    ///
    /// ```
    ///
    /// 3: Similar to what was done to the core_cord in step 1, the other.data is aligned to its edge which was aligned:
    /// ```
    /// let oedge = (1, 4);
    /// let cycle_to_rotate = Cycle::new([5, 8, 1, 4, 3, 2, 9], adj, edge_adj, verts);
    /// let result = cycle_to_rotate.rotate_to_edge(oedge.0, oedge.1);
    /// assert_eq!(result, [1, 8, 5, 9, 2, 3, 4])
    /// ```
    ///
    /// 4: When other.data and main.data are aligned accordingly in the steps above, the other.data is extended to the back of main.data:
    /// ```
    /// self.data.extend(&other.data);
    /// ```
    ///
    /// # Panics
    ///
    /// This function will eventually panic if the edge and oedge aren't parallel edges.
    /// But that check is actually done in the creating of the edges.
    ///
    ///
    /// # Complexity
    ///
    /// Unsure
    ///
    /// # Example from code:
    ///
    /// Iterate through wefts in loom and if the edges of core_cord intersect with the
    /// eadjs (adjacent edges) of other, join them.
    /// ```
    /// for key in loom.keys() {
    ///     let other = &mut loom[key].borrow_mut();
    ///     if let Some(warp_e) = core_cord.edges().intersection(&other.eadjs()).next() {
    ///         if let Some(weft_e) = edge_adj
    ///            .get(warp_e)
    ///           .unwrap()
    ///             .intersection(&other.edges())
    ///             .next()
    ///         {
    ///             // JOIN CODE WITH OTHER
    ///             core_cord.join(*warp_e, *weft_e, other);
    ///             key_to_remove.push(*key);
    ///             break;
    ///         }
    ///     }
    /// }
    /// ```
    pub fn join(&mut self, edge: Edge, oedge: Edge, other: &mut Cycle) {
        self.rotate_to_edge(edge.0, edge.1);
        let reversed = !self.adj[&edge.1].contains(&oedge.0);
        other.rotate_to_edge(
            if reversed { oedge.1 } else { oedge.0 },
            if reversed { oedge.0 } else { oedge.1 },
        );
        self.data.extend(&other.data);
    }

    /// 1: Rotate self.data so that the ends of self.data align to the edge. The lhs of edge should be the lhs of self.data.
    /// The rhs of edge should be the rhs of self.data
    /// ```
    /// let edge = (5, 1);
    /// let cycle_to_rotate = Cycle::new([3, 1, 5, 6, 8, 9, 2], adj, edge_adj, verts);
    /// let result = cycle_to_rotate.rotate_to_edge(2, 3);
    /// assert_eq!(result, [5, 6, 8, 9, 2, 3, 1])
    /// ```
    pub fn rotate_to_edge(&mut self, lhs: u32, rhs: u32) {
        if lhs == self.data[self.data.len() - 1] && rhs == self.data[0] {
            self.data.reverse();
        } else {
            match (
                self.data.iter().position(|&x| x == lhs).unwrap(),
                self.data.iter().position(|&x| x == rhs).unwrap(),
            ) {
                (idx_lhs, idx_rhs) if idx_lhs < idx_rhs => {
                    self.data.rotate_left(idx_rhs);
                    self.data.reverse()
                }
                (idx_lhs, _) => self.data.rotate_left(idx_lhs),
            }
        }
    }
}
