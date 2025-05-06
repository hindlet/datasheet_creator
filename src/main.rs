use app::{string_to_color32, DatasheetApp, DatasheetAppSettings};
use egui::{Color32, IconData, ThemePreference};
use include_assets::{include_dir, NamedArchive};
mod data;
mod app;


fn main() -> eframe::Result {
    let archive = NamedArchive::load(include_dir!("assets"));
    let icon_data = archive.get("Logo_128.png").unwrap();
    let icon = image::load_from_memory(icon_data).unwrap().to_rgba8();
    let (icon_width, icon_height) = icon.dimensions();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1080.0, 720.0]).with_icon(IconData {
            rgba: icon.to_vec(),
            width: icon_width,
            height: icon_height
        }),
        ..Default::default()
    };
    eframe::run_native( 
        "Datasheet Creator",
        options,
        Box::new(|cc| {
            if let Some(storage) = cc.storage {
                let theme_pref = if storage.get_string("Dark_Mode").unwrap_or("true".to_string()) == "true" {
                    ThemePreference::Dark
                } else {ThemePreference::Light};
                cc.egui_ctx.options_mut(|opt| opt.theme_preference = theme_pref);
                let bar_col = string_to_color32(storage.get_string("Bar_Colour").unwrap_or("".to_string())).unwrap_or(Color32::LIGHT_BLUE);
                let key_col = string_to_color32(storage.get_string("Keyword_Colour").unwrap_or("".to_string())).unwrap_or(Color32::LIGHT_BLUE);
                let default_faction_ability = storage.get_string("Default_Faction_Ability").unwrap_or("".to_string());
                let default_faction_keyword = storage.get_string("Default_Faction_Keyword").unwrap_or("".to_string());

                return Ok(Box::new(DatasheetApp {
                    settings: DatasheetAppSettings {
                        bar_colour: bar_col,
                        keyword_colour: key_col,
                        default_faction_ability,
                        default_faction_keyword,
                        dark_mode: theme_pref == ThemePreference::Dark,
                    },
                    ..Default::default()
                }))
            }
            Ok(Box::<DatasheetApp>::default())
            
        })
    )
}