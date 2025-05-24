use super::{Ability, Weapon, WeaponRenderTuple};
use serde::{Deserialize, Serialize};
use tera::Context;



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

impl UnitStats {
    pub fn add_context(&self, context: &mut Context) {
        context.insert("movement", &format!("{}", self.movement));
        context.insert("toughness", &self.toughness);
        context.insert("save", &self.save);
        if let Some(invuln) = self.invuln {
            context.insert("invuln", &format!("{}", invuln));
        } else {
            context.insert("invuln", &"None".to_string());
        }
        context.insert("wounds", &self.wounds);
        context.insert("leadership", &self.leadership);
        context.insert("oc", &self.oc);
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

    fn get_ranged_weapon_list(&self) -> Vec<WeaponRenderTuple> {
        let mut res = Vec::new();
        for weapon in self.ranged_weapons.iter() {
            res.push(weapon.get_render_data());
        }
        res
    }
    fn get_melee_weapon_list(&self) -> Vec<WeaponRenderTuple> {
        let mut res = Vec::new();
        for weapon in self.melee_weapons.iter() {
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
        context.insert("unit_composition", &self.composition);
        context.insert("leader", &self.leader.clone().unwrap_or(Vec::new()));
        context.insert("wargear_options", "none");

        context
    }

}