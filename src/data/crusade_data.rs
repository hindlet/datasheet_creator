use crate::data::Ability;
use egui::{ComboBox, Ui};
use serde::{Deserialize, Serialize};



#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CrusadeUnitData {
    pub exp: u32,
    pub upgrades: Vec<CrusadeUpgrade>,
    pub kills: u32,
}

impl Default for CrusadeUnitData {
    fn default() -> Self {
        Self {
            exp: 0,
            upgrades: Vec::new(),
            kills: 0
        }
    }
}





#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum CrusadeUpgrade {
    WeaponMod(WeaponMod),
    Relic(Ability),
    BattleTrait(Ability)
}

impl CrusadeUpgrade {

    pub fn to_string(&self) -> &str {
        match self {
            CrusadeUpgrade::WeaponMod(_) => "Weapon Mod",
            CrusadeUpgrade::Relic(_) => "Relic",
            CrusadeUpgrade::BattleTrait(_) => "Battle Trait",
        }
    }

    pub fn combo_box(&mut self, ui: &mut Ui, id: usize) {
        ComboBox::from_id_salt(id)
            .selected_text(format!("{}", self.to_string()))
            .show_ui(ui, |ui| {
                ui.selectable_value(self, CrusadeUpgrade::WeaponMod(WeaponMod::default()), "Weapon Mod");
                ui.selectable_value(self, CrusadeUpgrade::Relic(Ability::default()), "Relic");
                ui.selectable_value(self, CrusadeUpgrade::BattleTrait(Ability::default()), "Battle Trait");
            });
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum WeaponModChange {
    Attacks,
    Skill,
    Strength,
    AP,
    Damage,
    Precise,
}

impl WeaponModChange {
    pub fn to_string(&self) -> &str {
        match self {
            WeaponModChange::Attacks => "Attacks",
            WeaponModChange::Skill => "Skill",
            WeaponModChange::Strength => "Strength",
            WeaponModChange::AP => "AP",
            WeaponModChange::Damage => "Damage",
            WeaponModChange::Precise => "Precise"
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct WeaponMod {
    pub name: String,
    pub change_one: WeaponModChange,
    pub change_two: WeaponModChange,
    pub target: Option<(bool, usize)>
}

impl Default for WeaponMod {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            change_one: WeaponModChange::Attacks,
            change_two: WeaponModChange::Skill,
            target: None
        }
    }
}

impl WeaponMod {
    pub fn combo_boxes(&mut self, ui: &mut Ui, id: usize) {
        ComboBox::from_id_salt(id)
            .selected_text(format!("{}", self.change_one.to_string()))
            .show_ui(ui, |ui| {
                if self.change_two != WeaponModChange::Attacks {ui.selectable_value(&mut self.change_one, WeaponModChange::Attacks, "Attacks");}
                if self.change_two != WeaponModChange::Skill {ui.selectable_value(&mut self.change_one, WeaponModChange::Skill, "Skill");}
                if self.change_two != WeaponModChange::Strength {ui.selectable_value(&mut self.change_one, WeaponModChange::Strength, "Strength");}
                if self.change_two != WeaponModChange::AP {ui.selectable_value(&mut self.change_one, WeaponModChange::AP, "AP");}
                if self.change_two != WeaponModChange::Damage {ui.selectable_value(&mut self.change_one, WeaponModChange::Damage, "Damage");}
                if self.change_two != WeaponModChange::Precise {ui.selectable_value(&mut self.change_one, WeaponModChange::Precise, "Precise");}
            });
        ComboBox::from_id_salt(id + 1)
            .selected_text(format!("{}", self.change_two.to_string()))
            .show_ui(ui, |ui| {
                if self.change_one != WeaponModChange::Attacks {ui.selectable_value(&mut self.change_two, WeaponModChange::Attacks, "Attacks");}
                if self.change_one != WeaponModChange::Skill {ui.selectable_value(&mut self.change_two, WeaponModChange::Skill, "Skill");}
                if self.change_one != WeaponModChange::Strength {ui.selectable_value(&mut self.change_two, WeaponModChange::Strength, "Strength");}
                if self.change_one != WeaponModChange::AP {ui.selectable_value(&mut self.change_two, WeaponModChange::AP, "AP");}
                if self.change_one != WeaponModChange::Damage {ui.selectable_value(&mut self.change_two, WeaponModChange::Damage, "Damage");}
                if self.change_one != WeaponModChange::Precise {ui.selectable_value(&mut self.change_two, WeaponModChange::Precise, "Precise");}
            });
    }
}