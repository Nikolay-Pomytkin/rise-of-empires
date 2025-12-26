//! Game commands with tick stamps
//!
//! Commands represent player actions that affect the simulation.
//! Each command is stamped with a tick number and player ID for
//! deterministic replay and lockstep networking.

use crate::{EntityId, PlayerId, TechId};
use serde::{Deserialize, Serialize};

/// Types of units that can be produced
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnitType {
    Villager,
    Soldier,
}

impl UnitType {
    /// Get the population cost for this unit type
    pub fn population_cost(&self) -> u32 {
        match self {
            UnitType::Villager => 1,
            UnitType::Soldier => 1,
        }
    }
}

/// Types of buildings that can be constructed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BuildingType {
    TownCenter,
    Barracks,
}

/// A game command representing a player action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameCommand {
    /// Move selected entities to a target position
    Move {
        entities: Vec<EntityId>,
        target_x: f32,
        target_z: f32,
    },

    /// Order entities to gather from a resource node
    Gather {
        entities: Vec<EntityId>,
        node: EntityId,
    },

    /// Order a builder to construct a building
    Build {
        builder: EntityId,
        building_type: BuildingType,
        tile_x: i32,
        tile_z: i32,
    },

    /// Queue a unit for production at a building
    QueueUnit {
        building: EntityId,
        unit_type: UnitType,
    },

    /// Research a technology at a building
    ResearchTech { building: EntityId, tech_id: TechId },

    /// Cancel a production queue item
    CancelProduction {
        building: EntityId,
        queue_index: usize,
    },

    /// Stop current action for entities
    Stop { entities: Vec<EntityId> },
}

/// A command stamped with tick and player information for determinism
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StampedCommand {
    /// The simulation tick this command should execute on
    pub tick: u64,
    /// The player who issued this command
    pub player_id: PlayerId,
    /// Sequence number for stable ordering within same tick/player
    pub sequence: u64,
    /// The actual command
    pub command: GameCommand,
}

impl StampedCommand {
    pub fn new(tick: u64, player_id: PlayerId, sequence: u64, command: GameCommand) -> Self {
        Self {
            tick,
            player_id,
            sequence,
            command,
        }
    }
}

impl PartialEq for StampedCommand {
    fn eq(&self, other: &Self) -> bool {
        self.tick == other.tick
            && self.player_id == other.player_id
            && self.sequence == other.sequence
    }
}

impl Eq for StampedCommand {}

impl PartialOrd for StampedCommand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StampedCommand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Order by tick first, then player_id, then sequence
        self.tick
            .cmp(&other.tick)
            .then_with(|| self.player_id.0.cmp(&other.player_id.0))
            .then_with(|| self.sequence.cmp(&other.sequence))
    }
}
