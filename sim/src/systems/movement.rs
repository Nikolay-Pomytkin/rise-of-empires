//! Movement system
//!
//! Handles unit movement with naive steering.
//! TODO: A* pathfinding implementation
//! TODO: Formation movement

use bevy_ecs::prelude::*;
use bevy_time::{Fixed, Time};

use crate::components::*;

/// Naive movement system - direct steering toward target
pub fn movement_system(
    mut commands: Commands,
    mut units: Query<(
        Entity,
        &Unit,
        &mut SimPosition,
        &MoveTarget,
        Option<&mut Velocity>,
    )>,
    time: Res<Time<Fixed>>,
) {
    let dt = time.delta_secs();

    for (entity, unit, mut pos, target, velocity) in units.iter_mut() {
        let dx = target.x - pos.x;
        let dz = target.z - pos.z;
        let distance = (dx * dx + dz * dz).sqrt();

        const ARRIVAL_THRESHOLD: f32 = 0.1;

        if distance < ARRIVAL_THRESHOLD {
            // Arrived at target
            if target.stop_on_arrival {
                commands.entity(entity).remove::<MoveTarget>();
                if let Some(mut vel) = velocity {
                    *vel = Velocity::zero();
                }
            }
        } else {
            // Move toward target
            let speed = unit.move_speed;
            let move_distance = speed * dt;

            if move_distance >= distance {
                // Will arrive this tick
                pos.x = target.x;
                pos.z = target.z;
            } else {
                // Normalize direction and move
                let nx = dx / distance;
                let nz = dz / distance;
                pos.x += nx * move_distance;
                pos.z += nz * move_distance;

                // Update velocity for interpolation
                if let Some(mut vel) = velocity {
                    vel.x = nx * speed;
                    vel.z = nz * speed;
                }
            }
        }
    }
}

// =============================================================================
// A* Pathfinding Stub
// =============================================================================

/// Pathfinding request for async computation
/// TODO: A* pathfinding implementation
#[allow(dead_code)]
pub struct PathRequest {
    pub entity: Entity,
    pub start: (f32, f32),
    pub goal: (f32, f32),
}

/// Computed path
/// TODO: A* pathfinding implementation
#[allow(dead_code)]
pub struct Path {
    pub waypoints: Vec<(f32, f32)>,
    pub current_index: usize,
}

/// Pathfinding grid
/// TODO: A* pathfinding implementation
#[allow(dead_code)]
pub struct PathfindingGrid {
    pub width: u32,
    pub height: u32,
    pub tile_size: f32,
    pub blocked: Vec<bool>,
}

#[allow(dead_code)]
impl PathfindingGrid {
    pub fn new(width: u32, height: u32, tile_size: f32) -> Self {
        Self {
            width,
            height,
            tile_size,
            blocked: vec![false; (width * height) as usize],
        }
    }

    pub fn is_blocked(&self, x: u32, y: u32) -> bool {
        if x >= self.width || y >= self.height {
            return true;
        }
        self.blocked[(y * self.width + x) as usize]
    }

    pub fn set_blocked(&mut self, x: u32, y: u32, blocked: bool) {
        if x < self.width && y < self.height {
            self.blocked[(y * self.width + x) as usize] = blocked;
        }
    }

    /// Find path using A*
    /// TODO: A* pathfinding implementation
    pub fn find_path(&self, _start: (u32, u32), _goal: (u32, u32)) -> Option<Vec<(u32, u32)>> {
        // TODO: Implement A* algorithm
        // For now, return direct path
        None
    }
}

// =============================================================================
// Formation Stub
// =============================================================================

/// Formation types
/// TODO: Formation movement
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum FormationType {
    Line,
    Box,
    Wedge,
    Circle,
}

/// Formation component for groups of units
/// TODO: Formation movement
#[derive(Component, Clone, Debug)]
#[allow(dead_code)]
pub struct Formation {
    pub formation_type: FormationType,
    pub members: Vec<Entity>,
    pub spacing: f32,
}

#[allow(dead_code)]
impl Formation {
    pub fn new(formation_type: FormationType) -> Self {
        Self {
            formation_type,
            members: Vec::new(),
            spacing: 1.5,
        }
    }

    /// Calculate positions for formation members
    /// TODO: Formation movement
    pub fn calculate_positions(&self, _center: (f32, f32), _facing: f32) -> Vec<(f32, f32)> {
        // TODO: Implement formation position calculation
        Vec::new()
    }
}
