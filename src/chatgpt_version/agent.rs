use krabmaga::engine::agent::Agent;
use krabmaga::engine::location::Real2D;
use krabmaga::engine::schedule::Schedule;
use petgraph::algo::astar;
use petgraph::graph::NodeIndex;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone)]
struct MyAgent {
    position: Real2D,
    destination: Real2D,
    path: Vec<NodeIndex>,
}

impl Agent for MyAgent {
    fn step(&mut self, state: &mut dyn krabmaga::engine::state::State) {
        if let Some(next_node) = self.path.pop() {
            // Move towards the next node
            // ...
        }
    }
}

fn place_agent_randomly(network: &Network, rng: &mut impl Rng) -> MyAgent {
    let edge_indices: Vec<_> = network.graph.edge_indices().collect();
    let random_edge = edge_indices.choose(rng).expect("Network has no edges");
    let (source, target) = network
        .graph
        .edge_endpoints(*random_edge)
        .expect("Invalid edge");

    let source_point = network.graph[source];
    let target_point = network.graph[target];

    let t: f64 = rng.gen();
    let x = source_point.x() + t * (target_point.x() - source_point.x());
    let y = source_point.y() + t * (target_point.y() - source_point.y());

    MyAgent {
        position: Real2D { x, y },
        destination: Real2D { x: 0.0, y: 0.0 }, // Set a meaningful destination
        path: Vec::new(),
    }
}

fn calculate_path(network: &Network, start: NodeIndex, end: NodeIndex) -> Vec<NodeIndex> {
    if let Some((_, path)) = astar(
        &network.graph,
        start,
        |finish| finish == end,
        |e| *e.weight(),
        |_| 0.0,
    ) {
        path
    } else {
        Vec::new()
    }
}
