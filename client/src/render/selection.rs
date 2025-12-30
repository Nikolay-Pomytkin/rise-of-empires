//! Selection visual rendering (2D)

use bevy::prelude::*;

use super::{layers, TILE_SIZE};
use crate::input::SelectionState;

/// Marker for selection ring entity
#[derive(Component)]
pub struct SelectionRing {
    pub parent: Entity,
}

/// Update selection visuals
pub fn update_selection_visuals(
    mut commands: Commands,
    selection_state: Res<SelectionState>,
    mut existing_rings: Query<(Entity, &SelectionRing, &mut Transform)>,
    positions: Query<&sim::SimPosition>,
) {
    // Track which rings to remove and which parents already have rings
    let mut rings_to_remove = Vec::new();
    let mut existing_parents = Vec::new();

    // Update existing rings or mark for removal
    for (ring_entity, ring, mut transform) in existing_rings.iter_mut() {
        if !selection_state.selected.contains(&ring.parent) {
            rings_to_remove.push(ring_entity);
        } else if let Ok(pos) = positions.get(ring.parent) {
            // Update position (2D: X stays X, Z becomes Y)
            transform.translation.x = pos.x * TILE_SIZE;
            transform.translation.y = pos.z * TILE_SIZE;
            existing_parents.push(ring.parent);
        } else {
            // Parent entity no longer exists
            rings_to_remove.push(ring_entity);
        }
    }

    // Remove rings
    for ring_entity in rings_to_remove {
        commands.entity(ring_entity).despawn();
    }

    // Add rings for newly selected entities
    for &selected_entity in &selection_state.selected {
        if existing_parents.contains(&selected_entity) {
            continue;
        }

        // Get position of selected entity
        let Ok(pos) = positions.get(selected_entity) else {
            continue;
        };

        let world_x = pos.x * TILE_SIZE;
        let world_y = pos.z * TILE_SIZE;

        // Spawn selection ring at selection layer
        commands.spawn((
            Sprite {
                color: Color::srgba(0.0, 1.0, 0.0, 0.8),
                custom_size: Some(Vec2::splat(TILE_SIZE * 1.2)),
                ..default()
            },
            Transform::from_xyz(world_x, world_y, layers::SELECTION),
            SelectionRing {
                parent: selected_entity,
            },
        ));
    }
}
