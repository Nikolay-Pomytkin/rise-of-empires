//! Unit and building visual rendering (2D sprites)

use bevy::prelude::*;

use super::{layers, sim_pos_to_vec3, SpriteAssets, TILE_SIZE};

/// Marker for entities that have visuals spawned
#[derive(Component)]
pub struct HasVisual;

/// Resource to cache sprite materials (not needed for 2D, but kept for compatibility)
#[derive(Resource, Default)]
pub struct SpriteMaterials {
    // In 2D we use Sprite directly, no materials needed
}

/// Player colors for units/buildings
fn player_color(player_id: shared::PlayerId) -> Color {
    match player_id {
        shared::PlayerId::PLAYER_1 => Color::srgb(0.2, 0.5, 0.9), // Blue
        shared::PlayerId::PLAYER_2 => Color::srgb(0.9, 0.3, 0.2), // Red
        _ => Color::srgb(0.5, 0.5, 0.5),                          // Gray
    }
}

/// Update unit visuals (spawn sprites for new units)
pub fn update_unit_visuals(
    mut commands: Commands,
    sprite_assets: Option<Res<SpriteAssets>>,
    units: Query<
        (Entity, &sim::SimPosition, &sim::Owner, &sim::Unit),
        (With<sim::SimEntity>, Without<HasVisual>),
    >,
) {
    let count = units.iter().count();
    if count > 0 {
        bevy::log::info!("update_unit_visuals: found {} units without visuals", count);
    }
    
    for (entity, pos, owner, unit) in units.iter() {
        // Check if we have a sprite for this unit type
        let sprite_handle = sprite_assets
            .as_ref()
            .and_then(|assets| match unit.unit_type {
                shared::UnitType::Villager => assets.villager.clone(),
                shared::UnitType::Soldier => assets.soldier.clone(),
            });

        let world_pos = sim_pos_to_vec3(pos, layers::UNITS_BASE);
        
        // Size in pixels
        let size = match unit.unit_type {
            shared::UnitType::Villager => TILE_SIZE * 1.5,
            shared::UnitType::Soldier => TILE_SIZE * 1.8,
        };

        bevy::log::info!("Spawning unit {:?} at {:?} (Z={})", unit.unit_type, world_pos, world_pos.z);

        if let Some(texture) = sprite_handle {
            commands.entity(entity).insert((
                Sprite {
                    image: texture,
                    custom_size: Some(Vec2::splat(size)),
                    ..default()
                },
                Transform::from_translation(world_pos),
                HasVisual,
            ));
        } else {
            // Fallback to colored square
            commands.entity(entity).insert((
                Sprite {
                    color: player_color(owner.player_id),
                    custom_size: Some(Vec2::new(size * 0.5, size)),
                    ..default()
                },
                Transform::from_translation(world_pos),
                HasVisual,
            ));
        }
    }
}

/// Update building visuals using sprites
pub fn update_building_visuals_sprite(
    mut commands: Commands,
    sprite_assets: Option<Res<SpriteAssets>>,
    buildings: Query<
        (Entity, &sim::SimPosition, &sim::Owner, &sim::Building),
        (With<sim::SimEntity>, Without<HasVisual>),
    >,
) {
    let count = buildings.iter().count();
    if count > 0 {
        bevy::log::info!("update_building_visuals_sprite: found {} buildings without visuals", count);
    }
    
    for (entity, pos, owner, building) in buildings.iter() {
        // Check if we have a sprite for this building type
        let sprite_handle =
            sprite_assets
                .as_ref()
                .and_then(|assets| match building.building_type {
                    shared::BuildingType::TownCenter => assets.town_center.clone(),
                    shared::BuildingType::Barracks => assets.barracks.clone(),
                });

        let world_pos = sim_pos_to_vec3(pos, layers::BUILDINGS);

        // Size in pixels
        let size = match building.building_type {
            shared::BuildingType::TownCenter => TILE_SIZE * 4.0,
            shared::BuildingType::Barracks => TILE_SIZE * 3.0,
        };

        bevy::log::info!("Spawning building {:?} at {:?} (Z={})", building.building_type, world_pos, world_pos.z);

        if let Some(texture) = sprite_handle {
            commands.entity(entity).insert((
                Sprite {
                    image: texture,
                    custom_size: Some(Vec2::splat(size)),
                    ..default()
                },
                Transform::from_translation(world_pos),
                HasVisual,
            ));
        } else {
            // Fallback to colored square with player tint
            let color = if owner.player_id == shared::PlayerId::PLAYER_1 {
                Color::srgb(0.3, 0.5, 0.8) // Blue
            } else if owner.player_id == shared::PlayerId::PLAYER_2 {
                Color::srgb(0.8, 0.4, 0.3) // Red
            } else {
                Color::srgb(0.8, 0.7, 0.5) // Neutral
            };

            commands.entity(entity).insert((
                Sprite {
                    color,
                    custom_size: Some(Vec2::splat(size)),
                    ..default()
                },
                Transform::from_translation(world_pos),
                HasVisual,
            ));
        }
    }
}

/// Update resource node visuals
pub fn update_resource_node_visuals(
    mut commands: Commands,
    sprite_assets: Option<Res<SpriteAssets>>,
    nodes: Query<
        (Entity, &sim::SimPosition, &sim::ResourceNode),
        (With<sim::SimEntity>, Without<HasVisual>),
    >,
) {
    let count = nodes.iter().count();
    if count > 0 {
        bevy::log::info!("update_resource_node_visuals: found {} resource nodes without visuals", count);
    }
    
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

        let world_pos = sim_pos_to_vec3(pos, layers::RESOURCES);

        // Size in pixels
        let (width, height) = match node.resource_type {
            shared::ResourceType::Food => (TILE_SIZE * 1.5, TILE_SIZE * 1.5),
            shared::ResourceType::Wood => (TILE_SIZE * 2.0, TILE_SIZE * 3.0),
            shared::ResourceType::Gold => (TILE_SIZE * 2.0, TILE_SIZE * 2.0),
            shared::ResourceType::Stone => (TILE_SIZE * 2.0, TILE_SIZE * 1.5),
        };

        bevy::log::info!("Spawning resource {:?} at {:?} (Z={})", node.resource_type, world_pos, world_pos.z);

        if let Some(texture) = sprite_handle {
            commands.entity(entity).insert((
                Sprite {
                    image: texture,
                    custom_size: Some(Vec2::new(width, height)),
                    ..default()
                },
                Transform::from_translation(world_pos),
                HasVisual,
            ));
        } else {
            // Fallback to colored shapes
            let color = match node.resource_type {
                shared::ResourceType::Food => Color::srgb(0.2, 0.8, 0.3),  // Green
                shared::ResourceType::Wood => Color::srgb(0.4, 0.25, 0.1), // Brown
                shared::ResourceType::Gold => Color::srgb(1.0, 0.85, 0.0), // Gold
                shared::ResourceType::Stone => Color::srgb(0.6, 0.6, 0.65), // Gray
            };

            commands.entity(entity).insert((
                Sprite {
                    color,
                    custom_size: Some(Vec2::new(width, height)),
                    ..default()
                },
                Transform::from_translation(world_pos),
                HasVisual,
            ));
        }
    }
}
