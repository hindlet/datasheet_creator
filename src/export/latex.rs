use crate::data::Unit;
use std::{fs::write, path::PathBuf};


// Using ± as variable value marker
pub fn export_to_latex(unit: &Unit, template: &str, path: PathBuf) -> Result<(), std::io::Error> {
    
    // stats
    let mut result = template.replace("±NAME", &unit.name);
    result = result.replace("±MOVEMENT", &unit.stats.movement.to_string());
    result = result.replace("±TOUGHNESS", &unit.stats.toughness.to_string());
    result = result.replace("±SAVE", &unit.stats.save.to_string());
    result = result.replace("±WOUNDS", &unit.stats.wounds.to_string());
    result = result.replace("±LEADERSHIP", &unit.stats.leadership.to_string());
    result = result.replace("±OC", &unit.stats.oc.to_string());
    let mut inv = "".to_string();
    if let Some(val) = unit.stats.invuln {
        inv = format!("\\invuln{{{}}}", val);
    }
    result = result.replace("±INVULN", &inv);

    // ranged weapons
    let mut ranged_weapons = "".to_string();
    for weapon in unit.ranged_weapons.iter() {
        ranged_weapons.push_str(&format!("{} & {} & {} & {}+ & {} & -{} & {} \\\\", weapon.name, weapon.range.to_string(), weapon.attacks.to_string(), weapon.skill, weapon.strength, weapon.ap, weapon.damage.to_string()));
        let keywords = weapon.format_keywords();
        if keywords != "[]" {
            ranged_weapons.push_str(&format!("\n\\keyword{{{}}} & & & & & & \\\\", keywords.replace("[", "").replace("]", "")));
        }
        ranged_weapons.push_str("\n\\hline\n");
    }
    result = result.replace("±RANGED_WEAPONS", &ranged_weapons);

    // melee weapons
    let mut melee_weapons = "".to_string();
    for weapon in unit.melee_weapons.iter() {
        melee_weapons.push_str(&format!("{} & {} & {} & {}+ & {} & -{} & {} \\\\", weapon.name, weapon.range.to_string(), weapon.attacks.to_string(), weapon.skill, weapon.strength, weapon.ap, weapon.damage.to_string()));
        let keywords = weapon.format_keywords();
        if keywords != "[]" {
            melee_weapons.push_str(&format!("\n\\keyword{{{}}} & & & & & & \\\\", keywords.replace("[", "").replace("]", "")));
        }
        melee_weapons.push_str("\n\\hline\n");
    }
    result = result.replace("±MELEE_WEAPONS", &melee_weapons);


    // abilities
    let mut core_abilities = "".to_string();
    for (i, ability) in unit.core_abilities.iter().enumerate() {
        core_abilities.push_str(&ability);
        if i != unit.core_abilities.len() - 1 {
            core_abilities.push_str(", ");
        }
    }
    if core_abilities != "" {
        result = result.replace("±CORE_ABILITIES", &format!("Core: \\textbf{{{}}} \\\\\n\\hline[dotted]\n", core_abilities));
    } else {result = result.replace("±CORE_ABILITIES", "");}


    if let Some(ability) = &unit.faction_ability {
        result = result.replace("±FACTION_ABILITY", &format!("Faction: \\textbf{{{}}} \\\\\n\\hline[dotted]\n", &ability));
    } else {result = result.replace("±FACTION_ABILITY", "");}

    let mut abilities = "".to_string();
    for ability in unit.unique_abilities.iter() {
        abilities.push_str(&format!("\\textbf{{{}:}} {}\\\\\n", ability.name, ability.description));
    }
    result = result.replace("±ABILITIES", &abilities);


    let wargear_abilities = "".to_string();

    result = result.replace("±WARGEAR_ABILITIES", &wargear_abilities);

    let unit_composition = "".to_string();

    result = result.replace("±UNIT_COMPOSITION", &unit_composition);


    result = result.replace("±KEYWORDS", &unit.format_keywords());

    write(path, result)
}