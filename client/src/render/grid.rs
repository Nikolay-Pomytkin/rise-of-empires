//! Grid rendering (2D)
//!
//! Z-ordering layers (higher Z = rendered on top):
//! - Ground plane: Z = 0
//! - Grid lines: Z = 1
//! - Resources: Z = 2
//! - Buildings: Z = 3  
//! - Units: Z = 4-10
//! - UI/Selection: Z = 100+

use bevy::prelude::*;

use super::TILE_SIZE;

/// Z-layer constants for consistent ordering
pub mod layers {
    pub const GROUND: f32 = 0.0;
    pub const GRID_LINES: f32 = 1.0;
    pub const RESOURCES: f32 = 2.0;
    pub const BUILDINGS: f32 = 3.0;
    pub const UNITS_BASE: f32 = 4.0;
    pub const UNITS_MAX: f32 = 10.0;
    pub const SELECTION: f32 = 100.0;
    pub const PLACEMENT_GHOST: f32 = 101.0;
}

/// Grid configuration
#[derive(Resource, Clone)]
pub struct GridConfig {
    pub size: u32,
    pub tile_size: f32,
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            size: 200, // 200x200 tile map
            tile_size: 1.0,
        }
    }
}

impl GridConfig {
    /// Get the half-size of the map in world units (pixels)
    pub fn half_size(&self) -> f32 {
        (self.size as f32 * self.tile_size * TILE_SIZE) / 2.0
    }

    /// Get the map bounds in pixels (min_x, max_x, min_y, max_y)
    pub fn bounds(&self) -> (f32, f32, f32, f32) {
        let half = self.half_size();
        (-half, half, -half, half)
    }
}

/// Marker for ground plane entity
#[derive(Component)]
pub struct GroundPlane;

/// Marker for grid line entities
#[derive(Component)]
pub struct GridLine;

/// Setup the tile grid (2D ground plane)
pub fn setup_grid(mut commands: Commands) {
    let config = GridConfig::default();
    commands.insert_resource(config.clone());

    let map_size = config.size as f32 * config.tile_size * TILE_SIZE;
    
    bevy::log::info!("Setting up grid: {} tiles, {} pixels, half_size={}", 
                     config.size, map_size, config.half_size());

    // Create ground plane at Z=0 (bottom layer)
    commands.spawn((
        Sprite {
            color: Color::srgb(0.15, 0.25, 0.1), // Dark green grass
            custom_size: Some(Vec2::new(map_size, map_size)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, layers::GROUND),
        GroundPlane,
    ));
    
    bevy::log::info!("Ground plane spawned at Z={}", layers::GROUND);

    // Create grid lines (every 10 tiles) at Z=1
    let grid_spacing = 10;
    let line_color = Color::srgba(0.3, 0.3, 0.3, 0.5);
    let line_thickness = 2.0;
    let half_size = config.half_size();

    // Horizontal lines
    for i in (0..=config.size).step_by(grid_spacing) {
        let y = -half_size + i as f32 * config.tile_size * TILE_SIZE;
        commands.spawn((
            Sprite {
                color: line_color,
                custom_size: Some(Vec2::new(map_size, line_thickness)),
                ..default()
            },
            Transform::from_xyz(0.0, y, layers::GRID_LINES),
            GridLine,
        ));
    }

    // Vertical lines
    for i in (0..=config.size).step_by(grid_spacing) {
        let x = -half_size + i as f32 * config.tile_size * TILE_SIZE;
        commands.spawn((
            Sprite {
                color: line_color,
                custom_size: Some(Vec2::new(line_thickness, map_size)),
                ..default()
            },
            Transform::from_xyz(x, 0.0, layers::GRID_LINES),
            GridLine,
        ));
    }
    
    bevy::log::info!("Grid lines spawned at Z={}", layers::GRID_LINES);
    bevy::log::info!("Grid setup complete!");
}
