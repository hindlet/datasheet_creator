use super::{Weapon, Ability};
use serde::{Deserialize, Serialize};



#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UnitStats {
    pub movement: u32,
    pub toughness: u32,
    pub save: u32,
    pub invuln: Option<u32>,
    pub wounds: u32,
    pub leadership: u32,
    pub oc: u32,
}

impl Default for UnitStats {
    fn default() -> Self {
        UnitStats {
            movement: 0,
            toughness: 0,
            save: 0,
            invuln: None,
            wounds: 0,
            leadership: 0,
            oc: 0
        }
    }
}




#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Unit {
    pub name: String,
    pub stats: UnitStats,
    pub ranged_weapons: Vec<Weapon>,
    pub melee_weapons: Vec<Weapon>,
    pub faction_ability: Option<String>,
    pub core_abilities: Vec<String>,
    pub unique_abilities: Vec<Ability>,
    pub faction_keyword: String,
    pub keywords: Vec<String>,
    pub damaged: Option<u32>,
    pub composition: Vec<(u32, u32)>,
    pub leader: Option<Vec<String>>,

}

impl Default for Unit {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            stats: UnitStats::default(),
            ranged_weapons: Vec::new(),
            melee_weapons: Vec::new(),
            faction_ability: None,
            core_abilities: Vec::new(),
            unique_abilities: Vec::new(),
            faction_keyword: "".to_string(),
            keywords: Vec::new(),
            damaged: None,
            composition: Vec::new(),
            leader: None,
        }
    }
}

impl Unit {

    pub fn get_movement(&self) -> String {
        if self.keywords.contains(&"AIRCRAFT".to_string()) {
            return "20+".to_string();
        } else {return format!("{}", self.stats.movement);}
    }

    pub fn format_keywords(&self) -> String{
        let mut keywords: String = "".to_string();

        let last = self.keywords.len().checked_sub(1).unwrap_or(0);
        for (i, keyword) in self.keywords.iter().enumerate() {
            keywords += &keyword.to_uppercase();
            if i != last {
                keywords += ", ";
            }
        }

        keywords
    }
}