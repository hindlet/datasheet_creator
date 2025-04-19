use app::{string_to_color32, DatasheetApp};
use egui::ThemePreference;
mod to_pdf;
mod vals;
mod app;


fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1080.0, 720.0]),
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
                let bar_col = string_to_color32(storage.get_string("Bar_Colour").unwrap_or("".to_string())).unwrap_or_default();


                return Ok(Box::new(DatasheetApp {
                    bar_colour: bar_col,
                    ..Default::default()
                }))
            }
            Ok(Box::<DatasheetApp>::default())
            
        })
    )
}