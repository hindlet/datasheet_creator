use egui::{Color32, Context, RichText};
use egui_extras::{Column, TableBuilder};

use crate::data::{Ability, Range, Unit, Weapon, UnitStats, VariableValue};

#[derive(Clone)]
pub struct WeaponEditData {
    pub name: String,
    pub range: u32,
    pub attacks: String,
    pub skill: u32,
    pub strength: u32,
    pub ap: u32,
    pub damage: String,
    pub keywords: Vec<String>
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
            keywords: Vec::new()
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
            keywords: value.keywords.clone()
        }
    }
}

impl Into<Weapon> for WeaponEditData {
    fn into(self) -> Weapon {
        let mut sanitised_keywords = Vec::new();
        for keyword in self.keywords {
            if !keyword.is_empty() {
                sanitised_keywords.push(keyword);
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
            keywords: sanitised_keywords
        }
    }
}

#[derive(Clone)]
pub struct UnitEditData {
    pub name: String,
    pub filename: String,
    pub prev_filename: String,

    pub movement: u32,
    pub toughness: u32,
    pub save: u32,
    pub has_invuln: bool,
    pub invuln: u32,
    pub wounds: u32,
    pub leadership: u32,
    pub objective_control: u32,

    pub ranged_weapons: Vec<WeaponEditData>,
    pub melee_weapons: Vec<WeaponEditData>,

    pub has_faction_ability: bool,
    pub faction_ability: String,

    pub core_abilities: Vec<String>,
    pub unique_abilities: Vec<Ability>,
    pub faction_keyword: String,
    pub keywords: Vec<String>,

    pub has_damaged: bool,
    pub damaged: u32,

    pub composition: Vec<(u32, u32)>,

    pub can_lead: bool,
    pub leader: Vec<String>,

    // pub default_wargear: Option<String>,
    pub has_wargear_options: bool,
    pub wargear_options: String,
}

impl From<(&Unit, String)> for UnitEditData {
    fn from((value, filename): (&Unit, String)) -> Self {

        let mut ranged_weapons = Vec::new();
        for weapon in value.ranged_weapons.iter() {
            ranged_weapons.push(WeaponEditData::from(weapon));
        }

        let mut melee_weapons = Vec::new();
        for weapon in value.melee_weapons.iter() {
            melee_weapons.push(WeaponEditData::from(weapon));
        }
        

        Self {
            name: value.name.clone(),
            filename: filename.clone(),
            prev_filename: filename,
            
            movement: value.stats.movement,
            toughness: value.stats.toughness,
            save: value.stats.save,
            has_invuln: value.stats.invuln.is_some(),
            invuln: value.stats.invuln.unwrap_or(4),
            wounds: value.stats.wounds,
            leadership: value.stats.leadership,
            objective_control: value.stats.oc,
            
            ranged_weapons,
            melee_weapons,

            has_faction_ability: value.faction_ability.is_some(),
            faction_ability: value.faction_ability.clone().unwrap_or("".to_string()),

            core_abilities: value.core_abilities.clone(),

            unique_abilities: value.unique_abilities.clone(),

            faction_keyword: value.faction_keyword.clone(),

            keywords: value.keywords.clone(),

            has_damaged: value.damaged.is_some(),
            damaged: value.damaged.unwrap_or(4),

            composition: value.composition.clone(),

            can_lead: value.leader.is_some(),
            leader: value.leader.clone().unwrap_or(Vec::new()),

            has_wargear_options: value.wargear_options.is_some(),
            wargear_options: value.wargear_options.clone().unwrap_or("".to_string())
        }
    }
}

impl Into<Unit> for UnitEditData {
    fn into(self) -> Unit {
        let mut ranged_weapons = Vec::new();
        for weapon in self.ranged_weapons {
            ranged_weapons.push(weapon.into());
        }

        let mut melee_weapons = Vec::new();
        for weapon in self.melee_weapons {
            melee_weapons.push(weapon.into());
        }

        
        let mut sanitised_keywords = Vec::new();
        for keyword in self.keywords {
            if !keyword.is_empty() {
                sanitised_keywords.push(keyword);
            }
        }


        Unit {
            name: self.name,
            stats: UnitStats {
                movement: self.movement,
                toughness: self.toughness,
                save: self.save,
                invuln: if self.has_invuln {
                    Some(self.invuln)
                } else {
                    None
                },
                wounds: self.wounds,
                leadership: self.leadership,
                oc: self.objective_control
            },
            ranged_weapons,
            melee_weapons,
            faction_ability: if self.has_faction_ability {
                Some(self.faction_ability)
            } else {
                None
            },
            core_abilities: self.core_abilities,
            unique_abilities: self.unique_abilities,
            faction_keyword: self.faction_keyword,
            keywords: sanitised_keywords,
            damaged: if self.has_damaged {
                Some(self.damaged)
            } else {
                None
            },
            composition: self.composition,
            leader: if self.can_lead {
                Some(self.leader)
            } else {
                None
            },
            wargear_options: if self.has_wargear_options {
                Some(self.wargear_options)
            } else {
                None
            },
        }
    }
}




pub fn edit_unit(ctx: &Context, unit: &mut UnitEditData) {
    // let index = match app.open_files[app.selected_file] {
    //     OpenFile::Index(index) => index,
    //     _ => return
    // };
    // let unit = &mut app.working_dir[index.0].unit_edit_data[index.1];
    
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("General");
            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut unit.name);
            });
            ui.horizontal(|ui| {
                ui.label("Filename:");
                ui.text_edit_singleline(&mut unit.filename);
            });
            ui.horizontal(|ui| {
                ui.label("Movement:");
                ui.add(egui::DragValue::new(&mut unit.movement)
                    .range(1..=99)).on_hover_text("Inches");
            });
            ui.horizontal(|ui| {
                ui.label("Toughness:");
                ui.add(egui::DragValue::new(&mut unit.toughness)
                    .range(1..=99));
            });
            ui.horizontal(|ui| {
                ui.label("Save:");
                ui.add(egui::DragValue::new(&mut unit.save)
                    .range(1..=6));
            });
            ui.horizontal(|ui| {
                ui.label("Has Invulnerable Save:");
                ui.checkbox(&mut unit.has_invuln, "");
            });
            if unit.has_invuln {
                ui.horizontal(|ui| {
                    ui.label("Invulnerable Save:");
                    ui.add(egui::DragValue::new(&mut unit.invuln)
                        .range(1..=6));
                });
            }
            ui.horizontal(|ui| {
                ui.label("Wounds:");
                ui.add(egui::DragValue::new(&mut unit.wounds)
                    .range(1..=99));
            });
            ui.horizontal(|ui| {
                ui.label("Leadership:");
                ui.add(egui::DragValue::new(&mut unit.leadership)
                    .range(1..=12));
            });
            ui.horizontal(|ui| {
                ui.label("Objective Control:");
                ui.add(egui::DragValue::new(&mut unit.objective_control)
                    .range(0..=99));
            });

            ui.separator();
            ui.heading("Ranged Weapons");
            

            TableBuilder::new(ui)
                .id_salt(1)
                .striped(true)
                .resizable(false)
                .column(Column::auto().at_least(400.0))
                .column(Column::auto().at_least(75.0))
                .column(Column::auto().at_least(40.0))
                .column(Column::auto().at_least(40.0))
                .column(Column::auto().at_least(40.0))
                .column(Column::auto().at_least(40.0))
                .column(Column::auto().at_least(40.0))
                .column(Column::auto().at_least(800.0))
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .header(20.0, |mut header| {
                    for col_header in ["Name", "Range", "A", "BS", "S", "AP", "D", "Keywords"] {
                        header.col(|ui| {
                            ui.strong(RichText::new(col_header).size(15.0));
                        });
                    }
                })
                .body(|mut body| {
                    let mut to_remove = Vec::new();
                    for (i, weapon) in unit.ranged_weapons.iter_mut().enumerate() {
                        body.row(20.0, |mut row| {
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    if ui.button("X").on_hover_text("Delete").clicked() {
                                        to_remove.push(i);
                                    }
                                    ui.text_edit_singleline(&mut weapon.name);
                                });
                            });
                            row.col(|ui| {
                                ui.add(egui::DragValue::new(&mut weapon.range)
                                    .range(1..=300));
                            });
                            row.col(|ui| {
                                if !VariableValue::is_valid_variable_val(&weapon.attacks) {
                                    ui.style_mut().visuals.extreme_bg_color = Color32::RED;
                                }
                                ui.text_edit_singleline(&mut weapon.attacks);
                            });
                            row.col(|ui| {
                                ui.add(egui::DragValue::new(&mut weapon.skill)
                                    .range(1..=6));
                            });
                            row.col(|ui| {
                                ui.add(egui::DragValue::new(&mut weapon.strength)
                                    .range(1..=50));
                            });
                            row.col(|ui| {
                                ui.add(egui::DragValue::new(&mut weapon.ap)
                                    .range(0..=6));
                            });
                            row.col(|ui| {
                                if !VariableValue::is_valid_variable_val(&weapon.damage) {
                                    ui.style_mut().visuals.extreme_bg_color = Color32::RED;
                                }
                                ui.text_edit_singleline(&mut weapon.damage);
                            });
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    if ui.button("+").on_hover_text("Add keyword").clicked() {
                                        weapon.keywords.push("".to_string());
                                    }
                                    let mut to_remove = Vec::new();
                                    for (i, keyword) in weapon.keywords.iter_mut().enumerate() {
                                        if ui.button("-").on_hover_text("Remove keyword").clicked() {
                                            to_remove.push(i);
                                        }
                                        ui.add(egui::TextEdit::singleline(keyword).desired_width(80.0));
                                    }
                                    for (j, i) in to_remove.iter().enumerate() {
                                        weapon.keywords.remove(i - j);
                                    }
                                });
                            });
                        });
                    }
                    for (j, i) in to_remove.iter().enumerate() {
                        unit.ranged_weapons.remove(i - j);
                    }
                });

            if ui.button("Add new weapon").clicked() {
                unit.ranged_weapons.push(WeaponEditData::default());
            }
            ui.separator();
            ui.heading("Melee Weapons");
            TableBuilder::new(ui)
                .id_salt(2)
                .striped(true)
                .resizable(false)
                .column(Column::auto().at_least(400.0))
                .column(Column::auto().at_least(75.0))
                .column(Column::auto().at_least(40.0))
                .column(Column::auto().at_least(40.0))
                .column(Column::auto().at_least(40.0))
                .column(Column::auto().at_least(40.0))
                .column(Column::auto().at_least(40.0))
                .column(Column::auto().at_least(800.0))
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .header(20.0, |mut header| {
                    for col_header in ["Name", "Range", "A", "WS", "S", "AP", "D", "Keywords"] {
                        header.col(|ui| {
                            ui.strong(RichText::new(col_header).size(15.0));
                        });
                    }
                })
                .body(|mut body| {
                    let mut to_remove = Vec::new();
                    for (i, weapon) in unit.melee_weapons.iter_mut().enumerate() {
                        body.row(20.0, |mut row| {
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    if ui.button("X").on_hover_text("Delete").clicked() {
                                        to_remove.push(i);
                                    }
                                    ui.text_edit_singleline(&mut weapon.name);
                                });
                            });
                            row.col(|ui| {
                                ui.label("melee");
                            });
                            row.col(|ui| {
                                if !VariableValue::is_valid_variable_val(&weapon.attacks) {
                                    ui.style_mut().visuals.extreme_bg_color = Color32::RED;
                                }
                                ui.text_edit_singleline(&mut weapon.attacks);
                            });
                            row.col(|ui| {
                                ui.add(egui::DragValue::new(&mut weapon.skill)
                                    .range(1..=6));
                            });
                            row.col(|ui| {
                                ui.add(egui::DragValue::new(&mut weapon.strength)
                                    .range(1..=50));
                            });
                            row.col(|ui| {
                                ui.add(egui::DragValue::new(&mut weapon.ap)
                                    .range(0..=6));
                            });
                            row.col(|ui| {
                                if !VariableValue::is_valid_variable_val(&weapon.damage) {
                                    ui.style_mut().visuals.extreme_bg_color = Color32::RED;
                                }
                                ui.text_edit_singleline(&mut weapon.damage);
                            });
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    if ui.button("+").on_hover_text("Add keyword").clicked() {
                                        weapon.keywords.push("".to_string());
                                    }
                                    let mut to_remove = Vec::new();
                                    for (i, keyword) in weapon.keywords.iter_mut().enumerate() {
                                        if ui.button("-").on_hover_text("Remove keyword").clicked() {
                                            to_remove.push(i);
                                        }
                                        ui.add(egui::TextEdit::singleline(keyword).desired_width(80.0));
                                    }
                                    for (j, i) in to_remove.iter().enumerate() {
                                        weapon.keywords.remove(i - j);
                                    }
                                });
                            });
                        });
                    }
                    for (j, i) in to_remove.iter().enumerate() {
                        unit.melee_weapons.remove(i - j);
                    }
                });
            if ui.button("Add new weapon").clicked() {
                unit.melee_weapons.push(WeaponEditData {
                    range: 0,
                    ..Default::default()
                });
            }
            ui.separator();
            ui.heading("Abilities");

            ui.horizontal(|ui| {
                ui.label("Has Faction Ability:");
                ui.checkbox(&mut unit.has_faction_ability, "");
            });
            

            if unit.has_faction_ability {
                ui.horizontal(|ui| {
                    ui.label("Faction Ability:");
                    ui.text_edit_singleline(&mut unit.faction_ability);
                });
            }

            ui.strong("Core Abilities");
            TableBuilder::new(ui)
                .id_salt(3)
                .striped(true)
                .resizable(false)
                .column(Column::auto().at_least(200.0))
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .body(|mut body| {
                    let mut to_remove = Vec::new();
                    for (i, ability) in unit.core_abilities.iter_mut().enumerate() {
                        body.row(20.0, |mut row| {
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    if ui.button("X").on_hover_text("Delete").clicked() {
                                        to_remove.push(i);
                                    }
                                    ui.text_edit_singleline(ability);
                                });
                            });
                        });
                    }
                    for (j, i) in to_remove.iter().enumerate() {
                        unit.core_abilities.remove(i - j);
                    }
                });

            if ui.button("Add new core ability").clicked() {
                unit.core_abilities.push("".to_string());
            }

            TableBuilder::new(ui)
                .id_salt(4)
                .striped(true)
                .resizable(false)
                .column(Column::auto().at_least(200.0))
                .column(Column::auto().at_least(400.0))
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .header(20.0, |mut header| {
                    for col_header in ["Name", "Description"] {
                        header.col(|ui| {
                            ui.strong(RichText::new(col_header).size(15.0));
                        });
                    }
                })
                .body(|mut body| {
                    let mut to_remove = Vec::new();
                    for (i, ability) in unit.unique_abilities.iter_mut().enumerate() {
                        body.row(80.0, |mut row| {
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    if ui.button("X").on_hover_text("Delete").clicked() {
                                        to_remove.push(i);
                                    }
                                    ui.text_edit_singleline(&mut ability.name);
                                });
                            });
                            row.col(|ui| {
                                ui.text_edit_multiline(&mut ability.description);
                            });
                        });
                    }
                    for (j, i) in to_remove.iter().enumerate() {
                        unit.unique_abilities.remove(i - j);
                    }
                });

            if ui.button("Add new ability").clicked() {
                unit.unique_abilities.push(Ability::default());
            }

            ui.separator();
            ui.heading("Keywords");

            ui.horizontal(|ui| {
                ui.label("Faction Keyword:");
                ui.text_edit_singleline(&mut unit.faction_keyword);
            });

            TableBuilder::new(ui)
                .id_salt(5)
                .striped(true)
                .resizable(false)
                .column(Column::auto().at_least(200.0))
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .body(|mut body| {
                    let mut to_remove = Vec::new();
                    for (i, keyword) in unit.keywords.iter_mut().enumerate() {
                        body.row(20.0, |mut row| {
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    if ui.button("X").on_hover_text("Delete").clicked() {
                                        to_remove.push(i);
                                    }
                                    ui.text_edit_singleline(keyword);
                                });
                                
                            });
                        });
                    }
                    for (j, i) in to_remove.iter().enumerate() {
                        unit.keywords.remove(i - j);
                    }
                });

            if ui.button("Add new keyword").clicked() {
                unit.keywords.push("".to_string());
            }
            ui.separator();
        });
        
    });
    
}