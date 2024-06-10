use crate::model::agent::PedAgent;
use crate::model::urban_network::node::StreetNode;
use crate::model::urban_network::{
    street_network_from_osm, StreetEdgeLabel, StreetNetwork, StreetNetworkError,
    StreetNetworkPosition, StreetNetworkSpec,
};
use crate::INIT_EDGES;
use krabmaga::engine::fields::field::Field;
use krabmaga::engine::fields::field_2d::Field2D;
use krabmaga::engine::fields::network::{Edge, Network};
use krabmaga::engine::location::Real2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::rand::{distributions::Uniform, prelude::SliceRandom, rngs::ThreadRng, Rng};
use krabmaga::Distribution;
use krabmaga::{__Deref, rand};
use serde::de::value;
use std::any::Any;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug)]
pub enum UrbanNetworkStateError {
    OSMLoadingError(StreetNetworkError),
    OsmPbf(osmpbf::Error),
}

pub struct UrbanNetworkState {
    pub step: u64,
    //pub field: Field2D<PedAgent>,
    pub network: StreetNetwork,
    //pub osm_reader: Option<IndexedReader<File>>,
    pub discretization: f32,
    pub toroidal: bool,
    pub dim: (f32, f32),
    //pub num_nodes: u32,
    pub num_agents: u32,
    //pub rng: StdRng,
}

impl UrbanNetworkState {
    pub fn new(
        num_nodes: i64,
        dim: (f32, f32),
        num_agents: u32,
        d: f32,
        t: bool,
    ) -> UrbanNetworkState {
        let mut state = UrbanNetworkState {
            step: 0,
            //field: Field2D::new(dim.0, dim.1, d, t),
            network: StreetNetwork(Network::new(false)),
            discretization: d,
            toroidal: t,
            dim,
            //num_nodes,
            num_agents,
            //rng: StdRng::from_entropy(),
        };

        // Initialize Nodes and Network
        let mut rng = ThreadRng::default(); //&mut state.rng;
        let mut node_set = Vec::new();

        for node_id in 0_i64..num_nodes {
            let r1: f32 = rng.gen();
            let r2: f32 = rng.gen();

            let node = StreetNode::new(
                node_id,
                Real2D {
                    x: dim.0 * r1,
                    y: dim.1 * r2,
                },
            );
            //self.field1.set_object_location(node, node.loc);
            node_set.push(node);
            state.network.0.add_node(node);
            //schedule.schedule_repeating(Box::new(PedAgent), 0.0, 0);
        }
        state
            .network
            .0
            .preferential_attachment_BA(&node_set, INIT_EDGES);

        state
    }

    pub fn from_osm_file(
        filepath: &Path,
        num_agents: u32,
        discretization: f32,
        toroidal: bool,
    ) -> Result<UrbanNetworkState, UrbanNetworkStateError> {
        match street_network_from_osm(filepath) {
            Ok(network_spec) => {
                let StreetNetworkSpec { network, dim } = network_spec;
                return Ok(UrbanNetworkState {
                    step: 0,
                    //field: Field2D::new(dim.0, dim.1, discretization, toroidal),
                    network,
                    discretization,
                    toroidal,
                    dim,
                    num_agents,
                    //rng: StdRng::from_entropy(),
                });
            }
            Err(e) => return Err(UrbanNetworkStateError::OSMLoadingError(e)),
        };
    }
}

impl State for UrbanNetworkState {
    fn reset(&mut self) {
        self.step = 0;
        //self.field1 = Field2D::new(self.dim.0, self.dim.1, self.discretization, self.toroidal);
        //self.network = StreetNetwork(Network::new(false));
    }

    fn init(&mut self, schedule: &mut Schedule) {
        self.reset();

        let mut rng = ThreadRng::default();

        // Initialize Agents -- put partway down a random edge by first selecting a random edge and then randomizing position as a fraction of length

        let edge_list = self
            .network
            .0
            .edges
            .iter()
            .map(|cell| {
                cell.clone()
                    .into_inner()
                    .values()
                    .map(|val| val.clone())
                    .reduce(|acc, item| {
                        let mut new = acc;
                        new.extend(item.into_iter());
                        new
                    })
                    .unwrap_or_default()
            })
            .reduce(|acc, item| {
                let mut new = acc;
                new.extend(item.into_iter());
                new
            })
            .unwrap_or_default();

        // Place agents at a random point on a random street segment
        for agent_id in 0..self.num_agents {
            let starting_loc = StreetNetworkPosition::rand_from_edge_list(&edge_list, &mut rng);

            let agent = PedAgent::new(agent_id, starting_loc);
            print!("{:?}", &agent);
            schedule.schedule_repeating(Box::new(agent), 0.0, 0);
        }
    }

    fn update(&mut self, step: u64) {
        //self.field1.lazy_update();
        self.network.0.lazy_update();
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
