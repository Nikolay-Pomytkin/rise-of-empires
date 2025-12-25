//! ECS components for the simulation
//!
//! These components define the data attached to game entities.

mod building;
mod gatherer;
mod health;
mod owner;
mod resource_node;
mod unit;

pub use building::*;
pub use gatherer::*;
pub use health::*;
pub use owner::*;
pub use resource_node::*;
pub use unit::*;

use bevy_ecs::prelude::*;
use shared::EntityId;

/// Marker component linking Bevy Entity to simulation EntityId
#[derive(Component, Clone, Copy, Debug)]
pub struct SimEntity {
    pub id: EntityId,
}

impl SimEntity {
    pub fn new(id: EntityId) -> Self {
        Self { id }
    }
}

/// Position component for simulation entities
/// Uses Vec3 where Y is height (for 3D orthographic rendering)
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct SimPosition {
    pub x: f32,
    pub y: f32, // Height
    pub z: f32,
}

impl SimPosition {
    pub fn new(x: f32, z: f32) -> Self {
        Self { x, y: 0.0, z }
    }

    pub fn with_height(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn to_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    /// Distance to another position (ignoring height)
    pub fn distance_xz(&self, other: &SimPosition) -> f32 {
        let dx = self.x - other.x;
        let dz = self.z - other.z;
        (dx * dx + dz * dz).sqrt()
    }
}

/// Velocity component for moving entities
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Velocity {
    pub x: f32,
    pub z: f32,
}

impl Velocity {
    pub fn new(x: f32, z: f32) -> Self {
        Self { x, z }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, z: 0.0 }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag > 0.0001 {
            Self {
                x: self.x / mag,
                z: self.z / mag,
            }
        } else {
            Self::zero()
        }
    }
}

/// Movement target for units
#[derive(Component, Clone, Debug)]
pub struct MoveTarget {
    pub x: f32,
    pub z: f32,
    /// If true, stop when close. If false, keep moving.
    pub stop_on_arrival: bool,
}

impl MoveTarget {
    pub fn new(x: f32, z: f32) -> Self {
        Self {
            x,
            z,
            stop_on_arrival: true,
        }
    }
}

/// Selection state - which players have this entity selected
#[derive(Component, Clone, Debug, Default)]
pub struct Selected {
    pub by_players: Vec<shared::PlayerId>,
}

impl Selected {
    pub fn is_selected_by(&self, player_id: shared::PlayerId) -> bool {
        self.by_players.contains(&player_id)
    }

    pub fn select(&mut self, player_id: shared::PlayerId) {
        if !self.by_players.contains(&player_id) {
            self.by_players.push(player_id);
        }
    }

    pub fn deselect(&mut self, player_id: shared::PlayerId) {
        self.by_players.retain(|&p| p != player_id);
    }
}

