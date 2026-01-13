use egui::Ui;

use crate::{data::unit_stats::UnitStats, helper_funcs::select_drag_value_with_range_on_tab};


#[derive(Clone, Copy, Debug)]
pub struct EditStats {
    pub movement: u32,
    pub toughness: u32,
    pub save: u32,
    pub has_invuln: bool,
    pub invuln: u32,
    pub wounds: u32,
    pub leadership: u32,
    pub objective_control: u32,
}

impl From<UnitStats> for EditStats {
    fn from(value: UnitStats) -> Self {
        Self {
            movement: value.movement,
            toughness: value.toughness,
            save: value.save,
            has_invuln: value.invuln.is_some(),
            invuln: value.invuln.unwrap_or(4),
            wounds: value.wounds,
            leadership: value.leadership,
            objective_control: value.oc
        }
    }
}

impl From<&UnitStats> for EditStats {
    fn from(value: &UnitStats) -> Self {
        Self {
            movement: value.movement,
            toughness: value.toughness,
            save: value.save,
            has_invuln: value.invuln.is_some(),
            invuln: value.invuln.unwrap_or(4),
            wounds: value.wounds,
            leadership: value.leadership,
            objective_control: value.oc
        }
    }
}

impl Into<UnitStats> for EditStats {
    fn into(self) -> UnitStats {
        UnitStats {
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
        }
    }
}

impl Into<UnitStats> for &EditStats {
    fn into(self) -> UnitStats {
        UnitStats {
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
        }
    }
}


impl EditStats {
    pub fn render(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Movement:");
            select_drag_value_with_range_on_tab(&mut self.movement, 0..=99, ui);
        });
        ui.horizontal(|ui| {
            ui.label("Toughness:");
            select_drag_value_with_range_on_tab(&mut self.toughness, 1..=99, ui);
        });
        ui.horizontal(|ui| {
            ui.label("Save:");
            select_drag_value_with_range_on_tab(&mut self.save, 1..=6, ui);
        });
        ui.horizontal(|ui| {
            ui.label("Has Invulnerable Save:");
            ui.checkbox(&mut self.has_invuln, "");
        });
        if self.has_invuln {
            ui.horizontal(|ui| {
                ui.label("Invulnerable Save:");
                select_drag_value_with_range_on_tab(&mut self.invuln, 1..=6, ui);
            });
        }
        ui.horizontal(|ui| {
            ui.label("Wounds:");
            select_drag_value_with_range_on_tab(&mut self.wounds, 1..=99, ui);
        });
        ui.horizontal(|ui| {
            ui.label("Leadership:");
            select_drag_value_with_range_on_tab(&mut self.leadership, 1..=12, ui);
        });
        ui.horizontal(|ui| {
            ui.label("Objective Control:");
            select_drag_value_with_range_on_tab(&mut self.objective_control, 0..=99, ui);
        });
    }
}