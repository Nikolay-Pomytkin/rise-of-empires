//! Rendering systems
//!
//! 2D sprite-based rendering for grid, units, buildings, and selection.
//!
//! Z-ordering (higher Z = rendered on top):
//! - Ground: 0
//! - Grid lines: 1
//! - Resources: 2
//! - Buildings: 3
//! - Units: 4-10
//! - Selection/UI: 100+

mod animation;
mod building;
mod feedback;
mod grid;
mod selection;
mod sprites;
mod units;

pub use animation::*;
pub use building::*;
pub use feedback::*;
pub use grid::*;
pub use selection::*;
pub use sprites::*;
pub use units::*;

use bevy::prelude::*;
use crate::game_state::GameState;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VisualFeedbackPlugin)
            .init_resource::<SpriteMaterials>()
            .init_resource::<AnimationData>()
            .add_systems(Startup, (setup_grid, load_sprite_assets, setup_animation_data))
            .add_systems(
                Update,
                (
                    sync_transforms,
                    update_unit_visuals,
                    update_building_visuals_sprite,
                    update_resource_node_visuals,
                    update_selection_visuals,
                    update_placement_ghost,
                    // Animation systems
                    animate_sprites,
                    update_facing_direction,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

/// Convert SimPosition to Vec3 for 2D rendering
/// SimPosition uses X/Z for ground plane, we convert to X/Y for 2D sprites
/// Z component is for sprite layering (see grid.rs layers module)
pub fn sim_pos_to_vec3(pos: &sim::SimPosition, base_layer: f32) -> Vec3 {
    // X stays X, sim.Z becomes screen Y (ground plane)
    // For depth sorting within a layer, use Y position
    // Objects higher on screen (larger Y) should be behind (smaller Z)
    // This gives a simple isometric-like depth effect
    let depth_offset = -pos.z * 0.001; // Tiny offset based on Y for depth sorting
    Vec3::new(
        pos.x * TILE_SIZE, 
        pos.z * TILE_SIZE, 
        base_layer + depth_offset
    )
}

/// Pixels per tile unit
pub const TILE_SIZE: f32 = 32.0;

/// Sync Bevy Transform from SimPosition
/// Note: This only syncs X/Y, Z is set when visual is created based on entity type
fn sync_transforms(
    mut query: Query<(&sim::SimPosition, &mut Transform), Changed<sim::SimPosition>>,
) {
    for (sim_pos, mut transform) in query.iter_mut() {
        // Only update X and Y, preserve Z layer
        transform.translation.x = sim_pos.x * TILE_SIZE;
        transform.translation.y = sim_pos.z * TILE_SIZE;
        // Z is preserved from initial spawn (layer-based)
    }
}

/// System to animate sprites based on timer and configuration
fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(
        &AnimationConfig,
        &AnimationState,
        &FacingDirection,
        &mut AnimationTimer,
        &mut AnimationFrame,
        &mut Sprite,
    )>,
) {
    for (config, state, direction, mut timer, mut frame, mut sprite) in query.iter_mut() {
        timer.timer.tick(time.delta());
        
        if timer.timer.just_finished() {
            // Advance frame
            if config.looping {
                frame.current = (frame.current + 1) % config.frames_per_animation;
            } else if frame.current < config.frames_per_animation - 1 {
                frame.current += 1;
            }
            
            // Update sprite texture atlas index if using atlas
            if let Some(ref mut atlas) = sprite.texture_atlas {
                atlas.index = config.get_atlas_index(state, direction, frame.current);
            }
        }
    }
}

/// System to update facing direction based on movement
fn update_facing_direction(
    mut query: Query<(&sim::SimPosition, &mut FacingDirection), Changed<sim::SimPosition>>,
) {
    // For now, this is a simple implementation
    // In a full implementation, you'd track velocity or movement direction
    // and update FacingDirection based on that
    
    // This system could be enhanced to:
    // 1. Compare current position to previous position
    // 2. Calculate movement direction
    // 3. Update FacingDirection accordingly
    
    for (pos, mut _direction) in query.iter_mut() {
        // Placeholder - would need to track previous positions
        // let current = Vec2::new(pos.x, pos.z);
        // if let Some(prev) = prev_positions.get(&entity) {
        //     let delta = current - *prev;
        //     if delta.length_squared() > 0.001 {
        //         *direction = FacingDirection::from_vec2(delta);
        //     }
        // }
        let _ = pos; // Suppress unused warning
    }
}
