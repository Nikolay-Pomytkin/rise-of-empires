//! Animation system for sprite sheet animations
//!
//! Supports:
//! - Multi-directional sprites (8 directions for isometric view)
//! - Multiple animation states (idle, walk, attack, gather, etc.)
//! - Configurable frame timing
//! - Texture atlas based sprite sheets

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =============================================================================
// Data Types for Animation Definitions (loaded from .roe files)
// =============================================================================

/// Animation definition for a single animation state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationDef {
    pub frames: usize,
    pub frame_duration: f32,
    pub looping: bool,
}

/// Entity animation definition (unit, building, or resource)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityAnimationDef {
    pub sprite_sheet: String,
    pub frame_size: (u32, u32),
    pub animations: HashMap<String, AnimationDef>,
    pub directions: usize,
}

/// Root animation data structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnimationDataFile {
    pub units: HashMap<String, EntityAnimationDef>,
    pub buildings: HashMap<String, EntityAnimationDef>,
    pub resources: HashMap<String, EntityAnimationDef>,
}

/// Resource containing loaded animation data
#[derive(Resource, Default)]
pub struct AnimationData {
    pub data: AnimationDataFile,
    /// Loaded texture atlas handles keyed by sprite sheet path
    pub atlases: HashMap<String, Handle<TextureAtlasLayout>>,
    /// Loaded texture handles keyed by sprite sheet path
    pub textures: HashMap<String, Handle<Image>>,
}

impl AnimationData {
    /// Get animation definition for a unit type
    pub fn get_unit(&self, unit_type: &str) -> Option<&EntityAnimationDef> {
        self.data.units.get(unit_type)
    }

    /// Get animation definition for a building type
    pub fn get_building(&self, building_type: &str) -> Option<&EntityAnimationDef> {
        self.data.buildings.get(building_type)
    }

    /// Get animation definition for a resource type
    pub fn get_resource(&self, resource_type: &str) -> Option<&EntityAnimationDef> {
        self.data.resources.get(resource_type)
    }
}

// =============================================================================
// Direction and State Enums
// =============================================================================

/// 8 directions for isometric sprites
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, Component)]
pub enum FacingDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    #[default]
    South,
    SouthWest,
    West,
    NorthWest,
}

impl FacingDirection {
    /// Get direction index (0-7) for sprite sheet row selection
    pub fn index(&self) -> usize {
        match self {
            FacingDirection::South => 0,
            FacingDirection::SouthWest => 1,
            FacingDirection::West => 2,
            FacingDirection::NorthWest => 3,
            FacingDirection::North => 4,
            FacingDirection::NorthEast => 5,
            FacingDirection::East => 6,
            FacingDirection::SouthEast => 7,
        }
    }

    /// Get direction from a movement vector
    pub fn from_vec2(dir: Vec2) -> Self {
        if dir.length_squared() < 0.001 {
            return FacingDirection::South;
        }

        let angle = dir.y.atan2(dir.x).to_degrees();
        
        // Convert angle to 8 directions
        // 0 degrees = East, 90 = North, etc.
        let normalized = if angle < 0.0 { angle + 360.0 } else { angle };
        
        match normalized as i32 {
            338..=360 | 0..=22 => FacingDirection::East,
            23..=67 => FacingDirection::NorthEast,
            68..=112 => FacingDirection::North,
            113..=157 => FacingDirection::NorthWest,
            158..=202 => FacingDirection::West,
            203..=247 => FacingDirection::SouthWest,
            248..=292 => FacingDirection::South,
            293..=337 => FacingDirection::SouthEast,
            _ => FacingDirection::South,
        }
    }

    /// Get all directions in order
    pub fn all() -> [FacingDirection; 8] {
        [
            FacingDirection::South,
            FacingDirection::SouthWest,
            FacingDirection::West,
            FacingDirection::NorthWest,
            FacingDirection::North,
            FacingDirection::NorthEast,
            FacingDirection::East,
            FacingDirection::SouthEast,
        ]
    }
}

/// Animation state types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, Component)]
pub enum AnimationState {
    #[default]
    Idle,
    Walk,
    Attack,
    Gather,
    Die,
    Build,      // For buildings under construction
    Produce,    // For buildings producing units
}

impl AnimationState {
    /// Get row offset for this animation state in the sprite sheet
    /// Each state occupies a set of rows (one per direction)
    pub fn row_offset(&self, directions_per_state: usize) -> usize {
        let state_index = match self {
            AnimationState::Idle => 0,
            AnimationState::Walk => 1,
            AnimationState::Attack => 2,
            AnimationState::Gather => 3,
            AnimationState::Die => 4,
            AnimationState::Build => 0,   // Buildings don't have directions
            AnimationState::Produce => 1,
        };
        state_index * directions_per_state
    }
}

/// Timer for animation frame progression
#[derive(Component)]
pub struct AnimationTimer {
    pub timer: Timer,
}

impl Default for AnimationTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        }
    }
}

impl AnimationTimer {
    pub fn new(frame_duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(frame_duration, TimerMode::Repeating),
        }
    }
}

/// Current animation frame index
#[derive(Component, Default)]
pub struct AnimationFrame {
    pub current: usize,
}

/// Configuration for an animated sprite
#[derive(Component, Clone)]
pub struct AnimationConfig {
    /// Number of frames per animation
    pub frames_per_animation: usize,
    /// Number of directions (1 for buildings, 8 for units)
    pub directions: usize,
    /// Columns in the sprite sheet
    pub columns: usize,
    /// Whether to loop the animation
    pub looping: bool,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            frames_per_animation: 8,
            directions: 8,
            columns: 8,
            looping: true,
        }
    }
}

impl AnimationConfig {
    /// Create config for a unit (8 directions)
    pub fn unit(frames: usize) -> Self {
        Self {
            frames_per_animation: frames,
            directions: 8,
            columns: frames,
            looping: true,
        }
    }

    /// Create config for a building (1 direction, stages)
    pub fn building(stages: usize) -> Self {
        Self {
            frames_per_animation: stages,
            directions: 1,
            columns: stages,
            looping: false,
        }
    }

    /// Create config for a resource node (1 direction, depletion stages)
    pub fn resource(stages: usize) -> Self {
        Self {
            frames_per_animation: stages,
            directions: 1,
            columns: stages,
            looping: false,
        }
    }

    /// Calculate the sprite sheet index for current state
    pub fn get_atlas_index(
        &self,
        state: &AnimationState,
        direction: &FacingDirection,
        frame: usize,
    ) -> usize {
        let dir_index = if self.directions > 1 {
            direction.index()
        } else {
            0
        };
        
        let row = state.row_offset(self.directions) + dir_index;
        let col = frame % self.frames_per_animation;
        
        row * self.columns + col
    }
}

/// Bundle for animated sprites (components needed for animation)
#[derive(Bundle, Default)]
pub struct AnimationBundle {
    pub animation_state: AnimationState,
    pub facing_direction: FacingDirection,
    pub animation_timer: AnimationTimer,
    pub animation_frame: AnimationFrame,
    pub animation_config: AnimationConfig,
}

impl AnimationBundle {
    pub fn new(config: AnimationConfig, frame_duration: f32) -> Self {
        Self {
            animation_config: config,
            animation_timer: AnimationTimer::new(frame_duration),
            ..default()
        }
    }
}

// =============================================================================
// Animation Data Loading
// =============================================================================

/// Load animation data from the animations.roe file
#[cfg(not(target_arch = "wasm32"))]
pub fn load_animation_data() -> AnimationData {
    use std::fs;
    use std::path::Path;
    
    let path = Path::new("assets/data/animations.roe");
    
    match fs::read_to_string(path) {
        Ok(content) => {
            match ron::from_str::<AnimationDataFile>(&content) {
                Ok(data) => {
                    bevy::log::info!(
                        "Loaded animation data: {} units, {} buildings, {} resources",
                        data.units.len(),
                        data.buildings.len(),
                        data.resources.len()
                    );
                    AnimationData {
                        data,
                        atlases: HashMap::new(),
                        textures: HashMap::new(),
                    }
                }
                Err(e) => {
                    bevy::log::warn!("Failed to parse animations.roe: {}", e);
                    AnimationData::default()
                }
            }
        }
        Err(e) => {
            bevy::log::warn!("Failed to read animations.roe: {}", e);
            AnimationData::default()
        }
    }
}

/// For WASM, use embedded animation data
#[cfg(target_arch = "wasm32")]
pub fn load_animation_data() -> AnimationData {
    let content = include_str!("../../assets/data/animations.roe");
    
    match ron::from_str::<AnimationDataFile>(content) {
        Ok(data) => {
            bevy::log::info!(
                "Loaded embedded animation data: {} units, {} buildings, {} resources",
                data.units.len(),
                data.buildings.len(),
                data.resources.len()
            );
            AnimationData {
                data,
                atlases: HashMap::new(),
                textures: HashMap::new(),
            }
        }
        Err(e) => {
            bevy::log::warn!("Failed to parse embedded animations.roe: {}", e);
            AnimationData::default()
        }
    }
}

/// System to load animation data at startup
pub fn setup_animation_data(mut commands: Commands) {
    let animation_data = load_animation_data();
    commands.insert_resource(animation_data);
}
