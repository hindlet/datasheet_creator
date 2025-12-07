use egui::{Color32, Context, Rect, RichText, ScrollArea, Ui};
use egui_extras::{Column, TableBuilder};

use crate::{app::helper, data::{CrusadeUpgrade, Unit}};

use super::DatasheetAppSettings;



pub fn read_unit(settings: &DatasheetAppSettings, dark_mode: bool, ctx: &Context, unit: &Unit) {
    // let index = match app.open_files[app.selected_file] {
    //     OpenFile::Index(index) => index,
    //     _ => return
    // };
    // let unit = &app.working_dir[index.0].units[index.1];

    // let settings = app.get_settings();


    egui::TopBottomPanel::top("stats").show(ctx, |ui| {

        let frame = egui::Frame::canvas(ui.style())
            .inner_margin(0)
            .outer_margin(10)
            .corner_radius(10)
            .fill(egui::Color32::DARK_GRAY)
            .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY));

       
        let stat_title_color = if dark_mode {Color32::WHITE} else {Color32::BLACK};
        let show_stat_name_func = |ui: &mut Ui, stat: &str| {
            ui.vertical_centered(|ui| {
                ui.label(RichText::new(stat).color(stat_title_color).size(20.0));
            });
        };

        
        let show_stat_func = |ui: &mut Ui, stat: String| {
            frame.show(ui, |ui| {
                ui.set_width(50.0);
                ui.set_height(50.0);
                ui.centered_and_justified(|ui| {
                    ui.label(egui::RichText::new(stat).size(20.0).color(Color32::WHITE));
                });
            });
        };

        ui.horizontal(|ui| {
            ui.heading(egui::RichText::new(&unit.name).size(30.0));
            
        });
        
        egui::Grid::new("statsgrid").show(ui, |ui| {
            show_stat_name_func(ui, "M");
            show_stat_name_func(ui, "T");
            show_stat_name_func(ui, "Sv");
            if unit.stats.invuln.is_some() {
                show_stat_name_func(ui, "Inv");
            }
            show_stat_name_func(ui, "W");
            show_stat_name_func(ui, "ld");
            show_stat_name_func(ui, "OC");
            ui.end_row();
            show_stat_func(ui, format!("{}\"", unit.get_movement()));
            show_stat_func(ui, unit.stats.toughness.to_string());
            show_stat_func(ui, format!("{}+", unit.stats.save));
            if let Some(invuln) = unit.stats.invuln {
                show_stat_func(ui, format!("{}++", invuln));
            }
            show_stat_func(ui, unit.stats.wounds.to_string());
            show_stat_func(ui, format!("{}+", unit.stats.leadership));
            show_stat_func(ui, unit.stats.oc.to_string());
            if unit.crusade_unit {
                ui.label(egui::RichText::new(format!("{}({} exp)", unit.crusade_data.rank.to_string(), unit.crusade_data.exp)));
            }
        }); 
    });

    let paint_bg = |ui: &mut egui::Ui| {
        let gapless_rect = ui.max_rect().expand2(0.5 * ui.spacing().item_spacing);
        ui.painter().rect_filled(gapless_rect, 0.0, settings.bar_colour);
    };

    egui::SidePanel::right("abilities").resizable(true).min_width(200.0).show(ctx, |ui| {
        ui.vertical_centered_justified(|ui| {
            ui.painter().rect_filled(Rect::everything_above(185.0), 0.0, settings.bar_colour);
            ui.label(RichText::new("Abilities").size(15.0).color(Color32::BLACK))
        });

        if unit.core_abilities.len() != 0 {
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("CORE: "));
                let last = unit.core_abilities.len().checked_sub(1).unwrap_or(0);
                for (i, ability) in unit.core_abilities.iter().enumerate() {
                    if i < last{
                        ui.label(RichText::new(format!("{},", ability.to_render_string())).strong());
                    } else {
                        ui.label(RichText::new(ability.to_render_string()).strong());
                    }
                }
            });
            ui.separator();
        }
        
        if let Some(ability) = &unit.faction_ability {
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("FACTION: "));
                ui.label(RichText::new(ability).strong());
            });
            ui.separator();
        }

        egui::ScrollArea::vertical().show(ui, |ui| {
            
            for ability in unit.unique_abilities.iter() {
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new(format!("{}:", ability.name.to_uppercase())).strong());
                    ui.label(RichText::new(&ability.description));
                });
            }

            let mut has_crusade_ability = false;
            if unit.crusade_unit {
                for upgrade in unit.crusade_data.upgrades.iter() {
                    match upgrade {
                        CrusadeUpgrade::BattleTrait(ability) => {
                            ui.horizontal_wrapped(|ui| {
                                ui.label(RichText::new(format!("{}:", ability.name.to_uppercase())).strong());
                                ui.label(RichText::new(&ability.description));
                            });
                            has_crusade_ability = true;
                        }, 
                        CrusadeUpgrade::Relic(ability) => {
                            ui.horizontal_wrapped(|ui| {
                                ui.label(RichText::new(format!("{}:", ability.name.to_uppercase())).strong());
                                ui.label(RichText::new(&ability.description));
                            });
                            has_crusade_ability = true;
                        },
                        CrusadeUpgrade::Enhancement(ability) => {
                            ui.horizontal_wrapped(|ui| {
                                ui.label(RichText::new(format!("{}:", ability.name.to_uppercase())).strong());
                                ui.label(RichText::new(&ability.description));
                            });
                            has_crusade_ability = true;
                        },
                        CrusadeUpgrade::BattleScar(ability) => {
                            ui.horizontal_wrapped(|ui| {
                                ui.label(RichText::new(format!("{}:", ability.name.to_uppercase())).strong());
                                ui.label(RichText::new(&ability.description));
                            });
                            has_crusade_ability = true;
                        },
                        _ => {}
                    }
                }
            }
            

            if unit.unique_abilities.len() != 0 || has_crusade_ability {ui.separator();}

        });

        if !unit.faction_keyword.is_empty() {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Faction Keyword:");
                    ui.label(RichText::new(&unit.faction_keyword).color(settings.keyword_colour))
                });
                ui.separator();
            });
        }
        
    });


    

    egui::TopBottomPanel::bottom("Keywords").show(ctx, |ui| {
        ui.horizontal(|ui| {
            let last = unit.keywords.len().checked_sub(1).unwrap_or(0);
            for (i, keyword) in unit.keywords.iter().enumerate() {
                if i < last{
                    ui.label(RichText::new(format!("{},", keyword.to_uppercase())).color(settings.keyword_colour));
                } else {
                    ui.label(RichText::new(keyword.to_uppercase()).color(settings.keyword_colour));
                }
                
            }
        })
    });

        
    let ranged = if unit.crusade_unit {&unit.crusade_weapons.0} else {&unit.ranged_weapons};
    let melee = if unit.crusade_unit {&unit.crusade_weapons.1} else {&unit.melee_weapons};
    egui::CentralPanel::default().show(ctx, |ui| {
        ScrollArea::vertical().show(ui, |ui| {
            if ranged.len() > 0 {
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
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .header(20.0, |mut header| {
                        for col_header in ["Ranged Weapons", "Range", "A", "BS", "S", "AP", "D"] {
                            header.col(|ui| {
                                paint_bg(ui);
                                ui.strong(RichText::new(col_header).color(Color32::BLACK).size(15.0));
                            });
                        }
                    })
                    .body(|mut body| {
                        for (weapon, count) in ranged.iter() {
                            helper::draw_weapon_row(weapon, *count, &mut body, settings.keyword_colour);
                        }
                    });
                ui.separator();
            }
        
            if melee.len() > 0 {
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
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .header(20.0, |mut header| {
                        for col_header in ["Melee Weapons", "Range", "A", "WS", "S", "AP", "D"] {
                            header.col(|ui| {
                                paint_bg(ui);
                                ui.strong(RichText::new(col_header).color(Color32::BLACK).size(15.0));
                            });
                        }
                    })
                    .body(|mut body| {
                        for (weapon, count) in melee.iter() {
                            helper::draw_weapon_row(weapon, *count, &mut body, settings.keyword_colour);
                        }
                    });
            }
        });


        
        
    });
}