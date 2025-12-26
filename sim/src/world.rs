//! Simulation world state and entity management

use bevy_ecs::prelude::*;
use shared::{AgeId, EntityId, PlayerId, ResourceBundle};
use std::collections::HashMap;

/// Generator for unique entity IDs
#[derive(Resource, Default)]
pub struct EntityIdGenerator {
    next_id: u64,
}

impl EntityIdGenerator {
    pub fn next(&mut self) -> EntityId {
        self.next_id += 1;
        EntityId::new(self.next_id)
    }
}

/// Player state in the simulation
#[derive(Clone, Debug)]
pub struct PlayerState {
    pub id: PlayerId,
    pub resources: ResourceBundle,
    pub population: u32,
    pub population_cap: u32,
    pub current_age: AgeId,
    pub researched_techs: Vec<String>,
    pub tech_modifiers: TechModifiers,
}

impl PlayerState {
    pub fn new(id: PlayerId) -> Self {
        Self {
            id,
            resources: ResourceBundle::new(200, 200, 0, 0), // Starting resources
            population: 0,
            population_cap: 5, // Starting pop cap
            current_age: AgeId::new("dark_age"),
            researched_techs: Vec::new(),
            tech_modifiers: TechModifiers::default(),
        }
    }

    pub fn can_afford(&self, cost: &ResourceBundle) -> bool {
        self.resources.can_afford(cost)
    }

    pub fn spend(&mut self, cost: &ResourceBundle) -> bool {
        self.resources.subtract(cost)
    }

    pub fn add_resources(&mut self, amount: &ResourceBundle) {
        self.resources.add(amount);
    }
}

/// Accumulated tech modifiers for a player
#[derive(Clone, Debug, Default)]
pub struct TechModifiers {
    /// Gather rate bonus (percentage, e.g., 10 = +10%)
    pub gather_rate_bonus: HashMap<shared::ResourceType, i32>,
    /// Carry capacity bonus (flat)
    pub carry_capacity_bonus: i32,
    /// Build speed bonus (percentage)
    pub build_speed_bonus: i32,
    /// Unit production speed bonus (percentage)
    pub production_speed_bonus: i32,
    /// Unit HP bonus (percentage)
    pub unit_hp_bonus: i32,
    /// Unit attack bonus (flat)
    pub unit_attack_bonus: i32,
}

/// Main simulation world resource
#[derive(Resource, Default)]
pub struct SimWorld {
    /// Player states indexed by PlayerId
    pub players: HashMap<u8, PlayerState>,
    /// Mapping from our EntityId to Bevy Entity
    pub entity_map: HashMap<EntityId, Entity>,
    /// Reverse mapping
    pub reverse_entity_map: HashMap<Entity, EntityId>,
}

impl SimWorld {
    pub fn add_player(&mut self, player_id: PlayerId) {
        self.players
            .insert(player_id.0, PlayerState::new(player_id));
    }

    pub fn get_player(&self, player_id: PlayerId) -> Option<&PlayerState> {
        self.players.get(&player_id.0)
    }

    pub fn get_player_mut(&mut self, player_id: PlayerId) -> Option<&mut PlayerState> {
        self.players.get_mut(&player_id.0)
    }

    pub fn register_entity(&mut self, sim_id: EntityId, bevy_entity: Entity) {
        self.entity_map.insert(sim_id, bevy_entity);
        self.reverse_entity_map.insert(bevy_entity, sim_id);
    }

    pub fn unregister_entity(&mut self, sim_id: EntityId) {
        if let Some(bevy_entity) = self.entity_map.remove(&sim_id) {
            self.reverse_entity_map.remove(&bevy_entity);
        }
    }

    pub fn get_bevy_entity(&self, sim_id: EntityId) -> Option<Entity> {
        self.entity_map.get(&sim_id).copied()
    }

    pub fn get_sim_id(&self, bevy_entity: Entity) -> Option<EntityId> {
        self.reverse_entity_map.get(&bevy_entity).copied()
    }
}

/// Buffer for incoming commands
#[derive(Resource, Default)]
pub struct CommandBuffer {
    commands: Vec<shared::StampedCommand>,
    command_sequence: u64,
}

impl CommandBuffer {
    /// Add a command to the buffer
    pub fn push(&mut self, mut command: shared::StampedCommand) {
        command.sequence = self.command_sequence;
        self.command_sequence += 1;
        self.commands.push(command);
    }

    /// Create and push a command for the current tick
    pub fn push_command(&mut self, tick: u64, player_id: PlayerId, command: shared::GameCommand) {
        let stamped = shared::StampedCommand::new(tick, player_id, self.command_sequence, command);
        self.command_sequence += 1;
        self.commands.push(stamped);
    }

    /// Drain all commands for the given tick, sorted deterministically
    pub fn drain_for_tick(&mut self, tick: u64) -> Vec<shared::StampedCommand> {
        // Partition: commands for this tick vs future commands
        let (for_tick, remaining): (Vec<_>, Vec<_>) =
            self.commands.drain(..).partition(|cmd| cmd.tick <= tick);

        self.commands = remaining;

        // Sort deterministically
        let mut sorted = for_tick;
        sorted.sort();

        sorted
    }

    /// Check if there are any pending commands
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    /// Get the number of pending commands
    pub fn len(&self) -> usize {
        self.commands.len()
    }
}
