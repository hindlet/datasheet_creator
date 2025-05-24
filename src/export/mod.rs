use include_assets::{include_dir, NamedArchive};
use tera::Tera;
use std::{path::PathBuf, str};
use crate::data::Unit;
mod pdf;
mod latex;
mod html;


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
    latex: String,
    html: Tera
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

    let html_template = {
        let data = templates.get("template.html").unwrap();
        let template = match str::from_utf8(data) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence in HTML Template: {}", e)
        };
        let mut tera = Tera::default();
        let _ = tera.add_raw_template("datasheet", template);
        tera
    };

    


    ExportTemplates {
        latex: latex_template.to_string(),
        html: html_template
    }
}

pub fn export_unit(unit: &Unit, export_type: ExportType, export_path: PathBuf, export_templates: &ExportTemplates) {
    match export_type {
        ExportType::PDF => pdf::export_to_pdf(unit, &export_templates.html, export_path).unwrap(),
        ExportType::LATEX => latex::export_to_latex(unit, &export_templates.latex, export_path).unwrap(),
        ExportType::HTML => html::export_to_html(unit, &export_templates.html, export_path).unwrap(),
    };
}