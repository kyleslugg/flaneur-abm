use crate::model::agent::{AgentLoc, PedAgent};
use crate::model::edge::StreetEdgeLabel;
use crate::model::node::StreetNode;
use crate::INIT_EDGES;
use krabmaga::engine::fields::{field_2d::Field2D, field::Field};
use krabmaga::engine::fields::network::Network;
use krabmaga::engine::location::Real2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::rand;
use krabmaga::rand::{prelude::SliceRandom, rngs::StdRng, Rng};
use rand::SeedableRng;
use std::any::Any;
use std::collections::{HashMap, HashSet};

pub struct UrbanNetworkState {
    pub step: u64,
    //pub field: Field2D<PedAgent>,
    pub network: Network<StreetNode, StreetEdgeLabel>,
    pub discretization: f32,
    pub toroidal: bool,
    pub dim: (f32, f32),
    pub num_nodes: u32,
    pub num_agents: u32,
    pub rng: StdRng,
}

impl UrbanNetworkState {
    pub fn new(
        dim: (f32, f32),
        num_agents: u32,
        num_nodes: u32,
        d: f32,
        t: bool,
    ) -> UrbanNetworkState {
        UrbanNetworkState {
            step: 0,
            //field: Field2D::new(dim.0, dim.1, d, t),
            network: Network::new(false),
            discretization: d,
            toroidal: t,
            dim,
            num_nodes,
            num_agents,
            rng: StdRng::from_entropy(),
        }
    }

    pub fn load_network() {
        unimplemented!("This will load an external network rather than initializing one at random")
    }
}

impl State for UrbanNetworkState {
    fn reset(&mut self) {
        self.step = 0;
        //self.field1 = Field2D::new(self.dim.0, self.dim.1, self.discretization, self.toroidal);
        self.network = Network::new(false);
    }

    fn init(&mut self, schedule: &mut Schedule) {
        self.reset();

        let rng = &mut self.rng;
        let mut node_set = Vec::new();

        // Initialize Nodes and Network

        for node_id in 0..self.num_nodes {
            let r1: f32 = rng.gen();
            let r2: f32 = rng.gen();

            let node = StreetNode::new(
                node_id,
                Real2D {
                    x: self.dim.0 * r1,
                    y: self.dim.1 * r2,
                },
            );
            //self.field1.set_object_location(node, node.loc);
            node_set.push(node.clone());
            self.network.add_node(node);
            //schedule.schedule_repeating(Box::new(PedAgent), 0.0, 0);
        }
        self.network
            .preferential_attachment_BA(&node_set, INIT_EDGES);

        // Initialize Agents -- put halfway down a random edge
        for agent_id in 0..self.num_agents {
            let init_loc = node_set.choose(rng).and_then(|u| {
                self.network.get_edges(*u).and_then(|edges| {
                    edges.choose(rng).and_then(|edge| {
                        let this_u = u.id;
                        let this_v = if (edge.v == u.id) {
                            edge.u } else {edge.v};
                           
                        let edge_dist = edge.label.expect("If you've gotten this far without providing an edge label, I don't know how to help you. Label should be of the type StreetEdgeLabel.").len;
                        Some(AgentLoc::new(this_u, this_v, edge_dist/2.0))
                    })
                })
            });
            let agent = PedAgent::new(agent_id, init_loc.unwrap_or_default());
            schedule.schedule_repeating(Box::new(agent), 0.0, 0);
        }
    }

    fn update(&mut self, step: u64) {
        //self.field1.lazy_update();
        self.network.update();
        self.step = step;
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_state_mut(&mut self) -> &mut dyn State {
        self
    }

    fn as_state(&self) -> &dyn State {
        self
    }

    fn after_step(&mut self, _schedule: &mut Schedule) {
        // let mut susceptible: usize = 0;
        // let mut infected: usize = 0;
        // let mut resistant: usize = 0;
        // let agents = schedule.get_all_events();

        // for n in agents {
        //     let agent = n.downcast_ref::<NetNode>().unwrap();
        //     match agent.status {
        //         NodeStatus::Susceptible => {
        //             susceptible += 1;
        //         }
        //         NodeStatus::Infected => {
        //             infected += 1;
        //         }
        //         NodeStatus::Resistant => {
        //             resistant += 1;
        //         }
        //     }
        // }
        // println!(
        //     "Susceptible: {:?} Infected: {:?} Resistant: {:?} Tot: {:?}",
        //     susceptible,
        //     infected,
        //     resistant,
        //     susceptible + infected + resistant
        // );
    }
}
