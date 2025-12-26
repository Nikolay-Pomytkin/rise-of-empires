//! Snapshot generation system

use bevy_ecs::prelude::*;
use shared::{
    EntitySnapshot, EntityType, PlayerSnapshot, ProductionQueueState, QueueItem, WorldSnapshot,
};

use crate::{components::*, world::SimWorld, SnapshotEvent, TickScheduler};

/// Generate a world snapshot for rendering
#[allow(deprecated)]
pub fn generate_snapshot(
    tick: Res<TickScheduler>,
    sim_world: Res<SimWorld>,
    entities: Query<(
        &SimEntity,
        &SimPosition,
        Option<&Owner>,
        Option<&Health>,
        Option<&Selected>,
        Option<&Unit>,
        Option<&Building>,
        Option<&ResourceNode>,
        Option<&Gatherer>,
        Option<&ProductionQueue>,
    )>,
    mut snapshot_events: bevy_ecs::message::MessageWriter<SnapshotEvent>,
) {
    let mut snapshot = WorldSnapshot::new(tick.tick());

    // Collect entity snapshots
    for (
        sim_entity,
        pos,
        owner,
        health,
        selected,
        unit,
        building,
        resource_node,
        gatherer,
        queue,
    ) in entities.iter()
    {
        let entity_type = if let Some(u) = unit {
            EntityType::Unit(u.unit_type)
        } else if let Some(b) = building {
            EntityType::Building(b.building_type)
        } else if let Some(r) = resource_node {
            EntityType::ResourceNode(r.resource_type)
        } else {
            continue;
        };

        let entity_snapshot = EntitySnapshot {
            id: sim_entity.id,
            entity_type,
            position: pos.to_array(),
            owner: owner.map(|o| o.player_id),
            health: health.map(|h| (h.current, h.max)),
            selected_by: selected.map(|s| s.by_players.clone()).unwrap_or_default(),
            gatherer_state: gatherer.map(|g| shared::GathererState {
                target_node: g.target_node,
                carrying: g.carrying_type,
                carry_amount: g.carry_amount,
                carry_capacity: g.carry_capacity,
                is_returning: g.state == GathererState::ReturningToDropOff,
            }),
            production_queue: queue.map(|q| ProductionQueueState {
                items: q
                    .items
                    .iter()
                    .map(|item| QueueItem {
                        unit_type: item.unit_type,
                        ticks_remaining: item.ticks_remaining,
                        total_ticks: item.total_ticks,
                    })
                    .collect(),
            }),
            resource_remaining: resource_node.map(|r| r.remaining),
        };

        snapshot.entities.push(entity_snapshot);
    }

    // Collect player snapshots
    for (_, player) in sim_world.players.iter() {
        let player_snapshot = PlayerSnapshot {
            id: player.id,
            resources: player.resources,
            population: player.population,
            population_cap: player.population_cap,
            current_age: player.current_age.clone(),
            researched_techs: player.researched_techs.clone(),
        };
        snapshot.players.push(player_snapshot);
    }

    // Sort for determinism
    snapshot.entities.sort_by_key(|e| e.id.0);
    snapshot.players.sort_by_key(|p| p.id.0);

    snapshot_events.write(SnapshotEvent { snapshot });
}
