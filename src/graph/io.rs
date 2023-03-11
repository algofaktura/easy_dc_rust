extern crate serde;

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

use super::types::{Adjacency, EdgeAdjacency, VIMap, Verts, ZOrder};

#[derive(Serialize, Deserialize)]
struct Vertex {
    x: i32,
    y: i32,
    z: i32,
}

pub fn serialize_data(
    fpath: &str,
    verts: Verts,
    vi_map: VIMap,
    adj: Adjacency,
    edge_adj: EdgeAdjacency,
    z_adj: Adjacency,
    z_order: ZOrder,
) -> Result<(), Box<dyn std::error::Error>> {
    // Serialize the data to binary format
    let order: u32 = verts.len() as u32;
    let src_dir = String::from(format!(
        "{fpath}/graph_{order}",
        fpath = fpath,
        order = order
    ));
    let mut file = File::create(src_dir)?;
    let encoded = serde_cbor::to_vec(&(verts, vi_map, adj, edge_adj, z_adj, z_order))?;
    file.write_all(&encoded)?;

    Ok(())
}

pub fn deserialize_data(order: u32, fpath: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read the serialized data from binary format
    let src_dir = String::from(format!(
        "{fpath}/graph_{order}",
        fpath = fpath,
        order = order
    ));
    let mut file = File::open(src_dir)?;
    let mut encoded = Vec::new();
    file.read_to_end(&mut encoded)?;

    // Deserialize the data
    let (__verts, _vi_map, _adj, _edge_adj, _z_adj, _z_order): (
        Verts,
        VIMap,
        Adjacency,
        EdgeAdjacency,
        Vec<usize>,
        usize,
    ) = serde_cbor::from_slice(&encoded)?;

    // Use the deserialized data
    // ...
    Ok(())
}
