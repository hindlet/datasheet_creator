use egui::Color32;
use serde::{Deserialize, Serialize};



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct  DatasheetAppSettings {
    pub bar_colour: Color32,
    pub keyword_colour: Color32,
    pub dark_mode: bool,
}


impl DatasheetAppSettings {
    pub fn save(&self, storage: &mut dyn eframe::Storage) {
        storage.set_string("Dark_Mode", self.dark_mode.to_string());
        storage.set_string("Bar_Colour", color32_to_string(self.bar_colour));
        storage.set_string("Keyword_Colour", color32_to_string(self.keyword_colour));
        storage.flush();
    }
}

impl Default for DatasheetAppSettings{
    fn default() -> Self {
        Self {
            bar_colour: Color32::LIGHT_BLUE,
            keyword_colour: Color32::LIGHT_BLUE,
            dark_mode: true,
        }
    }
}

pub fn color32_to_string(colour: Color32) -> String {
    format!("[{},{},{},{}]", colour.r(), colour.g(), colour.b(), colour.a())
}

pub fn string_to_color32(string: String) -> Result<Color32, ()> {
    let temp = string.replace("[", "").replace("]", "");
    let nums: Vec<&str> = temp.split(",").collect();
    if nums.len() != 4 {return Err(());}
    if let Ok(r) = nums[0].parse() {
        if let Ok(g) = nums[1].parse() {
            if let Ok(b) = nums[2].parse() {
                if let Ok(a) = nums[3].parse() {
                    return Ok(Color32::from_rgba_unmultiplied(r, g, b, a));
                }
            }
        }
    }
    return Err(());
}