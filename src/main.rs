use app::{string_to_color32, DatasheetApp};
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
                if let Some(bar_col) =storage.get_string("Bar_Colour") {
                    return Ok(Box::new(DatasheetApp {
                        bar_colour: string_to_color32(bar_col).unwrap_or_default(),
                        ..Default::default()
                    }))
                }
            }
            Ok(Box::<DatasheetApp>::default())
            
        })
    )
}