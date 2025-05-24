use std::{collections::BTreeMap, fs::write, path::PathBuf};

use printpdf::{GeneratePdfOptions, Mm, PdfDocument, PdfSaveOptions, XmlRenderOptions};
use tera::Tera;

use crate::data::Unit;




pub fn export_to_pdf(unit: &Unit, template: &Tera, path: PathBuf) -> Result<(), std::io::Error>  {
    let context = unit.get_context();
    
    let rendered = template.render("datasheet", &context).unwrap();

    let mut warnings = Vec::new();

    let bytes = PdfDocument::from_html(&rendered, &BTreeMap::new(), &BTreeMap::new(), &GeneratePdfOptions::default(), &mut warnings)
        .unwrap()
        .save(&PdfSaveOptions::default(), &mut warnings);
    
    write(path, bytes)
}