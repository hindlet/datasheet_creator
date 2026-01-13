use egui::{ComboBox, Ui};

use crate::{data::{ChargeLevels, Range, VariableValue, Weapon, WeaponAbility, WeaponReference}, helper_funcs::select_text_on_tab};



#[derive(Clone)]
pub struct WeaponEditData {
    pub name: String,
    pub range: u32,
    pub attacks: String,
    pub skill: u32,
    pub strength: u32,
    pub ap: u32,
    pub damage: String,
    pub keywords: Vec<WeaponAbility>,
    pub charge_levels_info: (bool, Option<WeaponReference>, String) // has levels, is parent, level name
}

impl Default for WeaponEditData {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            range: 1,
            attacks: "1".to_string(),
            skill: 1,
            strength: 1,
            ap: 0,
            damage: "1".to_string(),
            keywords: Vec::new(),
            charge_levels_info: (false, None, "".to_string())
        }
    }
}

impl From<&Weapon> for WeaponEditData {
    fn from(value: &Weapon) -> Self {
        Self {
            name: value.name.clone(),
            range: match value.range {
                Range::Ranged(range) => range,
                _ => 0
            },
            attacks: value.attacks.to_string(),
            skill: value.skill,
            strength: value.strength,
            ap: value.ap.abs() as u32,
            damage: value.damage.to_string(),
            keywords: value.keywords.clone(),
            charge_levels_info: value.charge.to_edit()
        }
    }
}

impl Into<Weapon> for WeaponEditData {
    fn into(self) -> Weapon {
        let mut keywords = Vec::new();
        for keyword in self.keywords {
            match keyword {
                WeaponAbility::Sustained(_, text) => {
                    let val = VariableValue::from_string(&text).unwrap_or(VariableValue::Set(1));
                    keywords.push(WeaponAbility::Sustained(val, val.to_string()));
                },
                WeaponAbility::RapidFire(_, text) => {
                    let val = VariableValue::from_string(&text).unwrap_or(VariableValue::Set(1));
                    keywords.push(WeaponAbility::RapidFire(val, val.to_string()));
                },
                _ => keywords.push(keyword),
            }
        }

        Weapon {
            name: self.name,
            range: if self.range == 0 {
                Range::Melee
            } else {
                Range::Ranged(self.range)
            },
            attacks: VariableValue::from_string(&self.attacks).unwrap_or(VariableValue::Set(0)),
            skill: self.skill,
            strength: self.strength,
            ap: self.ap as i32,
            damage: VariableValue::from_string(&self.damage).unwrap_or(VariableValue::Set(0)),
            keywords: keywords,
            charge: ChargeLevels::from_edit(self.charge_levels_info.0, self.charge_levels_info.1, self.charge_levels_info.2) 
        }
    }
}


impl WeaponEditData {
    pub fn charge_edit_section(&mut self, ui: &mut Ui, index: usize, weapons: &Vec<WeaponReference>, id: usize) {
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.charge_levels_info.0, "");
            if self.charge_levels_info.0 {
                select_text_on_tab(self.charge_levels_info.2.len(), egui::TextEdit::singleline(&mut self.charge_levels_info.2).desired_width(40.0), ui);
                let label = if self.charge_levels_info.1.is_none() {"Parent"} else {"Child"};
                ComboBox::from_id_salt(id)
                    .selected_text(label)
                    .width(20.0)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.charge_levels_info.1, None, "Parent");
                        for (i, weapon) in weapons.iter().enumerate() {
                            if i == index {continue;}
                            ui.selectable_value(&mut self.charge_levels_info.1, Some(weapon.clone()), &weapon.name);
                        }
                    });
                
            }
        });
    }
}