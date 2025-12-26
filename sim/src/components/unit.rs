//! Unit components

use bevy_ecs::prelude::*;
use shared::UnitType;

/// Marker component for unit entities
#[derive(Component, Clone, Debug)]
pub struct Unit {
    pub unit_type: UnitType,
    /// Base movement speed (tiles per second)
    pub move_speed: f32,
}

impl Unit {
    pub fn villager() -> Self {
        Self {
            unit_type: UnitType::Villager,
            move_speed: 2.0,
        }
    }

    pub fn soldier() -> Self {
        Self {
            unit_type: UnitType::Soldier,
            move_speed: 2.5,
        }
    }
}

/// Marker for villager-specific behavior
#[derive(Component, Clone, Debug, Default)]
pub struct Villager;

/// Marker for soldier units (stub for future combat)
#[derive(Component, Clone, Debug, Default)]
pub struct Soldier;
