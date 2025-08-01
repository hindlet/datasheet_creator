
use serde::{Deserialize, Serialize};
use regex::Regex;

#[derive(Debug, Deserialize, Clone, Copy, Serialize, PartialEq)]
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




#[derive(Debug, Deserialize, Clone, Copy, Serialize, PartialEq)]
pub enum VariableValue {
    Set(u32),
    Rolled(u32, Dice, u32)
}

pub struct VariableValueConversionError;

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


    pub fn is_valid_variable_val(string: &str) -> bool {
        let dice_and_const = Regex::new(r"^\d+D[36]\+\d+$").unwrap();
        let die_and_const = Regex::new(r"^D[36]\+\d+$").unwrap();
        let dice_only = Regex::new(r"^\d+D[36]$").unwrap();
        let die_only = Regex::new(r"^D[36]$").unwrap();
        let const_only = Regex::new(r"^\d+$").unwrap();

        return dice_and_const.is_match(string) || die_and_const.is_match(string) || dice_only.is_match(string) || die_only.is_match(string) || const_only.is_match(string);
    }

    pub fn from_string(string: &str) -> Result<Self, VariableValueConversionError> {
        let dice_and_const = Regex::new(r"^\d+D[36]\+\d+$").unwrap();
        let die_and_const = Regex::new(r"^D[36]\+\d+$").unwrap();
        let dice_only = Regex::new(r"^\d+D[36]$").unwrap();
        let die_only = Regex::new(r"^D[36]$").unwrap();
        let const_only = Regex::new(r"^\d+$").unwrap();

        if dice_and_const.is_match(string) {
            let split_one: Vec<&str> = string.split("+").collect();
            let split_two: Vec<&str> = split_one[0].split("D").collect();

            let die_count: u32 = if split_two[0] == "" {
                0
            } else {
                split_two[0].parse().unwrap()
            };

            let die_type = if split_two[1] == "3" {
                Dice::D3
            } else {
                Dice::D6
            };

            let const_val: u32 = split_one[1].parse().unwrap();

            return Ok(VariableValue::Rolled(die_count, die_type, const_val));
        } else if die_and_const.is_match(string) {
            let split_one: Vec<&str> = string.split("+").collect();
            let split_two: String = split_one[0].replace("D", "");

            let die_type = if split_two == "3" {
                Dice::D3
            } else {
                Dice::D6
            };

            let const_val: u32 = split_one[1].parse().unwrap();

            return Ok(VariableValue::Rolled(1, die_type, const_val));
        } else if dice_only.is_match(string) {
            let split_one: Vec<&str> = string.split("D").collect();


            let die_count: u32 = if split_one[0] == "" {
                0
            } else {
                split_one[0].parse().unwrap()
            };

            let die_type = if split_one[1] == "3" {
                Dice::D3
            } else {
                Dice::D6
            };


            return Ok(VariableValue::Rolled(die_count, die_type, 0));
        } else if die_only.is_match(string) {
            let split_one: String = string.replace("D", "");

            let die_type = if split_one == "3" {
                Dice::D3
            } else {
                Dice::D6
            };


            return Ok(VariableValue::Rolled(1, die_type, 0));
        } else if const_only.is_match(string) {
            return Ok(VariableValue::Set(string.parse().unwrap()));
        } else {
            return Err(VariableValueConversionError);
        }
    }
}

