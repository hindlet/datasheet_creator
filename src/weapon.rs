use serde::Deserialize;


#[derive(Debug, Deserialize, Clone, Copy)]
pub enum Dice {
    D3,
    D6
}

impl Dice {
    pub fn to_string(&self) -> &str {
        match self {
            Dice::D3 => return "D3",
            Dice::D6 => return "D6"
        }
    }
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum VariableValue {
    Set(u32),
    Rolled(u32, Dice, u32)
}

impl VariableValue {
    pub fn to_string(&self) -> String {
        match self {
            VariableValue::Rolled(count, die, add) => {
                let mut text = String::new();
                if *count != 1 {
                    text += &format!("{}", count);
                }
                text += die.to_string();
                if *add != 0 {
                    text += &format!("+{}", add);
                }
                return text;
            },
            VariableValue::Set(val) => {
                return format!("{}", val);
            }
        }
    }
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum Range {
    Melee,
    Ranged(u32)
}

impl Range {
    pub fn to_string(&self) -> String {
        match self {
            Range::Melee => return "melee".to_string(),
            Range::Ranged(range) => return format!("{}\"", range)
        }
    }
}


#[derive(Debug, Deserialize)]
pub struct Weapon {
    pub name: String,
    pub range: Range,
    pub attacks: VariableValue,
    pub skill: u32,
    pub strength: u32,
    pub ap: i32,
    pub damage: VariableValue,
    pub keywords: Vec<String>
}

pub type WeaponTuple = (String, String, String, String, u32, String, String, Vec<String>);

impl Weapon {
    pub fn to_html_data(&self) -> WeaponTuple {
        let mut cased_keywords = Vec::new();
        for keyword in self.keywords.iter() {
            cased_keywords.push(keyword.to_uppercase());
        }

        let skill: String;
        if cased_keywords.contains(&"TORRENT".to_string()) {
            skill = "N/A".to_string();
        } else {
            skill = format!("{}+", self.skill)
        }

        let ap: String;
        if self.ap > 0 {
            ap = format!("-{}", self.ap);
        } else {
            ap = format!("{}", self.ap);
        }

        (
            self.name.clone(),
            self.range.to_string(),
            self.attacks.to_string(),
            skill,
            self.strength,
            ap,
            self.damage.to_string(),
            cased_keywords
        )
    }
}