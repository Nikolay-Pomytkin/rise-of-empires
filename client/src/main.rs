//! Rise RTS Client
//!
//! Bevy application for rendering, input, and UI.

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod bridge;
mod camera;
mod game_state;
mod input;
mod render;
mod save_load;
mod ui;

use bridge::BridgePlugin;
use camera::CameraPlugin;
use game_state::{GameState, GameStatePlugin};
use input::InputPlugin;
use render::RenderPlugin;
use save_load::SaveLoadPlugin;
use ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Rise RTS".to_string(),
                        resolution: bevy::window::WindowResolution::new(1280, 720),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    // Enable hot reloading
                    watch_for_changes_override: Some(true),
                    ..default()
                }),
        )
        .add_plugins(EguiPlugin::default())
        .add_plugins(GameStatePlugin)
        .add_plugins(sim::SimPlugin::default())
        .add_plugins((
            CameraPlugin,
            InputPlugin,
            RenderPlugin,
            UiPlugin,
            BridgePlugin,
            SaveLoadPlugin,
        ))
        .add_systems(OnEnter(GameState::InGame), setup_game)
        .run();
}

/// Initial game setup - spawn starting entities
fn setup_game(
    mut commands: Commands,
    mut sim_world: ResMut<sim::SimWorld>,
    mut id_gen: ResMut<sim::EntityIdGenerator>,
) {
    // Clear any existing state
    // TODO: Proper cleanup when returning to main menu

    // Add players
    sim_world.add_player(shared::PlayerId::PLAYER_1);
    sim_world.add_player(shared::PlayerId::PLAYER_2);

    // Spawn Player 1's Town Center
    spawn_town_center(
        &mut commands,
        &mut sim_world,
        &mut id_gen,
        shared::PlayerId::PLAYER_1,
        0.0,
        0.0,
    );

    // Spawn Player 1's starting villagers
    for i in 0..3 {
        spawn_villager(
            &mut commands,
            &mut sim_world,
            &mut id_gen,
            shared::PlayerId::PLAYER_1,
            -3.0 + i as f32 * 2.0,
            4.0,
        );
    }

    // Spawn some resource nodes
    // Food (berries)
    spawn_resource_node(
        &mut commands,
        &mut sim_world,
        &mut id_gen,
        shared::ResourceType::Food,
        8.0,
        0.0,
    );

    // Wood (trees)
    for i in 0..5 {
        spawn_resource_node(
            &mut commands,
            &mut sim_world,
            &mut id_gen,
            shared::ResourceType::Wood,
            -10.0 + i as f32 * 1.5,
            -8.0,
        );
    }

    // Gold mine
    spawn_resource_node(
        &mut commands,
        &mut sim_world,
        &mut id_gen,
        shared::ResourceType::Gold,
        -12.0,
        5.0,
    );

    // Stone quarry
    spawn_resource_node(
        &mut commands,
        &mut sim_world,
        &mut id_gen,
        shared::ResourceType::Stone,
        12.0,
        8.0,
    );

    info!("Game setup complete!");
}

fn spawn_town_center(
    commands: &mut Commands,
    sim_world: &mut sim::SimWorld,
    id_gen: &mut sim::EntityIdGenerator,
    player_id: shared::PlayerId,
    x: f32,
    z: f32,
) {
    let sim_id = id_gen.next();

    let entity = commands
        .spawn((
            sim::SimEntity::new(sim_id),
            sim::SimPosition::new(x, z),
            sim::Owner::new(player_id),
            sim::Building::town_center(),
            sim::TownCenter,
            sim::DropOffPoint,
            sim::ProductionQueue::new(5),
            sim::SpawnPoint::default(),
            sim::Health::new(2400),
            sim::Selected::default(),
        ))
        .id();

    sim_world.register_entity(sim_id, entity);

    // Town center provides population cap
    if let Some(player) = sim_world.get_player_mut(player_id) {
        player.population_cap += 5;
    }
}

fn spawn_villager(
    commands: &mut Commands,
    sim_world: &mut sim::SimWorld,
    id_gen: &mut sim::EntityIdGenerator,
    player_id: shared::PlayerId,
    x: f32,
    z: f32,
) {
    let sim_id = id_gen.next();

    let entity = commands
        .spawn((
            sim::SimEntity::new(sim_id),
            sim::SimPosition::new(x, z),
            sim::Owner::new(player_id),
            sim::Unit::villager(),
            sim::Villager,
            sim::Gatherer::new(),
            sim::Health::new(25),
            sim::CombatStats::villager(),
            sim::Selected::default(),
            sim::Velocity::zero(),
        ))
        .id();

    sim_world.register_entity(sim_id, entity);

    // Update population
    if let Some(player) = sim_world.get_player_mut(player_id) {
        player.population += 1;
    }
}

fn spawn_resource_node(
    commands: &mut Commands,
    sim_world: &mut sim::SimWorld,
    id_gen: &mut sim::EntityIdGenerator,
    resource_type: shared::ResourceType,
    x: f32,
    z: f32,
) {
    let sim_id = id_gen.next();

    let node = match resource_type {
        shared::ResourceType::Food => sim::ResourceNode::food(),
        shared::ResourceType::Wood => sim::ResourceNode::wood(),
        shared::ResourceType::Gold => sim::ResourceNode::gold(),
        shared::ResourceType::Stone => sim::ResourceNode::stone(),
    };

    let entity = commands
        .spawn((
            sim::SimEntity::new(sim_id),
            sim::SimPosition::new(x, z),
            sim::Owner::neutral(),
            node,
        ))
        .id();

    sim_world.register_entity(sim_id, entity);
}
