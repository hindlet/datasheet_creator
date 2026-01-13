use std::collections::BTreeMap;
use crate::data::{Ability, ChargeLevels, CrusadeRank, CrusadeUpgrade, Unit, VariableValue, Weapon, WeaponMod, abilities::{CoreAbility, WeaponAbility}, crusade_data::CrusadeUnitData, edit_data::{edit_stats::EditStats, weapon_edit_data::WeaponEditData}, index::WeaponReference, unit_stats::UnitStats};






#[derive(Clone)]
pub struct UnitEditData {
    pub name: String,
    pub filename: String,
    pub prev_filename: String, // hidden from user
    
    pub stats: EditStats,
    pub extra_stats: (String, Vec<(String, EditStats)>),

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

        let extra_statlines = {
            let mut extras = Vec::new();
            for (name, stats) in value.extra_statlines.1.iter() {
                extras.push((name.clone(), stats.into()));
            }
            (value.extra_statlines.0.clone(), extras)
        };
        

        Self {
            name: value.name.clone(),
            filename: filename.clone(),
            prev_filename: filename,
            
            stats: value.stats.into(),
            extra_stats: extra_statlines,
            
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

        let extra_statlines = {
            let mut extras: Vec<(String, UnitStats)> = Vec::new();
            for (name, stats) in self.extra_stats.1.iter() {
                extras.push((name.clone(), stats.into()));
            }
            (self.extra_stats.0.clone(), extras)
        };

        let mut crusade_ranged = Vec::new();
        let mut crusade_melee = Vec::new();
        let mut crusade_data = self.crusade_data.clone();
        if self.crusader {
            let mut upgrades: Vec<(usize, WeaponMod)> = Vec::new();
            // list of names of upgrade parents
            let mut upgrade_names: BTreeMap<WeaponReference, String> = BTreeMap::new();

            for upgrade in self.crusade_data.upgrades.iter() {
                match upgrade {
                    CrusadeUpgrade::WeaponMod(weapon_mod) => {
                        if weapon_mod.target.is_some() {
                            let mut found = false;
                            for upgrade in upgrades.iter_mut() {
                                if &upgrade.1 == weapon_mod {
                                    upgrade.0 += 1;
                                    found = true;
                                    break;
                                }
                            }
                            if !found {
                                upgrades.push((1, weapon_mod.clone()));
                            }
                        }
                    }
                    _ => {}
                }
            }

            let weapons = {
                let mut list = Vec::new();
                for (id, weapon) in ranged_weapons.iter().enumerate() {
                    list.push((true, id, weapon));
                }
                for (id, weapon) in melee_weapons.iter().enumerate() {
                    list.push((false, id, weapon));
                }
                list
            };

            for (ranged, index, (weapon, count)) in weapons.iter() {
                let mut count = *count;
                for (upgrade_count, upgrade) in upgrades.iter() {
                    let target = upgrade.target.as_ref().unwrap();
                    let upgrade_target =
                        target.is_id(*ranged, *index) // direct target
                        || (match &weapon.charge { // indirect target
                            ChargeLevels::Child(reference, _) => *target == *reference,
                            _ => false
                        });
                    
                    if upgrade_target {
                        let i = if count > *upgrade_count as u32 {*upgrade_count as u32} else {count};

                        let new_charge = match &weapon.charge {
                            ChargeLevels::None => ChargeLevels::None,
                            ChargeLevels::Parent(name) => {
                                upgrade_names.insert(WeaponReference::new(weapon.name.clone(), *ranged, *index), upgrade.name.clone());
                                ChargeLevels::Parent(name.clone())
                            },
                            ChargeLevels::Child(parent_ref, name) => {
                                if let Some(new_parent) = upgrade_names.get(&parent_ref) {
                                    ChargeLevels::Child(WeaponReference::new(new_parent.to_string(), parent_ref.ranged, parent_ref.id), name.clone())
                                } else {
                                    ChargeLevels::Child(parent_ref.clone(), name.clone())
                                }
                            }
                        };

                        let new_weapon = (Weapon {
                            name: upgrade.name.clone(),
                            range: weapon.range,
                            attacks: if upgrade.attacks() {weapon.attacks.add_one()} else {weapon.attacks},
                            skill: if upgrade.skill() {weapon.skill - 1} else {weapon.skill},
                            strength: if upgrade.strength() {weapon.strength + 1} else {weapon.strength},
                            ap: if upgrade.ap() {if weapon.ap <= 0 {weapon.ap - 1} else {weapon.ap + 1}} else {weapon.ap},
                            damage: if upgrade.damage() {weapon.damage.add_one()} else {weapon.damage},
                            keywords: if upgrade.precise() {
                                let mut keywords = weapon.keywords.clone();
                                keywords.push(WeaponAbility::Precise);
                                keywords
                            } else {weapon.keywords.clone()},
                            charge: new_charge
                        }, i);
                        count -= i;
                        if *ranged {
                            crusade_ranged.push(new_weapon);
                        } else {
                            crusade_melee.push(new_weapon);
                        }
                    } 
                }
                if *ranged {
                    if count > 0 {crusade_ranged.push((weapon.clone(), count));}
                } else {
                    if count > 0 {crusade_melee.push((weapon.clone(), count));}
                }
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
            stats: self.stats.into(),
            extra_statlines,
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