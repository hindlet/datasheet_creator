use std::{fs::{self, File}, path::PathBuf};

use edit_mode::{render_edit_mode, UnitEditData};
use eframe::App;
use egui::{CollapsingHeader, Context};
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

    files: Vec<PathBuf>,
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

    show_confirmation_dialog: bool,
    allowed_to_close: bool,
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
        let mut unit_edit_data = Vec::new();
        let dir = fs::read_dir(path.clone()).unwrap();
        let mut paths: Vec<_> = Vec::new();

        for path in dir {
            let path = path.unwrap().path();
            if let Some(extension) = path.extension() {
                if extension.to_str() == Some("ron") {
                    let f = File::open(path.clone()).unwrap();
                    let unit: Unit = from_reader(f).unwrap();
                    unit_edit_data.push(UnitEditData::from(&unit));
                    units.push(unit);
                    
                }
            }
            paths.push(path);
        }
        self.working_dir.push(DatasheetFolder {
            name: Some(name),
            units: units,
            unit_edit_data,
            files: paths
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

        let new_unit: Unit = self.working_dir[extra_dir].unit_edit_data[intra_dir].clone().into();
        self.working_dir[extra_dir].units[intra_dir] = new_unit;
        let s = to_string_pretty(&self.working_dir[extra_dir].units[intra_dir], config).expect("Failed to serialize");
        let _ = fs::write(self.working_dir[extra_dir].files[intra_dir].clone(), s);
    }

    fn reset_current(&mut self) {
        let (extra_dir, intra_dir) = self.open_files[self.selected_file];

        self.working_dir[extra_dir].unit_edit_data[intra_dir] = UnitEditData::from(&self.working_dir[extra_dir].units[intra_dir]);
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

            show_confirmation_dialog: false,
            allowed_to_close: false
        }
    }
}

impl App for DatasheetApp {
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
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
                        CollapsingHeader::new(name)
                            .default_open(false)
                            .show(ui, |ui| {
                                for (j, unit) in folder.units.iter().enumerate() {
                                    if ui.selectable_label(false, &unit.name).clicked() && !self.open_files.contains(&(i, j)) {
                                        self.open_files.push((i, j));
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
                            if ui.button("Delete Changes").clicked() {
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
    }
}