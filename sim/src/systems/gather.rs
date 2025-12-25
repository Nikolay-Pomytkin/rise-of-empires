//! Gather system
//!
//! Handles villager resource gathering:
//! - Move to resource node
//! - Harvest resources per tick
//! - Return to drop-off when full
//! - Deposit resources

use bevy_ecs::prelude::*;

use crate::{
    components::*,
    data::PlayerModifiers,
    world::SimWorld,
};

/// Gather system - handles the full gather cycle
pub fn gather_system(
    mut commands: Commands,
    mut sim_world: ResMut<SimWorld>,
    player_modifiers: Res<PlayerModifiers>,
    mut gatherers: Query<(
        Entity,
        &SimEntity,
        &Owner,
        &SimPosition,
        &mut Gatherer,
        &Unit,
    ), Without<ResourceNode>>,
    mut resource_nodes: Query<(Entity, &SimEntity, &SimPosition, &mut ResourceNode)>,
    drop_offs: Query<(Entity, &SimEntity, &SimPosition, &Owner), (With<DropOffPoint>, Without<Gatherer>)>,
) {
    // Collect gatherer updates to avoid borrow conflicts
    let mut gatherer_updates: Vec<(Entity, GathererUpdate)> = Vec::new();

    for (entity, sim_entity, owner, pos, gatherer, unit) in gatherers.iter() {
        // Get player modifiers for gather rate
        let modifiers = player_modifiers.get(owner.player_id);
        
        let update = process_gatherer(
            entity,
            sim_entity,
            owner,
            &pos,
            &gatherer,
            unit,
            &modifiers,
            &resource_nodes,
            &drop_offs,
        );
        if let Some(update) = update {
            gatherer_updates.push((entity, update));
        }
    }

    // Apply updates
    for (entity, update) in gatherer_updates {
        if let Ok((_, _sim_entity, owner, _pos, mut gatherer, _)) = gatherers.get_mut(entity) {
            let modifiers = player_modifiers.get(owner.player_id);
            
            match update {
                GathererUpdate::SetState(state) => {
                    gatherer.state = state;
                }
                GathererUpdate::MoveTo(x, z) => {
                    commands.entity(entity).insert(MoveTarget::new(x, z));
                }
                GathererUpdate::Harvest(base_amount, node_entity, resource_type) => {
                    // Apply gather rate bonus from techs
                    let multiplier = modifiers.gather_rate_multiplier(resource_type);
                    let amount = ((base_amount as f32) * multiplier).ceil() as u32;
                    
                    // Actually harvest from the node
                    if let Ok((_, _, _, mut node)) = resource_nodes.get_mut(node_entity) {
                        let harvested = node.harvest(amount);
                        gatherer.add_resource(harvested);
                        
                        // Check if full using modified carry capacity
                        let effective_capacity = modifiers.effective_carry_capacity(gatherer.carry_capacity);
                        if gatherer.carry_amount >= effective_capacity {
                            gatherer.state = GathererState::ReturningToDropOff;
                            node.current_gatherers = node.current_gatherers.saturating_sub(1);
                        }
                    }
                }
                GathererUpdate::Deposit => {
                    let (resource_type, amount) = gatherer.take_all();
                    if let Some(player) = sim_world.get_player_mut(owner.player_id) {
                        let mut bundle = shared::ResourceBundle::ZERO;
                        bundle.set(resource_type, amount);
                        player.add_resources(&bundle);
                    }
                    
                    // Go back to gathering if we have a target
                    if gatherer.target_node.is_some() {
                        gatherer.state = GathererState::MovingToNode;
                    } else {
                        gatherer.state = GathererState::Idle;
                    }
                }
                GathererUpdate::FindDropOff(drop_off_id) => {
                    gatherer.drop_off_target = Some(drop_off_id);
                }
                GathererUpdate::ClearTarget => {
                    gatherer.target_node = None;
                    gatherer.state = GathererState::Idle;
                }
            }
        }
    }
}

enum GathererUpdate {
    SetState(GathererState),
    MoveTo(f32, f32),
    Harvest(u32, Entity, shared::ResourceType), // base amount, node entity, resource type
    Deposit,
    #[allow(dead_code)]
    FindDropOff(shared::EntityId),
    ClearTarget,
}

fn process_gatherer(
    _entity: Entity,
    _sim_entity: &SimEntity,
    owner: &Owner,
    pos: &SimPosition,
    gatherer: &Gatherer,
    _unit: &Unit,
    modifiers: &crate::data::Modifiers,
    resource_nodes: &Query<(Entity, &SimEntity, &SimPosition, &mut ResourceNode)>,
    drop_offs: &Query<(Entity, &SimEntity, &SimPosition, &Owner), (With<DropOffPoint>, Without<Gatherer>)>,
) -> Option<GathererUpdate> {
    match gatherer.state {
        GathererState::Idle => {
            // If carrying resources, find a drop-off
            if gatherer.is_carrying() {
                return Some(GathererUpdate::SetState(GathererState::ReturningToDropOff));
            }
            None
        }

        GathererState::MovingToNode => {
            let Some(target_id) = gatherer.target_node else {
                return Some(GathererUpdate::SetState(GathererState::Idle));
            };

            // Find the target node
            let node_info = resource_nodes
                .iter()
                .find(|(_, sim_e, _, _)| sim_e.id == target_id);

            let Some((_node_entity, _, node_pos, node)) = node_info else {
                return Some(GathererUpdate::ClearTarget);
            };

            // Check if node is depleted
            if node.is_depleted() {
                return Some(GathererUpdate::ClearTarget);
            }

            // Check distance
            let distance = pos.distance_xz(node_pos);
            const GATHER_RANGE: f32 = 1.5;

            if distance <= GATHER_RANGE {
                return Some(GathererUpdate::SetState(GathererState::Gathering));
            }

            // Keep moving (MoveTarget should be set by command processor)
            None
        }

        GathererState::Gathering => {
            let Some(target_id) = gatherer.target_node else {
                return Some(GathererUpdate::SetState(GathererState::Idle));
            };

            // Find the target node
            let node_info = resource_nodes
                .iter()
                .find(|(_, sim_e, _, _)| sim_e.id == target_id);

            let Some((node_entity, _, _node_pos, node)) = node_info else {
                return Some(GathererUpdate::ClearTarget);
            };

            // Check if node is depleted
            if node.is_depleted() {
                return Some(GathererUpdate::ClearTarget);
            }

            // Check if full (using modified capacity)
            let effective_capacity = modifiers.effective_carry_capacity(gatherer.carry_capacity);
            if gatherer.carry_amount >= effective_capacity {
                return Some(GathererUpdate::SetState(GathererState::ReturningToDropOff));
            }

            // Harvest - base rate is 1 per tick, modified by tech bonuses
            Some(GathererUpdate::Harvest(1, node_entity, node.resource_type))
        }

        GathererState::ReturningToDropOff => {
            // Find nearest drop-off for this player
            let mut nearest_drop_off: Option<(shared::EntityId, f32, f32, f32)> = None;
            let mut nearest_distance = f32::MAX;

            for (_, drop_sim, drop_pos, drop_owner) in drop_offs.iter() {
                if drop_owner.player_id != owner.player_id {
                    continue;
                }

                let distance = pos.distance_xz(drop_pos);
                if distance < nearest_distance {
                    nearest_distance = distance;
                    nearest_drop_off = Some((drop_sim.id, drop_pos.x, drop_pos.z, distance));
                }
            }

            let Some((_drop_id, drop_x, drop_z, distance)) = nearest_drop_off else {
                // No drop-off found, stay idle
                return Some(GathererUpdate::SetState(GathererState::Idle));
            };

            const DROP_OFF_RANGE: f32 = 2.0;

            if distance <= DROP_OFF_RANGE {
                return Some(GathererUpdate::SetState(GathererState::Depositing));
            }

            // Move to drop-off
            Some(GathererUpdate::MoveTo(drop_x, drop_z))
        }

        GathererState::Depositing => {
            Some(GathererUpdate::Deposit)
        }
    }
}
