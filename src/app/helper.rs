use egui::{Color32, RichText};
use egui_extras::TableBody;

use crate::data::{ChargeLevels, Weapon};



pub fn draw_weapon_row(weapon: &Weapon, count: u32, body: &mut TableBody, keyword_colour: Color32) {
    let data = weapon.get_render_data();
    let has_keywords = data.7 != "[]";
    let height = if has_keywords{32.0} else {22.0};

    body.row(height, |mut row| {
        row.col(|ui| {
            let name = match &weapon.charge {
                ChargeLevels::None => weapon.name.clone(),
                ChargeLevels::Parent(level_name) => format!("{} - {}", weapon.name, level_name),
                ChargeLevels::Child(parent_ref, level_name) => format!("{} - {}", parent_ref.name, level_name)
            };
            let title = if count == 1 {name} else {format!("{}x {}", count, name)};

            if has_keywords {
                ui.vertical(|ui| {
                    ui.label(RichText::new(title).size(14.0));
                    ui.label(RichText::new(data.7).color(keyword_colour).size(10.5))
                });
            } else {
                ui.label(RichText::new(title).size(14.0));
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