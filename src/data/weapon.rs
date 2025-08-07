use serde::{Deserialize, Serialize};

use crate::data::abilities::WeaponAbility;

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
    pub keywords: Vec<WeaponAbility>
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