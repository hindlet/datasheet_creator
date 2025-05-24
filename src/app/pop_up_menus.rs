use egui::{global_theme_preference_switch, CollapsingHeader, Context};

use crate::export::ExportType;

use super::{datasheet_app::DatasheetFolder, DatasheetAppSettings};



pub fn quit_menu(ctx: &Context, result: &mut Option<bool>) {
    egui::Window::new("Do you want to quit?")
    .collapsible(false)
    .resizable(false)
    .show(ctx, |ui| {
        ui.label("Make sure to save your work :)");
        ui.horizontal(|ui| {
            if ui.button("No").clicked() {
                *result = Some(false);
            }

            if ui.button("Yes").clicked() {
                *result = Some(true);
            }
        });
    });
}

pub fn delete_window(ctx: &Context, result: &mut Option<bool>) {
    egui::Window::new("Confirm Deletion?")
    .collapsible(false)
    .resizable(false)
    .show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Cancel").clicked() {
                *result = Some(false);
            }

            if ui.button("Confirm").clicked() {
                *result = Some(true);
            }
        });
    });
}


pub fn folder_creation_window(ctx: &Context, result: &mut Option<bool>, name: &mut String) {
    egui::Window::new("Create a new Folder?")
    .collapsible(false)
    .resizable(false)
    .show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("Name: ");
            ui.text_edit_singleline(name);
        });
        ui.horizontal(|ui| {
            if ui.button("Cancel").clicked() {
                *result = Some(false);
            }

            if ui.button("Confirm").clicked() && name != ""  {
                *result = Some(true);
            }
        });
    });
}


pub fn new_unit_window(ctx: &Context, result: &mut Option<bool>, name: &mut String, folders: &Vec<DatasheetFolder>, selected_folder: &mut usize) {
    egui::Window::new("Create new Unit?")
    .collapsible(false)
    .resizable(false)
    .show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("Folder: ");
            egui::ComboBox::from_id_salt(10)
                .selected_text(folders[*selected_folder].name.clone())
                .show_ui(ui, |ui| {
                    for (i, folder) in folders.iter().enumerate() {
                        ui.selectable_value(selected_folder, i, folder.name.clone());
                    }
                })
        });
        ui.horizontal(|ui| {
            ui.label("Filename: ");
            ui.text_edit_singleline(name);
        });
        ui.horizontal(|ui| {
            if ui.button("Cancel").clicked() {
                *result = Some(false);
            }

            if ui.button("Confirm").clicked() && name != "" {
                *result = Some(true);
            }
        });
    });
}


pub fn settings_window(ctx: &Context, result: &mut Option<bool>, settings: &mut DatasheetAppSettings) {
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
                    ui.color_edit_button_srgba(&mut settings.bar_colour);
                });
                ui.horizontal(|ui| {
                    ui.label("Keyword Colour:");
                    ui.color_edit_button_srgba(&mut settings.keyword_colour);
                });
                ui.horizontal(|ui| {
                    ui.label("Default Faction Ability:");
                    ui.text_edit_singleline(&mut settings.default_faction_ability);
                });
                ui.horizontal(|ui| {
                    ui.label("Default Faction Keyword:");
                    ui.text_edit_singleline(&mut settings.default_faction_keyword);
                });
            });
        ui.horizontal(|ui| {
            if ui.button("Save for Folder").clicked() {
                *result = Some(true);
            }
            if ui.button("Close").clicked() {
                *result = Some(false);
            }
        });
        
    });
}



pub fn export_window(ctx: &Context, result: &mut Option<bool>, export_type: &mut ExportType) {
    egui::Window::new("Settings")
    .collapsible(false)
    .resizable(true)
    .show(ctx, |ui| {

        ui.horizontal(|ui| {
            ui.label("Export File Type: ");
            egui::ComboBox::from_id_salt(10)
                .selected_text(export_type.to_string())
                .show_ui(ui, |ui| {
                    ui.selectable_value(export_type, ExportType::PDF, "PDF");
                    // ui.selectable_value(export_type, ExportType::LATEX, "LaTeX");
                    ui.selectable_value(export_type, ExportType::HTML, "HTML");
                })
        });

        ui.horizontal(|ui| {
            if ui.button("Save").clicked() {
                *result = Some(true);
            }
            if ui.button("Cancel").clicked() {
                *result = Some(false);
            }
        });


    });
}