use serde::{Deserialize, Serialize};

use super::variable_val::VariableValue;


#[derive(Debug, Deserialize, Clone, Copy, Serialize)]
pub enum Range {
    Melee,
    Ranged(u32)
}

impl Range {
    pub fn to_string(&self) -> String {
        match self {
            Range::Melee => return "melee".to_string(),
            Range::Ranged(range) => return format!("{}\"", range)
        }
    }
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Weapon {
    pub name: String,
    pub range: Range,
    pub attacks: VariableValue,
    pub skill: u32,
    pub strength: u32,
    pub ap: i32,
    pub damage: VariableValue,
    pub keywords: Vec<String>
}

pub type WeaponTuple = (String, String, String, String, u32, String, String, Vec<String>);

impl Weapon {
    pub fn to_html_data(&self) -> WeaponTuple {
        let mut cased_keywords = Vec::new();
        for keyword in self.keywords.iter() {
            cased_keywords.push(keyword.to_uppercase());
        }

        let skill: String;
        if cased_keywords.contains(&"TORRENT".to_string()) {
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
            cased_keywords
        )
    }


    pub fn format_keywords(&self) -> String{
        let mut keywords: String = "[".to_string();

        let last = self.keywords.len().checked_sub(1).unwrap_or(0);
        for (i, keyword) in self.keywords.iter().enumerate() {
            keywords += &keyword.to_uppercase();
            if i != last {
                keywords += ", ";
            }
        }

        return keywords + "]";
    }
}