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
    pub fn to_string(&self) -> (String, f32) {
        match self {
            VariableValue::Rolled(count, die, add) => {
                let mut text = String::new();
                let mut offset = 0.7;
                if *count != 1 {
                    text += &format!("{}", count);
                    offset += 0.8;
                }
                text += die.to_string();
                if *add != 0 {
                    text += &format!("+{}", add);
                    offset += 1.3 + (*add >= 10) as i32 as f32 * 0.7;
                }
                return (text, offset);
            },
            VariableValue::Set(val) => {
                let offset = (*val >= 10) as i32 as f32 * 0.7;
                return (format!("{}", val), offset);
            }
        }
    }
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum Range {
    Melee,
    Ranged(u32)
}


#[derive(Debug, Deserialize)]
pub struct Weapon {
    pub name: String,
    pub range: Range,
    pub attacks: VariableValue,
    pub skill: u32,
    pub strength: u32,
    pub ap: u32,
    pub damage: VariableValue,
    pub keywords: Vec<String>
}