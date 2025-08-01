use serde::{Deserialize, Serialize};

use crate::data::VariableValue;


#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Ability {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
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
    DeadlyDemise(VariableValue),
    FightsFirst
}

impl CoreAbility {
    pub fn to_string(&self) -> String {
        match self {
            CoreAbility::DeepStrike => "Deep Strike".to_string(),
            CoreAbility::Scouts(x) => format!("Scouts {}\"", x),
            CoreAbility::Leader => "Leader".to_string(),
            CoreAbility::Infiltrators => "Infiltrators".to_string(),
            CoreAbility::LoneOp => "Lone Operative".to_string(),
            CoreAbility::FiringDeck(x) => format!("Firing Deck {}", x),
            CoreAbility::Stealth => "Stealth".to_string(),
            CoreAbility::FeelnoPain(x) => format!("Feel no Pain {}+", x),
            CoreAbility::DeadlyDemise(x) => format!("Deadly Demise {}", x.to_string()),
            CoreAbility::FightsFirst => "Fights First".to_string(),
            _ => "".to_string()
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub enum WeaponAbility {
    #[default]
    None,
    Assault,
    RapidFire(u32),
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
    Sustained(VariableValue),
    ExtraAttacks,
    AntiX(String, u32),
    OneShot,
}

impl WeaponAbility {
    pub fn to_string(&self) -> String {
        match self {
            WeaponAbility::Assault => "ASSAULT".to_string(),
            WeaponAbility::RapidFire(x) => format!("RAPID FIRE {}", x),
            WeaponAbility::IgnoresCover => "IGNORES COVER".to_string(),
            WeaponAbility::TwinLinked => "TWIN-LINKED".to_string(),
            WeaponAbility::Pistol => "PISTOL".to_string(),
            WeaponAbility::Torrent => "TORRENT".to_string(),
            WeaponAbility::Lethal => "LETHAL".to_string(),
            WeaponAbility::Lance => "LANCE".to_string(),
            WeaponAbility::Indirect => "INDIRECT FIRE".to_string(),
            WeaponAbility::Precision => "PRECISION".to_string(),
            WeaponAbility::Blast => "BLAST".to_string(),
            WeaponAbility::Melta(x) => format!("MELTA {}", x),
            WeaponAbility::Heavy => "HEAVY".to_string(),
            WeaponAbility::Hazardous => "HAZARDOUS".to_string(),
            WeaponAbility::Dev => "DEVASTATING WOUNDS".to_string(),
            WeaponAbility::Sustained(x) => format!("SUSTAINED HITS {}", x.to_string()),
            WeaponAbility::ExtraAttacks => "EXTRA ATTACKS".to_string(),
            WeaponAbility::AntiX(keyword, x) => format!("ANTI-{} {}+", keyword, x.clamp(&2, &6)),
            WeaponAbility::OneShot => "ONE SHOT".to_string(),
            _ => "".to_string()
        }
    }
}