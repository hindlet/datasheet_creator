use crate::data::{abilities::{CoreAbility, WeaponAbility}, crusade_data::CrusadeUnitData, CrusadeRank, CrusadeUpgrade, WeaponMod, WeaponModChange};

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
    pub keywords: Vec<WeaponAbility>,
    pub charge_levels_info: (bool, Option<usize>, String) // has levels, is parent, level name
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
            keywords: Vec::new(),
            charge_levels_info: (false, None, "".to_string())
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
            keywords: value.keywords.clone(),
            charge_levels_info: value.charge_levels_info.clone()
        }
    }
}

impl Into<Weapon> for WeaponEditData {
    fn into(self) -> Weapon {
        let mut keywords = Vec::new();
        for keyword in self.keywords {
            match keyword {
                WeaponAbility::Sustained(_, text) => {
                    let val = VariableValue::from_string(&text).unwrap_or(VariableValue::Set(1));
                    keywords.push(WeaponAbility::Sustained(val, val.to_string()));
                },
                _ => keywords.push(keyword),
            }
        }

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
            keywords: keywords,
            charge_levels_info: self.charge_levels_info
        }
    }
}

#[derive(Clone)]
pub struct UnitEditData {
    pub name: String,
    pub filename: String,
    pub prev_filename: String, // hidden from user
    

    pub movement: u32,
    pub toughness: u32,
    pub save: u32,
    pub has_invuln: bool,
    pub invuln: u32,
    pub wounds: u32,
    pub leadership: u32,
    pub objective_control: u32,

    pub ranged_weapons: Vec<(WeaponEditData, u32)>,
    pub melee_weapons: Vec<(WeaponEditData, u32)>,

    pub faction_ability: (bool, String),

    pub core_abilities: Vec<CoreAbility>,
    pub unique_abilities: Vec<Ability>,
    pub faction_keyword: String,
    pub keywords: Vec<String>,

    pub damaged: (bool, u32),

    pub leader: (bool, Vec<String>),

    pub crusader: bool,
    pub crusade_data: CrusadeUnitData
}

impl From<(&Unit, String)> for UnitEditData {
    fn from((value, filename): (&Unit, String)) -> Self {

        let mut ranged_weapons = Vec::new();
        for (weapon, count) in value.ranged_weapons.iter() {
            ranged_weapons.push((WeaponEditData::from(weapon), *count));
        }

        let mut melee_weapons = Vec::new();
        for (weapon, count) in value.melee_weapons.iter() {
            melee_weapons.push((WeaponEditData::from(weapon), *count));
        }
        

        Self {
            name: value.name.clone(),
            filename: filename.clone(),
            prev_filename: filename,
            
            
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

            crusader: value.crusade_unit,
            crusade_data: value.crusade_data.clone(),
        }
    }
}

impl Into<Unit> for UnitEditData {
    fn into(self) -> Unit {
        let mut ranged_weapons: Vec<(Weapon, u32)> = Vec::new();
        for (weapon, count) in self.ranged_weapons {
            ranged_weapons.push((weapon.into(), count));
        }

        let mut melee_weapons: Vec<(Weapon, u32)> = Vec::new();
        for (weapon, count) in self.melee_weapons {
            melee_weapons.push((weapon.into(), count));
        }

        
        let mut sanitised_keywords = Vec::new();
        for keyword in self.keywords {
            if !keyword.is_empty() {
                sanitised_keywords.push(keyword.to_uppercase());
            }
        }

        let mut core_abilities = Vec::new();
        for ability in self.core_abilities {
            match ability {
                CoreAbility::DeadlyDemise(_, text) => {
                    let val = VariableValue::from_string(&text).unwrap_or(VariableValue::Set(1));
                    core_abilities.push(CoreAbility::DeadlyDemise(val, val.to_string()));
                }
                _ => {core_abilities.push(ability);}
            }
        }

        let mut crusade_ranged = Vec::new();
        let mut crusade_melee = Vec::new();
        let mut crusade_data = self.crusade_data.clone();
        if self.crusader {
            let mut upgrades: Vec<(usize, WeaponMod)> = Vec::new();
            let mut ranged_upgrades = false;
            let mut melee_upgrades = true;

            for upgrade in self.crusade_data.upgrades.iter() {
                match upgrade {
                    CrusadeUpgrade::WeaponMod(weapon_mod) => {
                        if let Some(target) = &weapon_mod.target {
                            if target.0 {ranged_upgrades = true} else {melee_upgrades = true}
                            for upgrade in upgrades.iter_mut() {
                                if &upgrade.1 == weapon_mod {
                                    upgrade.0 += 1;
                                    continue;
                                }
                            }
                            upgrades.push((1, weapon_mod.clone()));
                        }
                    }
                    _ => {}
                }
            }

            if ranged_upgrades {
                for (index, (weapon, count)) in ranged_weapons.iter().enumerate() {
                        let mut count = *count;
                        for (upgrade_count, upgrade) in upgrades.iter() {
                            let target = upgrade.target.as_ref().unwrap();

                            if target.0 && target.1 == index && count > 0 {
                                let i = if count > *upgrade_count as u32 {*upgrade_count as u32} else {count};
                                crusade_ranged.push((Weapon {
                                    name: upgrade.name.clone(),
                                    range: weapon.range,
                                    attacks: if upgrade.attacks() {weapon.attacks.add_one()} else {weapon.attacks},
                                    skill: if upgrade.skill() {weapon.skill - 1} else {weapon.skill},
                                    strength: if upgrade.strength() {weapon.strength + 1} else {weapon.skill},
                                    ap: if upgrade.ap() {if weapon.ap <= 0 {weapon.ap - 1} else {weapon.ap + 1}} else {weapon.ap},
                                    damage: if upgrade.damage() {weapon.damage.add_one()} else {weapon.damage},
                                    keywords: if upgrade.precise() {
                                        let mut keywords = weapon.keywords.clone();
                                        keywords.push(WeaponAbility::Precise);
                                        keywords
                                    } else {weapon.keywords.clone()},
                                    charge_levels_info: weapon.charge_levels_info.clone()
                                }, i));
                                count -= i;
                            }  
                        }
                        if count > 0 {crusade_ranged.push((weapon.clone(), count));}
                    }
            } else {
                crusade_ranged = ranged_weapons.clone()
            }

            if melee_upgrades {
                for (index, (weapon, count)) in melee_weapons.iter().enumerate() {
                        let mut count = *count;
                        for (upgrade_count, upgrade) in upgrades.iter() {
                            let target = upgrade.target.as_ref().unwrap();

                            if !target.0 && target.1 == index && count > 0 {
                                let i = if count > *upgrade_count as u32 {*upgrade_count as u32} else {count};
                                crusade_ranged.push((Weapon {
                                    name: upgrade.name.clone(),
                                    range: weapon.range,
                                    attacks: if upgrade.attacks() {weapon.attacks.add_one()} else {weapon.attacks},
                                    skill: if upgrade.skill() {weapon.skill - 1} else {weapon.skill},
                                    strength: if upgrade.strength() {weapon.strength + 1} else {weapon.skill},
                                    ap: if upgrade.ap() {if weapon.ap <= 0 {weapon.ap - 1} else {weapon.ap + 1}} else {weapon.ap},
                                    damage: if upgrade.damage() {weapon.damage.add_one()} else {weapon.damage},
                                    keywords: if upgrade.precise() {
                                        let mut keywords = weapon.keywords.clone();
                                        keywords.push(WeaponAbility::Precise);
                                        keywords
                                    } else {weapon.keywords.clone()},
                                    charge_levels_info: weapon.charge_levels_info.clone()
                                }, i));
                                count -= i;
                            }  
                        }
                        if count > 0 {crusade_ranged.push((weapon.clone(), count));}
                    }
            } else {
                crusade_melee = melee_weapons.clone()
            }

            crusade_data.rank = match crusade_data.exp {
                0..6 => {CrusadeRank::BattleReady},
                6..16 => {CrusadeRank::Blooded},
                16..31 => {CrusadeRank::BattleHardended},
                31..51 => {CrusadeRank::Heroic},
                51.. => {CrusadeRank::Legendary},
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
            core_abilities: core_abilities,
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

            crusade_unit: self.crusader,
            crusade_data: crusade_data,
            crusade_weapons: (crusade_ranged, crusade_melee),

            ..Default::default()
        }
    }
}