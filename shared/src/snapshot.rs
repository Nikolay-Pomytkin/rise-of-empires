//! World snapshots for rendering and replay validation
//!
//! Snapshots capture the complete state of the simulation at a point in time.
//! Used for:
//! - Rendering interpolation
//! - Replay validation (hash comparison)
//! - State synchronization in netcode

use crate::{AgeId, BuildingType, EntityId, PlayerId, ResourceBundle, ResourceType, UnitType};
use serde::{Deserialize, Serialize};

/// Types of entities in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityType {
    Unit(UnitType),
    Building(BuildingType),
    ResourceNode(ResourceType),
}

/// State of a gatherer unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GathererState {
    pub target_node: Option<EntityId>,
    pub carrying: ResourceType,
    pub carry_amount: u32,
    pub carry_capacity: u32,
    pub is_returning: bool,
}

/// An item in a production queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueItem {
    pub unit_type: UnitType,
    pub ticks_remaining: u32,
    pub total_ticks: u32,
}

/// State of a production queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionQueueState {
    pub items: Vec<QueueItem>,
}

/// Snapshot of a single entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySnapshot {
    pub id: EntityId,
    pub entity_type: EntityType,
    pub position: [f32; 3],
    pub owner: Option<PlayerId>,
    pub health: Option<(u32, u32)>, // (current, max)
    pub selected_by: Vec<PlayerId>,
    pub gatherer_state: Option<GathererState>,
    pub production_queue: Option<ProductionQueueState>,
    pub resource_remaining: Option<u32>, // For resource nodes
}

/// Snapshot of a player's state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSnapshot {
    pub id: PlayerId,
    pub resources: ResourceBundle,
    pub population: u32,
    pub population_cap: u32,
    pub current_age: AgeId,
    pub researched_techs: Vec<String>,
}

/// Complete snapshot of the world state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSnapshot {
    pub tick: u64,
    pub entities: Vec<EntitySnapshot>,
    pub players: Vec<PlayerSnapshot>,
}

impl WorldSnapshot {
    pub fn new(tick: u64) -> Self {
        Self {
            tick,
            entities: Vec::new(),
            players: Vec::new(),
        }
    }

    /// Compute a deterministic hash of this snapshot for replay validation
    pub fn compute_hash(&self) -> u64 {
        use std::hash::{Hash, Hasher};

        // Use a simple but deterministic hasher
        struct DeterministicHasher(u64);

        impl Hasher for DeterministicHasher {
            fn finish(&self) -> u64 {
                self.0
            }

            fn write(&mut self, bytes: &[u8]) {
                for byte in bytes {
                    self.0 = self.0.wrapping_mul(31).wrapping_add(*byte as u64);
                }
            }
        }

        let mut hasher = DeterministicHasher(0);

        // Hash tick
        self.tick.hash(&mut hasher);

        // Hash entities in order
        for entity in &self.entities {
            entity.id.0.hash(&mut hasher);
            // Hash position as fixed-point to avoid float issues
            let px = (entity.position[0] * 1000.0) as i64;
            let py = (entity.position[1] * 1000.0) as i64;
            let pz = (entity.position[2] * 1000.0) as i64;
            px.hash(&mut hasher);
            py.hash(&mut hasher);
            pz.hash(&mut hasher);

            if let Some((current, max)) = entity.health {
                current.hash(&mut hasher);
                max.hash(&mut hasher);
            }

            if let Some(remaining) = entity.resource_remaining {
                remaining.hash(&mut hasher);
            }

            if let Some(ref gs) = entity.gatherer_state {
                gs.carry_amount.hash(&mut hasher);
            }
        }

        // Hash player states
        for player in &self.players {
            player.id.0.hash(&mut hasher);
            player.resources.food.hash(&mut hasher);
            player.resources.wood.hash(&mut hasher);
            player.resources.gold.hash(&mut hasher);
            player.resources.stone.hash(&mut hasher);
            player.population.hash(&mut hasher);
        }

        hasher.finish()
    }
}
