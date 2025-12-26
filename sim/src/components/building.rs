//! Building components

use bevy_ecs::prelude::*;
use shared::{BuildingType, ResourceBundle, UnitType};

/// Marker component for building entities
#[derive(Component, Clone, Debug)]
pub struct Building {
    pub building_type: BuildingType,
    /// Size in tiles (width, depth)
    pub size: (u32, u32),
}

impl Building {
    pub fn town_center() -> Self {
        Self {
            building_type: BuildingType::TownCenter,
            size: (4, 4),
        }
    }

    pub fn barracks() -> Self {
        Self {
            building_type: BuildingType::Barracks,
            size: (3, 3),
        }
    }
}

/// Marker for buildings that can receive resource drop-offs
#[derive(Component, Clone, Debug, Default)]
pub struct DropOffPoint;

/// Marker for TownCenter buildings
#[derive(Component, Clone, Debug, Default)]
pub struct TownCenter;

/// Marker for Barracks buildings (stub)
#[derive(Component, Clone, Debug, Default)]
pub struct Barracks;

/// Production queue for buildings that can train units
#[derive(Component, Clone, Debug, Default)]
pub struct ProductionQueue {
    pub items: Vec<ProductionItem>,
    /// Maximum queue size
    pub max_size: usize,
}

impl ProductionQueue {
    pub fn new(max_size: usize) -> Self {
        Self {
            items: Vec::new(),
            max_size,
        }
    }

    pub fn can_queue(&self) -> bool {
        self.items.len() < self.max_size
    }

    pub fn queue(&mut self, item: ProductionItem) -> bool {
        if self.can_queue() {
            self.items.push(item);
            true
        } else {
            false
        }
    }

    pub fn cancel(&mut self, index: usize) -> Option<ProductionItem> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn current(&self) -> Option<&ProductionItem> {
        self.items.first()
    }

    pub fn current_mut(&mut self) -> Option<&mut ProductionItem> {
        self.items.first_mut()
    }
}

/// An item in a production queue
#[derive(Clone, Debug)]
pub struct ProductionItem {
    pub unit_type: UnitType,
    pub cost: ResourceBundle,
    pub ticks_remaining: u32,
    pub total_ticks: u32,
}

impl ProductionItem {
    pub fn villager() -> Self {
        Self {
            unit_type: UnitType::Villager,
            cost: ResourceBundle::new(50, 0, 0, 0),
            ticks_remaining: 50, // 2.5 seconds at 20 Hz
            total_ticks: 50,
        }
    }

    pub fn soldier() -> Self {
        Self {
            unit_type: UnitType::Soldier,
            cost: ResourceBundle::new(60, 0, 20, 0),
            ticks_remaining: 60, // 3 seconds at 20 Hz
            total_ticks: 60,
        }
    }

    pub fn progress(&self) -> f32 {
        if self.total_ticks == 0 {
            1.0
        } else {
            1.0 - (self.ticks_remaining as f32 / self.total_ticks as f32)
        }
    }

    pub fn tick(&mut self) -> bool {
        if self.ticks_remaining > 0 {
            self.ticks_remaining -= 1;
        }
        self.ticks_remaining == 0
    }
}

/// Spawn point for units produced by this building
#[derive(Component, Clone, Debug)]
pub struct SpawnPoint {
    /// Offset from building center where units spawn
    pub offset_x: f32,
    pub offset_z: f32,
}

impl Default for SpawnPoint {
    fn default() -> Self {
        Self {
            offset_x: 0.0,
            offset_z: 2.5,
        }
    }
}
