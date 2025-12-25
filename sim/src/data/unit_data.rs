//! Unit data definitions

use serde::{Deserialize, Serialize};
use shared::{ResourceBundle, UnitType};

/// Unit definition loaded from data files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitDef {
    pub id: String,
    pub name: String,
    pub unit_type: UnitType,
    pub cost: UnitCost,
    pub train_time_ticks: u32,
    pub hp: u32,
    pub speed: f32,
    pub attack: u32,
    pub attack_range: f32,
    pub armor: u32,
    pub population: u32,
}

/// Cost to train a unit
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnitCost {
    #[serde(default)]
    pub food: u32,
    #[serde(default)]
    pub wood: u32,
    #[serde(default)]
    pub gold: u32,
    #[serde(default)]
    pub stone: u32,
}

impl From<UnitCost> for ResourceBundle {
    fn from(cost: UnitCost) -> Self {
        ResourceBundle::new(cost.food, cost.wood, cost.gold, cost.stone)
    }
}

/// Collection of unit definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitDefs {
    pub units: Vec<UnitDef>,
}

impl UnitDefs {
    pub fn get(&self, id: &str) -> Option<&UnitDef> {
        self.units.iter().find(|u| u.id == id)
    }
}

impl Default for UnitDefs {
    fn default() -> Self {
        Self {
            units: vec![
                UnitDef {
                    id: "villager".to_string(),
                    name: "Villager".to_string(),
                    unit_type: UnitType::Villager,
                    cost: UnitCost { food: 50, ..Default::default() },
                    train_time_ticks: 50,
                    hp: 25,
                    speed: 2.0,
                    attack: 3,
                    attack_range: 0.5,
                    armor: 0,
                    population: 1,
                },
                UnitDef {
                    id: "soldier".to_string(),
                    name: "Soldier".to_string(),
                    unit_type: UnitType::Soldier,
                    cost: UnitCost { food: 60, gold: 20, ..Default::default() },
                    train_time_ticks: 60,
                    hp: 40,
                    speed: 2.5,
                    attack: 8,
                    attack_range: 1.0,
                    armor: 1,
                    population: 1,
                },
            ],
        }
    }
}

