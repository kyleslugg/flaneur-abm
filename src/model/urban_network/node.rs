use std::{
    fmt,
    hash::{Hash, Hasher},
};

use krabmaga::engine::{fields::field_2d::Location2D, location::Real2D};

#[derive(Copy, Clone, Eq, Default)]
pub struct StreetNode {
    pub id: i64,
    pub loc: Real2D,
}

impl StreetNode {
    pub fn new(id: i64, loc: Real2D) -> Self {
        StreetNode { id, loc }
    }
}

impl Hash for StreetNode {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl PartialEq for StreetNode {
    fn eq(&self, other: &StreetNode) -> bool {
        self.id == other.id
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
        write!(f, "{} loc {}", self.id, self.loc)
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
