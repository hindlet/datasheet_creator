use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Wargear {
    pub count: u32,
    pub wargear: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum WargearCount {
    All,
    One,
    UpTo(u32)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum WargearOptionType {
    Add,
    Replace
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WargearOption {
    count: WargearCount,
    wargear_type: WargearOptionType,

}