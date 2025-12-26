//! Unit and building visual rendering

use bevy::prelude::*;

use super::{sim_pos_to_vec3, Billboard, GameMaterials, SpriteAssets};

/// Marker for entities that have visuals spawned
#[derive(Component)]
pub struct HasVisual;

/// Resource to cache sprite materials
#[derive(Resource, Default)]
pub struct SpriteMaterials {
    pub materials: std::collections::HashMap<AssetId<Image>, Handle<StandardMaterial>>,
}

/// Get or create a material for a sprite texture
fn get_sprite_material(
    texture: &Handle<Image>,
    sprite_materials: &mut SpriteMaterials,
    materials: &mut Assets<StandardMaterial>,
) -> Handle<StandardMaterial> {
    let id = texture.id();
    if let Some(mat) = sprite_materials.materials.get(&id) {
        return mat.clone();
    }

    let mat = materials.add(StandardMaterial {
        base_color_texture: Some(texture.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,     // Don't apply lighting to sprites
        cull_mode: None, // Show both sides
        ..default()
    });
    sprite_materials.materials.insert(id, mat.clone());
    mat
}

/// Update unit visuals (spawn meshes for new units)
pub fn update_unit_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mat_assets: ResMut<Assets<StandardMaterial>>,
    mut sprite_materials: ResMut<SpriteMaterials>,
    game_materials: Option<Res<GameMaterials>>,
    sprite_assets: Option<Res<SpriteAssets>>,
    units: Query<
        (Entity, &sim::SimPosition, &sim::Owner, &sim::Unit),
        (With<sim::SimEntity>, Without<HasVisual>),
    >,
) {
    let Some(game_materials) = game_materials else {
        return;
    };

    for (entity, pos, owner, unit) in units.iter() {
        // Check if we have a sprite for this unit type
        let sprite_handle = sprite_assets
            .as_ref()
            .and_then(|assets| match unit.unit_type {
                shared::UnitType::Villager => assets.villager.clone(),
                shared::UnitType::Soldier => assets.soldier.clone(),
            });

        if let Some(texture) = sprite_handle {
            // Use textured quad billboard
            let size = match unit.unit_type {
                shared::UnitType::Villager => 1.5,
                shared::UnitType::Soldier => 1.8,
            };

            let material = get_sprite_material(&texture, &mut sprite_materials, &mut mat_assets);
            let quad = meshes.add(Rectangle::new(size, size));

            commands.entity(entity).insert((
                Mesh3d(quad),
                MeshMaterial3d(material),
                Transform::from_translation(sim_pos_to_vec3(pos) + Vec3::Y * size / 2.0),
                Billboard,
                HasVisual,
            ));
        } else {
            // Fallback to colored cube
            let material = match owner.player_id {
                shared::PlayerId::PLAYER_1 => game_materials.player1_unit.clone(),
                shared::PlayerId::PLAYER_2 => game_materials.player2_unit.clone(),
                _ => game_materials.neutral.clone(),
            };

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
}

/// Update building visuals using sprites when available
pub fn update_building_visuals_sprite(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mat_assets: ResMut<Assets<StandardMaterial>>,
    mut sprite_materials: ResMut<SpriteMaterials>,
    game_materials: Option<Res<GameMaterials>>,
    sprite_assets: Option<Res<SpriteAssets>>,
    buildings: Query<
        (Entity, &sim::SimPosition, &sim::Owner, &sim::Building),
        (With<sim::SimEntity>, Without<HasVisual>),
    >,
) {
    let Some(game_materials) = game_materials else {
        return;
    };

    for (entity, pos, owner, building) in buildings.iter() {
        // Check if we have a sprite for this building type
        let sprite_handle =
            sprite_assets
                .as_ref()
                .and_then(|assets| match building.building_type {
                    shared::BuildingType::TownCenter => assets.town_center.clone(),
                    shared::BuildingType::Barracks => assets.barracks.clone(),
                });

        if let Some(texture) = sprite_handle {
            // Use textured quad billboard
            let size = match building.building_type {
                shared::BuildingType::TownCenter => 5.0,
                shared::BuildingType::Barracks => 4.0,
            };

            let material = get_sprite_material(&texture, &mut sprite_materials, &mut mat_assets);
            let quad = meshes.add(Rectangle::new(size, size));

            commands.entity(entity).insert((
                Mesh3d(quad),
                MeshMaterial3d(material),
                Transform::from_translation(sim_pos_to_vec3(pos) + Vec3::Y * size / 2.0),
                Billboard,
                HasVisual,
            ));
        } else {
            // Fallback to colored cube
            let (material, size, height) = match building.building_type {
                shared::BuildingType::TownCenter => {
                    let mat = if owner.player_id == shared::PlayerId::PLAYER_1 {
                        game_materials.player1_unit.clone()
                    } else if owner.player_id == shared::PlayerId::PLAYER_2 {
                        game_materials.player2_unit.clone()
                    } else {
                        game_materials.town_center.clone()
                    };
                    (mat, 3.0, 2.5)
                }
                shared::BuildingType::Barracks => {
                    let mat = if owner.player_id == shared::PlayerId::PLAYER_1 {
                        game_materials.player1_unit.clone()
                    } else if owner.player_id == shared::PlayerId::PLAYER_2 {
                        game_materials.player2_unit.clone()
                    } else {
                        game_materials.barracks.clone()
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
}

/// Update resource node visuals
pub fn update_resource_node_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mat_assets: ResMut<Assets<StandardMaterial>>,
    mut sprite_materials: ResMut<SpriteMaterials>,
    game_materials: Option<Res<GameMaterials>>,
    sprite_assets: Option<Res<SpriteAssets>>,
    nodes: Query<
        (Entity, &sim::SimPosition, &sim::ResourceNode),
        (With<sim::SimEntity>, Without<HasVisual>),
    >,
) {
    let Some(game_materials) = game_materials else {
        return;
    };

    for (entity, pos, node) in nodes.iter() {
        // Check if we have a sprite for this resource type
        let sprite_handle = sprite_assets
            .as_ref()
            .and_then(|assets| match node.resource_type {
                shared::ResourceType::Food => assets.berry_bush.clone(),
                shared::ResourceType::Wood => assets.tree.clone(),
                shared::ResourceType::Gold => assets.gold_mine.clone(),
                shared::ResourceType::Stone => assets.stone_quarry.clone(),
            });

        if let Some(texture) = sprite_handle {
            // Use textured quad billboard
            let (width, height) = match node.resource_type {
                shared::ResourceType::Food => (1.5, 1.5),
                shared::ResourceType::Wood => (2.5, 3.5),
                shared::ResourceType::Gold => (2.0, 2.0),
                shared::ResourceType::Stone => (2.0, 1.5),
            };

            let material = get_sprite_material(&texture, &mut sprite_materials, &mut mat_assets);
            let quad = meshes.add(Rectangle::new(width, height));

            commands.entity(entity).insert((
                Mesh3d(quad),
                MeshMaterial3d(material),
                Transform::from_translation(sim_pos_to_vec3(pos) + Vec3::Y * height / 2.0),
                Billboard,
                HasVisual,
            ));
        } else {
            // Fallback to colored shapes
            let (material, mesh, height) = match node.resource_type {
                shared::ResourceType::Food => (
                    game_materials.food_node.clone(),
                    meshes.add(Sphere::new(0.6)),
                    0.6,
                ),
                shared::ResourceType::Wood => (
                    game_materials.wood_node.clone(),
                    meshes.add(Cylinder::new(0.3, 1.5)),
                    0.75,
                ),
                shared::ResourceType::Gold => (
                    game_materials.gold_node.clone(),
                    meshes.add(Cuboid::new(0.8, 0.6, 0.8)),
                    0.3,
                ),
                shared::ResourceType::Stone => (
                    game_materials.stone_node.clone(),
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
}
