use krabmaga::engine::agent::Agent;
use krabmaga::engine::state::State;
use krabmaga::rand::rngs::ThreadRng;

use crate::UrbanNetworkState;

#[derive(Clone, Copy, Debug)]
pub struct AgentLoc {
    from_node: u32,
    to_node: u32,
    edge_dist: f32,
}

impl AgentLoc {
    pub fn new(from_node: u32, to_node: u32, edge_dist: f32) -> Self {
        AgentLoc {
            from_node,
            to_node,
            edge_dist,
        }
    }
}

impl Default for AgentLoc {
    fn default() -> Self {
        AgentLoc::new(0, 0, 0.0)
    }
}
// #[derive(Clone)]
// pub struct AgentEncounter {
//     id: u32,
//     loc: AgentLoc,
// }

#[derive(Clone, Debug)]
pub struct PedAgent {
    pub id: u32,
    pub loc: AgentLoc,
    pub dest: Option<AgentLoc>, //pub status: AgentStatus,
                                //pub encounters: Vec<AgentEncounter>,
}

impl PedAgent {
    pub fn new(id: u32, init_loc: AgentLoc) -> Self {
        PedAgent {
            id,
            loc: init_loc,
            dest: None, // status: init_status,
                        // encounters: Vec::<AgentEncounter>::new(),
        }
    }

    pub fn update_network_loc(&mut self, state: &mut UrbanNetworkState) {
        // Get random number gen from state
        let rng = ThreadRng::default();

        // Placeholder for next location
        let next_loc = Some(AgentLoc::default());

        // If agent has changed location, reassign to new location
        if let Some(loc) = next_loc {
            // Set own location to new location
            self.loc = loc;
        }
    }
}

impl Agent for PedAgent {
    fn step(&mut self, state: &mut dyn State) {
        let state = state
            .as_any_mut()
            .downcast_mut::<UrbanNetworkState>()
            .unwrap();

        // Check and see if agent can/will move; if so, update location
        self.update_network_loc(state);
    }
}
