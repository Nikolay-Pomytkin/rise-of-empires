//! Production system
//!
//! Handles unit production from buildings.

use bevy_ecs::prelude::*;
use shared::UnitType;

use crate::{
    components::*,
    world::{EntityIdGenerator, SimWorld},
};

/// Production system - ticks queues and spawns completed units
pub fn production_system(
    mut commands: Commands,
    mut sim_world: ResMut<SimWorld>,
    mut id_gen: ResMut<EntityIdGenerator>,
    mut buildings: Query<(
        Entity,
        &SimEntity,
        &Owner,
        &SimPosition,
        &mut ProductionQueue,
        Option<&SpawnPoint>,
    )>,
) {
    let mut spawns: Vec<(PlayerId, UnitType, f32, f32)> = Vec::new();

    for (_, _, owner, pos, mut queue, spawn_point) in buildings.iter_mut() {
        if queue.is_empty() {
            continue;
        }

        // Tick the current item
        let completed = {
            if let Some(item) = queue.current_mut() {
                item.tick()
            } else {
                false
            }
        };

        if completed {
            // Remove completed item and spawn unit
            if let Some(item) = queue.items.first() {
                let unit_type = item.unit_type;

                // Calculate spawn position
                let spawn_offset = spawn_point
                    .map(|sp| (sp.offset_x, sp.offset_z))
                    .unwrap_or((0.0, 2.5));
                let spawn_x = pos.x + spawn_offset.0;
                let spawn_z = pos.z + spawn_offset.1;

                spawns.push((owner.player_id, unit_type, spawn_x, spawn_z));
            }

            // Remove the completed item
            queue.items.remove(0);
        }
    }

    // Spawn completed units
    for (player_id, unit_type, spawn_x, spawn_z) in spawns {
        spawn_unit(
            &mut commands,
            &mut sim_world,
            &mut id_gen,
            player_id,
            unit_type,
            spawn_x,
            spawn_z,
        );
    }
}

use shared::PlayerId;

fn spawn_unit(
    commands: &mut Commands,
    sim_world: &mut SimWorld,
    id_gen: &mut EntityIdGenerator,
    player_id: PlayerId,
    unit_type: UnitType,
    x: f32,
    z: f32,
) {
    let sim_id = id_gen.next();

    let mut entity_commands = commands.spawn((
        SimEntity::new(sim_id),
        SimPosition::new(x, z),
        Owner::new(player_id),
        Selected::default(),
        Velocity::zero(),
    ));

    match unit_type {
        UnitType::Villager => {
            entity_commands.insert((
                Unit::villager(),
                Villager,
                Gatherer::new(),
                Health::new(25),
                CombatStats::villager(),
            ));
        }
        UnitType::Soldier => {
            entity_commands.insert((
                Unit::soldier(),
                Soldier,
                Health::new(40),
                CombatStats::soldier(),
            ));
        }
    }

    let bevy_entity = entity_commands.id();
    sim_world.register_entity(sim_id, bevy_entity);

    // Update player population
    if let Some(player) = sim_world.get_player_mut(player_id) {
        player.population += unit_type.population_cost();
    }
}
