//! Rendering systems
//!
//! 2D sprite-based rendering for grid, units, buildings, and selection.

mod building;
mod feedback;
mod grid;
mod selection;
mod sprites;
mod units;

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
            .add_systems(Startup, (setup_grid, load_sprite_assets))
            .add_systems(
                Update,
                (
                    sync_transforms,
                    update_unit_visuals,
                    update_building_visuals_sprite,
                    update_resource_node_visuals,
                    update_selection_visuals,
                    update_placement_ghost,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

/// Convert SimPosition to Vec3 for 2D rendering
/// SimPosition uses X/Z for ground plane, we convert to X/Y for 2D
/// The Z component is used for sprite ordering (depth)
/// In Bevy 2D: higher Z = closer to camera (in front), lower Z = behind
pub fn sim_pos_to_vec3(pos: &sim::SimPosition) -> Vec3 {
    // X stays X, Z becomes Y (ground plane)
    // Ground is at Z=-100, entities should be at positive Z (in front)
    // Use Z in range 0-100 for entities
    // Higher sim.z (further up on map) should render behind (lower Z)
    let z_order = 50.0 - pos.z.clamp(-50.0, 50.0);
    Vec3::new(pos.x * TILE_SIZE, pos.z * TILE_SIZE, z_order)
}

/// Pixels per tile unit
pub const TILE_SIZE: f32 = 32.0;

/// Sync Bevy Transform from SimPosition
fn sync_transforms(
    mut query: Query<(&sim::SimPosition, &mut Transform), Changed<sim::SimPosition>>,
) {
    for (sim_pos, mut transform) in query.iter_mut() {
        let new_pos = sim_pos_to_vec3(sim_pos);
        transform.translation.x = new_pos.x;
        transform.translation.y = new_pos.y;
        // Keep Z for ordering
        transform.translation.z = new_pos.z;
    }
}
