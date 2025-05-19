use include_assets::{include_dir, NamedArchive};
use std::{path::PathBuf, str};
use crate::data::Unit;
mod pdf;
mod latex;


#[derive(PartialEq, Clone, Copy)]
pub enum ExportType {
    PDF,
    LATEX,
    HTML
}

impl ExportType {
    pub fn to_string(&self) -> String{
        match self {
            ExportType::PDF => "PDF".to_string(),
            ExportType::LATEX => "LaTeX".to_string(),
            ExportType::HTML => "HTML".to_string(),
        }
    }

    pub const fn get_extensions(&self) -> &[&str; 1]{
        match self {
            ExportType::PDF => &["pdf"],
            ExportType::LATEX => &["tex"],
            ExportType::HTML => &["html"],
        }
    }

    
}

pub struct ExportTemplates {
    latex: String
}

impl ExportTemplates {
    pub fn get_template(&self, format: ExportType) -> &str{
        match format {
            ExportType::PDF => &self.latex,
            ExportType::LATEX => &self.latex,
            ExportType::HTML => &self.latex,
        }
    }
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

pub fn export_unit(unit: &Unit, export_type: ExportType, export_path: PathBuf, export_templates: &ExportTemplates) {
    match export_type {
        ExportType::PDF => (),
        ExportType::LATEX => latex::export_to_latex(unit, export_templates.get_template(export_type), export_path).unwrap(),
        ExportType::HTML => (),
    };
}