//! Command processor system
//!
//! Processes incoming commands in deterministic order.

use bevy_ecs::prelude::*;
use shared::{EntityId, GameCommand, PlayerId, UnitType};

use crate::{
    components::*,
    world::{CommandBuffer, EntityIdGenerator, SimWorld},
    TickScheduler,
};

/// Process all commands for the current tick
pub fn process_commands(
    mut commands: Commands,
    mut buffer: ResMut<CommandBuffer>,
    mut tick_scheduler: ResMut<TickScheduler>,
    mut sim_world: ResMut<SimWorld>,
    mut id_gen: ResMut<EntityIdGenerator>,
    mut units: Query<(
        Entity,
        &SimEntity,
        &Owner,
        Option<&mut MoveTarget>,
        Option<&mut Gatherer>,
    )>,
    mut buildings: Query<(Entity, &SimEntity, &Owner, &mut ProductionQueue), With<Building>>,
    resource_nodes: Query<(Entity, &SimEntity, &ResourceNode)>,
) {
    // Advance tick
    tick_scheduler.advance();
    let current_tick = tick_scheduler.tick();

    // Drain and process commands
    let tick_commands = buffer.drain_for_tick(current_tick);

    for stamped in tick_commands {
        match stamped.command {
            GameCommand::Move {
                entities,
                target_x,
                target_z,
            } => {
                handle_move_command(
                    &mut commands,
                    &sim_world,
                    &entities,
                    stamped.player_id,
                    target_x,
                    target_z,
                    &mut units,
                );
            }

            GameCommand::Gather { entities, node } => {
                handle_gather_command(
                    &mut commands,
                    &sim_world,
                    &entities,
                    stamped.player_id,
                    node,
                    &mut units,
                    &resource_nodes,
                );
            }

            GameCommand::QueueUnit {
                building,
                unit_type,
            } => {
                handle_queue_unit_command(
                    &mut sim_world,
                    building,
                    stamped.player_id,
                    unit_type,
                    &mut buildings,
                );
            }

            GameCommand::CancelProduction {
                building,
                queue_index,
            } => {
                handle_cancel_production(
                    &mut sim_world,
                    building,
                    stamped.player_id,
                    queue_index,
                    &mut buildings,
                );
            }

            GameCommand::Stop { entities } => {
                handle_stop_command(&mut commands, &sim_world, &entities, stamped.player_id, &mut units);
            }

            GameCommand::Build { .. } => {
                // TODO: Implement building construction
            }

            GameCommand::ResearchTech { .. } => {
                // TODO: Implement tech research
            }
        }
    }
}

fn handle_move_command(
    commands: &mut Commands,
    sim_world: &SimWorld,
    entities: &[EntityId],
    player_id: PlayerId,
    target_x: f32,
    target_z: f32,
    units: &mut Query<(
        Entity,
        &SimEntity,
        &Owner,
        Option<&mut MoveTarget>,
        Option<&mut Gatherer>,
    )>,
) {
    for entity_id in entities {
        if let Some(bevy_entity) = sim_world.get_bevy_entity(*entity_id) {
            if let Ok((entity, _, owner, move_target, gatherer)) = units.get_mut(bevy_entity) {
                // Verify ownership
                if owner.player_id != player_id {
                    continue;
                }

                // Clear gatherer state if any
                if let Some(mut gatherer) = gatherer {
                    gatherer.target_node = None;
                    gatherer.state = GathererState::Idle;
                }

                // Set or update move target
                if move_target.is_some() {
                    // Update existing
                    if let Ok((_, _, _, Some(mut mt), _)) = units.get_mut(entity) {
                        mt.x = target_x;
                        mt.z = target_z;
                    }
                } else {
                    // Insert new
                    commands.entity(entity).insert(MoveTarget::new(target_x, target_z));
                }
            }
        }
    }
}

fn handle_gather_command(
    commands: &mut Commands,
    sim_world: &SimWorld,
    entities: &[EntityId],
    player_id: PlayerId,
    node_id: EntityId,
    units: &mut Query<(
        Entity,
        &SimEntity,
        &Owner,
        Option<&mut MoveTarget>,
        Option<&mut Gatherer>,
    )>,
    resource_nodes: &Query<(Entity, &SimEntity, &ResourceNode)>,
) {
    // Find the resource node
    let node_info = resource_nodes.iter().find(|(_, sim_entity, _)| sim_entity.id == node_id);

    let Some((_, _, resource_node)) = node_info else {
        return;
    };

    let resource_type = resource_node.resource_type;

    for entity_id in entities {
        if let Some(bevy_entity) = sim_world.get_bevy_entity(*entity_id) {
            if let Ok((entity, _, owner, _, gatherer)) = units.get_mut(bevy_entity) {
                // Verify ownership
                if owner.player_id != player_id {
                    continue;
                }

                // Only villagers can gather
                if let Some(mut gatherer) = gatherer {
                    gatherer.set_target(node_id, resource_type);
                } else {
                    // Not a gatherer, just move to the node
                    // (Would need node position here)
                }
            }
        }
    }
}

fn handle_queue_unit_command(
    sim_world: &mut SimWorld,
    building_id: EntityId,
    player_id: PlayerId,
    unit_type: UnitType,
    buildings: &mut Query<(Entity, &SimEntity, &Owner, &mut ProductionQueue), With<Building>>,
) {
    for (_, sim_entity, owner, mut queue) in buildings.iter_mut() {
        if sim_entity.id != building_id {
            continue;
        }

        // Verify ownership
        if owner.player_id != player_id {
            return;
        }

        // Check queue capacity
        if !queue.can_queue() {
            return;
        }

        // Get production item
        let item = match unit_type {
            UnitType::Villager => ProductionItem::villager(),
            UnitType::Soldier => ProductionItem::soldier(),
        };

        // Check if player can afford
        if let Some(player) = sim_world.get_player_mut(player_id) {
            if !player.spend(&item.cost) {
                return;
            }
        } else {
            return;
        }

        // Add to queue
        queue.queue(item);
        return;
    }
}

fn handle_cancel_production(
    sim_world: &mut SimWorld,
    building_id: EntityId,
    player_id: PlayerId,
    queue_index: usize,
    buildings: &mut Query<(Entity, &SimEntity, &Owner, &mut ProductionQueue), With<Building>>,
) {
    for (_, sim_entity, owner, mut queue) in buildings.iter_mut() {
        if sim_entity.id != building_id {
            continue;
        }

        // Verify ownership
        if owner.player_id != player_id {
            return;
        }

        // Cancel and refund
        if let Some(item) = queue.cancel(queue_index) {
            if let Some(player) = sim_world.get_player_mut(player_id) {
                player.add_resources(&item.cost);
            }
        }
        return;
    }
}

fn handle_stop_command(
    commands: &mut Commands,
    sim_world: &SimWorld,
    entities: &[EntityId],
    player_id: PlayerId,
    units: &mut Query<(
        Entity,
        &SimEntity,
        &Owner,
        Option<&mut MoveTarget>,
        Option<&mut Gatherer>,
    )>,
) {
    for entity_id in entities {
        if let Some(bevy_entity) = sim_world.get_bevy_entity(*entity_id) {
            if let Ok((entity, _, owner, _, gatherer)) = units.get_mut(bevy_entity) {
                // Verify ownership
                if owner.player_id != player_id {
                    continue;
                }

                // Clear move target
                commands.entity(entity).remove::<MoveTarget>();

                // Clear gatherer state
                if let Some(mut gatherer) = gatherer {
                    gatherer.target_node = None;
                    gatherer.state = GathererState::Idle;
                }
            }
        }
    }
}

