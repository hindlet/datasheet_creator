use std::{fs::{self, remove_file, File}, path::PathBuf};

use crate::data::Unit;

use super::{edit_settings::settings_panel, edit_unit::{edit_unit, UnitEditData}, read_unit::read_unit};
use eframe::App;
use egui::{global_theme_preference_switch, CollapsingHeader, Color32, Context, RichText, ThemePreference};
use ron::{
    de::from_reader,
    ser::{to_string_pretty, PrettyConfig}
};
use super::settings::DatasheetAppSettings;


#[derive(PartialEq)]
pub enum OpenFile {
    Index((usize, usize)),
    Settings
}




pub struct DatasheetFolder {
    name: Option<String>,
    pub units: Vec<Unit>,
    pub unit_edit_data: Vec<UnitEditData>,

    path: String,
}

pub enum DatasheetAppMode {
    Edit,
    Read
}



pub struct DatasheetApp {
    pub working_dir_name: String,
    pub working_dir: Vec<DatasheetFolder>,
    pub open_files: Vec<OpenFile>,
    pub selected_file: usize,
    pub mode: DatasheetAppMode,
    pub folder_path: String,

    pub deleting: Option<(usize, usize)>,
    pub new_unit: (bool, usize, String),

    pub show_confirmation_dialog: bool,
    pub allowed_to_close: bool,



    pub settings_menu_open: bool,
    pub settings: DatasheetAppSettings,
    pub folder_settings: Option<DatasheetAppSettings>
}

impl DatasheetApp {
    fn open_folder(&mut self, path: PathBuf) {
        self.folder_path = path.as_path().to_str().unwrap().to_string();
        let dir = fs::read_dir(path).unwrap();
        self.working_dir = Vec::new();
        self.open_files = Vec::new();
        self.selected_file = 0;
        
        
        for path in dir {
            let path = path.unwrap();
            if path.file_type().unwrap().is_dir() {
                self.read_dir(path.path());
                continue;
            } else if path.file_name() == "SETTINGS.ron" {
                let f = File::open(path.path().clone()).unwrap();
                let settings: DatasheetAppSettings = from_reader(f).unwrap();
                self.folder_settings = Some(settings);
            }
        }
    }

    fn read_dir(&mut self, path: PathBuf) {

        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        let mut units = Vec::new();
        let mut unit_edit_data = Vec::new();
        let dir = fs::read_dir(path.clone()).unwrap();

        for path in dir {
            let path = path.unwrap().path();
            if let Some(extension) = path.extension() {
                if extension.to_str() == Some("ron") {
                    let f = File::open(path.clone()).unwrap();
                    let unit: Unit = from_reader(f).unwrap();
                    unit_edit_data.push(UnitEditData::from((&unit, path.file_name().unwrap().to_str().unwrap().replace(".ron", "").to_string())));
                    units.push(unit);
                }
            }
        }
        self.working_dir.push(DatasheetFolder {
            name: Some(name),
            units: units,
            unit_edit_data,
            path: path.to_str().unwrap().to_string(),
        });
    }


    fn display_current(&mut self, ctx: &Context) {
        match self.open_files[self.selected_file] {
            OpenFile::Settings => settings_panel(self, ctx),
            OpenFile::Index(index) => {
                match self.mode {
                    DatasheetAppMode::Edit => edit_unit(ctx,  &mut self.working_dir[index.0].unit_edit_data[index.1]),
                    DatasheetAppMode::Read => read_unit(self.get_settings(), ctx, &self.working_dir[index.0].units[index.1]),
                }
            },
        };
    }


    pub fn save_current(&mut self) {
        let (extra_dir, intra_dir) = match self.open_files[self.selected_file] {
            OpenFile::Index(index) => index,
            _ => return
        };

        let config = PrettyConfig::new()
            .depth_limit(2)
            .separate_tuple_members(true)
            .enumerate_arrays(true);


        let mut data = self.working_dir[extra_dir].unit_edit_data[intra_dir].clone();
        
        
        let new_unit: Unit = data.clone().into();
        self.working_dir[extra_dir].units[intra_dir] = new_unit;
        let s = to_string_pretty(&self.working_dir[extra_dir].units[intra_dir], config).expect("Failed to serialize");


        if data.prev_filename != data.filename {
            self.delete_unit(extra_dir, intra_dir);
            data.prev_filename = data.filename.clone(); // update filename
        }

        let path = format!("{}/{}.ron", self.working_dir[extra_dir].path, data.filename.clone());
        let _ = fs::write(path, s);
    }

    fn reset_current(&mut self) {
        let (extra_dir, intra_dir) = match self.open_files[self.selected_file] {
            OpenFile::Index(index) => index,
            _ => return
        };

        self.working_dir[extra_dir].unit_edit_data[intra_dir] = UnitEditData::from((&self.working_dir[extra_dir].units[intra_dir], self.working_dir[extra_dir].unit_edit_data[intra_dir].prev_filename.clone()));
    }

    fn create_unit(&mut self, folder: usize, filename: String) {

        let i= self.working_dir[folder].unit_edit_data.len();
        let settings = self.get_settings();
        let faction_ability = if settings.default_faction_ability.is_empty() {None} else {Some(settings.default_faction_ability.clone())};

        let new_unit = Unit {
            name: filename.clone(),
            faction_ability: faction_ability,
            ..Default::default()
        };
        let new_unit_edit_data = UnitEditData::from((&new_unit, filename));

        self.working_dir[folder].units.push(new_unit);
        self.working_dir[folder].unit_edit_data.push(new_unit_edit_data);

        let config = PrettyConfig::new()
            .depth_limit(2)
            .separate_tuple_members(true)
            .enumerate_arrays(true);


        let s = to_string_pretty(&self.working_dir[folder].units[i], config).expect("Failed to serialize");
        let _ = fs::write(format!("{}/{}.ron", self.working_dir[folder].path, self.working_dir[folder].unit_edit_data[i].prev_filename), s);
        self.selected_file = self.open_files.len();
        self.open_files.push(OpenFile::Index((folder, i)));
    }

    fn delete_unit(&mut self, folder: usize, file: usize) {
        
        let _ = remove_file(format!("{}/{}.ron", self.working_dir[folder].path, self.working_dir[folder].unit_edit_data[file].prev_filename));
        self.working_dir[folder].units.remove(file);
        self.working_dir[folder].unit_edit_data.remove(file);
        let index = OpenFile::Index((folder, file));
        for (i, open_file) in self.open_files.iter().enumerate() {
            if open_file == &index {
                self.open_files.remove(i);
                break;
            }
        }
        for (i, index) in self.open_files.iter().enumerate() {
            match index {
                OpenFile::Index(index) => {
                    if index == &(folder, file) {
                        self.open_files.remove(i);
                        break;
                    }
                },
                _ => return
            }
        }
    }


    pub fn get_settings(&self) -> &DatasheetAppSettings {
        if let Some(settings) = &self.folder_settings {
            return settings;
        }
        return &self.settings;
    }

    fn save_folder_settings(&self) {
        if let Some(settings) = &self.folder_settings {
            let config = PrettyConfig::new()
            .depth_limit(2)
            .separate_tuple_members(true)
            .enumerate_arrays(true);

            let s = to_string_pretty(settings, config).expect("Failed to serialize");
            let _ = fs::write(format!("{}/SETTINGS.ron", self.folder_path), s);
        }
    }

    fn copy_unit(&mut self, unit: &Unit, folder_index: usize, filename: String) {
        self.working_dir[folder_index].units.push(unit.clone());
        self.working_dir[folder_index].unit_edit_data.push(UnitEditData::from((unit, filename)));
    }
}



impl Default for DatasheetApp {
    fn default() -> Self {
        DatasheetApp {
            working_dir_name: "No Folder Open".to_string(),
            working_dir: Vec::new(),
            open_files: Vec::new(),
            folder_path: "".to_string(),
            selected_file: 0,
            mode: DatasheetAppMode::Read,
            deleting: None,
            new_unit: (false, 0, "".to_string()),

            show_confirmation_dialog: false,
            allowed_to_close: false,
            settings_menu_open: false,
            settings: DatasheetAppSettings::default(),
            folder_settings: None
        }
    }
}

impl App for DatasheetApp {
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        let mut copy_data = None;
        egui::SidePanel::left("LeftPanel").min_width(150.0).resizable(false).show(ctx, |ui| {

            if ui.button(RichText::new(&self.working_dir_name).size(15.0)).clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.working_dir_name = path.file_name().unwrap().to_str().unwrap().to_string();
                    self.open_folder(path);
                }
            }
            if self.working_dir_name != "No Folder Open".to_string() {
                ui.horizontal(|ui| {
                    if ui.button("New Unit").clicked() {
                        self.new_unit = (true, 0, "".to_string());
                    }

                    ui.reset_style();
                    // if self.deleting.0 == true {
                    //     ui.style_mut().visuals.widgets.active.weak_bg_fill = Color32::RED;
                    //     ui.style_mut().visuals.widgets.inactive.weak_bg_fill = Color32::RED;
                    //     ui.style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::RED;
                    // }
                    // if ui.button("Delete Unit").clicked() {
                    //     self.deleting.0 ^= true;
                    // }
                });
            }
            
            egui::ScrollArea::vertical().show(ui, |ui| {
                if self.folder_settings.is_some() {
                    if ui.selectable_label(false, "Settings").clicked() {
                        if !self.open_files.contains(&OpenFile::Settings) {
                            self.selected_file = self.open_files.len();
                            self.open_files.push(OpenFile::Settings);
                        } else {
                            self.selected_file = self.open_files.iter().position(|u| u == &OpenFile::Settings).unwrap();
                        }
                    }
                }

                
                for (i, folder) in self.working_dir.iter_mut().enumerate() {

                    if let Some(name) = &folder.name {
                        CollapsingHeader::new(name)
                            .default_open(false)
                            .show(ui, |ui| {
                                for (j, unit) in folder.units.iter().enumerate() {
                                    let selected: bool;
                                    if let Some(index) = self.deleting {
                                        if index == (i, j) {
                                            ui.style_mut().visuals.selection.bg_fill = Color32::RED;
                                            selected = true;
                                        } else {selected = false;}
                                    } else {selected = false;}
                                    let unit_label = ui.selectable_label(selected, &unit.name);
                                    if unit_label.clicked() {
                                        let new_file = OpenFile::Index((i, j));
                                        if !self.open_files.contains(&new_file) {
                                            self.selected_file = self.open_files.len();
                                            self.open_files.push(new_file);
                                        } else {
                                            self.selected_file = self.open_files.iter().position(|u| u == &new_file).unwrap();
                                        }
                                    }
                                    unit_label.context_menu(|ui| {
                                        if ui.selectable_label(false, "Delete Unit").clicked() {
                                            self.deleting = Some((i, j));
                                            ui.close_menu();
                                        }
                                        if ui.selectable_label(false, "Duplicate").clicked() {
                                            let new_filename_start = folder.unit_edit_data[j].filename.clone();
                                            
                                            let mut k = 1;
                                            let mut taken = true;
                                            let mut new_filename: String = "".to_string();
                                            while taken {
                                                taken = false;
                                                new_filename = format!("{}_{}", new_filename_start, k);
                                                for edit_data in folder.unit_edit_data.iter() {
                                                    if edit_data.filename == new_filename {
                                                        taken = true;
                                                        break;
                                                    }
                                                }
                                                k += 1
                                            };
                                            copy_data = Some((unit.clone(), i, new_filename));
                                            ui.close_menu();
                                        }
                                    });
                                }
                            });
                    }
                }
            });
        });
        
        if let Some((unit, index, filename)) = copy_data {
            self.copy_unit(&unit, index, filename);
        }

            
        egui::TopBottomPanel::top("TopPanel").min_height(25.0).show(ctx, |ui| {
            // DOES NOT WORK WITHOUT ABILITY TO HORIZONTAL SCROLL
            egui::ScrollArea::horizontal().scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden).show(ui, |ui| {
                ui.horizontal(|ui| {
                    let mut to_close = Vec::new();
                    for (i, file) in self.open_files.iter().enumerate() {
                        if i != 0 {
                            ui.label("|");
                        }
                        if ui.selectable_label(false, "X").clicked() {
                            to_close.push(i);
                            if (i < self.selected_file) || (i == self.selected_file && i != 0) {
                                self.selected_file -=1;
                            }
                        }
                        match file {
                            OpenFile::Index((extra_dir, intra_dir)) => {
                                if ui.selectable_label(false, self.working_dir[*extra_dir].units[*intra_dir].name.clone()).clicked() {
                                    self.selected_file = i;
                                };
                            },
                            OpenFile::Settings => {
                                if ui.selectable_label(false, "Settings").clicked() {
                                    self.selected_file = i;
                                };
                            }
                        }
                        
                    }
                    for item in to_close {
                        self.open_files.remove(item);
                    }
                })
            });
        });

        egui::TopBottomPanel::bottom("BottomPanel").min_height(25.0).show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Settings").clicked() {
                    self.settings_menu_open = true;
                }
                match self.mode {
                    DatasheetAppMode::Edit => {
                        if ui.button("Edit Mode").clicked() {
                            self.mode = DatasheetAppMode::Read
                        };
                        if self.open_files.len() > self.selected_file {
                            if ui.button("Save Changes").clicked() {
                                self.save_current();
                            }
                            if ui.button("Discard Changes").clicked() {
                                self.reset_current();
                            }
                        }
                    },
                    DatasheetAppMode::Read => {
                        if ui.button("Read Mode").clicked() {
                            self.mode = DatasheetAppMode::Edit
                        }
                    }
                }
            })
            
        });

        if self.open_files.len() > self.selected_file {
            self.display_current(ctx);
        } else {
            egui::CentralPanel::default().show(ctx, |_| {});
        }


        if ctx.input(|i| i.viewport().close_requested()) {
            if self.allowed_to_close {
                // do nothing - we will close
            } else {
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                self.show_confirmation_dialog = true;
            }
        }

        if self.show_confirmation_dialog {
            egui::Window::new("Do you want to quit?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("Make sure to save your work :)");
                    ui.horizontal(|ui| {
                        if ui.button("No").clicked() {
                            self.show_confirmation_dialog = false;
                            self.allowed_to_close = false;
                        }

                        if ui.button("Yes").clicked() {
                            self.show_confirmation_dialog = false;
                            self.allowed_to_close = true;
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
        }

        if let Some((i, j)) = self.deleting {
            egui::Window::new("Confirm Deletion?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            self.deleting = None;
                        }

                        if ui.button("Confirm").clicked() {
                            self.delete_unit(i, j);
                            self.deleting = None;
                        }
                    });
                });
        }

        if self.new_unit.0 {
            if self.working_dir.len() != 0 {
                egui::Window::new("Create new Unit?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Folder: ");
                        egui::ComboBox::from_id_salt(10)
                            .selected_text(self.working_dir[self.new_unit.1].name.clone().unwrap_or("Main".to_string()))
                            .show_ui(ui, |ui| {
                                for (i, folder) in self.working_dir.iter().enumerate() {
                                    ui.selectable_value(&mut self.new_unit.1, i, folder.name.clone().unwrap_or("Main".to_string()));
                                }
                            })
                    });
                    ui.horizontal(|ui| {
                        ui.label("Filename: ");
                        ui.text_edit_singleline(&mut self.new_unit.2);
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            self.new_unit.0 = false;
                        }

                        if ui.button("Confirm").clicked() && self.new_unit.2 != "" {
                            self.create_unit(self.new_unit.1, self.new_unit.2.clone());
                            self.new_unit.0 = false;
                        }
                    });
                });
            } else {
                egui::Window::new("Please add Folders Before Making Units")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    if ui.button("I Promise I Will").clicked() {
                        self.new_unit.0 = false;
                    };
                });
            }
        }

        if self.settings_menu_open {
            egui::Window::new("Settings")
                .collapsible(false)
                .resizable(true)
                
                .show(ctx, |ui| {
                    CollapsingHeader::new("UI")
                        .default_open(false)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label("Light Mode:");
                                global_theme_preference_switch(ui);
                            });
                            ui.horizontal(|ui| {
                                ui.label("Bar Colour:");
                                ui.color_edit_button_srgba(&mut self.settings.bar_colour);
                            });
                            ui.horizontal(|ui| {
                                ui.label("Keyword Colour:");
                                ui.color_edit_button_srgba(&mut self.settings.keyword_colour);
                            });
                            ui.horizontal(|ui| {
                                ui.label("Default Faction Ability:");
                                ui.text_edit_singleline(&mut self.settings.default_faction_ability);
                            });
                            ui.horizontal(|ui| {
                                ui.label("Default Faction Keyword:");
                                ui.text_edit_singleline(&mut self.settings.default_faction_keyword);
                            });
                        });
                    ui.horizontal(|ui| {
                        if ui.button("Save for Folder").clicked() {
                            self.folder_settings = Some(self.settings.clone());
                        }
                        if ui.button("Close").clicked() {
                            self.settings_menu_open = false;
                        }
                    });
                   
                });
        }

        self.settings.dark_mode = ctx.options(|opt| opt.theme_preference == ThemePreference::Dark);
    }


    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        self.settings.save(storage);
        self.save_folder_settings();
    }


}