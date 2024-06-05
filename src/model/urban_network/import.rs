use std::{collections::HashSet, hash::Hash};

use geo::{HaversineDistance, Point};
use krabmaga::engine::{
    fields::network::{Edge, EdgeOptions},
    location::Real2D,
};
use osmpbf::{BlobReader, Element, HeaderBBox, IndexedReader};

use crate::model::urban_network::{edge::StreetEdgeLabel, node::StreetNode};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Clone)]
pub struct OsmSegmentInfo {
    u_id: i64,
    v_id: i64,
    length: f64,
}

#[derive(Clone)]
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

pub struct Coord {
    x: f64,
    y: f64,
}

pub struct OsmNetworkComponents {
    pub nodes: HashSet<OsmNodeInfo>,
    pub ways: Vec<OsmWayInfo>,
    pub bounding_box: Option<HeaderBBox>,
}

impl OsmNetworkComponents {
    pub fn new() -> Self {
        OsmNetworkComponents {
            nodes: HashSet::new(),
            ways: Vec::new(),
            bounding_box: None,
        }
    }
}

pub fn read_osm(filepath: &str) -> Result<OsmNetworkComponents, osmpbf::Error> {
    let mut bbox: Option<HeaderBBox> = None;

    if let Ok(mut reader) = BlobReader::from_path(filepath) {
        for blob_res in reader {
            if let Ok(hblock) = blob_res.and_then(|blob| blob.to_headerblock()) {
                bbox = hblock.bbox()
            }
        }
    };

    match IndexedReader::from_path(filepath) {
        Ok(mut reader) => {
            let mut components = OsmNetworkComponents::new();

            let res = reader.read_ways_and_deps(
                |_| true,
                |element| match element {
                    Element::Node(n) => {
                        let new_node = OsmNodeInfo {
                            id: n.id(),
                            nano_lat: n.nano_lat(),
                            nano_lon: n.nano_lon(),
                        };
                        components.nodes.insert(new_node);
                    }
                    Element::DenseNode(n) => {
                        let new_node = OsmNodeInfo {
                            id: n.id(),
                            nano_lat: n.nano_lat(),
                            nano_lon: n.nano_lon(),
                        };
                        components.nodes.insert(new_node);
                    }
                    Element::Way(w) => {
                        // Get segment info
                        let mut segments = Vec::<OsmSegmentInfo>::new();
                        let node_locs = w.node_locations();
                        match w.node_locations().peekable().peek() {
                            None => {
                                panic!(
                            "Unable to load OSM file: HeaderBlock does not specify node locations"
                        );
                            }
                            Some(_) => {
                                let full_nodes = w.refs().zip(node_locs);
                                full_nodes.clone().zip(full_nodes.skip(1)).for_each(
                                    |((u, uloc), (v, vloc))| {
                                        let segment_dist =
                                            Point::new(uloc.lon(), uloc.lat()).haversine_distance(
                                                &Point::new(vloc.lon(), vloc.lat()),
                                            );
                                        segments.push(OsmSegmentInfo {
                                            u_id: u,
                                            v_id: v,
                                            length: segment_dist,
                                        });
                                    },
                                )
                            }
                        }

                        let new_way = OsmWayInfo {
                            id: w.id(),
                            node_ids: w.refs().collect(),
                            segments,
                        };
                        components.ways.push(new_way);
                    }
                    Element::Relation(_) => {}
                },
            );

            match res {
                Ok(_) => {
                    components.bounding_box = bbox;
                    Ok(components)
                }
                Err(e) => Err(e),
            }
        }

        Err(e) => Err(e),
    }
}
