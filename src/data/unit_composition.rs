use super::Wargear;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UnitComposition {
    comp: Vec<(u32, String, Vec<Wargear>)>
}

impl Default for UnitComposition {
    fn default() -> Self {
        UnitComposition {
            comp: Vec::new()
        }
    }
}