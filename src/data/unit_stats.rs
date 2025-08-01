use serde::{Deserialize, Serialize};
use tera::Context;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UnitStats {
    pub movement: u32,
    pub toughness: u32,
    pub save: u32,
    pub invuln: Option<u32>,
    pub wounds: u32,
    pub leadership: u32,
    pub oc: u32,
}

impl Default for UnitStats {
    fn default() -> Self {
        UnitStats {
            movement: 0,
            toughness: 0,
            save: 0,
            invuln: None,
            wounds: 0,
            leadership: 0,
            oc: 0
        }
    }
}

impl UnitStats {
    pub fn add_context(&self, context: &mut Context) {
        context.insert("movement", &format!("{}", self.movement));
        context.insert("toughness", &self.toughness);
        context.insert("save", &self.save);
        if let Some(invuln) = self.invuln {
            context.insert("invuln", &format!("{}", invuln));
        } else {
            context.insert("invuln", &"None".to_string());
        }
        context.insert("wounds", &self.wounds);
        context.insert("leadership", &self.leadership);
        context.insert("oc", &self.oc);
    }
}