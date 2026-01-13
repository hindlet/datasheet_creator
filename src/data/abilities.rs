use egui::{ComboBox, Ui};
use serde::{Deserialize, Serialize};

use crate::data::VariableValue;


#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct Ability {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub enum CoreAbility {
    #[default]
    None,
    DeepStrike,
    Scouts(u32),
    Leader,
    Infiltrators,
    LoneOp,
    FiringDeck(u32),
    Stealth,
    FeelnoPain(u32),
    DeadlyDemise(VariableValue, String),
    FightsFirst
}

impl CoreAbility {

    pub fn to_render_string(&self) -> String {
        match self {
            CoreAbility::Scouts(x) => format!("Scouts {}\"", x),
            CoreAbility::FiringDeck(x) => format!("Firing Deck {}", x),
            CoreAbility::FeelnoPain(x) => format!("Feel no Pain {}+", x),
            CoreAbility::DeadlyDemise(x, _) => format!("Deadly Demise {}", x.to_string()),
            CoreAbility::None => "".to_string(),
            _ => self.to_string().to_string()
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            CoreAbility::DeepStrike => "Deep Strike",
            CoreAbility::Scouts(_) => "Scouts",
            CoreAbility::Leader => "Leader",
            CoreAbility::Infiltrators => "Infiltrators",
            CoreAbility::LoneOp => "Lone Operative",
            CoreAbility::FiringDeck(_) => "Firing Deck",
            CoreAbility::Stealth => "Stealth",
            CoreAbility::FeelnoPain(_) => "Feel no Pain",
            CoreAbility::DeadlyDemise(_, _) => "Deadly Demise",
            CoreAbility::FightsFirst => "Fights First",
            CoreAbility::None => "NONE"
        }
    }

    pub fn combo_box(&mut self, ui: &mut Ui, id: usize) {
        ComboBox::from_id_salt(id)
            .selected_text(format!("{}", self.to_string()))
            .show_ui(ui, |ui| {
                ui.selectable_value(self, CoreAbility::None, "NONE");
                ui.selectable_value(self, CoreAbility::Scouts(1), "Scouts");
                ui.selectable_value(self, CoreAbility::DeepStrike, "Deep Strike");
                ui.selectable_value(self, CoreAbility::Leader, "Leader");
                ui.selectable_value(self, CoreAbility::Infiltrators, "Infiltrators");
                ui.selectable_value(self, CoreAbility::LoneOp, "Lone Operative");
                ui.selectable_value(self, CoreAbility::FiringDeck(1), "Firing Deck");
                ui.selectable_value(self, CoreAbility::Stealth, "Stealth");
                ui.selectable_value(self, CoreAbility::FeelnoPain(6), "Feel no Pain");
                ui.selectable_value(self, CoreAbility::DeadlyDemise(VariableValue::Set(1), "1".to_string()), "Deadly Demise");
                ui.selectable_value(self, CoreAbility::FightsFirst, "Fights First");
            });
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub enum WeaponAbility {
    #[default]
    None,
    Assault,
    RapidFire(VariableValue, String),
    IgnoresCover,
    TwinLinked,
    Pistol,
    Torrent,
    Lethal,
    Lance,
    Indirect,
    Precision,
    Blast,
    Melta(u32),
    Heavy,
    Hazardous,
    Dev,
    Sustained(VariableValue, String),
    ExtraAttacks,
    AntiX(String, u32),
    OneShot,
    Precise, // Crusade Only
    Pyschic,
    Conversion,
    Custom(String)
}

impl WeaponAbility {
    pub fn to_render_string(&self) -> String {
        match self {
            WeaponAbility::RapidFire(x, _) => format!("RAPID FIRE {}", x.to_string()),
            WeaponAbility::Melta(x) => format!("MELTA {}", x),
            WeaponAbility::Sustained(x, _) => format!("SUSTAINED HITS {}", x.to_string()),
            WeaponAbility::AntiX(keyword, x) => format!("ANTI-{} {}+", keyword.to_uppercase(), x.clamp(&2, &6)),
            WeaponAbility::None => "".to_string(),
            WeaponAbility::Custom(x) => x.clone(),
            _ => self.to_string().to_string()
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            WeaponAbility::Assault => "ASSAULT",
            WeaponAbility::RapidFire(_, _) => "RAPID FIRE",
            WeaponAbility::IgnoresCover => "IGNORES COVER",
            WeaponAbility::TwinLinked => "TWIN-LINKED",
            WeaponAbility::Pistol => "PISTOL",
            WeaponAbility::Torrent => "TORRENT",
            WeaponAbility::Lethal => "LETHAL HITS",
            WeaponAbility::Lance => "LANCE",
            WeaponAbility::Indirect => "INDIRECT FIRE",
            WeaponAbility::Precision => "PRECISION",
            WeaponAbility::Blast => "BLAST",
            WeaponAbility::Melta(_) => "MELTA",
            WeaponAbility::Heavy => "HEAVY",
            WeaponAbility::Hazardous => "HAZARDOUS",
            WeaponAbility::Dev => "DEVASTATING WOUNDS",
            WeaponAbility::Sustained(_, _) => "SUSTAINED HITS",
            WeaponAbility::ExtraAttacks => "EXTRA ATTACKS",
            WeaponAbility::AntiX(_, _) => "ANTI-X",
            WeaponAbility::OneShot => "ONE SHOT",
            WeaponAbility::None => "NONE",
            WeaponAbility::Precise => "PRECISE",
            WeaponAbility::Pyschic => "PSYCHIC",
            WeaponAbility::Conversion => "CONVERSION",
            WeaponAbility::Custom(_) => "CUSTOM",
        }
    }

    pub fn combo_box_ranged(&mut self, ui: &mut Ui, id: usize) {
        ComboBox::from_id_salt(id)
            .selected_text(format!("{}", self.to_string()))
            .show_ui(ui, |ui| {
                ui.selectable_value(self, WeaponAbility::None, "NONE");
                ui.selectable_value(self, WeaponAbility::Assault, "ASSAULT");
                ui.selectable_value(self, WeaponAbility::RapidFire(VariableValue::Set(1), "1".to_string()), "RAPID FIRE");
                ui.selectable_value(self, WeaponAbility::IgnoresCover, "IGNORES COVER");
                ui.selectable_value(self, WeaponAbility::TwinLinked, "TWIN-LINKED");
                ui.selectable_value(self, WeaponAbility::Pistol, "PISTOL");
                ui.selectable_value(self, WeaponAbility::Torrent, "TORRENT");
                ui.selectable_value(self, WeaponAbility::Lethal, "LETHAL HITS");
                ui.selectable_value(self, WeaponAbility::Indirect, "INDIRECT");
                ui.selectable_value(self, WeaponAbility::Precision, "PRECISION");
                ui.selectable_value(self, WeaponAbility::Blast, "BLAST");
                ui.selectable_value(self, WeaponAbility::Melta(1), "MELTA");
                ui.selectable_value(self, WeaponAbility::Heavy, "HEAVY");
                ui.selectable_value(self, WeaponAbility::Hazardous, "HAZARDOUS");
                ui.selectable_value(self, WeaponAbility::Dev, "DEVASTATING WOUNDS");
                ui.selectable_value(self, WeaponAbility::Sustained(VariableValue::Set(1), "1".to_string()), "SUSTAINED HITS");
                ui.selectable_value(self, WeaponAbility::AntiX("".to_string(), 2), "ANTI-X");
                ui.selectable_value(self, WeaponAbility::OneShot, "ONE SHOT");
                ui.selectable_value(self, WeaponAbility::Pyschic, "PSYCHIC");
                ui.selectable_value(self, WeaponAbility::Conversion, "CONVERSION");
                ui.selectable_value(self, WeaponAbility::Custom("name".to_string()), "CUSTOM");
            });
            
    }

    pub fn combo_box_melee(&mut self, ui: &mut Ui, id: usize) {
        ComboBox::from_id_salt(id)
            .selected_text(format!("{}", self.to_string()))
            .show_ui(ui, |ui| {
                ui.selectable_value(self, WeaponAbility::None, "NONE");
                ui.selectable_value(self, WeaponAbility::IgnoresCover, "IGNORES COVER");
                ui.selectable_value(self, WeaponAbility::TwinLinked, "TWIN-LINKED");
                ui.selectable_value(self, WeaponAbility::Lethal, "LETHAL HITS");
                ui.selectable_value(self, WeaponAbility::Lance, "LANCE");
                ui.selectable_value(self, WeaponAbility::Precision, "PRECISION");
                ui.selectable_value(self, WeaponAbility::Hazardous, "HAZARDOUS");
                ui.selectable_value(self, WeaponAbility::Dev, "DEVASTATING WOUNDS");
                ui.selectable_value(self, WeaponAbility::Sustained(VariableValue::Set(1), "1".to_string()), "SUSTAINED HITS");
                ui.selectable_value(self, WeaponAbility::ExtraAttacks, "EXTRA ATTACKS");
                ui.selectable_value(self, WeaponAbility::AntiX("".to_string(), 2), "ANTI-X");
                ui.selectable_value(self, WeaponAbility::OneShot, "ONE SHOT");
                ui.selectable_value(self, WeaponAbility::Pyschic, "PSYCHIC");
                ui.selectable_value(self, WeaponAbility::Custom("name".to_string()), "CUSTOM");
            });
            
    }
}