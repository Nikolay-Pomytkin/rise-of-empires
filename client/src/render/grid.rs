//! Grid rendering (2D)

use bevy::prelude::*;

use super::TILE_SIZE;

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

/// Setup the tile grid (2D ground plane)
pub fn setup_grid(mut commands: Commands) {
    let config = GridConfig::default();
    commands.insert_resource(config.clone());

    let map_size = config.size as f32 * config.tile_size * TILE_SIZE;

    // Create ground plane as a colored sprite
    commands.spawn((
        Sprite {
            color: Color::srgb(0.15, 0.25, 0.1), // Dark green grass
            custom_size: Some(Vec2::new(map_size, map_size)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -1000.0), // Far back in Z ordering
    ));

    // Create grid lines (every 10 tiles)
    let grid_spacing = 10;
    let line_color = Color::srgba(0.2, 0.2, 0.2, 0.3);
    let line_thickness = 1.0;
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
            Transform::from_xyz(0.0, y, -999.0),
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
            Transform::from_xyz(x, 0.0, -999.0),
        ));
    }
}
