use std::{fs::{self, DirEntry, File}, path::PathBuf};

use eframe::App;
use ron::de::from_reader;

use crate::to_pdf::Unit;



pub struct DatasheetFolder {
    name: Option<String>,
    units: Vec<Unit>,
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
        let dir = fs::read_dir(path).unwrap();
        

        for path in dir {
            let path = path.unwrap().path();
            if let Some(extension) = path.extension() {
                if extension.to_str() == Some("ron") {
                    let f = File::open(path.clone()).unwrap();
                    let unit: Unit = from_reader(f).unwrap();
                    units.push(unit);
                }
            }
        }
        self.working_dir.push(DatasheetFolder {
            name: Some(name),
            units: units,
            expanded: false
        });
    }

}



impl Default for DatasheetApp {
    fn default() -> Self {
        DatasheetApp {
            working_dir_name: "No Folder Open".to_string(),
            working_dir: Vec::new(),
            open_files: Vec::new(),
            selected_file: 0,
            mode: DataSheetAppMode::Edit
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

        egui::CentralPanel::default().show(ctx, |ui| {
            
            egui::TopBottomPanel::top("TopPanel").min_height(25.0).show(ctx, |ui| {
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
                });
            });

            egui::CentralPanel::default().show(ctx, |ui| {
                if self.open_files.len() > self.selected_file {
                    let index = self.open_files[self.selected_file];
                    let unit = &self.working_dir[index.0].units[index.1];
                    ui.heading(&unit.name);

                    


                }   
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
            
        });

        
    }
}