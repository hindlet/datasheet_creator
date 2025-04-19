use app::DatasheetApp;
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
        Box::new(|_| {
            Ok(Box::<DatasheetApp>::default())
        })
    )
}