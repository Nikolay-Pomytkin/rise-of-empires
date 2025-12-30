//! Building placement ghost rendering (2D)

use bevy::prelude::*;

use super::{layers, TILE_SIZE};
use crate::input::{BuildingPlacementState, InputState};

/// Marker component for placement ghost
#[derive(Component)]
pub struct PlacementGhost;

/// Update placement ghost position and visibility
pub fn update_placement_ghost(
    mut commands: Commands,
    placement: Res<BuildingPlacementState>,
    input_state: Res<InputState>,
    mut ghost: Query<(Entity, &mut Transform, &mut Sprite), With<PlacementGhost>>,
) {
    match (placement.placing, ghost.single_mut()) {
        // Has placement, has ghost -> update position
        (Some(_), Ok((_, mut transform, _))) => {
            if let Some(pos) = input_state.mouse_world_pos {
                // Snap to tile grid (in 2D, pos.x and pos.y are the ground plane)
                let tile_x = (pos.x / TILE_SIZE).round() * TILE_SIZE;
                let tile_y = (pos.y / TILE_SIZE).round() * TILE_SIZE;
                transform.translation = Vec3::new(tile_x, tile_y, layers::PLACEMENT_GHOST);
            }
        }
        // Has placement, no ghost -> spawn ghost
        (Some(building_type), Err(_)) => {
            let size = match building_type {
                shared::BuildingType::TownCenter => TILE_SIZE * 4.0,
                shared::BuildingType::Barracks => TILE_SIZE * 3.0,
            };

            let pos = input_state.mouse_world_pos.unwrap_or(Vec3::ZERO);
            let tile_x = (pos.x / TILE_SIZE).round() * TILE_SIZE;
            let tile_y = (pos.y / TILE_SIZE).round() * TILE_SIZE;

            commands.spawn((
                Sprite {
                    color: Color::srgba(0.2, 0.8, 0.2, 0.5), // Semi-transparent green
                    custom_size: Some(Vec2::splat(size)),
                    ..default()
                },
                Transform::from_xyz(tile_x, tile_y, layers::PLACEMENT_GHOST),
                PlacementGhost,
            ));
        }
        // No placement, has ghost -> despawn
        (None, Ok((entity, _, _))) => {
            commands.entity(entity).despawn();
        }
        // No placement, no ghost -> nothing
        (None, Err(_)) => {}
    }
}
