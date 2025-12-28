//! Save and load game system
//!
//! Handles serializing and deserializing game state to/from files.

use bevy::ecs::message::{Message, MessageReader, MessageWriter};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Save file metadata and data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveFile {
    /// Save file format version
    pub version: u32,
    /// Timestamp when saved (Unix epoch seconds)
    pub timestamp: u64,
    /// Display name for the save
    pub name: String,
    /// Current game tick
    pub tick: u64,
    /// The world snapshot
    pub snapshot: shared::WorldSnapshot,
    /// RNG state for deterministic continuation
    pub rng_seed: u64,
    /// Entity ID generator state
    pub next_entity_id: u64,
}

impl SaveFile {
    pub const CURRENT_VERSION: u32 = 1;
    pub const EXTENSION: &'static str = "roesave";

    pub fn new(name: String, tick: u64, snapshot: shared::WorldSnapshot, rng_seed: u64, next_entity_id: u64) -> Self {
        Self {
            version: Self::CURRENT_VERSION,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            name,
            tick,
            snapshot,
            rng_seed,
            next_entity_id,
        }
    }
}

/// Resource for managing saves
#[derive(Resource)]
pub struct SaveManager {
    /// Directory where saves are stored
    pub save_dir: PathBuf,
}

impl Default for SaveManager {
    fn default() -> Self {
        // Use platform-appropriate save directory
        let save_dir = if let Some(data_dir) = dirs::data_local_dir() {
            data_dir.join("RiseRTS").join("saves")
        } else {
            PathBuf::from("saves")
        };

        Self { save_dir }
    }
}

impl SaveManager {
    /// Ensure save directory exists
    pub fn ensure_save_dir(&self) -> std::io::Result<()> {
        fs::create_dir_all(&self.save_dir)
    }

    /// Get path for a save file
    pub fn save_path(&self, name: &str) -> PathBuf {
        self.save_dir.join(format!("{}.{}", name, SaveFile::EXTENSION))
    }

    /// List all available saves
    pub fn list_saves(&self) -> Vec<SaveFileInfo> {
        let mut saves = Vec::new();

        if let Ok(entries) = fs::read_dir(&self.save_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some(SaveFile::EXTENSION) {
                    if let Ok(data) = fs::read(&path) {
                        if let Ok(save) = ron::de::from_bytes::<SaveFile>(&data) {
                            saves.push(SaveFileInfo {
                                filename: path.file_stem()
                                    .and_then(|s| s.to_str())
                                    .unwrap_or("unknown")
                                    .to_string(),
                                name: save.name,
                                timestamp: save.timestamp,
                                tick: save.tick,
                            });
                        }
                    }
                }
            }
        }

        // Sort by timestamp, newest first
        saves.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        saves
    }

    /// Save game to file
    pub fn save_game(&self, save: &SaveFile) -> Result<PathBuf, SaveError> {
        self.ensure_save_dir().map_err(SaveError::Io)?;

        let filename = sanitize_filename(&save.name);
        let path = self.save_path(&filename);

        let data = ron::ser::to_string_pretty(save, ron::ser::PrettyConfig::default())
            .map_err(SaveError::Serialize)?;

        fs::write(&path, data).map_err(SaveError::Io)?;

        info!("Game saved to {:?}", path);
        Ok(path)
    }

    /// Load game from file
    pub fn load_game(&self, filename: &str) -> Result<SaveFile, SaveError> {
        let path = self.save_path(filename);

        let data = fs::read(&path).map_err(SaveError::Io)?;
        let save: SaveFile = ron::de::from_bytes(&data).map_err(SaveError::Deserialize)?;

        if save.version > SaveFile::CURRENT_VERSION {
            return Err(SaveError::VersionMismatch {
                file_version: save.version,
                current_version: SaveFile::CURRENT_VERSION,
            });
        }

        info!("Game loaded from {:?}", path);
        Ok(save)
    }

    /// Delete a save file
    pub fn delete_save(&self, filename: &str) -> Result<(), SaveError> {
        let path = self.save_path(filename);
        fs::remove_file(&path).map_err(SaveError::Io)?;
        info!("Save deleted: {:?}", path);
        Ok(())
    }
}

/// Brief info about a save file (for listing)
#[derive(Debug, Clone)]
pub struct SaveFileInfo {
    pub filename: String,
    pub name: String,
    pub timestamp: u64,
    pub tick: u64,
}

impl SaveFileInfo {
    /// Format timestamp as human-readable string
    pub fn formatted_time(&self) -> String {
        use std::time::{Duration, UNIX_EPOCH};

        let datetime = UNIX_EPOCH + Duration::from_secs(self.timestamp);
        // Simple formatting - in production you'd use chrono
        format!("{:?}", datetime)
    }
}

/// Errors that can occur during save/load
#[derive(Debug)]
pub enum SaveError {
    Io(std::io::Error),
    Serialize(ron::Error),
    Deserialize(ron::error::SpannedError),
    VersionMismatch { file_version: u32, current_version: u32 },
}

impl std::fmt::Display for SaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SaveError::Io(e) => write!(f, "IO error: {}", e),
            SaveError::Serialize(e) => write!(f, "Serialization error: {}", e),
            SaveError::Deserialize(e) => write!(f, "Deserialization error: {}", e),
            SaveError::VersionMismatch { file_version, current_version } => {
                write!(f, "Save version {} is newer than game version {}", file_version, current_version)
            }
        }
    }
}

/// Sanitize a string for use as a filename
fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' || c == ' ' {
                c
            } else {
                '_'
            }
        })
        .collect::<String>()
        .trim()
        .to_string()
}

/// Event to trigger a save
#[derive(Message, Clone)]
pub struct SaveGameEvent {
    pub name: String,
}

/// Event to trigger a load
#[derive(Message, Clone)]
pub struct LoadGameEvent {
    pub filename: String,
}

/// Event emitted when save completes
#[derive(Message, Clone)]
pub struct SaveCompleteEvent {
    pub success: bool,
    pub path: Option<PathBuf>,
    pub error: Option<String>,
}

/// Event emitted when load completes
#[derive(Message, Clone)]
pub struct LoadCompleteEvent {
    pub success: bool,
    pub save: Option<SaveFile>,
    pub error: Option<String>,
}

pub struct SaveLoadPlugin;

impl Plugin for SaveLoadPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SaveManager>()
            .add_message::<SaveGameEvent>()
            .add_message::<LoadGameEvent>()
            .add_message::<SaveCompleteEvent>()
            .add_message::<LoadCompleteEvent>()
            .add_systems(Update, (handle_save_events, handle_load_events));
    }
}

/// System to handle save requests
fn handle_save_events(
    mut save_events: MessageReader<SaveGameEvent>,
    mut complete_events: MessageWriter<SaveCompleteEvent>,
    save_manager: Res<SaveManager>,
    tick_scheduler: Res<sim::TickScheduler>,
    sim_world: Res<sim::SimWorld>,
    rng: Res<sim::SimRng>,
    id_gen: Res<sim::EntityIdGenerator>,
    // Query all entities to build snapshot
    units: Query<(
        &sim::SimEntity,
        &sim::SimPosition,
        &sim::Owner,
        &sim::Unit,
        &sim::Health,
        Option<&sim::Gatherer>,
        Option<&sim::Selected>,
    )>,
    buildings: Query<(
        &sim::SimEntity,
        &sim::SimPosition,
        &sim::Owner,
        &sim::Building,
        &sim::Health,
        Option<&sim::ProductionQueue>,
        Option<&sim::Selected>,
    )>,
    resource_nodes: Query<(
        &sim::SimEntity,
        &sim::SimPosition,
        &sim::ResourceNode,
    )>,
) {
    for event in save_events.read() {
        let tick = tick_scheduler.tick();

        // Build snapshot
        let mut snapshot = shared::WorldSnapshot::new(tick);

        // Add player states
        for (_, player) in sim_world.players.iter() {
            snapshot.players.push(shared::PlayerSnapshot {
                id: player.id,
                resources: player.resources.clone(),
                population: player.population,
                population_cap: player.population_cap,
                current_age: player.current_age.clone(),
                researched_techs: player.researched_techs.clone(),
            });
        }

        // Add units
        for (sim_entity, pos, owner, unit, health, gatherer, selected) in units.iter() {
            let gatherer_state = gatherer.map(|g| shared::GathererState {
                target_node: g.target_node,
                carrying: g.carrying_type,
                carry_amount: g.carry_amount as u32,
                carry_capacity: g.carry_capacity as u32,
                is_returning: matches!(g.state, sim::GathererState::ReturningToDropOff),
            });

            snapshot.entities.push(shared::EntitySnapshot {
                id: sim_entity.id,
                entity_type: shared::EntityType::Unit(unit.unit_type),
                position: pos.to_array(),
                owner: Some(owner.player_id),
                health: Some((health.current, health.max)),
                selected_by: selected.map(|s| s.by_players.clone()).unwrap_or_default(),
                gatherer_state,
                production_queue: None,
                resource_remaining: None,
            });
        }

        // Add buildings
        for (sim_entity, pos, owner, building, health, queue, selected) in buildings.iter() {
            let production_queue = queue.map(|q| shared::ProductionQueueState {
                items: q.items.iter().map(|item| shared::QueueItem {
                    unit_type: item.unit_type,
                    ticks_remaining: item.ticks_remaining,
                    total_ticks: item.total_ticks,
                }).collect(),
            });

            snapshot.entities.push(shared::EntitySnapshot {
                id: sim_entity.id,
                entity_type: shared::EntityType::Building(building.building_type),
                position: pos.to_array(),
                owner: Some(owner.player_id),
                health: Some((health.current, health.max)),
                selected_by: selected.map(|s| s.by_players.clone()).unwrap_or_default(),
                gatherer_state: None,
                production_queue,
                resource_remaining: None,
            });
        }

        // Add resource nodes
        for (sim_entity, pos, node) in resource_nodes.iter() {
            snapshot.entities.push(shared::EntitySnapshot {
                id: sim_entity.id,
                entity_type: shared::EntityType::ResourceNode(node.resource_type),
                position: pos.to_array(),
                owner: None,
                health: None,
                selected_by: vec![],
                gatherer_state: None,
                production_queue: None,
                resource_remaining: Some(node.remaining),
            });
        }

        // Create save file
        let save = SaveFile::new(
            event.name.clone(),
            tick,
            snapshot,
            rng.current_seed(),
            id_gen.current_id(),
        );

        // Write to disk
        match save_manager.save_game(&save) {
            Ok(path) => {
                complete_events.write(SaveCompleteEvent {
                    success: true,
                    path: Some(path),
                    error: None,
                });
            }
            Err(e) => {
                error!("Failed to save game: {}", e);
                complete_events.write(SaveCompleteEvent {
                    success: false,
                    path: None,
                    error: Some(e.to_string()),
                });
            }
        }
    }
}

/// System to handle load requests
fn handle_load_events(
    mut load_events: MessageReader<LoadGameEvent>,
    mut complete_events: MessageWriter<LoadCompleteEvent>,
    save_manager: Res<SaveManager>,
) {
    for event in load_events.read() {
        match save_manager.load_game(&event.filename) {
            Ok(save) => {
                complete_events.write(LoadCompleteEvent {
                    success: true,
                    save: Some(save),
                    error: None,
                });
            }
            Err(e) => {
                error!("Failed to load game: {}", e);
                complete_events.write(LoadCompleteEvent {
                    success: false,
                    save: None,
                    error: Some(e.to_string()),
                });
            }
        }
    }
}
