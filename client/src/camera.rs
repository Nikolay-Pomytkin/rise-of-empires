//! Camera controller
//!
//! 2D orthographic camera with pan and zoom controls.
//! Uses a top-down view with Y-axis representing depth (for sprite sorting).

use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

use crate::game_state::GameState;
use crate::render::GridConfig;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraSettings::default())
            .add_systems(Startup, setup_camera)
            .add_systems(
                Update,
                (camera_pan, camera_zoom, camera_edge_pan, clamp_camera_to_bounds)
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

/// Camera configuration
#[derive(Resource)]
pub struct CameraSettings {
    /// Pan speed (units per second)
    pub pan_speed: f32,
    /// Zoom speed
    pub zoom_speed: f32,
    /// Minimum zoom (closest - larger scale)
    pub min_zoom: f32,
    /// Maximum zoom (furthest - smaller scale)
    pub max_zoom: f32,
    /// Edge pan margin in pixels
    pub edge_pan_margin: f32,
    /// Enable edge panning
    pub edge_pan_enabled: bool,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            pan_speed: 500.0,
            zoom_speed: 0.1,
            min_zoom: 0.5,   // Zoomed in
            max_zoom: 3.0,   // Zoomed out
            edge_pan_margin: 20.0,
            edge_pan_enabled: true,
        }
    }
}

/// Marker for the main game camera
#[derive(Component)]
pub struct MainCamera;

/// Camera state
#[derive(Component)]
pub struct CameraState {
    pub zoom: f32,
}

impl Default for CameraState {
    fn default() -> Self {
        Self { zoom: 1.0 }
    }
}

fn setup_camera(mut commands: Commands) {
    let camera_state = CameraState::default();

    // Spawn 2D camera
    // Camera2d includes default OrthographicProjection
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 0.0),
        MainCamera,
        camera_state,
    ));
}

/// Handle keyboard camera panning (WASD / Arrow keys)
fn camera_pan(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    settings: Res<CameraSettings>,
    mut camera: Query<(&mut Transform, &CameraState), With<MainCamera>>,
) {
    let Ok((mut transform, state)) = camera.single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;

    // WASD / Arrow keys - in 2D, X is left/right, Y is up/down
    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if direction != Vec2::ZERO {
        direction = direction.normalize();
        // Scale pan speed by zoom level so it feels consistent
        let adjusted_speed = settings.pan_speed * state.zoom;
        transform.translation.x += direction.x * adjusted_speed * time.delta_secs();
        transform.translation.y += direction.y * adjusted_speed * time.delta_secs();
    }
}

/// Handle mouse scroll zoom
fn camera_zoom(
    mut scroll_events: EventReader<MouseWheel>,
    settings: Res<CameraSettings>,
    mut camera: Query<(&mut Projection, &mut CameraState), With<MainCamera>>,
) {
    let Ok((mut projection, mut state)) = camera.single_mut() else {
        return;
    };

    for event in scroll_events.read() {
        let scroll_amount = match event.unit {
            MouseScrollUnit::Line => event.y,
            MouseScrollUnit::Pixel => event.y / 100.0,
        };

        // Zoom in/out (scroll up = zoom in = smaller scale)
        state.zoom -= scroll_amount * settings.zoom_speed * state.zoom;
        state.zoom = state.zoom.clamp(settings.min_zoom, settings.max_zoom);

        // Update the projection scale
        if let Projection::Orthographic(ref mut ortho) = *projection {
            ortho.scale = state.zoom;
        }
    }
}

/// Handle edge-of-screen panning
fn camera_edge_pan(
    windows: Query<&Window>,
    settings: Res<CameraSettings>,
    time: Res<Time>,
    mut camera: Query<(&mut Transform, &CameraState), With<MainCamera>>,
) {
    if !settings.edge_pan_enabled {
        return;
    }

    let Ok(window) = windows.single() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let Ok((mut transform, state)) = camera.single_mut() else {
        return;
    };

    let margin = settings.edge_pan_margin;
    let width = window.width();
    let height = window.height();

    let mut direction = Vec2::ZERO;

    // Check edges
    if cursor_pos.x < margin {
        direction.x -= 1.0;
    } else if cursor_pos.x > width - margin {
        direction.x += 1.0;
    }

    // Note: screen Y is inverted (0 at top)
    if cursor_pos.y < margin {
        direction.y += 1.0;
    } else if cursor_pos.y > height - margin {
        direction.y -= 1.0;
    }

    if direction != Vec2::ZERO {
        direction = direction.normalize();
        let adjusted_speed = settings.pan_speed * 0.5 * state.zoom;
        transform.translation.x += direction.x * adjusted_speed * time.delta_secs();
        transform.translation.y += direction.y * adjusted_speed * time.delta_secs();
    }
}

/// Clamp camera position to map boundaries
fn clamp_camera_to_bounds(
    grid_config: Option<Res<GridConfig>>,
    mut camera: Query<(&mut Transform, &CameraState), With<MainCamera>>,
    windows: Query<&Window>,
) {
    let Some(config) = grid_config else {
        return;
    };

    let Ok((mut transform, state)) = camera.single_mut() else {
        return;
    };

    let Ok(window) = windows.single() else {
        return;
    };

    // In 2D, the map is on XY plane
    // Convert grid bounds to 2D (grid uses X/Z, we use X/Y)
    let half_size = config.half_size();
    
    // Calculate visible area based on zoom and window size
    let visible_width = window.width() * state.zoom / 2.0;
    let visible_height = window.height() * state.zoom / 2.0;

    // Clamp camera position
    let min_x = -half_size + visible_width;
    let max_x = half_size - visible_width;
    let min_y = -half_size + visible_height;
    let max_y = half_size - visible_height;

    // Only clamp if the map is larger than the visible area
    if max_x > min_x {
        transform.translation.x = transform.translation.x.clamp(min_x, max_x);
    }
    if max_y > min_y {
        transform.translation.y = transform.translation.y.clamp(min_y, max_y);
    }
}
