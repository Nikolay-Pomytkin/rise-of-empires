//! Gatherer component for villagers

use bevy_ecs::prelude::*;
use shared::{EntityId, ResourceType};

/// Gatherer state for villager units
#[derive(Component, Clone, Debug)]
pub struct Gatherer {
    /// Target resource node (if gathering)
    pub target_node: Option<EntityId>,
    /// Drop-off building to return resources to
    pub drop_off_target: Option<EntityId>,
    /// Type of resource being carried
    pub carrying_type: ResourceType,
    /// Amount currently carried
    pub carry_amount: u32,
    /// Maximum carry capacity
    pub carry_capacity: u32,
    /// Base gather rate (resources per tick)
    pub gather_rate: f32,
    /// Accumulated fractional resources (for sub-1 rates)
    pub gather_accumulator: f32,
    /// Current state
    pub state: GathererState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GathererState {
    Idle,
    MovingToNode,
    Gathering,
    ReturningToDropOff,
    Depositing,
}

impl Default for Gatherer {
    fn default() -> Self {
        Self {
            target_node: None,
            drop_off_target: None,
            carrying_type: ResourceType::Food,
            carry_amount: 0,
            carry_capacity: 10,
            gather_rate: 0.5, // 0.5 resources per tick = 10 per second at 20 Hz
            gather_accumulator: 0.0,
            state: GathererState::Idle,
        }
    }
}

impl Gatherer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_full(&self) -> bool {
        self.carry_amount >= self.carry_capacity
    }

    pub fn is_carrying(&self) -> bool {
        self.carry_amount > 0
    }

    pub fn add_resource(&mut self, amount: u32) {
        self.carry_amount = (self.carry_amount + amount).min(self.carry_capacity);
    }

    pub fn take_all(&mut self) -> (ResourceType, u32) {
        let result = (self.carrying_type, self.carry_amount);
        self.carry_amount = 0;
        result
    }

    pub fn set_target(&mut self, node: EntityId, resource_type: ResourceType) {
        self.target_node = Some(node);
        self.carrying_type = resource_type;
        self.state = GathererState::MovingToNode;
    }

    pub fn clear_target(&mut self) {
        self.target_node = None;
        self.state = if self.is_carrying() {
            GathererState::ReturningToDropOff
        } else {
            GathererState::Idle
        };
    }
}
