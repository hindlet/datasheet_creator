use crate::data::{abilities::CoreAbility, crusade_data::CrusadeUnitData};

use super::{unit_composition::UnitComposition, unit_stats::UnitStats, Ability, Weapon, WeaponRenderTuple};
use serde::{Deserialize, Serialize};
use tera::Context;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Unit {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub stats: UnitStats,

    #[serde(default)]
    pub extra_statlines: (String, Vec<(String, UnitStats)>),
   
    #[serde(default)]
    pub ranged_weapons: Vec<(Weapon, u32)>,
    #[serde(default)]
    pub melee_weapons: Vec<(Weapon, u32)>,
    
    #[serde(default)]
    pub faction_ability: Option<String>,
    #[serde(default)]
    pub core_abilities: Vec<CoreAbility>,
    #[serde(default)]
    pub unique_abilities: Vec<Ability>,

    #[serde(default)]
    pub faction_keyword: String,
    #[serde(default)]
    pub keywords: Vec<String>,

    #[serde(default)]
    pub damaged: Option<u32>,
    #[serde(default)]
    pub leader: Option<Vec<String>>,

    #[serde(default)]
    pub unit_comp: UnitComposition,

    #[serde(default)]
    pub crusade_unit: bool,
    #[serde(default)]
    pub crusade_data: CrusadeUnitData,
    #[serde(default)]
    pub crusade_weapons: (Vec<(Weapon, u32)>, Vec<(Weapon, u32)>) // used for display in crusade, allows us to apply weapon upgrades
}

impl Default for Unit {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            stats: UnitStats::default(),
            extra_statlines: ("".to_string(), Vec::new()),

            ranged_weapons: Vec::new(),
            melee_weapons: Vec::new(),

            faction_ability: None,
            core_abilities: vec![CoreAbility::None],
            unique_abilities: Vec::new(),

            faction_keyword: "".to_string(),
            keywords: Vec::new(),

            damaged: None,
            leader: None,

            unit_comp: UnitComposition::default(),

            crusade_unit: false,
            crusade_data: CrusadeUnitData::default(),
            crusade_weapons: (Vec::new(), Vec::new())
        }
    }
}

impl Unit {

    pub fn get_movement(&self) -> String {
        if self.keywords.contains(&"AIRCRAFT".to_string()) {
            return "20+".to_string();
        } else {return format!("{}", self.stats.movement);}
    }

    // pub fn format_keywords(&self) -> String{
    //     let mut keywords: String = "".to_string();

    //     let last = self.keywords.len().checked_sub(1).unwrap_or(0);
    //     for (i, keyword) in self.keywords.iter().enumerate() {
    //         keywords += &keyword.to_uppercase();
    //         if i != last {
    //             keywords += ", ";
    //         }
    //     }

    //     keywords
    // }

    fn get_ranged_weapon_list(&self) -> Vec<WeaponRenderTuple> {
        let mut res = Vec::new();
        for (weapon, _) in self.ranged_weapons.iter() {
            res.push(weapon.get_render_data());
        }
        res
    }

    fn get_melee_weapon_list(&self) -> Vec<WeaponRenderTuple> {
        let mut res = Vec::new();
        for (weapon, _) in self.melee_weapons.iter() {
            res.push(weapon.get_render_data());
        }
        res
    }

    pub fn get_context(
        &self
    ) -> Context {
        let mut context = Context::new();

        let mut cased_keywords = Vec::new();
        for keyword in self.keywords.iter() {
            cased_keywords.push(keyword.to_uppercase());
        }
        
        
        context.insert("unit_name", &self.name);
        self.stats.add_context(&mut context);

        // aircraft movement box
        if cased_keywords.contains(&"AIRCRAFT".to_string()) {
            context.insert("movement", &"20+".to_string());
        }
        // leader keyword
        // if self.leader.is_some() && !self.core_abilities.contains(&"Leader".to_string()){
        //     self.core_abilities.push("Leader".to_string());
        // }

        // damage bracket
        let damaged: String;
        if let Some(damaged_wall) = self.damaged {
            damaged = format!("{}", damaged_wall);
        } else {
            damaged = "none".to_string();
        }
        
        context.insert("ranged_weapons", &self.get_ranged_weapon_list());
        context.insert("melee_weapons", &self.get_melee_weapon_list());
        context.insert("faction_ability", &self.faction_ability.clone().unwrap_or("none".to_string()));
        context.insert("core_abilities", &self.core_abilities);
        context.insert("unique_abilities", &self.unique_abilities);
        context.insert("faction_keyword", &self.faction_keyword.to_uppercase());
        context.insert("keywords", &self.keywords);
        context.insert("damaged", &damaged);
        context.insert("leader", &self.leader.clone().unwrap_or(Vec::new()));
        context.insert("wargear_options", "none");

        context
    }

}