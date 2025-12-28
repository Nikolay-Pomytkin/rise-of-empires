//! Entity and player identifiers
//!
//! All IDs are simple newtypes over u64 for type safety and determinism.

use serde::{Deserialize, Serialize};

/// Unique identifier for game entities (units, buildings, resource nodes)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct EntityId(pub u64);

impl EntityId {
    pub const INVALID: EntityId = EntityId(0);

    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn is_valid(&self) -> bool {
        self.0 != 0
    }
}

impl Default for EntityId {
    fn default() -> Self {
        Self::INVALID
    }
}

/// Unique identifier for players
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct PlayerId(pub u8);

impl PlayerId {
    pub const NEUTRAL: PlayerId = PlayerId(0);
    pub const PLAYER_1: PlayerId = PlayerId(1);
    pub const PLAYER_2: PlayerId = PlayerId(2);

    pub fn new(id: u8) -> Self {
        Self(id)
    }

    pub fn is_neutral(&self) -> bool {
        self.0 == 0
    }
}

impl Default for PlayerId {
    fn default() -> Self {
        Self::NEUTRAL
    }
}

/// Identifier for technologies in the tech tree
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TechId(pub String);

impl TechId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl From<&str> for TechId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// Identifier for ages in the tech tree
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgeId(pub String);

impl AgeId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl From<&str> for AgeId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// Identifier for empires (civilizations)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EmpireId(pub String);

impl EmpireId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl From<&str> for EmpireId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl std::fmt::Display for EmpireId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Identifier for leaders
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LeaderId(pub String);

impl LeaderId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl From<&str> for LeaderId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl std::fmt::Display for LeaderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
