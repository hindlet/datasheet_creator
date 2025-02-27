use ron::de::from_reader;
use std::fs::File;
mod unit;
use unit::Unit;
mod weapon;



fn main() {
    let path  = format!("{}/assets/test.ron", env!("CARGO_MANIFEST_DIR"));
    let f = File::open(path).unwrap();
    let test_unit: Unit = from_reader(f).unwrap();
    // println!("{:?}", test_unit);
    test_unit.render();
}
