use crate::weapon::{Range, VariableValue, Weapon};
use serde::Deserialize;
use std::fmt::format;
use std::fs::OpenOptions;
use std::{fs::File, io::Write};
use std::io::BufWriter;
use printpdf::*;
use image_crate::codecs::png::PngDecoder;


const STATS_BAR_HEIGHT: Mm = Mm(88.0);
const STATS_LABEL_HEIGHT: Mm = Mm(STATS_BAR_HEIGHT.0 + 8.0);
const STATS_TEXT_HEIGHT: Mm = Mm(STATS_BAR_HEIGHT.0 + 2.25);
const STATS_FONT_SIZE: f32 = 14.0;
const STATS_LABEL_FONT_SIZE: f32 = STATS_FONT_SIZE - 4.0;

const WEAPON_START_HEIGHT: Mm = Mm(STATS_BAR_HEIGHT.0 - 11.25);
const WEAPON_FONT_SIZE: f32 = 7.5;


#[derive(Debug, Deserialize)]
pub struct UnitStats {
    pub movement: u32,
    pub toughness: u32,
    pub save: u32,
    pub invuln: Option<u32>,
    pub wounds: u32,
    pub leadership: u32,
    pub oc: u32,
}

#[derive(Debug, Deserialize)]
pub enum AbilityType {
    Core,
    Faction,
    Misc
}

#[derive(Debug, Deserialize)]
pub struct Ability {
    name: String,
    ability_type: AbilityType,
    description: String,
}

#[derive(Debug, Deserialize)]
pub struct Unit {
    pub name: String,
    pub stats: UnitStats,
    pub ranged_weapons: Vec<Weapon>,
    pub melee_weapons: Vec<Weapon>,
    pub abilities: Vec<Ability>,
    pub keywords: Vec<String>
}


impl Unit {

    

    pub fn render(
        &self
    ) {
        let path = format!("{}.pdf", self.name);
        let (doc, page1, layer1) = PdfDocument::new("PDF_Document_title", Mm(161.5), Mm(107.1), "Layer 1");
        let current_layer = doc.get_page(page1).get_layer(layer1);
        let basic_font = doc.add_external_font(File::open("assets/Conduit-ITC-Std-Font.otf").unwrap()).unwrap();
        let title_font = doc.add_external_font(File::open("assets/Roboto_Condensed-Bold.ttf").unwrap()).unwrap();

        let weapons_offset = self.draw_name_and_stats(current_layer.clone(), &title_font, &basic_font);
        
        let mut current_height = self.draw_ranged_weapons(current_layer.clone(), &title_font, &basic_font, weapons_offset);
        current_height = self.draw_melee_weapons(current_layer.clone(), &title_font, &basic_font, current_height);


        // Melee Weapons


        doc.save(&mut BufWriter::new(File::create(path).unwrap())).unwrap();

    }


    fn draw_name_and_stats(&self, current_layer: PdfLayerReference, title_font: &IndirectFontRef, text_font: &IndirectFontRef) -> Mm {
        // Name and Stats
        let mut stat_bar_file = File::open("assets/StatsBar.png").unwrap();
        let stats_bar = Image::try_from(PngDecoder::new(&mut stat_bar_file).unwrap()).unwrap(); // NOTE FOR FUTURE: PNGS WITH ALPHA CHANNEL DO NOT WORK
        stats_bar.add_to_layer(current_layer.clone(), ImageTransform{
            scale_x: Some(1.75),
            scale_y: Some(1.75),
            translate_x: Some(Mm(1.0)),
            translate_y: Some(STATS_BAR_HEIGHT),
            // dpi: Some(10.0),
            ..Default::default()
        });
        let weapons_offset: Mm;
        if let Some(invuln) = self.stats.invuln {
            let mut stats_square_file = File::open("assets/StatSquare.png").unwrap();
            let stats_square = Image::try_from(PngDecoder::new(&mut stats_square_file).unwrap()).unwrap(); // NOTE FOR FUTURE: PNGS WITH ALPHA CHANNEL DO NOT WORK
            stats_square.add_to_layer(current_layer.clone(), ImageTransform{
                scale_x: Some(1.75),
                scale_y: Some(1.75),
                translate_x: Some(Mm(18.75)),
                translate_y: Some(Mm(STATS_BAR_HEIGHT.0 - 8.0)),
                // dpi: Some(10.0),
                ..Default::default()
            });
            current_layer.use_text(format!("{}+", invuln), STATS_FONT_SIZE, Mm(20.25), Mm(STATS_TEXT_HEIGHT.0 - 7.8), text_font);
            current_layer.use_text("Invulnerable Save", STATS_FONT_SIZE - 5.0, Mm(27.0), Mm(STATS_TEXT_HEIGHT.0 - 7.5), text_font);
            weapons_offset = Mm(0.0);
        } else {weapons_offset = Mm(7.0);}
        current_layer.use_text(self.name.clone(), 20.0, Mm(1.0), Mm(100.1), title_font);
        current_layer.use_text("M", STATS_LABEL_FONT_SIZE, Mm(3.75), STATS_LABEL_HEIGHT, title_font);
        current_layer.use_text("T", STATS_LABEL_FONT_SIZE, Mm(13.25), STATS_LABEL_HEIGHT, title_font);
        current_layer.use_text("Sv", STATS_LABEL_FONT_SIZE, Mm(21.25), STATS_LABEL_HEIGHT, title_font);
        current_layer.use_text("W", STATS_LABEL_FONT_SIZE, Mm(30.25), STATS_LABEL_HEIGHT, title_font);
        current_layer.use_text("Ld", STATS_LABEL_FONT_SIZE, Mm(39.25), STATS_LABEL_HEIGHT, title_font);
        current_layer.use_text("OC", STATS_LABEL_FONT_SIZE, Mm(47.75), STATS_LABEL_HEIGHT, title_font);

        let movement_offset = (self.stats.movement >= 10) as i32 as f32 * -1.8;
        current_layer.use_text(format!("{}\"", self.stats.movement), STATS_FONT_SIZE, Mm(3.2 + movement_offset), STATS_TEXT_HEIGHT, text_font);
        let toughess_offset = (self.stats.toughness >= 10) as i32 as f32 * -1.8;
        current_layer.use_text(format!("{}", self.stats.toughness), STATS_FONT_SIZE, Mm(12.5 + toughess_offset), STATS_TEXT_HEIGHT, text_font);
        current_layer.use_text(format!("{}+", self.stats.save), STATS_FONT_SIZE, Mm(20.25), STATS_TEXT_HEIGHT, text_font);
        let wounds_offset = (self.stats.wounds >= 10) as i32 as f32 * -1.8;
        current_layer.use_text(format!("{}", self.stats.wounds), STATS_FONT_SIZE, Mm(30.1 + wounds_offset), STATS_TEXT_HEIGHT, text_font);
        current_layer.use_text(format!("{}+", self.stats.leadership), STATS_FONT_SIZE, Mm(37.9), STATS_TEXT_HEIGHT, text_font);
        current_layer.use_text(format!("{}", self.stats.oc), STATS_FONT_SIZE, Mm(47.7), STATS_TEXT_HEIGHT, text_font);
        return weapons_offset;
    }

    fn draw_ranged_weapons(&self, current_layer: PdfLayerReference, title_font: &IndirectFontRef, text_font: &IndirectFontRef, initial_offset: Mm) -> Mm{
        let mut current_height = WEAPON_START_HEIGHT + initial_offset;

        // little box
        let ranged_bar = Line::from_iter(vec![
            (Point::new(Mm(0.0), current_height + Mm(2.5)), false),
            (Point::new(Mm(100.0), current_height + Mm(2.5)), false),
            (Point::new(Mm(100.0), current_height - Mm(0.75)), false),
            (Point::new(Mm(0.0), current_height - Mm(0.75)), false)
        ]);
        current_layer.add_line(ranged_bar);

        current_layer.use_text("Ranged Weapons", WEAPON_FONT_SIZE, Mm(1.0), current_height, title_font);
        let mut pos = 26.0;
        current_layer.use_text("Range", WEAPON_FONT_SIZE, Mm(pos), current_height, title_font);
        pos += 12.0;
        current_layer.use_text("A", WEAPON_FONT_SIZE, Mm(pos), current_height, title_font);
        pos += 6.0;
        current_layer.use_text("BS", WEAPON_FONT_SIZE, Mm(pos), current_height, title_font);
        pos += 7.0;
        current_layer.use_text("S", WEAPON_FONT_SIZE, Mm(pos), current_height, title_font);
        pos += 6.0;
        current_layer.use_text("AP", WEAPON_FONT_SIZE, Mm(pos), current_height, title_font);
        pos += 7.0;
        current_layer.use_text("D", WEAPON_FONT_SIZE, Mm(pos), current_height, title_font);
        current_height -= Mm(3.0);
        
        for weapon in self.ranged_weapons.iter() {
            current_height = Self::draw_weapon(&self, weapon, current_layer.clone(), text_font, current_height);
        }
        return current_height - Mm(0.5);
    }

    fn draw_melee_weapons(&self, current_layer: PdfLayerReference, title_font: &IndirectFontRef, text_font: &IndirectFontRef, mut current_height: Mm) -> Mm {
        // little box
        let melee_bar = Line::from_iter(vec![
            (Point::new(Mm(0.0), current_height + Mm(2.7)), false),
            (Point::new(Mm(100.0), current_height + Mm(2.7)), false),
            (Point::new(Mm(100.0), current_height - Mm(0.75)), false),
            (Point::new(Mm(0.0), current_height - Mm(0.75)), false)
        ]);
        current_layer.add_line(melee_bar);

        current_layer.use_text("Melee Weapons", WEAPON_FONT_SIZE, Mm(1.0), current_height, title_font);
        let mut pos = 26.0;
        current_layer.use_text("Range", WEAPON_FONT_SIZE, Mm(pos), current_height, title_font);
        pos += 12.0;
        current_layer.use_text("A", WEAPON_FONT_SIZE, Mm(pos), current_height, title_font);
        pos += 6.0;
        current_layer.use_text("BS", WEAPON_FONT_SIZE, Mm(pos), current_height, title_font);
        pos += 7.0;
        current_layer.use_text("S", WEAPON_FONT_SIZE, Mm(pos), current_height, title_font);
        pos += 6.0;
        current_layer.use_text("AP", WEAPON_FONT_SIZE, Mm(pos), current_height, title_font);
        pos += 7.0;
        current_layer.use_text("D", WEAPON_FONT_SIZE, Mm(pos), current_height, title_font);
        current_height -= Mm(3.0);

        for weapon in self.melee_weapons.iter() {
            current_height = Self::draw_weapon(&self, weapon, current_layer.clone(), text_font, current_height);
        }
        return current_height;
    }

    fn draw_weapon(&self, weapon: &Weapon, current_layer: PdfLayerReference, font: &IndirectFontRef, mut current_height: Mm) -> Mm{

        current_layer.use_text(weapon.name.clone(), WEAPON_FONT_SIZE, Mm(1.0), current_height, font);
        let mut pos = 26.0;

        match weapon.range {
            Range::Melee => {
                current_layer.use_text("Melee", WEAPON_FONT_SIZE, Mm(pos), current_height, font);
            },
            Range::Ranged(range) => {
                let offset = (range >= 10) as i32 as f32 * 0.8 - 2.4;
                current_layer.use_text(format!("{}\"", range), WEAPON_FONT_SIZE, Mm(pos - offset), current_height, font);
            }
        }
        pos += 12.0;


        let (text, offset) = weapon.attacks.to_string();
        current_layer.use_text(text, WEAPON_FONT_SIZE, Mm(pos - offset), current_height, font);
        pos += 6.3;
        

        current_layer.use_text(format!("{}+", weapon.skill), WEAPON_FONT_SIZE, Mm(pos), current_height, font);
        pos += 7.0;

        let offset = (weapon.strength >= 10) as i32 as f32 * 0.7;
        current_layer.use_text(format!("{}", weapon.strength), WEAPON_FONT_SIZE, Mm(pos - offset), current_height, font);
        pos += 6.5;

        let (ap_text, offset) = if weapon.ap == 0 {
            (format!("{}", weapon.ap), 0.0)
        } else {
            (format!("-{}", weapon.ap), 0.5)
        };
        current_layer.use_text(ap_text, WEAPON_FONT_SIZE, Mm(pos - offset), current_height, font);
        pos += 6.3;

        let (text, offset) = weapon.damage.to_string();
        current_layer.use_text(text, WEAPON_FONT_SIZE, Mm(pos - offset), current_height, font);

        current_layer.add_line(Line::from_iter(vec![
            (Point::new(Mm(0.0), current_height - Mm(1.0)), false),
            (Point::new(Mm(75.0), current_height - Mm(1.0)), false)
        ]));

        current_height -= Mm(3.2);
        return current_height;
    }



}