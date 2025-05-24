use std::{fs::write, path::PathBuf};

use tera::Tera;

use crate::data::Unit;




pub fn export_to_html(unit: &Unit, template: &Tera, path: PathBuf) -> Result<(), std::io::Error>  {
    let context = unit.get_context();
    
    let rendered = template.render("datasheet", &context).unwrap();

    write(path, rendered)
}