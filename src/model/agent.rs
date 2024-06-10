use std::fmt::Display;

use crate::model::urban_network::StreetNetworkPosition;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::fields::field_2d::Location2D;
use krabmaga::engine::location::Real2D;
use krabmaga::engine::state::State;
use krabmaga::rand::rngs::ThreadRng;

use crate::UrbanNetworkState;

use super::urban_network::StreetNode;

// #[derive(Clone)]
// pub struct AgentEncounter {
//     id: u32,
//     loc: AgentLoc,
// }

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct PedAgent {
    pub id: u32,
    pub loc: StreetNetworkPosition,
    pub dest: Option<StreetNetworkPosition>,
    pub path: Option<Vec<StreetNode>>, //pub status: AgentStatus,
                                       //pub encounters: Vec<AgentEncounter>,
}

impl PedAgent {
    pub fn new(id: u32, init_loc: StreetNetworkPosition) -> Self {
        PedAgent {
            id,
            loc: init_loc,
            dest: None,
            path: None, // status: init_status,
                        // encounters: Vec::<AgentEncounter>::new(),
        }
    }

    pub fn update_network_loc(&mut self, state: &mut UrbanNetworkState) {
        // Get random number gen from state
        let rng = ThreadRng::default();

        // Placeholder for next location
        let next_loc = Some(StreetNetworkPosition::default());

        // If agent has changed location, reassign to new location
        if let Some(loc) = next_loc {
            // Set own location to new location
            self.loc = loc;
        }
    }
}

impl Agent for PedAgent {
    fn step(&mut self, state: &mut dyn State) {
        println!("Agent {} on step {}", self.id, "???");
        let state = state
            .as_any_mut()
            .downcast_mut::<UrbanNetworkState>()
            .unwrap();

        // // Check and see if agent can/will move; if so, update location

        match self.dest {
            Some(destination) => {}
            None => {}
        }
        // self.update_network_loc(state);
    }
}

impl Eq for PedAgent {}

impl Location2D<Real2D> for PedAgent {
    fn get_location(self) -> Real2D {
        todo!()
    }

    fn set_location(&mut self, loc: Real2D) {
        todo!()
    }
}

impl Display for PedAgent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rep = format!(
            "ID: {}\nCurrent Location: {}\nDestination: {:?}\n",
            self.id, self.loc, self.dest
        );
        f.write_str(rep.as_str())
    }
}
