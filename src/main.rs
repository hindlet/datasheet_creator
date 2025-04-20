use app::{string_to_color32, DatasheetApp};
use egui::{IconData, ThemePreference};
mod to_pdf;
mod vals;
mod app;


fn main() -> eframe::Result {
    let icon = image::open("assets/Logo_128.png").expect("Failed to open icon path").to_rgba8();
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