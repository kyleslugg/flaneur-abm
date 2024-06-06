use krabmaga::hashbrown::{self, HashMap};
use serde_with::serde_as;
use std::cell::RefCell;
use std::fmt::Display;
use std::hash::Hash;
use std::path::Path;

use indicatif::ProgressBar;
use krabmaga::engine::fields::network::{Edge, EdgeOptions, Network};
use osmpbf::HeaderBBox;
use serde::{Deserialize, Serialize};

use super::import::read_osm;

use super::{StreetEdgeLabel, StreetNode};

#[derive(Serialize, Deserialize)]
#[serde(remote = "krabmaga::engine::fields::network::Edge")]
struct EdgeDef<L: Clone + Hash + Display> {
    pub u: u32,
    pub v: u32,
    pub label: Option<L>,
    pub weight: Option<f32>,
}

#[derive(Serialize, Deserialize)]
#[serde_as]
#[serde(remote = "krabmaga::engine::fields::network::Network")]
struct NetworkDef<O: Hash + Eq + Clone + Display, L: Clone + Hash + Display> {
    #[serde_as(as = "hashbrown::HashMap<DisplayFromStr, Vec<Edge<L>>>")]
    pub edges: Vec<RefCell<HashMap<u32, Vec<Edge<L>>>>>,
    pub read: usize,
    pub write: usize,
    #[serde(with = "krabmaga::hashbrown::HashMap")]
    pub nodes2id: Vec<RefCell<HashMap<O, u32>>>,
    #[serde(with = "krabmaga::hashbrown::HashMap")]
    pub id2nodes: Vec<RefCell<HashMap<u32, O>>>,
    pub direct: bool,
}

#[derive(Serialize, Deserialize)]

pub struct StreetNetwork(#[serde(with = "NetworkDef")] pub Network<StreetNode, StreetEdgeLabel>);

// impl Serialize for StreetNetwork {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         serializer.serialize_newtype_struct("StreetNetwork", {
//             let mut state =
//                 serializer.serialize_struct("Network<StreetNode, StreetEdgeLabel>", 6)?;
//             state.serialize_field("edges",{&self.0.edges});
//             &"placeholder"
//         })
//     }
// }
pub struct StreetNetworkSpec {
    pub network: StreetNetwork,
    pub dim: (f32, f32),
}

#[derive(Debug)]
pub enum StreetNetworkError {
    Parse(osmpbf::Error),
}

pub fn street_network_from_osm(filepath: &Path) -> Result<StreetNetworkSpec, StreetNetworkError> {
    match read_osm(filepath) {
        Ok(osm_spec) => {
            // Generate StreetNodes from osm_spec's nodes
            println!("{}", "Processing OSM nodes as KBM nodes...");
            let pb = ProgressBar::new(osm_spec.nodes.len() as u64);
            let nodes: Vec<StreetNode> = pb
                .wrap_iter(osm_spec.nodes.iter())
                .map(|n| (*n).into())
                .collect();

            // Generate edges from osm_spec's ways
            println!("{}", "Processing OSM ways as KBM edges...");
            let pb = ProgressBar::new(osm_spec.ways.len() as u64);
            let edges: Vec<Edge<StreetEdgeLabel>> = pb.wrap_iter(osm_spec.ways.iter()).map(|e| {
                                                            e.as_edges()}).reduce(|acc, el|{
                                                            acc.into_iter().chain(el).collect()
                                                        }).expect("If you've reached this point, your list of OSM segments is improperly formatted");

            // Instantiate network
            let network = Network::<StreetNode, StreetEdgeLabel>::new(true);
            nodes.into_iter().for_each(|n| {
                network.add_node(n);
            });
            edges.into_iter().for_each(|e| {
                let u = network
                    .get_object(e.u)
                    .expect("Node not present in previously loaded set");
                let v = network
                    .get_object(e.v)
                    .expect("Node not present in previously loaded set");
                network.add_edge(
                    u,
                    v,
                    EdgeOptions::WeightedLabeled(
                        e.label.expect("Missing edge label"),
                        e.weight.expect("Missing edge weight"),
                    ),
                );
            });

            // Calculate dimensions from bounding box
            let HeaderBBox {
                left,
                right,
                top,
                bottom,
            } = osm_spec.bounding_box;

            let dim = ((right - left) as f32, (top - bottom) as f32);

            Ok(StreetNetworkSpec {
                network: StreetNetwork(network),
                dim,
            })
        }

        Err(e) => Err(StreetNetworkError::Parse(e)),
    }
}
