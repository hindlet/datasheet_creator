use serde::{Deserialize, Serialize};


#[derive(Debug, Default, Deserialize, Clone, Serialize, PartialOrd, Ord, Eq)]
pub struct WeaponReference {
    pub name: String,
    pub ranged: bool,
    pub id: usize
}

impl WeaponReference {
    pub fn new(name: String, ranged: bool, id: usize) -> Self {
        Self {
            name,
            ranged,
            id
        }
    }

    pub fn is_id(&self, ranged: bool, id: usize) -> bool {
        self.ranged == ranged && self.id == id
    }
}

impl PartialEq for WeaponReference {
    fn eq(&self, other: &Self) -> bool {
        self.ranged == other.ranged && self.id == other.id
    }
}