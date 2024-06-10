use geo_types::{Coordinate, Point};
use krabmaga::engine::field::network::Network;
use osmpbfreader::{OsmObj, OsmPbfReader};
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use rand::Rng;
use std::fs::File;

fn read_osm_pbf(file_path: &str) -> Result<Network, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let mut pbf = OsmPbfReader::new(file);

    let mut graph = Graph::<Point<f64>, f64, Undirected>::new();
    let mut node_map = std::collections::HashMap::new();

    for obj in pbf.par_iter().filter_map(Result::ok) {
        match obj {
            OsmObj::Node(node) => {
                let point = Point::new(node.lon(), node.lat());
                let idx = graph.add_node(point);
                node_map.insert(node.id, idx);
            }
            OsmObj::Way(way) => {
                for window in way.nodes.windows(2) {
                    if let (Some(&from), Some(&to)) = (window.get(0), window.get(1)) {
                        if let (Some(&from_idx), Some(&to_idx)) =
                            (node_map.get(&from), node_map.get(&to))
                        {
                            graph.add_edge(from_idx, to_idx, 1.0);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Ok(Network { graph })
}
