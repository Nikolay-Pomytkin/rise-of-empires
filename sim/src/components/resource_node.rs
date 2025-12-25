//! Resource node components

use bevy_ecs::prelude::*;
use shared::ResourceType;

/// A gatherable resource node (tree, gold mine, etc.)
#[derive(Component, Clone, Debug)]
pub struct ResourceNode {
    pub resource_type: ResourceType,
    /// Amount of resources remaining
    pub remaining: u32,
    /// Maximum amount (for display)
    pub max_amount: u32,
    /// How many gatherers can work on this node simultaneously
    pub max_gatherers: u32,
    /// Current gatherers working on this node
    pub current_gatherers: u32,
}

impl ResourceNode {
    pub fn food() -> Self {
        Self {
            resource_type: ResourceType::Food,
            remaining: 300,
            max_amount: 300,
            max_gatherers: 8,
            current_gatherers: 0,
        }
    }

    pub fn wood() -> Self {
        Self {
            resource_type: ResourceType::Wood,
            remaining: 150,
            max_amount: 150,
            max_gatherers: 4,
            current_gatherers: 0,
        }
    }

    pub fn gold() -> Self {
        Self {
            resource_type: ResourceType::Gold,
            remaining: 800,
            max_amount: 800,
            max_gatherers: 3,
            current_gatherers: 0,
        }
    }

    pub fn stone() -> Self {
        Self {
            resource_type: ResourceType::Stone,
            remaining: 400,
            max_amount: 400,
            max_gatherers: 3,
            current_gatherers: 0,
        }
    }

    pub fn is_depleted(&self) -> bool {
        self.remaining == 0
    }

    pub fn can_gather(&self) -> bool {
        !self.is_depleted() && self.current_gatherers < self.max_gatherers
    }

    /// Harvest resources, returns amount actually gathered
    pub fn harvest(&mut self, amount: u32) -> u32 {
        let gathered = amount.min(self.remaining);
        self.remaining -= gathered;
        gathered
    }
}

