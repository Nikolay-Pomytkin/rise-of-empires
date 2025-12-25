//! Grid rendering

use bevy::prelude::*;

/// Grid configuration
#[derive(Resource, Clone)]
pub struct GridConfig {
    pub size: u32,
    pub tile_size: f32,
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            size: 50,
            tile_size: 1.0,
        }
    }
}

/// Setup the tile grid
pub fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let config = GridConfig::default();
    commands.insert_resource(config.clone());

    let half_size = (config.size as f32 * config.tile_size) / 2.0;

    // Create ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(config.size as f32 * config.tile_size, config.size as f32 * config.tile_size))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.15, 0.25, 0.1),
            perceptual_roughness: 0.9,
            ..default()
        })),
        Transform::from_xyz(0.0, -0.01, 0.0),
    ));

    // Create grid lines
    let grid_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.2, 0.2, 0.2, 0.3),
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    // Create line mesh (thin box)
    let line_thickness = 0.02;
    let line_height = 0.001;

    // Horizontal lines (along X)
    for i in 0..=config.size {
        let z = -half_size + i as f32 * config.tile_size;
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(config.size as f32 * config.tile_size, line_height, line_thickness))),
            MeshMaterial3d(grid_material.clone()),
            Transform::from_xyz(0.0, 0.0, z),
        ));
    }

    // Vertical lines (along Z)
    for i in 0..=config.size {
        let x = -half_size + i as f32 * config.tile_size;
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(line_thickness, line_height, config.size as f32 * config.tile_size))),
            MeshMaterial3d(grid_material.clone()),
            Transform::from_xyz(x, 0.0, 0.0),
        ));
    }
}

