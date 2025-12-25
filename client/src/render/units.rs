//! Unit and building visual rendering

use bevy::prelude::*;

use super::{GameMaterials, sim_pos_to_vec3};

/// Marker for entities that have visuals spawned
#[derive(Component)]
pub struct HasVisual;

/// Update unit visuals (spawn meshes for new units)
pub fn update_unit_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Option<Res<GameMaterials>>,
    units: Query<
        (Entity, &sim::SimPosition, &sim::Owner, &sim::Unit),
        (With<sim::SimEntity>, Without<HasVisual>),
    >,
) {
    let Some(materials) = materials else { return };

    for (entity, pos, owner, unit) in units.iter() {
        let material = match owner.player_id {
            shared::PlayerId::PLAYER_1 => materials.player1_unit.clone(),
            shared::PlayerId::PLAYER_2 => materials.player2_unit.clone(),
            _ => materials.neutral.clone(),
        };

        // Unit size based on type
        let (width, height) = match unit.unit_type {
            shared::UnitType::Villager => (0.4, 0.8),
            shared::UnitType::Soldier => (0.5, 1.0),
        };

        commands.entity(entity).insert((
            Mesh3d(meshes.add(Cuboid::new(width, height, width))),
            MeshMaterial3d(material),
            Transform::from_translation(sim_pos_to_vec3(pos) + Vec3::Y * height / 2.0),
            HasVisual,
        ));
    }
}

/// Update building visuals
pub fn update_building_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Option<Res<GameMaterials>>,
    buildings: Query<
        (Entity, &sim::SimPosition, &sim::Owner, &sim::Building),
        (With<sim::SimEntity>, Without<HasVisual>),
    >,
) {
    let Some(materials) = materials else { return };

    for (entity, pos, owner, building) in buildings.iter() {
        let (material, size, height) = match building.building_type {
            shared::BuildingType::TownCenter => {
                let mat = if owner.player_id == shared::PlayerId::PLAYER_1 {
                    materials.player1_unit.clone()
                } else if owner.player_id == shared::PlayerId::PLAYER_2 {
                    materials.player2_unit.clone()
                } else {
                    materials.town_center.clone()
                };
                (mat, 3.0, 2.5)
            }
            shared::BuildingType::Barracks => {
                let mat = if owner.player_id == shared::PlayerId::PLAYER_1 {
                    materials.player1_unit.clone()
                } else if owner.player_id == shared::PlayerId::PLAYER_2 {
                    materials.player2_unit.clone()
                } else {
                    materials.barracks.clone()
                };
                (mat, 2.5, 2.0)
            }
        };

        commands.entity(entity).insert((
            Mesh3d(meshes.add(Cuboid::new(size, height, size))),
            MeshMaterial3d(material),
            Transform::from_translation(sim_pos_to_vec3(pos) + Vec3::Y * height / 2.0),
            HasVisual,
        ));
    }
}

/// Update resource node visuals
pub fn update_resource_node_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Option<Res<GameMaterials>>,
    nodes: Query<
        (Entity, &sim::SimPosition, &sim::ResourceNode),
        (With<sim::SimEntity>, Without<HasVisual>),
    >,
) {
    let Some(materials) = materials else { return };

    for (entity, pos, node) in nodes.iter() {
        let (material, mesh, height) = match node.resource_type {
            shared::ResourceType::Food => (
                materials.food_node.clone(),
                meshes.add(Sphere::new(0.6)),
                0.6,
            ),
            shared::ResourceType::Wood => (
                materials.wood_node.clone(),
                meshes.add(Cylinder::new(0.3, 1.5)),
                0.75,
            ),
            shared::ResourceType::Gold => (
                materials.gold_node.clone(),
                meshes.add(Cuboid::new(0.8, 0.6, 0.8)),
                0.3,
            ),
            shared::ResourceType::Stone => (
                materials.stone_node.clone(),
                meshes.add(Cuboid::new(1.0, 0.5, 1.0)),
                0.25,
            ),
        };

        commands.entity(entity).insert((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(sim_pos_to_vec3(pos) + Vec3::Y * height),
            HasVisual,
        ));
    }
}

