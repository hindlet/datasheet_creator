use std::{fs::{self, DirEntry, File}, path::PathBuf};

use eframe::App;
use egui::{Color32, Context, RichText, Ui};
use egui_extras::{Column, TableBuilder};
use ron::{
    de::from_reader,
    ser::{to_string_pretty, PrettyConfig}
};

use crate::to_pdf::Unit;



pub struct DatasheetFolder {
    name: Option<String>,
    units: Vec<Unit>,
    path: PathBuf,
    files: Vec<PathBuf>,
    expanded: bool
}

pub enum DataSheetAppMode {
    Edit,
    Read
}

pub struct DatasheetApp {
    working_dir_name: String,
    working_dir: Vec<DatasheetFolder>,
    open_files: Vec<(usize, usize)>,
    selected_file: usize,
    mode: DataSheetAppMode,


}


impl DatasheetApp {
    fn open_folder(&mut self, path: PathBuf) {
        let dir = fs::read_dir(path).unwrap();
        
        for path in dir {
            let path = path.unwrap();
            if path.file_type().unwrap().is_dir() {
                // println!("Name: {}", path.path().display());
                self.read_dir(path.path());
                continue;
            }
            // if let Some(extension) = path.path().extension() {
            //     if extension.to_str() == Some("ron") {
            //         println!("Name: {}", path.path().display())
            //     }
            // }
        }
    }

    fn read_dir(&mut self, path: PathBuf) {

        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        let mut units = Vec::new();
        let dir = fs::read_dir(path.clone()).unwrap();
        let mut paths: Vec<_> = Vec::new();

        for path in dir {
            let path = path.unwrap().path();
            if let Some(extension) = path.extension() {
                if extension.to_str() == Some("ron") {
                    let f = File::open(path.clone()).unwrap();
                    let unit: Unit = from_reader(f).unwrap();
                    units.push(unit);
                }
            }
            paths.push(path);
        }
        self.working_dir.push(DatasheetFolder {
            name: Some(name),
            units: units,
            expanded: false,
            path,
            files: paths
        });
    }


    fn display_current(&mut self, ctx: &Context) {
        match self.mode {
            DataSheetAppMode::Edit => self.display_edit_mode(ctx),
            DataSheetAppMode::Read => self.display_read_mode(ctx),
        }
    }

    fn display_edit_mode(&mut self, ctx: &Context) {
        let index = self.open_files[self.selected_file];
        let unit = &mut self.working_dir[index.0].units[index.1];
        // ui.heading(egui::RichText::new(&unit.name).size(30.0));
        // egui::ScrollArea::vertical().show(ctx, |ui| {

        // });
    }

    fn display_read_mode(&self, ctx: &Context) {
        let index = self.open_files[self.selected_file];
        let unit = &self.working_dir[index.0].units[index.1];
    

        egui::TopBottomPanel::top("stats").show(ctx, |ui| {

            let frame = egui::Frame::canvas(ui.style())
                .inner_margin(0)
                .outer_margin(10)
                .corner_radius(10)
                .fill(egui::Color32::DARK_GRAY)
                .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY));
        
            let show_stat_name_func = |ui: &mut Ui, stat: &str| {
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new(stat).color(Color32::WHITE).size(20.0));
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
                show_stat_name_func(ui, "W");
                show_stat_name_func(ui, "ld");
                show_stat_name_func(ui, "OC");
                ui.end_row();
                show_stat_func(ui, format!("{}\"", unit.stats.movement));
                show_stat_func(ui, unit.stats.toughness.to_string());
                show_stat_func(ui, format!("{}+", unit.stats.save));
                show_stat_func(ui, unit.stats.wounds.to_string());
                show_stat_func(ui, format!("{}+", unit.stats.leadership));
                show_stat_func(ui, unit.stats.oc.to_string());
            }); 
        });

        egui::SidePanel::right("abilities").resizable(true).min_width(200.0).show(ctx, |ui| {
            ui.label("Test");
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label("Ability");
            })
        });

        egui::TopBottomPanel::bottom("Keywords").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let last = unit.keywords.len() - 1;
                for (i, keyword) in unit.keywords.iter().enumerate() {
                    if i < last{
                        ui.label(RichText::new(format!("{},", keyword.to_uppercase())).color(Color32::LIGHT_BLUE));
                    } else {
                        ui.label(RichText::new(keyword.to_uppercase()).color(Color32::LIGHT_BLUE));
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
                    header.col(|ui| {
                        ui.strong("Ranged Weapons");
                    });
                    header.col(|ui| {
                        ui.strong("Range");
                    });
                    header.col(|ui| {
                        ui.strong("A");
                    });
                    header.col(|ui| {
                        ui.strong("BS");
                    });
                    header.col(|ui| {
                        ui.strong("S");
                    });
                    header.col(|ui| {
                        ui.strong("AP");
                    });
                    header.col(|ui| {
                        ui.strong("D");
                    });
                })
                .body(|mut body| {
                    for weapon in unit.ranged_weapons.iter() {
                        let data = weapon.to_html_data();
                        body.row(20.0, |mut row| {
                            row.col(|ui| {
                                ui.label(data.0);
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
                    header.col(|ui| {
                        ui.strong("Melee Weapons");
                    });
                    header.col(|ui| {
                        ui.strong("Range");
                    });
                    header.col(|ui| {
                        ui.strong("A");
                    });
                    header.col(|ui| {
                        ui.strong("BS");
                    });
                    header.col(|ui| {
                        ui.strong("S");
                    });
                    header.col(|ui| {
                        ui.strong("AP");
                    });
                    header.col(|ui| {
                        ui.strong("D");
                    });
                })
                .body(|mut body| {
                    for weapon in unit.melee_weapons.iter() {
                        let data = weapon.to_html_data();
                        body.row(20.0, |mut row| {
                            row.col(|ui| {
                                ui.label(data.0);
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


    fn save_current(&self) {
        let (extra_dir, intra_dir) = self.open_files[self.selected_file];
        let config = PrettyConfig::new()
            .depth_limit(2)
            .separate_tuple_members(true)
            .enumerate_arrays(true);
        let s = to_string_pretty(&self.working_dir[extra_dir].units[intra_dir], config).expect("Failed to serialize");
        let _ = fs::write(self.working_dir[extra_dir].files[intra_dir].clone(), s);
    }

}



impl Default for DatasheetApp {
    fn default() -> Self {
        DatasheetApp {
            working_dir_name: "No Folder Open".to_string(),
            working_dir: Vec::new(),
            open_files: Vec::new(),
            selected_file: 0,
            mode: DataSheetAppMode::Read
        }
    }
}

impl App for DatasheetApp {
    
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        
        egui::SidePanel::left("LeftPanel").min_width(150.0).resizable(false).show(ctx, |ui| {
            

            if ui.button(&self.working_dir_name).clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.working_dir_name = path.file_name().unwrap().to_str().unwrap().to_string();
                    self.open_folder(path);
                }
            }
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (i, folder) in self.working_dir.iter_mut().enumerate() {
                    if let Some(name) = &folder.name {
                        if folder.expanded {
                            if ui.label(format!("□ {}", name)).clicked() {
                                folder.expanded = false;
                            };
                            for (j, unit) in folder.units.iter().enumerate() {
                                if ui.label(format!("> {}", unit.name)).clicked() && !self.open_files.contains(&(i, j)) {
                                    self.open_files.push((i, j));
                                }
                            }
                            ui.label("");
                        } else {
                            if ui.label(format!("– {}", name)).clicked() {
                                folder.expanded = true;
                            };
                        }
                    }
                }
            })
        });

            
        egui::TopBottomPanel::top("TopPanel").min_height(25.0).show(ctx, |ui| {
            // DOES NOT WORK WITHOUT ABILITY TO HORIZONTAL SCROLL
            egui::ScrollArea::horizontal().scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden).show(ui, |ui| {
                ui.horizontal(|ui| {
                    let mut to_close = Vec::new();
                    for (i, (extra_dir, intra_dir)) in self.open_files.iter().enumerate() {
                        if i != 0 {
                            ui.label("|");
                        }
                        if ui.label("X").clicked() {
                            to_close.push(i);
                        }
                        if ui.label(self.working_dir[*extra_dir].units[*intra_dir].name.clone()).clicked() {
                            self.selected_file = i;
                        };
                    }
                    for item in to_close {
                        self.open_files.remove(item);
                    }
                })
            });
        });

        egui::TopBottomPanel::bottom("BottomPanel").min_height(25.0).show(ctx, |ui| {
            match self.mode {
                DataSheetAppMode::Edit => {
                    if ui.button("Edit").clicked() {
                        self.mode = DataSheetAppMode::Read
                    }
                },
                DataSheetAppMode::Read => {
                    if ui.button("Read").clicked() {
                        self.mode = DataSheetAppMode::Edit
                    }
                }
            }
        });

        if self.open_files.len() > self.selected_file {
            self.display_current(ctx);
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {

            });
        }

        
    }
}