//! Health component

use bevy_ecs::prelude::*;

/// Health component for damageable entities
#[derive(Component, Clone, Debug)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}

impl Health {
    pub fn new(max: u32) -> Self {
        Self { current: max, max }
    }

    pub fn is_dead(&self) -> bool {
        self.current == 0
    }

    pub fn is_full(&self) -> bool {
        self.current == self.max
    }

    pub fn percentage(&self) -> f32 {
        if self.max == 0 {
            0.0
        } else {
            self.current as f32 / self.max as f32
        }
    }

    pub fn damage(&mut self, amount: u32) {
        self.current = self.current.saturating_sub(amount);
    }

    pub fn heal(&mut self, amount: u32) {
        self.current = (self.current + amount).min(self.max);
    }
}

/// Combat stats for units (stub for future implementation)
#[derive(Component, Clone, Debug)]
pub struct CombatStats {
    /// Base attack damage
    pub attack_damage: u32,
    /// Attack range in tiles
    pub attack_range: f32,
    /// Ticks between attacks
    pub attack_cooldown: u32,
    /// Current cooldown remaining
    pub cooldown_remaining: u32,
    /// Armor (damage reduction)
    pub armor: u32,
}

impl Default for CombatStats {
    fn default() -> Self {
        Self {
            attack_damage: 5,
            attack_range: 1.0,
            attack_cooldown: 20, // 1 second at 20 Hz
            cooldown_remaining: 0,
            armor: 0,
        }
    }
}

impl CombatStats {
    pub fn villager() -> Self {
        Self {
            attack_damage: 3,
            attack_range: 0.5,
            attack_cooldown: 30,
            cooldown_remaining: 0,
            armor: 0,
        }
    }

    pub fn soldier() -> Self {
        Self {
            attack_damage: 8,
            attack_range: 1.0,
            attack_cooldown: 20,
            cooldown_remaining: 0,
            armor: 1,
        }
    }

    pub fn can_attack(&self) -> bool {
        self.cooldown_remaining == 0
    }

    pub fn tick_cooldown(&mut self) {
        if self.cooldown_remaining > 0 {
            self.cooldown_remaining -= 1;
        }
    }

    pub fn start_cooldown(&mut self) {
        self.cooldown_remaining = self.attack_cooldown;
    }
}

