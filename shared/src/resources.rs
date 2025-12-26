//! Resource types and bundles
//!
//! Resources are the core economy types: food, wood, gold, stone.

use serde::{Deserialize, Serialize};

/// The four resource types in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    Food,
    Wood,
    Gold,
    Stone,
}

impl ResourceType {
    /// All resource types for iteration
    pub const ALL: [ResourceType; 4] = [
        ResourceType::Food,
        ResourceType::Wood,
        ResourceType::Gold,
        ResourceType::Stone,
    ];
}

/// A bundle of resources (used for costs, storage, etc.)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceBundle {
    pub food: u32,
    pub wood: u32,
    pub gold: u32,
    pub stone: u32,
}

impl ResourceBundle {
    pub const ZERO: ResourceBundle = ResourceBundle {
        food: 0,
        wood: 0,
        gold: 0,
        stone: 0,
    };

    pub fn new(food: u32, wood: u32, gold: u32, stone: u32) -> Self {
        Self {
            food,
            wood,
            gold,
            stone,
        }
    }

    /// Get the amount of a specific resource type
    pub fn get(&self, resource_type: ResourceType) -> u32 {
        match resource_type {
            ResourceType::Food => self.food,
            ResourceType::Wood => self.wood,
            ResourceType::Gold => self.gold,
            ResourceType::Stone => self.stone,
        }
    }

    /// Set the amount of a specific resource type
    pub fn set(&mut self, resource_type: ResourceType, amount: u32) {
        match resource_type {
            ResourceType::Food => self.food = amount,
            ResourceType::Wood => self.wood = amount,
            ResourceType::Gold => self.gold = amount,
            ResourceType::Stone => self.stone = amount,
        }
    }

    /// Add resources from another bundle
    pub fn add(&mut self, other: &ResourceBundle) {
        self.food = self.food.saturating_add(other.food);
        self.wood = self.wood.saturating_add(other.wood);
        self.gold = self.gold.saturating_add(other.gold);
        self.stone = self.stone.saturating_add(other.stone);
    }

    /// Subtract resources, returning true if successful (had enough)
    pub fn subtract(&mut self, other: &ResourceBundle) -> bool {
        if self.can_afford(other) {
            self.food -= other.food;
            self.wood -= other.wood;
            self.gold -= other.gold;
            self.stone -= other.stone;
            true
        } else {
            false
        }
    }

    /// Check if we have at least as much as the other bundle
    pub fn can_afford(&self, cost: &ResourceBundle) -> bool {
        self.food >= cost.food
            && self.wood >= cost.wood
            && self.gold >= cost.gold
            && self.stone >= cost.stone
    }

    /// Get total resources
    pub fn total(&self) -> u32 {
        self.food + self.wood + self.gold + self.stone
    }
}

impl std::ops::Add for ResourceBundle {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            food: self.food.saturating_add(other.food),
            wood: self.wood.saturating_add(other.wood),
            gold: self.gold.saturating_add(other.gold),
            stone: self.stone.saturating_add(other.stone),
        }
    }
}

impl std::ops::AddAssign for ResourceBundle {
    fn add_assign(&mut self, other: Self) {
        self.add(&other);
    }
}
