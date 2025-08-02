use crate::data::abilities::{CoreAbility, WeaponAbility};

use super::{Ability, Range, Unit, UnitStats, VariableValue, Weapon};

#[derive(Clone)]
pub struct WeaponEditData {
    pub name: String,
    pub range: u32,
    pub attacks: String,
    pub skill: u32,
    pub strength: u32,
    pub ap: u32,
    pub damage: String,
    pub keywords: Vec<WeaponAbility>
}

impl Default for WeaponEditData {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            range: 1,
            attacks: "1".to_string(),
            skill: 1,
            strength: 1,
            ap: 0,
            damage: "1".to_string(),
            keywords: Vec::new()
        }
    }
}

impl From<&Weapon> for WeaponEditData {
    fn from(value: &Weapon) -> Self {
        Self {
            name: value.name.clone(),
            range: match value.range {
                Range::Ranged(range) => range,
                _ => 0
            },
            attacks: value.attacks.to_string(),
            skill: value.skill,
            strength: value.strength,
            ap: value.ap.abs() as u32,
            damage: value.damage.to_string(),
            keywords: value.keywords.clone()
        }
    }
}

impl Into<Weapon> for WeaponEditData {
    fn into(self) -> Weapon {
        Weapon {
            name: self.name,
            range: if self.range == 0 {
                Range::Melee
            } else {
                Range::Ranged(self.range)
            },
            attacks: VariableValue::from_string(&self.attacks).unwrap_or(VariableValue::Set(0)),
            skill: self.skill,
            strength: self.strength,
            ap: self.ap as i32,
            damage: VariableValue::from_string(&self.damage).unwrap_or(VariableValue::Set(0)),
            keywords: self.keywords
        }
    }
}

#[derive(Clone)]
pub struct UnitEditData {
    pub name: String,
    pub filename: String,
    pub prev_filename: String, // hidden from user
    pub crusader: bool,

    pub movement: u32,
    pub toughness: u32,
    pub save: u32,
    pub has_invuln: bool,
    pub invuln: u32,
    pub wounds: u32,
    pub leadership: u32,
    pub objective_control: u32,

    pub ranged_weapons: Vec<WeaponEditData>,
    pub melee_weapons: Vec<WeaponEditData>,

    pub faction_ability: (bool, String),

    pub core_abilities: Vec<CoreAbility>,
    pub unique_abilities: Vec<Ability>,
    pub faction_keyword: String,
    pub keywords: Vec<String>,

    pub damaged: (bool, u32),

    pub leader: (bool, Vec<String>),

    // pub wargear_options: (bool, Vec<WargearOption>),
}

impl From<(&Unit, String)> for UnitEditData {
    fn from((value, filename): (&Unit, String)) -> Self {

        let mut ranged_weapons = Vec::new();
        for weapon in value.ranged_weapons.iter() {
            ranged_weapons.push(WeaponEditData::from(weapon));
        }

        let mut melee_weapons = Vec::new();
        for weapon in value.melee_weapons.iter() {
            melee_weapons.push(WeaponEditData::from(weapon));
        }
        

        Self {
            name: value.name.clone(),
            filename: filename.clone(),
            prev_filename: filename,
            crusader: false,
            
            movement: value.stats.movement,
            toughness: value.stats.toughness,
            save: value.stats.save,
            has_invuln: value.stats.invuln.is_some(),
            invuln: value.stats.invuln.unwrap_or(4),
            wounds: value.stats.wounds,
            leadership: value.stats.leadership,
            objective_control: value.stats.oc,
            
            ranged_weapons,
            melee_weapons,

            faction_ability: (value.faction_ability.is_some(), value.faction_ability.clone().unwrap_or("".to_string())),

            core_abilities: value.core_abilities.clone(),

            unique_abilities: value.unique_abilities.clone(),

            faction_keyword: value.faction_keyword.clone(),

            keywords: value.keywords.clone(),

            damaged: (value.damaged.is_some(), value.damaged.unwrap_or(4)),

            leader: (value.leader.is_some(), value.leader.clone().unwrap_or(Vec::new())),

            // wargear_options: (false, Vec::new()),
        }
    }
}

impl Into<Unit> for UnitEditData {
    fn into(self) -> Unit {
        let mut ranged_weapons = Vec::new();
        for weapon in self.ranged_weapons {
            ranged_weapons.push(weapon.into());
        }

        let mut melee_weapons = Vec::new();
        for weapon in self.melee_weapons {
            melee_weapons.push(weapon.into());
        }

        
        let mut sanitised_keywords = Vec::new();
        for keyword in self.keywords {
            if !keyword.is_empty() {
                sanitised_keywords.push(keyword.to_uppercase());
            }
        }


        Unit {
            name: self.name,
            stats: UnitStats {
                movement: self.movement,
                toughness: self.toughness,
                save: self.save,
                invuln: if self.has_invuln {
                    Some(self.invuln)
                } else {
                    None
                },
                wounds: self.wounds,
                leadership: self.leadership,
                oc: self.objective_control
            },
            ranged_weapons,
            melee_weapons,
            faction_ability: if self.faction_ability.0 {
                Some(self.faction_ability.1)
            } else {
                None
            },
            core_abilities: self.core_abilities,
            unique_abilities: self.unique_abilities,
            faction_keyword: self.faction_keyword,
            keywords: sanitised_keywords,
            damaged: if self.damaged.0 {
                Some(self.damaged.1)
            } else {
                None
            },
            leader: if self.leader.0 {
                Some(self.leader.1)
            } else {
                None
            },

            ..Default::default()
        }
    }
}