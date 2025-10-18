use serde::{Deserialize, Serialize};

use crate::data::{abilities::WeaponAbility, index::WeaponReference};

use super::variable_val::VariableValue;


#[derive(Debug, Deserialize, Clone, Copy, Serialize)]
pub enum Range {
    Melee,
    Ranged(u32)
}

impl Range {
    pub fn to_string(&self) -> String {
        match self {
            Range::Melee => return "Melee".to_string(),
            Range::Ranged(range) => return format!("{}\"", range)
        }
    }
}

#[derive(Debug, Default, Deserialize, Clone, Serialize)]
pub enum ChargeLevels {
    #[default]
    None,
    Parent(String),
    Child(WeaponReference, String)
}

impl ChargeLevels {
    pub fn to_edit(&self) -> (bool, Option<WeaponReference>, String) {
        match self {
            ChargeLevels::None => (false, None, "".to_string()),
            ChargeLevels::Parent(name) => (true, None, name.clone()),
            ChargeLevels::Child(reference, name) => (true, Some(reference.clone()), name.clone()),
        }
    }

    pub fn from_edit(exists: bool, reference: Option<WeaponReference>, name: String) -> Self {
        match (exists, reference) {
            (false, _) => Self::None,
            (true, None) => Self::Parent(name.clone()),
            (true, Some(weapon_ref)) => Self::Child(weapon_ref, name.clone())
        }
    }
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Weapon {
    #[serde(default)]
    pub name: String,
    pub range: Range,
    #[serde(default)]
    pub attacks: VariableValue,
    #[serde(default)]
    pub skill: u32,
    #[serde(default)]
    pub strength: u32,
    #[serde(default)]
    pub ap: i32,
    #[serde(default)]
    pub damage: VariableValue,
    #[serde(default)]
    pub keywords: Vec<WeaponAbility>,
    #[serde(default)]
    pub charge: ChargeLevels
}

pub type WeaponRenderTuple = (String, String, String, String, u32, String, String, String);

impl Weapon {

    pub fn get_render_data(&self) -> WeaponRenderTuple {
        let skill: String;
        if self.keywords.contains(&WeaponAbility::Torrent) {
            skill = "N/A".to_string();
        } else {
            skill = format!("{}+", self.skill)
        }

        let ap: String;
        if self.ap > 0 {
            ap = format!("-{}", self.ap);
        } else {
            ap = format!("{}", self.ap);
        }

        (
            self.name.clone(),
            self.range.to_string(),
            self.attacks.to_string(),
            skill,
            self.strength,
            ap,
            self.damage.to_string(),
            self.format_keywords()
        )
    }

    pub fn format_keywords(&self) -> String{
        if self.keywords.len() == 0 {return "[]".to_string();} // zero keywords case
        if self.keywords.len() == 1 && self.keywords[0] == WeaponAbility::None {return "[]".to_string();}
        let mut keywords: String = "[".to_string();

        for keyword in self.keywords.iter() {
            if keyword == &WeaponAbility::None {continue;}
            keywords += &keyword.to_render_string();
            keywords += ", ";
        }

        let i = keywords.len() - 2; // remove the last comma + space
        return keywords[..i].to_string() + "]";
    }
}