use ron::de::from_reader;
use std::fs::File;
mod unit;
use unit::Unit;
mod weapon;


pub fn render_card(path: String) {
    let f = File::open(path).unwrap();
    let unit: Unit = from_reader(f).unwrap();
    unit.render();
}