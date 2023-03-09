use std::{iter::zip, collections::{HashMap, HashSet, VecDeque}};

use crate::{
    graph::types::Verts,
    structs::frozenedge::Frozen
};

pub type TourRef<'a> = Vec<&'a Verti16>;
pub type ThreadRef<'a> = VecDeque<&'a Verti16>;
pub type EdgeRef<'a> = (&'a Verti16, &'a Verti16);
pub type AdjacencyRef<'a> = HashMap<&'a Verti16, HashSet<&'a Verti16>>;

#[derive(Clone, Debug)]
pub struct Cycle<'a> {
    data: TourRef<'a>,
    prev: TourRef<'a>,
    _eadjs: HashSet<Frozen<'a>>,
    _edges: HashSet<Frozen<'a>>,
    verts: &'a Verts,
    adj: &'a AdjacencyRef<'a>,
}



impl<'a> Cycle<'a> {
    pub fn new(
        data: &'a ThreadRef,
        adj: &'a AdjacencyRef,
        verts: &'a Verts,
    ) -> &'a mut Cycle<'a> {
        let cycle = Cycle {
            data: data.iter().cloned().collect::<TourRef>(),
            prev: TourRef::new(),
            _eadjs: HashSet::new(),
            _edges: HashSet::new(),
            verts,
            adj,
        };
        Box::leak(Box::new(cycle))
    }

    pub fn new_from_vec(
        data: &TourRef<'a>,
        adj: &'a AdjacencyRef,
        verts: &'a Verts,
    ) -> &'a mut Cycle<'a> {
        let cycle = Cycle {
            data: data.iter().cloned().collect::<TourRef>(),
            prev: TourRef::new(),
            _eadjs: HashSet::new(),
            _edges: HashSet::new(),
            verts,
            adj,
        };
        Box::leak(Box::new(cycle))
    }

    pub fn retrieve(&self) -> Vec<Verti16> {
        self.data
            .iter()
            .map(|v| **v)
            .collect()
    }

    pub fn rotate_to_edge(&mut self, left: &Verti16, right: &Verti16) {
        if left == *self.data.last().unwrap() && right == *self.data.first().unwrap() {
            self.data.reverse();
        } else {
            match (
                self.data.iter().position(|&x| x == left).unwrap(),
                self.data.iter().position(|&x| x == right).unwrap(),
            ) {
                (ixl, ixr) if ixl < ixr => {
                    self.data.rotate_left(ixr);
                    self.data.reverse()
                }
                (ixl, _) => self.data.rotate_left(ixl),
            }
        }
    }

    pub fn join(&mut self, edge: &EdgeRef, oedge: &EdgeRef, other: &'a mut Cycle) {
        self.rotate_to_edge(edge.0, edge.1);
        let reversed = !self.adj.get(&edge.1).unwrap().contains(&oedge.0);
        other.rotate_to_edge(
            if reversed { oedge.1 } else { oedge.0 },
            if reversed { oedge.0 } else { oedge.1 },
        );
        self.data.extend(&other.data);
    }

    pub fn join2(&mut self, edge: HashSet<&Verti16>, oedge: HashSet<&Verti16>, other: &mut Cycle<'a>) {
        let e_edge: Vec<&Verti16> = edge.into_iter().collect::<Vec<_>>();
        let o_edge: Vec<&Verti16> = oedge.into_iter().collect::<Vec<_>>();
        self.rotate_to_edge(e_edge[0], e_edge[1]);
        let reversed = !self.adj.get(&e_edge[1]).unwrap().contains(&o_edge[0]);
        other.rotate_to_edge(
            if reversed { o_edge[1] } else { o_edge[0] },
            if reversed { o_edge[0] } else { o_edge[1] },
        );
        let mut other_data = other.data.iter().map(|edge| edge.clone()).collect::<Vec<_>>();
        self.data.append(&mut other_data);
    }
    
    pub fn make_edges(&self) -> HashSet<(&Verti16, &Verti16)> {
        zip(
            self.data.clone(),
            [&self.data[1..], &self.data[..1]].concat(),
        )
        .map(|(a, b)| if a < b { (a, b) } else { (b, a) })
        .collect::<HashSet<_>>()
    }

    pub fn get_eadjs<'v, 'b, 'c>(
        &'a mut self, 
        edge_adjs: &'a HashMap<&'a Frozen, HashSet<&'a (&'a Frozen, &'a Frozen)>>
    ) -> HashSet<&'a (&'a Frozen, &'a Frozen)>
        where 'a: 'b, 'b: 'c, 'c: 'a
    {
        self.get_edges().clone()
            .iter()
            .flat_map(|edge| edge_adjs.get(&edge).unwrap().iter().map(|&t| t))
            .collect::<HashSet<&'a (&'a Frozen, &'a Frozen)>>()

    }

    pub fn get_edges(&mut self) -> HashSet<Frozen> {
        if self.prev != self.data {
            self._edges = zip(
                self.data.clone(),
                [&self.data[1..], &self.data[..1]].concat(),
            )
            .into_iter()
            // problem is that we are wanting a reference to the edges. which is kind 
            // inconvenient as we'll have to get the referencdes and just can't construct it.
            
            .map(|edge| Frozen::new(edge))
            .filter(| frozen | frozen.is_valid_edge())
            .collect::<HashSet<Frozen>>();
            self.prev = self.data.clone()
        }
        self._edges.clone()
    }

}

pub fn minmax_edge<'a>((m, n): (&'a Verti16, &'a Verti16)) -> (&'a Verti16, &'a Verti16) {
    if m < n {
        (m, n)
    } else {
        (n, m)
    }
}