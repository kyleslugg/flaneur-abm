use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    path::Path,
};

use geo::{HaversineDistance, Point};
use indicatif::ProgressBar;
use krabmaga::engine::{
    fields::network::{Edge, EdgeOptions},
    location::Real2D,
};
use osmpbf::{BlobReader, Element, HeaderBBox, IndexedReader};
use serde::{Deserialize, Serialize};

use crate::model::urban_network::{edge::StreetEdgeLabel, node::StreetNode};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct OsmNodeInfo {
    id: i64,
    nano_lat: i64,
    nano_lon: i64,
}
impl OsmNodeInfo {
    const NANO_DIVISOR: f64 = 1.0e9;
}

impl From<OsmNodeInfo> for StreetNode {
    fn from(val: OsmNodeInfo) -> Self {
        StreetNode::new(
            val.id,
            Real2D {
                x: (val.nano_lon as f64 / OsmNodeInfo::NANO_DIVISOR) as f32,
                y: (val.nano_lat as f64 / OsmNodeInfo::NANO_DIVISOR) as f32,
            },
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OsmSegmentInfo {
    u_id: i64,
    v_id: i64,
    length: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OsmWayInfo {
    id: i64,
    node_ids: Vec<i64>,
    segments: Vec<OsmSegmentInfo>,
}

impl OsmWayInfo {
    pub fn as_edges(&self) -> Vec<Edge<StreetEdgeLabel>> {
        let edges: Vec<Edge<StreetEdgeLabel>> = self
            .segments
            .iter()
            .map(|seg| {
                let edge_options = EdgeOptions::WeightedLabeled(
                    StreetEdgeLabel {
                        len: seg.length as f32,
                        id: self.id as u32,
                    },
                    seg.length as f32,
                );
                let u_node = seg.u_id as u32;
                let v_node = seg.v_id as u32;
                Edge::new(u_node, v_node, edge_options)
            })
            .collect();
        edges
    }
}

//This lets us derive Serialize for a remote struct by defining an identical local one.
#[derive(Serialize, Deserialize)]
#[serde(remote = "osmpbf::block::HeaderBBox")]
pub struct HeaderBBoxDef {
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
}

#[derive(Serialize, Deserialize)]
pub struct OsmNetworkComponents {
    pub nodes: HashSet<OsmNodeInfo>,
    pub ways: Vec<OsmWayInfo>,

    #[serde(with = "HeaderBBoxDef")]
    pub bounding_box: HeaderBBox,
}

impl OsmNetworkComponents {
    pub fn new() -> Self {
        OsmNetworkComponents {
            nodes: HashSet::new(),
            ways: Vec::new(),
            bounding_box: HeaderBBox {
                left: 0.0,
                right: 0.0,
                top: 0.0,
                bottom: 0.0,
            },
        }
    }
}

pub fn read_osm(filepath: &Path) -> Result<OsmNetworkComponents, osmpbf::Error> {
    let mut bbox: HeaderBBox;

    if let Ok(reader) = BlobReader::from_path(filepath) {
        for blob_res in reader {
            if let Ok(hblock) = blob_res.and_then(|blob| blob.to_headerblock()) {
                bbox = hblock.bbox().expect("File should contain bounding box")
            }
        }
    };

    match IndexedReader::from_path(filepath) {
        Ok(mut reader) => {
            let mut components = OsmNetworkComponents::new();
            let mut local_node_index: HashMap<i64, OsmNodeInfo> = HashMap::new();

            let res = reader.read_ways_and_deps(
                |_| true,
                |element| match element {
                    Element::Node(n) => {
                        let new_node = OsmNodeInfo {
                            id: n.id(),
                            nano_lat: n.nano_lat(),
                            nano_lon: n.nano_lon(),
                        };
                        local_node_index.insert(n.id(), new_node.clone());
                        components.nodes.insert(new_node);
                    }
                    Element::DenseNode(n) => {
                        let new_node = OsmNodeInfo {
                            id: n.id(),
                            nano_lat: n.nano_lat(),
                            nano_lon: n.nano_lon(),
                        };
                        local_node_index.insert(n.id(), new_node.clone());
                        components.nodes.insert(new_node);
                    }
                    Element::Way(w) => {
                        // Get segment info
                        let mut segments = Vec::<OsmSegmentInfo>::new();
                        let node_ids = w.refs();
                        match node_ids.clone().next() {
                            None => {
                                print!("Unable to load edge {}: no node references found", w.id());
                            }
                            Some(_) => {
                                let node_pairs = w.refs().zip(node_ids.clone().skip(1));
                                node_pairs.for_each(|(u, v)| {
                                    segments.push(OsmSegmentInfo {
                                        u_id: u,
                                        v_id: v,
                                        length: -1.0,
                                    });
                                });
                                components.ways.push(OsmWayInfo {
                                    id: w.id(),
                                    node_ids: node_ids.collect(),
                                    segments,
                                })
                            }
                        }
                    }
                    Element::Relation(_) => {}
                },
            );

            // Wrap processing step in progress bar
            println!("{}", "Processing way segment lengths...");
            let pb = ProgressBar::new(components.ways.len() as u64);

            pb.wrap_iter(components.ways.iter_mut()).for_each(|edge| {
                edge.segments.iter_mut().for_each(|segment| {
                    if let Some(u) = local_node_index.get(&segment.u_id) {
                        if let Some(v) = local_node_index.get(&segment.v_id) {
                            let segment_dist = Point::new(
                                (u.nano_lon as f64 / 1e9_f64),
                                (u.nano_lat as f64 / 1e9_f64),
                            )
                            .haversine_distance(&Point::new(
                                (v.nano_lon as f64 / 1e9_f64),
                                (v.nano_lat as f64 / 1e9_f64),
                            ));

                            segment.length = segment_dist;
                        } else {
                            print!("Failed to calculate distance on segment {} - {}: node {} not found.", segment.u_id, segment.v_id, segment.v_id)
                        }
                    }
                });
            });

            Ok(components)
        }

        Err(e) => Err(e),
    }
}
