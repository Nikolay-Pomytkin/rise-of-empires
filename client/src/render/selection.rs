//! Selection visual rendering

use bevy::prelude::*;

use super::GameMaterials;
use crate::input::SelectionState;

/// Marker for selection ring entity
#[derive(Component)]
pub struct SelectionRing {
    pub parent: Entity,
}

/// Update selection visuals
pub fn update_selection_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Option<Res<GameMaterials>>,
    selection_state: Res<SelectionState>,
    mut existing_rings: Query<(Entity, &SelectionRing, &mut Transform)>,
    positions: Query<&sim::SimPosition>,
) {
    let Some(materials) = materials else { return };

    // Track which rings to remove and which parents already have rings
    let mut rings_to_remove = Vec::new();
    let mut existing_parents = Vec::new();

    // Update existing rings or mark for removal
    for (ring_entity, ring, mut transform) in existing_rings.iter_mut() {
        if !selection_state.selected.contains(&ring.parent) {
            rings_to_remove.push(ring_entity);
        } else if let Ok(pos) = positions.get(ring.parent) {
            // Update position
            transform.translation = Vec3::new(pos.x, 0.05, pos.z);
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

        // Spawn selection ring
        commands.spawn((
            Mesh3d(meshes.add(Torus::new(0.4, 0.5))),
            MeshMaterial3d(materials.selection_ring.clone()),
            Transform::from_translation(Vec3::new(pos.x, 0.05, pos.z))
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            SelectionRing {
                parent: selected_entity,
            },
        ));
    }
}
