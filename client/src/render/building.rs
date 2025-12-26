//! Building placement ghost rendering

use bevy::prelude::*;

use super::GameMaterials;
use crate::input::{BuildingPlacementState, InputState};

/// Marker component for placement ghost
#[derive(Component)]
pub struct PlacementGhost;

/// Update placement ghost position and visibility
pub fn update_placement_ghost(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    placement: Res<BuildingPlacementState>,
    input_state: Res<InputState>,
    mut ghost: Query<(Entity, &mut Transform), With<PlacementGhost>>,
) {
    match (placement.placing, ghost.single_mut()) {
        // Has placement, has ghost -> update position
        (Some(_), Ok((_, mut transform))) => {
            if let Some(pos) = input_state.mouse_world_pos {
                // Snap to tile grid
                let tile_x = pos.x.round();
                let tile_z = pos.z.round();
                transform.translation = Vec3::new(tile_x, 1.5, tile_z);
            }
        }
        // Has placement, no ghost -> spawn ghost
        (Some(building_type), Err(_)) => {
            let size = match building_type {
                shared::BuildingType::TownCenter => 3.0,
                shared::BuildingType::Barracks => 2.5,
            };

            let pos = input_state.mouse_world_pos.unwrap_or(Vec3::ZERO);
            let tile_x = pos.x.round();
            let tile_z = pos.z.round();

            // Semi-transparent ghost material
            let ghost_material = materials.add(StandardMaterial {
                base_color: Color::srgba(0.2, 0.8, 0.2, 0.5),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            });

            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(size, 2.0, size))),
                MeshMaterial3d(ghost_material),
                Transform::from_translation(Vec3::new(tile_x, 1.0, tile_z)),
                PlacementGhost,
            ));
        }
        // No placement, has ghost -> despawn
        (None, Ok((entity, _))) => {
            commands.entity(entity).despawn();
        }
        // No placement, no ghost -> nothing
        (None, Err(_)) => {}
    }
}
