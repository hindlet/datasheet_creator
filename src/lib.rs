use ron::de::from_reader;
use tera::Tera;
use std::{fs::{remove_file, File}, io::Write, path::PathBuf, str::FromStr};
mod unit;
use unit::Unit;
mod weapon;
use html2pdf::{run, Margin, Options};

// removes the doc from the path
fn strip_path(path: &String) -> String{
    let mut sections: Vec<&str> = path.split("/").collect();
    sections.pop();
    return sections.join("/");
}

/// Renders a single card and writes its pdf to the same folder
pub fn render_card(path: String, delete_html: bool, output_directory: Option<String>) {
    // let path  = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), path);
    let f = File::open(path.clone()).unwrap();
    let unit: Unit = from_reader(f).unwrap();

    let output_path: String;
    if let Some(out_path) = output_directory {
        output_path = out_path;
    } else {
        output_path = strip_path(&path);
    }

    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let context = unit.get_context();

    let result = tera.render("datasheet.html", &context).unwrap();
    let html_path = unit.get_html_path(&output_path);
    let mut html_file = File::create(html_path.clone()).unwrap();
    let _ = html_file.write(result.as_bytes());

    let pdf_path = unit.get_pdf_path(&output_path);
    let _ = run(&Options {
        input: PathBuf::from_str(&html_path).unwrap(),
        output: Some(PathBuf::from_str(&pdf_path).unwrap()),
        landscape: true,
        background: true,
        wait: None,
        header: None,
        footer: None,
        paper: Some(html2pdf::PaperSize::A5),
        scale: None,
        range: None,
        margin: Some(Margin::All(0.0)),
        disable_sandbox: false
    });

    if delete_html {
        let _ = remove_file(html_path);
    }
}

// pub fn render_folder(path: String) {
//     let f = File::open(path).unwrap();
//     // let unit: Unit = from_reader(f).unwrap();
//     // unit.render();
// }