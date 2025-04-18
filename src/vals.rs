use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Copy, Serialize)]
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

#[derive(Debug, Deserialize, Clone, Copy, Serialize)]
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