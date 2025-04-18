use egui::Context;

use super::DatasheetApp;







pub fn render_edit_mode(app: &mut DatasheetApp, ctx: &Context) {
    let index = app.open_files[app.selected_file];
    let unit = &mut app.working_dir[index.0].units[index.1];
    

    
}