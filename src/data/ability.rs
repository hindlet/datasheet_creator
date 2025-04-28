use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Ability {
    pub name: String,
    pub description: String,
}