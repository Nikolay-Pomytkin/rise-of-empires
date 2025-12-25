//! Game data resource - holds all loaded RON data and computed modifiers

use bevy_ecs::prelude::*;
use shared::{PlayerId, ResourceType};
use std::collections::HashMap;

use crate::data::{TechTree, TechEffect, UnitDefs};

/// Resource holding all game data loaded from RON files
#[derive(Resource)]
pub struct GameData {
    pub tech_tree: TechTree,
    pub unit_defs: UnitDefs,
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            tech_tree: TechTree::default(),
            unit_defs: UnitDefs::default(),
        }
    }
}

impl GameData {
    /// Load game data from RON strings
    pub fn load(tech_ron: &str, units_ron: &str) -> Result<Self, ron::error::SpannedError> {
        let tech_tree: TechTree = ron::from_str(tech_ron)?;
        let unit_defs: UnitDefs = ron::from_str(units_ron)?;
        Ok(Self { tech_tree, unit_defs })
    }
}

/// Per-player modifiers computed from researched techs
#[derive(Resource, Default)]
pub struct PlayerModifiers {
    pub modifiers: HashMap<PlayerId, Modifiers>,
}

impl PlayerModifiers {
    pub fn get(&self, player_id: PlayerId) -> Modifiers {
        self.modifiers.get(&player_id).cloned().unwrap_or_default()
    }

    pub fn get_mut(&mut self, player_id: PlayerId) -> &mut Modifiers {
        self.modifiers.entry(player_id).or_default()
    }

    /// Recalculate modifiers for a player based on their researched techs
    pub fn recalculate(&mut self, player_id: PlayerId, researched_techs: &[String], tech_tree: &TechTree) {
        let mods = self.get_mut(player_id);
        *mods = Modifiers::default();

        for tech_id in researched_techs {
            if let Some(tech) = tech_tree.get_tech(tech_id) {
                for effect in &tech.effects {
                    mods.apply_effect(effect);
                }
            }
        }
    }
}

/// Computed modifiers for a single player
#[derive(Debug, Clone, Default)]
pub struct Modifiers {
    /// Gather rate bonus per resource type (percentage, e.g., 10 = +10%)
    pub gather_rate_bonus: HashMap<ResourceType, i32>,
    /// Flat carry capacity bonus
    pub carry_capacity_bonus: i32,
    /// Build speed bonus (percentage)
    pub build_speed_bonus: i32,
    /// Production speed bonus (percentage)
    pub production_speed_bonus: i32,
    /// Unit HP bonus (percentage)
    pub unit_hp_bonus: i32,
    /// Unit attack bonus (flat)
    pub unit_attack_bonus: i32,
}

impl Modifiers {
    /// Create default modifiers (no bonuses)
    pub fn new() -> Self {
        Self::default()
    }

    /// Apply a tech effect to these modifiers
    pub fn apply_effect(&mut self, effect: &TechEffect) {
        match effect {
            TechEffect::GatherRateBonus { resource, percent } => {
                *self.gather_rate_bonus.entry(*resource).or_insert(0) += percent;
            }
            TechEffect::CarryCapacityBonus { amount } => {
                self.carry_capacity_bonus += amount;
            }
            TechEffect::BuildSpeedBonus { percent } => {
                self.build_speed_bonus += percent;
            }
            TechEffect::ProductionSpeedBonus { percent } => {
                self.production_speed_bonus += percent;
            }
            TechEffect::UnitHpBonus { percent } => {
                self.unit_hp_bonus += percent;
            }
            TechEffect::UnitAttackBonus { amount } => {
                self.unit_attack_bonus += amount;
            }
            TechEffect::UnlockUnit { .. } | TechEffect::UnlockBuilding { .. } => {
                // Handled elsewhere (unlock tracking)
            }
        }
    }

    /// Get the gather rate multiplier for a resource (1.0 = normal, 1.1 = +10%)
    pub fn gather_rate_multiplier(&self, resource: ResourceType) -> f32 {
        let bonus = self.gather_rate_bonus.get(&resource).copied().unwrap_or(0);
        1.0 + (bonus as f32 / 100.0)
    }

    /// Get effective carry capacity (base + bonus)
    pub fn effective_carry_capacity(&self, base: u32) -> u32 {
        (base as i32 + self.carry_capacity_bonus).max(1) as u32
    }
}

