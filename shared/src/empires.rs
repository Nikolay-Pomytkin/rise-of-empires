//! Empire and Leader definitions
//!
//! Empires provide civilization identity with unique units, buildings, and economic bonuses.
//! Leaders provide military bonuses and passive traits.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::ids::{LeaderId, EmpireId, TechId};
use crate::resources::{ResourceBundle, ResourceType};
use crate::commands::{UnitType, BuildingType};

/// Definition of an empire/civilization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmpireDef {
    /// Unique identifier for this empire
    pub id: EmpireId,
    /// Display name
    pub name: String,
    /// Brief description of the empire's playstyle
    pub description: String,
    /// Theme/focus (e.g., "Infantry, Engineering")
    pub theme: String,
    /// Available leaders for this empire (1-2)
    pub leaders: Vec<LeaderDef>,
    /// Unique units this empire can build
    #[serde(default)]
    pub unique_units: Vec<UniqueUnitDef>,
    /// Unique buildings this empire can construct
    #[serde(default)]
    pub unique_buildings: Vec<UniqueBuildingDef>,
    /// Resource gathering and economic bonuses
    #[serde(default)]
    pub resource_bonuses: ResourceBonuses,
    /// Technology research bonuses
    #[serde(default)]
    pub tech_bonuses: TechBonuses,
}

impl Default for EmpireDef {
    fn default() -> Self {
        Self {
            id: EmpireId::new("default"),
            name: "Default Empire".to_string(),
            description: "A basic empire with no special bonuses.".to_string(),
            theme: "Balanced".to_string(),
            leaders: vec![],
            unique_units: vec![],
            unique_buildings: vec![],
            resource_bonuses: ResourceBonuses::default(),
            tech_bonuses: TechBonuses::default(),
        }
    }
}

/// Definition of a leader
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderDef {
    /// Unique identifier for this leader
    pub id: LeaderId,
    /// Display name
    pub name: String,
    /// Title (e.g., "The Conqueror", "The Wise")
    pub title: String,
    /// Brief description of the leader's bonuses
    pub description: String,
    /// Military combat and training bonuses
    #[serde(default)]
    pub military_bonuses: MilitaryBonuses,
    /// Passive traits that apply throughout the game
    #[serde(default)]
    pub passive_traits: Vec<PassiveTrait>,
}

impl Default for LeaderDef {
    fn default() -> Self {
        Self {
            id: LeaderId::new("default"),
            name: "Default Leader".to_string(),
            title: "".to_string(),
            description: "A basic leader with no special bonuses.".to_string(),
            military_bonuses: MilitaryBonuses::default(),
            passive_traits: vec![],
        }
    }
}

/// Unique unit that an empire can build
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniqueUnitDef {
    /// Identifier for this unique unit
    pub id: String,
    /// Display name
    pub name: String,
    /// Which standard unit this replaces (if any)
    pub replaces: Option<UnitType>,
    /// Base stats for this unit
    pub stats: UniqueUnitStats,
    /// Cost to produce
    pub cost: ResourceBundle,
    /// Training time in ticks
    pub train_time: u32,
    /// Description of what makes this unit special
    pub description: String,
}

/// Stats for a unique unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniqueUnitStats {
    pub health: u32,
    pub attack: i32,
    pub defense: i32,
    pub speed: f32,
    pub range: f32,
    /// Unit category for bonus calculations
    pub category: UnitCategory,
}

impl Default for UniqueUnitStats {
    fn default() -> Self {
        Self {
            health: 50,
            attack: 5,
            defense: 2,
            speed: 3.0,
            range: 1.0,
            category: UnitCategory::Infantry,
        }
    }
}

/// Categories of units for bonus calculations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnitCategory {
    Infantry,
    Cavalry,
    Ranged,
    Siege,
    Naval,
    Villager,
}

/// Unique building that an empire can construct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniqueBuildingDef {
    /// Identifier for this unique building
    pub id: String,
    /// Display name
    pub name: String,
    /// Which standard building this replaces (if any)
    pub replaces: Option<BuildingType>,
    /// Cost to construct
    pub cost: ResourceBundle,
    /// Build time in ticks
    pub build_time: u32,
    /// Description of what makes this building special
    pub description: String,
    /// Special effects this building provides
    #[serde(default)]
    pub effects: Vec<BuildingEffect>,
}

/// Special effects that a unique building can provide
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BuildingEffect {
    /// Increase population cap
    PopulationBonus(i32),
    /// Generate resources per tick
    ResourceGeneration(ResourceType, f32),
    /// Bonus to research speed
    ResearchSpeedBonus(f32),
    /// Bonus to unit training speed
    TrainingSpeedBonus(f32),
    /// Heal nearby units
    HealingAura(f32),
    /// Increase attack of nearby units
    AttackAura(i32),
}

/// Resource gathering and economic bonuses from an empire
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourceBonuses {
    /// Percentage bonus to gather rates per resource type (0.10 = +10%)
    #[serde(default)]
    pub gather_rate: HashMap<ResourceType, f32>,
    /// Extra starting resources
    #[serde(default)]
    pub starting_resources: ResourceBundle,
    /// Discount on market trades (0.10 = 10% cheaper)
    #[serde(default)]
    pub market_discount: f32,
    /// Bonus resources from killing enemy units
    #[serde(default)]
    pub kill_bonus: HashMap<ResourceType, f32>,
}

/// Military bonuses from a leader
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MilitaryBonuses {
    /// Flat attack bonus per unit category
    #[serde(default)]
    pub attack_bonus: HashMap<UnitCategory, i32>,
    /// Flat defense bonus per unit category
    #[serde(default)]
    pub defense_bonus: HashMap<UnitCategory, i32>,
    /// Percentage speed bonus per unit category (0.10 = +10%)
    #[serde(default)]
    pub speed_bonus: HashMap<UnitCategory, f32>,
    /// Percentage training speed modifier (0.85 = 15% faster)
    #[serde(default = "default_one")]
    pub training_speed: f32,
}

fn default_one() -> f32 {
    1.0
}

/// Technology research bonuses from an empire
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TechBonuses {
    /// Percentage research speed modifier (0.90 = 10% faster)
    #[serde(default = "default_one")]
    pub research_speed: f32,
    /// Percentage discount on age advancement costs (0.10 = 10% cheaper)
    #[serde(default)]
    pub age_advance_discount: f32,
    /// Technologies the empire starts with for free
    #[serde(default)]
    pub free_techs: Vec<TechId>,
    /// Technologies only this empire can research
    #[serde(default)]
    pub unique_techs: Vec<UniqueTechDef>,
}

/// A technology unique to a specific empire
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniqueTechDef {
    pub id: TechId,
    pub name: String,
    pub description: String,
    pub cost: ResourceBundle,
    pub research_time: u32,
    pub effects: Vec<TechEffect>,
}

/// Effects that a technology can have
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TechEffect {
    /// Bonus to gather rate for a resource
    GatherRateBonus { resource: ResourceType, percent: i32 },
    /// Bonus to unit attack
    AttackBonus { category: UnitCategory, amount: i32 },
    /// Bonus to unit defense
    DefenseBonus { category: UnitCategory, amount: i32 },
    /// Bonus to unit speed
    SpeedBonus { category: UnitCategory, percent: f32 },
    /// Bonus to building HP
    BuildingHpBonus { percent: i32 },
    /// Bonus to population cap
    PopulationCapBonus { amount: i32 },
}

/// Passive traits that apply throughout the game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PassiveTrait {
    /// Units heal X% faster when idle
    HealingRate(f32),
    /// Units have X% more vision range
    VisionRange(f32),
    /// Buildings construct X% faster
    BuildSpeed(f32),
    /// +X to maximum population cap
    PopulationCap(i32),
    /// Start with X extra villagers
    StartingVillagers(i32),
    /// Units cost X% less food
    FoodCostReduction(f32),
    /// First building of each type is free
    FreeBuildingOnce,
    /// Villagers work X% faster when near Town Center
    TownCenterBonus(f32),
    /// Military units train X% faster
    MilitaryTrainingSpeed(f32),
    /// Siege units deal X% more damage to buildings
    SiegeDamageBonus(f32),
}

impl std::fmt::Display for PassiveTrait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PassiveTrait::HealingRate(v) => write!(f, "+{:.0}% healing rate", v * 100.0),
            PassiveTrait::VisionRange(v) => write!(f, "+{:.0}% vision range", v * 100.0),
            PassiveTrait::BuildSpeed(v) => write!(f, "+{:.0}% build speed", v * 100.0),
            PassiveTrait::PopulationCap(v) => write!(f, "+{} population cap", v),
            PassiveTrait::StartingVillagers(v) => write!(f, "+{} starting villagers", v),
            PassiveTrait::FoodCostReduction(v) => write!(f, "-{:.0}% food cost", v * 100.0),
            PassiveTrait::FreeBuildingOnce => write!(f, "First building of each type is free"),
            PassiveTrait::TownCenterBonus(v) => write!(f, "+{:.0}% gather rate near Town Center", v * 100.0),
            PassiveTrait::MilitaryTrainingSpeed(v) => write!(f, "+{:.0}% military training speed", v * 100.0),
            PassiveTrait::SiegeDamageBonus(v) => write!(f, "+{:.0}% siege damage to buildings", v * 100.0),
        }
    }
}

/// Player's empire and leader selection for game setup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSetup {
    pub empire: EmpireId,
    pub leader: LeaderId,
}

impl PlayerSetup {
    pub fn new(empire: impl Into<EmpireId>, leader: impl Into<LeaderId>) -> Self {
        Self {
            empire: empire.into(),
            leader: leader.into(),
        }
    }
}
