use egui::{Color32, Context, Rect, RichText, Ui};
use egui_extras::{Column, TableBuilder};

use super::DatasheetApp;



pub fn render_read_mode(app: &mut DatasheetApp, ctx: &Context) {
    let index = app.open_files[app.selected_file];
    let unit = &app.working_dir[index.0].units[index.1];


    egui::TopBottomPanel::top("stats").show(ctx, |ui| {

        let frame = egui::Frame::canvas(ui.style())
            .inner_margin(0)
            .outer_margin(10)
            .corner_radius(10)
            .fill(egui::Color32::DARK_GRAY)
            .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY));

       
        let stat_title_color = if app.dark_mode {Color32::WHITE} else {Color32::BLACK};
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

        ui.heading(egui::RichText::new(&unit.name).size(30.0));
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
            show_stat_func(ui, format!("{}\"", unit.stats.movement));
            show_stat_func(ui, unit.stats.toughness.to_string());
            show_stat_func(ui, format!("{}+", unit.stats.save));
            if let Some(invuln) = unit.stats.invuln {
                show_stat_func(ui, format!("{}++", invuln));
            }
            show_stat_func(ui, unit.stats.wounds.to_string());
            show_stat_func(ui, format!("{}+", unit.stats.leadership));
            show_stat_func(ui, unit.stats.oc.to_string());
        }); 
    });

    let paint_bg = |ui: &mut egui::Ui| {
        let gapless_rect = ui.max_rect().expand2(0.5 * ui.spacing().item_spacing);
        ui.painter().rect_filled(gapless_rect, 0.0, app.bar_colour);
    };

    egui::SidePanel::right("abilities").resizable(true).min_width(200.0).show(ctx, |ui| {
        ui.vertical_centered_justified(|ui| {
            ui.painter().rect_filled(Rect::everything_above(185.0), 0.0, app.bar_colour);
            ui.label(RichText::new("Abilities").size(15.0).color(Color32::BLACK))
        });

        if unit.core_abilities.len() != 0 {
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("CORE: "));
                let last = unit.core_abilities.len().checked_sub(1).unwrap_or(0);
                for (i, ability) in unit.core_abilities.iter().enumerate() {
                    if i < last{
                        ui.label(RichText::new(format!("{},", ability)).strong());
                    } else {
                        ui.label(RichText::new(ability).strong());
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
            
            let last = unit.unique_abilities.len().checked_sub(1).unwrap_or(0);
            for (i, ability) in unit.unique_abilities.iter().enumerate() {
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new(format!("{}:", ability.name.to_uppercase())).strong());
                    ui.label(RichText::new(&ability.description));
                });
                if i < last {
                    ui.separator();
                }
            }

        })
    });


   

    egui::TopBottomPanel::bottom("Keywords").show(ctx, |ui| {
        ui.horizontal(|ui| {
            let last = unit.keywords.len().checked_sub(1).unwrap_or(0);
            for (i, keyword) in unit.keywords.iter().enumerate() {
                if i < last{
                    ui.label(RichText::new(format!("{},", keyword.to_uppercase())).color(app.keyword_colour));
                } else {
                    ui.label(RichText::new(keyword.to_uppercase()).color(app.keyword_colour));
                }
                
            }
        })
    });
        
    egui::CentralPanel::default().show(ctx, |ui| {
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
                for weapon in unit.ranged_weapons.iter() {
                    let data = weapon.get_render_data();
                    let has_keywords = data.7 != "[]";
                    let height = if has_keywords{30.0} else {20.0};

                    body.row(height, |mut row| {
                        row.col(|ui| {
                            if has_keywords {
                                ui.vertical(|ui| {
                                    ui.label(data.0);
                                    ui.label(RichText::new(data.7).color(app.keyword_colour).size(10.5))
                                });
                            } else {
                                ui.label(data.0);
                            }
                        });
                        row.col(|ui| {
                            ui.label(data.1);
                        });
                        row.col(|ui| {
                            ui.label(data.2);
                        });
                        row.col(|ui| {
                            ui.label(data.3);
                        });
                        row.col(|ui| {
                            ui.label(data.4.to_string());
                        });
                        row.col(|ui| {
                            ui.label(data.5);
                        });
                        row.col(|ui| {
                            ui.label(data.6);
                        });
                    });
                }
            });
        ui.separator();
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
                for weapon in unit.melee_weapons.iter() {
                    let data = weapon.get_render_data();
                    let has_keywords = data.7 != "[]";
                    let height = if has_keywords{30.0} else {20.0};

                    body.row(height, |mut row| {
                        row.col(|ui| {
                            if has_keywords {
                                ui.vertical(|ui| {
                                    ui.label(data.0);
                                    ui.label(RichText::new(data.7).color(app.keyword_colour).size(10.5))
                                });
                            } else {
                                ui.label(data.0);
                            }
                        });
                        row.col(|ui| {
                            ui.label(data.1);
                        });
                        row.col(|ui| {
                            ui.label(data.2);
                        });
                        row.col(|ui| {
                            ui.label(data.3);
                        });
                        row.col(|ui| {
                            ui.label(data.4.to_string());
                        });
                        row.col(|ui| {
                            ui.label(data.5);
                        });
                        row.col(|ui| {
                            ui.label(data.6);
                        });
                    });
                }
            });
    });
}