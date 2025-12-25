//! Tech tree data structures and loading

use serde::{Deserialize, Serialize};
use shared::{ResourceBundle, ResourceType, TechId, AgeId};
use std::collections::HashMap;

/// Effect that a technology can apply
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TechEffect {
    /// Bonus to gather rate for a resource type (percentage)
    GatherRateBonus { resource: ResourceType, percent: i32 },
    /// Bonus to carry capacity (flat amount)
    CarryCapacityBonus { amount: i32 },
    /// Bonus to build speed (percentage)
    BuildSpeedBonus { percent: i32 },
    /// Bonus to unit production speed (percentage)
    ProductionSpeedBonus { percent: i32 },
    /// Bonus to unit HP (percentage)
    UnitHpBonus { percent: i32 },
    /// Bonus to unit attack (flat amount)
    UnitAttackBonus { amount: i32 },
    /// Unlock a unit type
    UnlockUnit { unit_id: String },
    /// Unlock a building type
    UnlockBuilding { building_id: String },
}

/// A single technology definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub cost: TechCost,
    pub research_time_ticks: u32,
    pub effects: Vec<TechEffect>,
    /// Required techs to research this one
    pub requires: Vec<String>,
    /// Required age to research
    pub required_age: Option<String>,
}

/// Cost to research a technology
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TechCost {
    #[serde(default)]
    pub food: u32,
    #[serde(default)]
    pub wood: u32,
    #[serde(default)]
    pub gold: u32,
    #[serde(default)]
    pub stone: u32,
}

impl From<TechCost> for ResourceBundle {
    fn from(cost: TechCost) -> Self {
        ResourceBundle::new(cost.food, cost.wood, cost.gold, cost.stone)
    }
}

/// An age in the tech tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgeDef {
    pub id: String,
    pub name: String,
    pub order: u32,
    /// Cost to advance to this age
    pub cost: TechCost,
    /// Time in ticks to advance
    pub advance_time_ticks: u32,
    /// Required age (previous age)
    pub requires: Option<String>,
}

/// Complete tech tree definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechTree {
    pub ages: Vec<AgeDef>,
    pub techs: Vec<TechDef>,
}

impl TechTree {
    /// Load tech tree from RON string
    pub fn from_ron(data: &str) -> Result<Self, ron::error::SpannedError> {
        ron::from_str(data)
    }

    /// Get a tech by ID
    pub fn get_tech(&self, id: &str) -> Option<&TechDef> {
        self.techs.iter().find(|t| t.id == id)
    }

    /// Get an age by ID
    pub fn get_age(&self, id: &str) -> Option<&AgeDef> {
        self.ages.iter().find(|a| a.id == id)
    }

    /// Get techs available in a given age
    pub fn techs_for_age(&self, age_id: &str) -> Vec<&TechDef> {
        self.techs
            .iter()
            .filter(|t| t.required_age.as_deref() == Some(age_id) || t.required_age.is_none())
            .collect()
    }

    /// Check if a tech can be researched given current state
    pub fn can_research(
        &self,
        tech_id: &str,
        current_age: &str,
        researched: &[String],
    ) -> bool {
        let Some(tech) = self.get_tech(tech_id) else {
            return false;
        };

        // Check if already researched
        if researched.contains(&tech.id) {
            return false;
        }

        // Check age requirement
        if let Some(ref req_age) = tech.required_age {
            let current_order = self.get_age(current_age).map(|a| a.order).unwrap_or(0);
            let required_order = self.get_age(req_age).map(|a| a.order).unwrap_or(0);
            if current_order < required_order {
                return false;
            }
        }

        // Check tech requirements
        for req in &tech.requires {
            if !researched.contains(req) {
                return false;
            }
        }

        true
    }
}

/// Default tech tree for testing
impl Default for TechTree {
    fn default() -> Self {
        Self {
            ages: vec![
                AgeDef {
                    id: "dark_age".to_string(),
                    name: "Dark Age".to_string(),
                    order: 1,
                    cost: TechCost::default(),
                    advance_time_ticks: 0,
                    requires: None,
                },
                AgeDef {
                    id: "feudal_age".to_string(),
                    name: "Feudal Age".to_string(),
                    order: 2,
                    cost: TechCost { food: 500, wood: 0, gold: 0, stone: 0 },
                    advance_time_ticks: 260, // 13 seconds at 20 Hz
                    requires: Some("dark_age".to_string()),
                },
            ],
            techs: vec![
                TechDef {
                    id: "loom".to_string(),
                    name: "Loom".to_string(),
                    description: "Villagers +15 HP".to_string(),
                    cost: TechCost { food: 0, wood: 0, gold: 50, stone: 0 },
                    research_time_ticks: 50,
                    effects: vec![TechEffect::UnitHpBonus { percent: 15 }],
                    requires: vec![],
                    required_age: None,
                },
                TechDef {
                    id: "wheelbarrow".to_string(),
                    name: "Wheelbarrow".to_string(),
                    description: "+10% gather rate, +3 carry capacity".to_string(),
                    cost: TechCost { food: 175, wood: 50, gold: 0, stone: 0 },
                    research_time_ticks: 150,
                    effects: vec![
                        TechEffect::GatherRateBonus { resource: ResourceType::Food, percent: 10 },
                        TechEffect::GatherRateBonus { resource: ResourceType::Wood, percent: 10 },
                        TechEffect::GatherRateBonus { resource: ResourceType::Gold, percent: 10 },
                        TechEffect::GatherRateBonus { resource: ResourceType::Stone, percent: 10 },
                        TechEffect::CarryCapacityBonus { amount: 3 },
                    ],
                    requires: vec![],
                    required_age: Some("feudal_age".to_string()),
                },
                TechDef {
                    id: "double_bit_axe".to_string(),
                    name: "Double-Bit Axe".to_string(),
                    description: "+20% wood gathering".to_string(),
                    cost: TechCost { food: 100, wood: 50, gold: 0, stone: 0 },
                    research_time_ticks: 50,
                    effects: vec![
                        TechEffect::GatherRateBonus { resource: ResourceType::Wood, percent: 20 },
                    ],
                    requires: vec![],
                    required_age: Some("feudal_age".to_string()),
                },
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tech_tree_default() {
        let tree = TechTree::default();
        assert_eq!(tree.ages.len(), 2);
        assert_eq!(tree.techs.len(), 3);
    }

    #[test]
    fn test_can_research() {
        let tree = TechTree::default();
        
        // Loom available in dark age
        assert!(tree.can_research("loom", "dark_age", &[]));
        
        // Wheelbarrow requires feudal age
        assert!(!tree.can_research("wheelbarrow", "dark_age", &[]));
        assert!(tree.can_research("wheelbarrow", "feudal_age", &[]));
        
        // Can't research already researched tech
        assert!(!tree.can_research("loom", "dark_age", &["loom".to_string()]));
    }
}

