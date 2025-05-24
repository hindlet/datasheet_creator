use std::{collections::BTreeMap, fs::write, path::PathBuf};

use printpdf::{GeneratePdfOptions, Mm, PdfDocument, PdfSaveOptions, XmlRenderOptions};
use tera::Tera;

use crate::data::Unit;




pub fn export_to_pdf(unit: &Unit, template: &Tera, path: PathBuf) -> Result<(), std::io::Error>  {
    let document = PdfDocument::new(&unit.name);


    Ok(())
}