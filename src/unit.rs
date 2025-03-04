use crate::weapon::{Range, VariableValue, Weapon, WeaponTuple};
use serde::{Deserialize, Serialize};
use tera::Context;



#[derive(Debug, Deserialize)]
pub struct UnitStats {
    pub movement: u32,
    pub toughness: u32,
    pub save: u32,
    pub invuln: Option<u32>,
    pub wounds: u32,
    pub leadership: u32,
    pub oc: u32,
}

impl UnitStats {
    pub fn add_context(&self, context: &mut Context) {
        context.insert("movement", &format!("{}\"", self.movement));
        context.insert("toughness", &self.toughness);
        context.insert("save", &format!("{}+", self.save));
        if let Some(invuln) = self.invuln {
            context.insert("invuln", &format!("{}+", invuln));
        } else {
            context.insert("invuln", &"None".to_string());
        }
        context.insert("wounds", &self.wounds);
        context.insert("leadership", &format!("{}+", self.leadership));
        context.insert("oc", &self.oc);
    }
}



#[derive(Debug, Deserialize, Serialize)]
pub struct Ability {
    name: String,
    description: String,
}

#[derive(Debug, Deserialize)]
pub struct Unit {
    pub name: String,
    pub stats: UnitStats,
    pub ranged_weapons: Vec<Weapon>,
    pub melee_weapons: Vec<Weapon>,
    pub faction_ability: Option<String>,
    pub core_abilities: Vec<String>,
    pub unique_abilities: Vec<Ability>,
    pub keywords: Vec<String>
}


impl Unit {

    

    pub fn get_html_path(&self, dir_path: &String) -> String {
        return format!("{}/{}.html", dir_path, self.name);
    }


    pub fn get_pdf_path(&self, dir_path: &String) -> String {
        return format!("{}/{}.pdf", dir_path, self.name);
    }


    fn get_ranged_weapon_list(&self) -> Vec<WeaponTuple> {
        let mut res = Vec::new();
        for weapon in self.ranged_weapons.iter() {
            res.push(weapon.to_html_data());
        }
        res
    }
    fn get_melee_weapon_list(&self) -> Vec<WeaponTuple> {
        let mut res = Vec::new();
        for weapon in self.melee_weapons.iter() {
            res.push(weapon.to_html_data());
        }
        res
    }

    pub fn get_context(
        &self
    ) -> Context {
        let mut context = Context::new();
        
        context.insert("unit_name", &self.name);
        self.stats.add_context(&mut context);
        context.insert("ranged_weapons", &self.get_ranged_weapon_list());
        context.insert("melee_weapons", &self.get_melee_weapon_list());
        context.insert("faction_ability", &self.faction_ability.clone().unwrap_or("none".to_string()));
        context.insert("core_abilities", &self.core_abilities);
        context.insert("unique_abilities", &self.unique_abilities);
        context.insert("keywords", &self.keywords);

        return context;
    }



}