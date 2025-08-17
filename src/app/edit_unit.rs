use egui::{Color32, Context, RichText};
use egui_extras::{Column, TableBuilder};
use crate::{data::{Ability, CoreAbility, CrusadeUpgrade, UnitEditData, VariableValue, WeaponAbility, WeaponEditData, WeaponMod}, helper_funcs::{select_drag_value_with_range_on_tab, select_text_on_tab}};







pub fn edit_unit(ctx: &Context, unit: &mut UnitEditData) {

    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::both().show(ui, |ui| {
            ui.heading("General");
            ui.horizontal(|ui| {
                ui.label("Name:");
                select_text_on_tab( unit.name.len(), egui::TextEdit::singleline(&mut unit.name), ui);
            });
            ui.horizontal(|ui| {
                ui.label("Filename:");
                select_text_on_tab(unit.filename.len(), egui::TextEdit::singleline(&mut unit.filename), ui);
                // ui.text_edit_singleline(&mut unit.filename);
            });
            ui.horizontal(|ui| {
                ui.label("Is a Crusade Unit:");
                ui.checkbox( &mut unit.crusader, "");
            });
            ui.horizontal(|ui| {
                ui.label("Movement:");
                select_drag_value_with_range_on_tab(&mut unit.movement, 1..=99, ui);
                // ui.add(egui::DragValue::new(&mut unit.movement)
                //     .range(1..=99)).on_hover_text("Inches");
            });
            ui.horizontal(|ui| {
                ui.label("Toughness:");
                select_drag_value_with_range_on_tab(&mut unit.toughness, 1..=99, ui);
                // ui.add(egui::DragValue::new(&mut unit.toughness)
                //     .range(1..=99));
            });
            ui.horizontal(|ui| {
                ui.label("Save:");
                select_drag_value_with_range_on_tab(&mut unit.save, 1..=6, ui);
                // ui.add(egui::DragValue::new(&mut unit.save)
                //     .range(1..=6));
            });
            ui.horizontal(|ui| {
                ui.label("Has Invulnerable Save:");
                ui.checkbox(&mut unit.has_invuln, "");
            });
            if unit.has_invuln {
                ui.horizontal(|ui| {
                    ui.label("Invulnerable Save:");
                    select_drag_value_with_range_on_tab(&mut unit.invuln, 1..=6, ui);
                    // ui.add(egui::DragValue::new(&mut unit.invuln)
                    //     .range(1..=6));
                });
            }
            ui.horizontal(|ui| {
                ui.label("Wounds:");
                select_drag_value_with_range_on_tab(&mut unit.wounds, 1..=99, ui);
                // ui.add(egui::DragValue::new(&mut unit.wounds)
                //     .range(1..=99));
                
            });
            ui.horizontal(|ui| {
                ui.label("Leadership:");
                select_drag_value_with_range_on_tab(&mut unit.leadership, 1..=12, ui);
                // ui.add(egui::DragValue::new(&mut unit.leadership)
                //     .range(1..=12));
            });
            ui.horizontal(|ui| {
                ui.label("Objective Control:");
                select_drag_value_with_range_on_tab(&mut unit.objective_control, 1..=99, ui);
                // ui.add(egui::DragValue::new(&mut unit.objective_control)
                //     .range(0..=99));
            });

            ui.separator();
            ui.heading("Ranged Weapons");
            
            // ranged weapons
            TableBuilder::new(ui)
                .id_salt(1)
                .striped(true)
                .resizable(false)
                .column(Column::auto().at_least(400.0))
                .column(Column::auto().at_least(40.0))
                .column(Column::auto().at_least(40.0))
                .column(Column::auto().at_least(30.0))
                .column(Column::auto().at_least(40.0))
                .column(Column::auto().at_least(40.0))
                .column(Column::auto().at_least(40.0))
                .column(Column::auto().at_least(40.0))
                .column(Column::auto().at_least(40.0))
                .column(Column::auto())
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .header(20.0, |mut header| {
                    for col_header in ["Name", "Count", "Range", "A", "BS", "S", "AP", "D", "Charge", "Keywords"] {
                        header.col(|ui| {
                            ui.strong(RichText::new(col_header).size(15.0));
                        });
                    }
                })
                .body(|mut body| {
                    let mut to_remove = Vec::new();
                    let weapons_list = &unit.ranged_weapons.clone();
                    for (i, (weapon, count)) in unit.ranged_weapons.iter_mut().enumerate() {
                        body.row(20.0, |mut row| {
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    if ui.button("X").on_hover_text("Delete").clicked() {
                                        to_remove.push(i);
                                    }
                                    select_text_on_tab(weapon.name.len(), egui::TextEdit::singleline(&mut weapon.name), ui);
                                });
                            });
                            row.col(|ui| {
                                select_drag_value_with_range_on_tab(count, 1..=300, ui);
                            });
                            row.col(|ui| {
                                select_drag_value_with_range_on_tab(&mut weapon.range, 1..=300, ui);
                            });
                            row.col(|ui| {
                                if !VariableValue::is_valid_variable_val(&weapon.attacks) {
                                    ui.style_mut().visuals.extreme_bg_color = Color32::RED;
                                }
                                select_text_on_tab(weapon.attacks.len(), egui::TextEdit::singleline(&mut weapon.attacks), ui);
                                // select_text_on_tab(&mut weapon.attacks, ui);
                                // ui.text_edit_singleline(&mut weapon.attacks);
                            });
                            row.col(|ui| {
                                select_drag_value_with_range_on_tab(&mut weapon.skill, 1..=6, ui);
                            });
                            row.col(|ui| {
                                select_drag_value_with_range_on_tab(&mut weapon.strength, 1..=99, ui);
                            });
                            row.col(|ui| {
                                select_drag_value_with_range_on_tab(&mut weapon.ap, 0..=6, ui);
                            });
                            row.col(|ui| {
                                if !VariableValue::is_valid_variable_val(&weapon.damage) {
                                    ui.style_mut().visuals.extreme_bg_color = Color32::RED;
                                }
                                select_text_on_tab(weapon.damage.len(), egui::TextEdit::singleline(&mut weapon.damage), ui);
                                // select_text_on_tab(&mut weapon.damage, ui);
                                // ui.text_edit_singleline(&mut weapon.damage);
                            });
                            row.col(|ui| {
                                weapon.charge_edit_section(ui, i, weapons_list, i * 50 + 10005000912);
                            });
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    if ui.button("+").on_hover_text("Add keyword").clicked() {
                                        weapon.keywords.push(WeaponAbility::None);
                                    }
                                    let mut to_remove = Vec::new();
                                    for (j, keyword) in weapon.keywords.iter_mut().enumerate() {
                                        if ui.button("-").on_hover_text("Remove keyword").clicked() {
                                            to_remove.push(j);
                                        }
                                        keyword.combo_box_ranged(ui, i * 50 + j + 10000000);
                                        match keyword {
                                            WeaponAbility::Sustained(_, x) => {
                                                if !VariableValue::is_valid_variable_val(&x) {
                                                    ui.style_mut().visuals.extreme_bg_color = Color32::RED;
                                                }
                                                select_text_on_tab(x.len(), egui::TextEdit::singleline(x), ui);
                                            },
                                            WeaponAbility::RapidFire(x) => {
                                                select_drag_value_with_range_on_tab(x, 1..=99, ui);
                                            },
                                            WeaponAbility::AntiX(keyword, x) => {
                                                select_text_on_tab(keyword.len(), egui::TextEdit::singleline(keyword), ui);
                                                select_drag_value_with_range_on_tab(x, 1..=99, ui);
                                            },
                                            WeaponAbility::Melta(x) => {
                                                select_drag_value_with_range_on_tab( x, 1..=99, ui);
                                            },
                                            _ => {}
                                        }
                                    
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
                unit.ranged_weapons.push((WeaponEditData::default(), 1));
            }
            ui.separator();
            ui.heading("Melee Weapons");

            // ranged weapons
            TableBuilder::new(ui)
                .id_salt(2)
                .striped(true)
                .resizable(false)
                .column(Column::auto().at_least(400.0))
                .column(Column::auto().at_least(40.0))
                .column(Column::auto().at_least(40.0))
                .column(Column::auto().at_least(30.0))
                .column(Column::auto().at_least(40.0))
                .column(Column::auto().at_least(40.0))
                .column(Column::auto().at_least(40.0))
                .column(Column::auto().at_least(40.0))
                .column(Column::auto().at_least(40.0))
                .column(Column::auto())
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .header(20.0, |mut header| {
                    for col_header in ["Name", "Count", "Range", "A", "WS", "S", "AP", "D", "Charge", "Keywords"] {
                        header.col(|ui| {
                            ui.strong(RichText::new(col_header).size(15.0));
                        });
                    }
                })
                .body(|mut body| {
                    let mut to_remove = Vec::new();
                    for (i, (weapon, count)) in unit.melee_weapons.iter_mut().enumerate() {
                        body.row(20.0, |mut row| {
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    if ui.button("X").on_hover_text("Delete").clicked() {
                                        to_remove.push(i);
                                    }
                                    select_text_on_tab(weapon.name.len(), egui::TextEdit::singleline(&mut weapon.name), ui);
                                });
                            });
                            
                            row.col(|ui| {
                                select_drag_value_with_range_on_tab(count, 1..=300, ui);
                            });
                            row.col(|ui| {
                                select_drag_value_with_range_on_tab(&mut weapon.range, 1..=300, ui);
                            });
                            row.col(|ui| {
                                if !VariableValue::is_valid_variable_val(&weapon.attacks) {
                                    ui.style_mut().visuals.extreme_bg_color = Color32::RED;
                                }
                                select_text_on_tab(weapon.attacks.len(), egui::TextEdit::singleline(&mut weapon.attacks), ui);
                                // select_text_on_tab(&mut weapon.attacks, ui);
                                // ui.text_edit_singleline(&mut weapon.attacks);
                            });
                            row.col(|ui| {
                                select_drag_value_with_range_on_tab(&mut weapon.skill, 1..=6, ui);
                            });
                            row.col(|ui| {
                                select_drag_value_with_range_on_tab(&mut weapon.strength, 1..=99, ui);
                            });
                            row.col(|ui| {
                                select_drag_value_with_range_on_tab(&mut weapon.ap, 0..=6, ui);
                            });
                            row.col(|ui| {
                                if !VariableValue::is_valid_variable_val(&weapon.damage) {
                                    ui.style_mut().visuals.extreme_bg_color = Color32::RED;
                                }
                                select_text_on_tab(weapon.damage.len(), egui::TextEdit::singleline(&mut weapon.damage), ui);
                                // select_text_on_tab(&mut weapon.damage, ui);
                                // ui.text_edit_singleline(&mut weapon.damage);
                            });
                            row.col(|ui| {
                                ui.checkbox(&mut weapon.charge_levels_info.0, "");
                            });
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    if ui.button("+").on_hover_text("Add keyword").clicked() {
                                        weapon.keywords.push(WeaponAbility::None);
                                    }
                                    let mut to_remove = Vec::new();
                                    for (j, keyword) in weapon.keywords.iter_mut().enumerate() {
                                        if ui.button("-").on_hover_text("Remove keyword").clicked() {
                                            to_remove.push(j);
                                        }
                                        keyword.combo_box_melee(ui, i * 50 + j + 900000);
                                        match keyword {
                                            WeaponAbility::Sustained(_, x) => {
                                                if !VariableValue::is_valid_variable_val(&x) {
                                                    ui.style_mut().visuals.extreme_bg_color = Color32::RED;
                                                }
                                                select_text_on_tab(x.len(), egui::TextEdit::singleline(x), ui);
                                            },
                                            WeaponAbility::RapidFire(x) => {
                                                select_drag_value_with_range_on_tab(x, 1..=99, ui);
                                            },
                                            WeaponAbility::AntiX(keyword, x) => {
                                                select_text_on_tab(keyword.len(), egui::TextEdit::singleline(keyword), ui);
                                                select_drag_value_with_range_on_tab(x, 1..=99, ui);
                                            },
                                            WeaponAbility::Melta(x) => {
                                                select_drag_value_with_range_on_tab(x, 1..=99, ui);
                                            },
                                            _ => {}
                                        }
                                    
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
                unit.melee_weapons.push((WeaponEditData {
                    range: 0,
                    ..Default::default()
                }, 1));
            }
            ui.separator();
            ui.heading("Abilities");

            ui.horizontal(|ui| {
                ui.label("Has Faction Ability:");
                ui.checkbox(&mut unit.faction_ability.0, "");
            });
            

            if unit.faction_ability.0 {
                ui.horizontal(|ui| {
                    ui.label("Faction Ability:");
                    select_text_on_tab(unit.faction_ability.1.len(), egui::TextEdit::singleline(&mut unit.faction_ability.1), ui);
                    // ui.text_edit_singleline(&mut unit.faction_ability);
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
                                    ability.combo_box(ui, i * 99 + 4000);
                                    match ability {
                                        CoreAbility::Scouts(x) => {
                                            select_drag_value_with_range_on_tab(x, 1..=99, ui);
                                        },
                                        CoreAbility::FiringDeck(x) => {
                                            select_drag_value_with_range_on_tab(x, 1..=99, ui);
                                        },
                                        CoreAbility::FeelnoPain(x) => {
                                            select_drag_value_with_range_on_tab(x, 1..=6, ui);
                                        },
                                        CoreAbility::DeadlyDemise(_, x) => {
                                            if !VariableValue::is_valid_variable_val(&x) {
                                                ui.style_mut().visuals.extreme_bg_color = Color32::RED;
                                            }
                                            select_text_on_tab(x.len(), egui::TextEdit::singleline(x), ui);
                                        },
                                        _ => {}
                                    }
                                });
                            });
                        });
                    }
                    for (j, i) in to_remove.iter().enumerate() {
                        unit.core_abilities.remove(i - j);
                    }
                });

            if ui.button("Add new core ability").clicked() {
                unit.core_abilities.push(CoreAbility::None);
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
                                    select_text_on_tab(ability.name.len(), egui::TextEdit::singleline(&mut ability.name), ui);
                                    // ui.text_edit_singleline(&mut ability.name);
                                });
                            });
                            row.col(|ui| {
                                select_text_on_tab(ability.description.len(), egui::TextEdit::multiline(&mut ability.description), ui);
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
                                    select_text_on_tab(keyword.len(), egui::TextEdit::singleline(keyword), ui);
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


            if unit.crusader {
                ui.heading("Crusade Information");
                ui.horizontal(|ui| {
                    ui.label("Experience:");
                    select_drag_value_with_range_on_tab(&mut unit.crusade_data.exp, 0..=500, ui);
                });

                ui.horizontal(|ui| {
                    ui.label("Kills:");
                    select_drag_value_with_range_on_tab(&mut unit.crusade_data.kills, 0..=500, ui);
                });


                ui.heading("Crusade Upgrades");
                TableBuilder::new(ui)
                    .id_salt(6)
                    .striped(true)
                    .resizable(false)
                    .column(Column::auto().at_least(200.0))
                    .column(Column::auto().at_least(200.0))
                    .column(Column::auto().at_least(400.0))
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .header(20.0, |mut header| {
                        for col_header in ["Type", "Name", "Description"] {
                            header.col(|ui| {
                                ui.strong(RichText::new(col_header).size(15.0));
                            });
                        }
                    })
                    .body(|mut body| {
                        let mut to_remove = Vec::new();
                        for (i, upgrade) in unit.crusade_data.upgrades.iter_mut().enumerate() {
                            let height = match upgrade {
                                CrusadeUpgrade::WeaponMod(_) => 40.0,
                                _ => 80.0
                            };
                            body.row(height, |mut row| {
                                row.col(|ui| {
                                    ui.horizontal(|ui| {
                                        if ui.button("X").on_hover_text("Delete").clicked() {
                                            to_remove.push(i);
                                        }
                                        upgrade.combo_box(ui, 7 + i * 3);
                                    });
                                });
                                row.col(|ui| {
                                    match upgrade {
                                        CrusadeUpgrade::BattleTrait(ability) => {
                                            ui.horizontal(|ui| {
                                                select_text_on_tab(ability.name.len(), egui::TextEdit::singleline(&mut ability.name), ui);
                                            });
                                        },
                                        CrusadeUpgrade::Relic(ability) => {
                                            ui.horizontal(|ui| {
                                                select_text_on_tab(ability.name.len(), egui::TextEdit::singleline(&mut ability.name), ui);
                                            });
                                        },
                                        CrusadeUpgrade::WeaponMod(weapon_mod) => {
                                            ui.horizontal(|ui| {
                                                select_text_on_tab(weapon_mod.name.len(), egui::TextEdit::singleline(&mut weapon_mod.name), ui);
                                            });
                                        }
                                    }
                                });
                                row.col(|ui| {
                                    match upgrade {
                                        CrusadeUpgrade::BattleTrait(ability) => {
                                            select_text_on_tab(ability.description.len(), egui::TextEdit::multiline(&mut ability.description), ui);
                                        },
                                        CrusadeUpgrade::Relic(ability) => {
                                            select_text_on_tab(ability.description.len(), egui::TextEdit::multiline(&mut ability.description), ui);
                                        },
                                        CrusadeUpgrade::WeaponMod(weapon_mod) => {
                                            ui.horizontal(|ui| {
                                                weapon_mod.combo_boxes(ui, 8 + i * 4);
                                                weapon_mod.target_select(ui, 10 + i * 4, &unit.ranged_weapons, &unit.melee_weapons);
                                            });
                                        }
                                    }
                                });
                            });
                        }
                        for (j, i) in to_remove.iter().enumerate() {
                            unit.crusade_data.upgrades.remove(i - j);
                        }
                    });

                if ui.button("Add new Upgrade").clicked() {
                    unit.crusade_data.upgrades.push(CrusadeUpgrade::WeaponMod(WeaponMod::default()));
                }

                ui.separator();
            }

        });
        
        
    });

    
    
}