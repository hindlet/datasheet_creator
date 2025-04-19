use std::{fmt::format, fs::{self, remove_file, File}, path::PathBuf};

use edit_mode::{render_edit_mode, UnitEditData};
use eframe::App;
use egui::{CollapsingHeader, Color32, Context, RichText};
use read_mode::render_read_mode;
use ron::{
    de::from_reader,
    ser::{to_string_pretty, PrettyConfig}
};

use crate::to_pdf::Unit;
mod read_mode;
mod edit_mode;



pub struct DatasheetFolder {
    name: Option<String>,
    units: Vec<Unit>,
    unit_edit_data: Vec<UnitEditData>,

    path: String,
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
    deleting: (bool, Option<(usize, usize, String)>),
    new_unit: (bool, usize, String),

    show_confirmation_dialog: bool,
    allowed_to_close: bool,
}


impl DatasheetApp {
    fn open_folder(&mut self, path: PathBuf) {
        let dir = fs::read_dir(path).unwrap();
        self.working_dir = Vec::new();
        self.open_files = Vec::new();
        self.selected_file = 0;
        
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
        match self.mode {
            DataSheetAppMode::Edit => render_edit_mode(self, ctx),
            DataSheetAppMode::Read => render_read_mode(self, ctx),
        }
    }

    

    


    pub fn save_current(&mut self) {
        let (extra_dir, intra_dir) = self.open_files[self.selected_file];
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
            // let _ = remove_file(format!("{}/{}.ron", self.working_dir[extra_dir].path, data.prev_filename.clone()));
            data.prev_filename = data.filename.clone(); // update filename
        }

        let path = format!("{}/{}.ron", self.working_dir[extra_dir].path, data.filename.clone());
        let _ = fs::write(path, s);
    }

    fn reset_current(&mut self) {
        let (extra_dir, intra_dir) = self.open_files[self.selected_file];

        self.working_dir[extra_dir].unit_edit_data[intra_dir] = UnitEditData::from((&self.working_dir[extra_dir].units[intra_dir], self.working_dir[extra_dir].unit_edit_data[intra_dir].prev_filename.clone()));
    }

    fn create_unit(&mut self, folder: usize, filename: String) {

        let i= self.working_dir[folder].unit_edit_data.len();

        let new_unit = Unit {
            name: filename.clone(),
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
    }

    fn delete_unit(&mut self, folder: usize, file: usize) {
        
        let _ = remove_file(format!("{}/{}.ron", self.working_dir[folder].path, self.working_dir[folder].unit_edit_data[file].prev_filename));
        self.working_dir[folder].units.remove(file);
        self.working_dir[folder].unit_edit_data.remove(file);
        for (i, index) in self.open_files.iter().enumerate() {
            if index == &(folder, file) {
                self.open_files.remove(i);
                break;
            }
        }
    }
}



impl Default for DatasheetApp {
    fn default() -> Self {
        DatasheetApp {
            working_dir_name: "No Folder Open".to_string(),
            working_dir: Vec::new(),
            open_files: Vec::new(),
            selected_file: 0,
            mode: DataSheetAppMode::Read,
            deleting: (false, None),
            new_unit: (false, 0, "".to_string()),

            show_confirmation_dialog: false,
            allowed_to_close: false
        }
    }
}

impl App for DatasheetApp {
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
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
                    if self.deleting.0 == true {
                        ui.style_mut().visuals.widgets.active.weak_bg_fill = Color32::RED;
                        ui.style_mut().visuals.widgets.inactive.weak_bg_fill = Color32::RED;
                        ui.style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::RED;
                    }
                    if ui.button("Delete Unit").clicked() {
                        self.deleting.0 ^= true;
                    }
                });
            }
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (i, folder) in self.working_dir.iter_mut().enumerate() {
                    if self.deleting.0 {
                        ui.style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::RED;
                    }
                    if let Some(name) = &folder.name {
                        CollapsingHeader::new(name)
                            .default_open(false)
                            .show(ui, |ui| {
                                for (j, unit) in folder.units.iter().enumerate() {
                                    if ui.selectable_label(false, &unit.name).clicked() {
                                        if !self.open_files.contains(&(i, j)) && !self.deleting.0 {
                                            self.open_files.push((i, j));
                                        }
                                        if self.deleting.0 {
                                            self.deleting.1 = Some((i, j, unit.name.clone()));
                                        }
                                        
                                    }
                                }
                            });
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
                        if ui.selectable_label(false, "X").clicked() {
                            to_close.push(i);
                        }
                        if ui.selectable_label(false, self.working_dir[*extra_dir].units[*intra_dir].name.clone()).clicked() {
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
                    ui.horizontal(|ui| {
                        if ui.button("Edit Mode").clicked() {
                            self.mode = DataSheetAppMode::Read
                        };
                        if self.open_files.len() > self.selected_file {
                            if ui.button("Save Changes").clicked() {
                                self.save_current();
                            }
                            if ui.button("Discard Changes").clicked() {
                                self.reset_current();
                            }
                        }
                    });
                },
                DataSheetAppMode::Read => {
                    if ui.button("Read Mode").clicked() {
                        self.mode = DataSheetAppMode::Edit
                    }
                }
            }
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

        if let Some((i, j, unit_name)) = self.deleting.1.clone() {
            egui::Window::new("Confirm Deletion?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(format!("Do you want to delete {}", unit_name));
                    ui.horizontal(|ui| {
                        if ui.button("No").clicked() {
                            self.deleting.1 = None;
                        }

                        if ui.button("Yes").clicked() {
                            self.delete_unit(i, j);
                            self.deleting.1 = None;
                        }
                    });
                });
        }

        if self.new_unit.0 {
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
        }

    }
}