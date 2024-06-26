use std::{
    fmt,
    hash::{Hash, Hasher},
};

use krabmaga::engine::{fields::field_2d::Location2D, location::Real2D};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(remote = "krabmaga::engine::location::Real2D")]
pub struct Real2DDef {
    pub x: f32,
    pub y: f32,
}
#[derive(Copy, Clone, Eq, Default, Debug, Serialize, Deserialize)]
pub struct StreetNode {
    pub osm_id: i64,
    #[serde(with = "Real2DDef")]
    pub loc: Real2D,
}

impl StreetNode {
    pub fn new(id: i64, loc: Real2D) -> Self {
        StreetNode { osm_id: id, loc }
    }
}

impl Hash for StreetNode {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.osm_id.hash(state);
    }
}

impl PartialEq for StreetNode {
    fn eq(&self, other: &StreetNode) -> bool {
        self.osm_id == other.osm_id
    }
}

impl Location2D<Real2D> for StreetNode {
    fn get_location(self) -> Real2D {
        self.loc
    }

    fn set_location(&mut self, loc: Real2D) {
        self.loc = loc;
    }
}

impl fmt::Display for StreetNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} loc {}", self.osm_id, self.loc)
    }
}

// impl fmt::Display for NodeStatus {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             NodeStatus::Susceptible => write!(f, "Susceptible"),
//             NodeStatus::Infected => write!(f, "Infected"),
//             NodeStatus::Resistant => write!(f, "Resistant"),
//         }
//     }
// }
