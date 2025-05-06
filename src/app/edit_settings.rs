use egui::Context;

use super::DatasheetApp;



pub fn settings_panel(app: &mut DatasheetApp, ctx: &Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading(egui::RichText::new("Settings").size(30.0));
        let settings = app.folder_settings.as_mut().unwrap();

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
    });
}