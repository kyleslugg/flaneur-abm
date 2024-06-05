use krabmaga::engine::fields::network::{Edge, EdgeOptions, Network};
use osmpbf::HeaderBBox;

use super::import::read_osm;

use super::{StreetEdgeLabel, StreetNode};

pub type StreetNetwork = Network<StreetNode, StreetEdgeLabel>;
pub struct StreetNetworkSpec {
    pub network: StreetNetwork,
    pub dim: (f32, f32),
}

pub enum StreetNetworkError {
    Parse(osmpbf::Error),
}

pub fn street_network_from_osm(filepath: &str) -> Result<StreetNetworkSpec, StreetNetworkError> {
    match read_osm(filepath) {
        Ok(osm_spec) => {
            // Generate StreetNodes from osm_spec's nodes
            let nodes: Vec<StreetNode> = osm_spec.nodes.iter().map(|n| (*n).into()).collect();

            // Generate edges from osm_spec's ways
            let edges: Vec<Edge<StreetEdgeLabel>> = osm_spec.ways.iter().map(|e| {
                                                            e.as_edges()}).reduce(|acc, el|{
                                                            acc.into_iter().chain(el.into_iter()).collect()
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
            let mut dim: (f32, f32) = (0.0, 0.0);
            if let Some(bbox) = osm_spec.bounding_box {
                let HeaderBBox {
                    left,
                    right,
                    top,
                    bottom,
                } = bbox;
                dim = ((right - left) as f32, (top - bottom) as f32);
            };

            Ok(StreetNetworkSpec { network, dim })
        }

        Err(e) => return Err(StreetNetworkError::Parse(e)),
    }
}
