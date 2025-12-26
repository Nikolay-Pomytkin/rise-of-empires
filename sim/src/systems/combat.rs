//! Combat system (stub)
//!
//! TODO: Full combat implementation
//! TODO: Projectile system

use bevy_ecs::prelude::*;

use crate::components::*;

/// Combat system - handles attacks and damage
/// Currently just a stub that ticks cooldowns
pub fn combat_system(mut units: Query<(Entity, &mut CombatStats, &mut Health)>) {
    // Tick cooldowns
    for (_, mut stats, _) in units.iter_mut() {
        stats.tick_cooldown();
    }

    // TODO: Implement actual combat
    // - Find enemies in range
    // - Attack if cooldown ready
    // - Apply damage
    // - Handle unit death
}

// =============================================================================
// Combat Stubs
// =============================================================================

/// Attack target component
/// TODO: Full combat implementation
#[derive(Component, Clone, Debug)]
#[allow(dead_code)]
pub struct AttackTarget {
    pub target: Entity,
}

/// Projectile component
/// TODO: Projectile system
#[derive(Component, Clone, Debug)]
#[allow(dead_code)]
pub struct Projectile {
    pub source: Entity,
    pub target: Entity,
    pub damage: u32,
    pub speed: f32,
}

/// Damage event
/// TODO: Full combat implementation
#[derive(Event)]
#[allow(dead_code)]
pub struct DamageEvent {
    pub target: Entity,
    pub damage: u32,
    pub source: Option<Entity>,
}

/// Death event
/// TODO: Full combat implementation
#[derive(Event)]
#[allow(dead_code)]
pub struct DeathEvent {
    pub entity: Entity,
    pub killer: Option<Entity>,
}
