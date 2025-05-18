use include_assets::{include_dir, NamedArchive};
use std::str;
use crate::data::Unit;
mod pdf;
mod latex;


pub enum ExportType {
    PDF,
    Latex
}

pub struct ExportTemplates {
    latex: String
}


pub fn load_export_templates() -> ExportTemplates {
    let templates = NamedArchive::load(include_dir!("templates"));

    let latex_template = {
        let data = templates.get("template.tex").unwrap();
        match str::from_utf8(data) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence in Latex Template: {}", e)
        }
    };


    ExportTemplates {
        latex: latex_template.to_string()
    }
}

pub fn export(unit: &Unit, export_type: ExportType, export_path: String) {

}