//! Camera controller
//!
//! 3D orthographic camera with pan and zoom controls.

use bevy::prelude::*;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraSettings::default())
            .add_systems(Startup, setup_camera)
            .add_systems(Update, (camera_pan, camera_zoom, camera_edge_pan));
    }
}

/// Camera configuration
#[derive(Resource)]
pub struct CameraSettings {
    /// Pan speed (units per second)
    pub pan_speed: f32,
    /// Zoom speed
    pub zoom_speed: f32,
    /// Minimum zoom (closest)
    pub min_zoom: f32,
    /// Maximum zoom (furthest)
    pub max_zoom: f32,
    /// Edge pan margin in pixels
    pub edge_pan_margin: f32,
    /// Enable edge panning
    pub edge_pan_enabled: bool,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            pan_speed: 20.0,
            zoom_speed: 0.01,
            min_zoom: 0.01,
            max_zoom: 0.2,
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
        Self { zoom: 0.05 } // Smaller = more zoomed in for orthographic
    }
}

fn setup_camera(mut commands: Commands) {
    // Spawn 3D orthographic camera at 45-degree angle
    let camera_state = CameraState::default();
    let zoom = camera_state.zoom;

    // Position camera looking down at ~45 degrees from above
    let camera_height = 30.0;
    let camera_back = 30.0;

    let camera_pos = Vec3::new(0.0, camera_height, camera_back);

    commands.spawn((
        Camera3d::default(),
        Projection::Orthographic(OrthographicProjection {
            scale: zoom,
            near: -1000.0,
            far: 1000.0,
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_translation(camera_pos).looking_at(Vec3::ZERO, Vec3::Y),
        MainCamera,
        camera_state,
    ));

    // Add ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 500.0,
        affects_lightmapped_meshes: false,
    });

    // Add directional light (sun)
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(1.0, 0.95, 0.9),
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

/// Handle keyboard camera panning (WASD / Arrow keys)
fn camera_pan(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    settings: Res<CameraSettings>,
    mut camera: Query<&mut Transform, With<MainCamera>>,
) {
    let Ok(mut transform) = camera.single_mut() else {
        return;
    };

    let mut direction = Vec3::ZERO;

    // Get camera's forward and right vectors projected onto XZ plane
    let forward = transform.forward();
    let right = transform.right();

    // Project onto XZ plane for RTS-style movement
    let forward_xz = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
    let right_xz = Vec3::new(right.x, 0.0, right.z).normalize_or_zero();

    // WASD / Arrow keys
    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        direction += forward_xz;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        direction -= forward_xz;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        direction -= right_xz;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        direction += right_xz;
    }

    if direction != Vec3::ZERO {
        direction = direction.normalize();
        transform.translation += direction * settings.pan_speed * time.delta_secs();
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

        // Zoom in/out
        state.zoom -= scroll_amount * settings.zoom_speed;
        state.zoom = state.zoom.clamp(settings.min_zoom, settings.max_zoom);

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
    mut camera: Query<&mut Transform, With<MainCamera>>,
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

    let Ok(mut transform) = camera.single_mut() else {
        return;
    };

    let margin = settings.edge_pan_margin;
    let width = window.width();
    let height = window.height();

    let mut direction = Vec3::ZERO;

    // Get camera's forward and right vectors projected onto XZ plane
    let forward = transform.forward();
    let right = transform.right();
    let forward_xz = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
    let right_xz = Vec3::new(right.x, 0.0, right.z).normalize_or_zero();

    // Check edges
    if cursor_pos.x < margin {
        direction -= right_xz;
    } else if cursor_pos.x > width - margin {
        direction += right_xz;
    }

    if cursor_pos.y < margin {
        direction += forward_xz;
    } else if cursor_pos.y > height - margin {
        direction -= forward_xz;
    }

    if direction != Vec3::ZERO {
        direction = direction.normalize();
        transform.translation += direction * settings.pan_speed * 0.5 * time.delta_secs();
    }
}

