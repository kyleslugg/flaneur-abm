use std::fmt;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct StreetEdgeLabel {
    pub len: f32,
    pub id: u32,
}

impl StreetEdgeLabel {
    pub fn new(len: f32, id: u32) -> Self {
        StreetEdgeLabel { len, id }
    }
}

impl fmt::Display for StreetEdgeLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{length: {}, id: {}}}", self.len, self.id)
    }
}

impl Hash for StreetEdgeLabel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        (self.len.round() as i32).hash(state);
    }
}
