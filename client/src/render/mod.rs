//! Rendering systems
//!
//! Grid, units, buildings, and selection rendering.

mod feedback;
mod grid;
mod selection;
mod sprites;
mod units;

pub use feedback::*;
pub use grid::*;
pub use selection::*;
pub use sprites::*;
pub use units::*;

use bevy::prelude::*;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VisualFeedbackPlugin)
            .init_resource::<SpriteMaterials>()
            .add_systems(Startup, (setup_grid, setup_materials, load_sprite_assets))
            .add_systems(
                Update,
                (
                    sync_transforms,
                    update_unit_visuals,
                    update_building_visuals_sprite,
                    update_resource_node_visuals,
                    update_selection_visuals,
                    billboard_system,
                ),
            );
    }
}

/// Convert SimPosition to Vec3
pub fn sim_pos_to_vec3(pos: &sim::SimPosition) -> Vec3 {
    Vec3::new(pos.x, pos.y, pos.z)
}

/// Materials for rendering
#[derive(Resource)]
pub struct GameMaterials {
    pub player1_unit: Handle<StandardMaterial>,
    pub player2_unit: Handle<StandardMaterial>,
    pub neutral: Handle<StandardMaterial>,
    pub town_center: Handle<StandardMaterial>,
    pub barracks: Handle<StandardMaterial>,
    pub food_node: Handle<StandardMaterial>,
    pub wood_node: Handle<StandardMaterial>,
    pub gold_node: Handle<StandardMaterial>,
    pub stone_node: Handle<StandardMaterial>,
    pub selection_ring: Handle<StandardMaterial>,
    pub grid_line: Handle<StandardMaterial>,
}

fn setup_materials(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    let game_materials = GameMaterials {
        player1_unit: materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.5, 0.9),
            ..default()
        }),
        player2_unit: materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.3, 0.2),
            ..default()
        }),
        neutral: materials.add(StandardMaterial {
            base_color: Color::srgb(0.5, 0.5, 0.5),
            ..default()
        }),
        town_center: materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.7, 0.5),
            ..default()
        }),
        barracks: materials.add(StandardMaterial {
            base_color: Color::srgb(0.6, 0.4, 0.3),
            ..default()
        }),
        food_node: materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.8, 0.3),
            ..default()
        }),
        wood_node: materials.add(StandardMaterial {
            base_color: Color::srgb(0.4, 0.25, 0.1),
            ..default()
        }),
        gold_node: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.85, 0.0),
            ..default()
        }),
        stone_node: materials.add(StandardMaterial {
            base_color: Color::srgb(0.6, 0.6, 0.65),
            ..default()
        }),
        selection_ring: materials.add(StandardMaterial {
            base_color: Color::srgba(0.0, 1.0, 0.0, 0.8),
            unlit: true,
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        grid_line: materials.add(StandardMaterial {
            base_color: Color::srgba(0.3, 0.3, 0.3, 0.5),
            unlit: true,
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
    };

    commands.insert_resource(game_materials);
}

/// Sync Bevy Transform from SimPosition
fn sync_transforms(mut query: Query<(&sim::SimPosition, &mut Transform), Changed<sim::SimPosition>>) {
    for (sim_pos, mut transform) in query.iter_mut() {
        transform.translation = sim_pos_to_vec3(sim_pos);
    }
}

