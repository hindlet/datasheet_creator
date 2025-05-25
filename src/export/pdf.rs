use std::{fs::write, path::PathBuf};

use headless_chrome::{types::PrintToPdfOptions, Browser, LaunchOptions};
use tera::Tera;
use crate::data::Unit;
use anyhow::Result;



pub fn export_to_pdf(unit: &Unit, template: &Tera, path: PathBuf) -> Result<()>  {
    let context = unit.get_context();
    
    let rendered = template.render("datasheet", &context).unwrap();


    let browser = Browser::new(LaunchOptions::default()).unwrap();
    let tab = browser.new_tab()?;
    tab.navigate_to(format!("data:text/html;charset=utf-8,{}", rendered).as_str())?;
    let bytes = tab.print_to_pdf(Some(PrintToPdfOptions {
        print_background: Some(true),
        ..Default::default()
    })).unwrap();
    
    write(path, bytes)?;
    Ok(())
}