//! Fixed tick scheduler for deterministic simulation
//!
//! The simulation runs at a fixed tick rate (default 20 Hz) independent
//! of frame rate. This ensures determinism for replays and lockstep.

use bevy_ecs::prelude::*;

/// Configuration for the tick system
#[derive(Resource, Clone)]
pub struct TickConfig {
    /// Ticks per second
    pub tick_rate: u32,
}

impl Default for TickConfig {
    fn default() -> Self {
        Self { tick_rate: 20 }
    }
}

/// Tracks the current simulation tick
#[derive(Resource)]
pub struct TickScheduler {
    /// Current tick number
    pub current_tick: u64,
    /// Ticks per second
    pub tick_rate: u32,
    /// Duration of one tick in seconds
    pub tick_duration: f32,
}

impl TickScheduler {
    pub fn new(tick_rate: u32) -> Self {
        Self {
            current_tick: 0,
            tick_rate,
            tick_duration: 1.0 / tick_rate as f32,
        }
    }

    /// Advance to the next tick
    pub fn advance(&mut self) {
        self.current_tick += 1;
    }

    /// Get the current tick
    pub fn tick(&self) -> u64 {
        self.current_tick
    }

    /// Convert ticks to seconds
    pub fn ticks_to_seconds(&self, ticks: u32) -> f32 {
        ticks as f32 * self.tick_duration
    }

    /// Convert seconds to ticks (rounded)
    pub fn seconds_to_ticks(&self, seconds: f32) -> u32 {
        (seconds / self.tick_duration).round() as u32
    }
}

impl Default for TickScheduler {
    fn default() -> Self {
        Self::new(20)
    }
}
