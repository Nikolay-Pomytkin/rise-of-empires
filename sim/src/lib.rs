//! Deterministic simulation core for Rise RTS
//!
//! This crate contains all authoritative game logic that runs on fixed ticks.
//! It is designed to be deterministic for replay and lockstep networking.
//!
//! # Architecture
//!
//! - Fixed tick rate (configurable, default 20 Hz)
//! - Commands are processed in deterministic order (tick -> player_id -> sequence)
//! - All random operations use a seeded deterministic RNG
//! - No I/O or windowing - pure simulation
//!
//! # Usage
//!
//! ```ignore
//! use sim::SimPlugin;
//! use bevy_app::prelude::*;
//!
//! App::new()
//!     .add_plugins(SimPlugin::default())
//!     .run();
//! ```

pub mod components;
pub mod data;
pub mod rng;
pub mod systems;
pub mod tick;
pub mod world;

#[cfg(test)]
mod tests;

// Re-export bevy_ecs types for client convenience
pub use bevy_app::prelude::*;
pub use bevy_ecs::prelude::*;

pub use components::*;
pub use data::*;
pub use rng::*;
pub use tick::*;
pub use world::*;

/// Main simulation plugin
///
/// Configurable tick rate and RNG seed.
#[derive(Clone)]
pub struct SimPlugin {
    /// Ticks per second (default: 20)
    pub tick_rate: u32,
    /// RNG seed for determinism (default: 12345)
    pub rng_seed: u64,
}

impl Default for SimPlugin {
    fn default() -> Self {
        Self {
            tick_rate: 20,
            rng_seed: 12345,
        }
    }
}

impl bevy_app::Plugin for SimPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        use bevy_app::FixedUpdate;

        // Initialize tick scheduler
        let tick_config = TickConfig {
            tick_rate: self.tick_rate,
        };

        app.insert_resource(tick_config)
            .insert_resource(TickScheduler::new(self.tick_rate))
            .insert_resource(SimRng::new(self.rng_seed))
            .insert_resource(CommandBuffer::default())
            .insert_resource(SimWorld::default())
            .insert_resource(EntityIdGenerator::default())
            .insert_resource(GameData::default())
            .insert_resource(PlayerModifiers::default())
            .add_message::<SnapshotEvent>()
            .add_systems(FixedUpdate, systems::process_commands)
            .add_systems(
                FixedUpdate,
                systems::movement_system.after(systems::process_commands),
            )
            .add_systems(
                FixedUpdate,
                systems::gather_system.after(systems::movement_system),
            )
            .add_systems(
                FixedUpdate,
                systems::production_system.after(systems::gather_system),
            )
            .add_systems(
                FixedUpdate,
                systems::combat_system.after(systems::production_system),
            )
            .add_systems(
                FixedUpdate,
                systems::generate_snapshot.after(systems::combat_system),
            );
    }
}

/// Event emitted when a new snapshot is generated
#[derive(bevy_ecs::message::Message, Clone)]
pub struct SnapshotEvent {
    pub snapshot: shared::WorldSnapshot,
}
