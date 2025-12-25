//! Owner component

use bevy_ecs::prelude::*;
use shared::PlayerId;

/// Owner component - which player owns this entity
#[derive(Component, Clone, Copy, Debug)]
pub struct Owner {
    pub player_id: PlayerId,
}

impl Owner {
    pub fn new(player_id: PlayerId) -> Self {
        Self { player_id }
    }

    pub fn neutral() -> Self {
        Self {
            player_id: PlayerId::NEUTRAL,
        }
    }

    pub fn is_neutral(&self) -> bool {
        self.player_id.is_neutral()
    }
}

impl Default for Owner {
    fn default() -> Self {
        Self::neutral()
    }
}

