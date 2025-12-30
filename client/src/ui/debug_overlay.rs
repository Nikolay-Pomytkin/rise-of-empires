//! Debug overlay showing camera, entity, and rendering info

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::camera::{CameraState, MainCamera};
use crate::render::{layers, HasVisual};

/// Debug overlay UI - shows camera position, zoom, entity counts
pub fn ui_debug_overlay(
    mut contexts: EguiContexts,
    camera_query: Query<(&Transform, &CameraState, &Projection), With<MainCamera>>,
    windows: Query<&Window>,
    // Entity counts
    all_entities: Query<Entity>,
    sim_entities: Query<Entity, With<sim::SimEntity>>,
    visual_entities: Query<Entity, With<HasVisual>>,
    sprites: Query<(Entity, &Sprite, &Transform)>,
    units: Query<&sim::Unit>,
    buildings: Query<(&sim::Building, &Transform)>,
    resources: Query<&sim::ResourceNode>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    egui::Window::new("🔧 Debug")
        .anchor(egui::Align2::RIGHT_TOP, [-10.0, 40.0])
        .resizable(false)
        .collapsible(true)
        .default_open(true)
        .show(ctx, |ui| {
            // Camera info
            ui.heading("Camera");
            if let Ok((transform, state, projection)) = camera_query.single() {
                ui.label(format!(
                    "Position: ({:.0}, {:.0}, {:.0})",
                    transform.translation.x, transform.translation.y, transform.translation.z
                ));
                ui.label(format!("Zoom: {:.2}", state.zoom));
                
                if let Projection::Orthographic(ortho) = projection {
                    ui.label(format!("Scale: {:.2}", ortho.scale));
                    ui.label(format!("Near/Far: {:.0}/{:.0}", ortho.near, ortho.far));
                }
            } else {
                ui.label("Camera not found!");
            }

            ui.separator();
            
            // Z-Layer reference
            ui.heading("Z Layers");
            ui.label(format!("Ground: {}", layers::GROUND));
            ui.label(format!("Grid: {}", layers::GRID_LINES));
            ui.label(format!("Resources: {}", layers::RESOURCES));
            ui.label(format!("Buildings: {}", layers::BUILDINGS));
            ui.label(format!("Units: {}-{}", layers::UNITS_BASE, layers::UNITS_MAX));

            ui.separator();

            // Window info
            ui.heading("Window");
            if let Ok(window) = windows.single() {
                ui.label(format!("Size: {}x{}", window.width() as u32, window.height() as u32));
                if let Some(pos) = window.cursor_position() {
                    ui.label(format!("Cursor: ({:.0}, {:.0})", pos.x, pos.y));
                }
            }

            ui.separator();

            // Entity counts
            ui.heading("Entities");
            ui.label(format!("Total: {}", all_entities.iter().count()));
            ui.label(format!("SimEntity: {}", sim_entities.iter().count()));
            ui.label(format!("HasVisual: {}", visual_entities.iter().count()));
            ui.label(format!("Sprites: {}", sprites.iter().count()));
            
            ui.separator();
            
            ui.heading("Game Objects");
            ui.label(format!("Units: {}", units.iter().count()));
            ui.label(format!("Buildings: {}", buildings.iter().count()));
            ui.label(format!("Resources: {}", resources.iter().count()));
            
            // Show first building's transform
            ui.separator();
            ui.heading("First Building");
            if let Some((building, transform)) = buildings.iter().next() {
                ui.label(format!("Type: {:?}", building.building_type));
                ui.label(format!(
                    "Pos: ({:.0}, {:.0}, Z={:.1})",
                    transform.translation.x, transform.translation.y, transform.translation.z
                ));
            } else {
                ui.label("No buildings");
            }
            
            // Show first few sprites with Z values
            ui.separator();
            ui.heading("Sample Sprites");
            for (i, (_entity, sprite, transform)) in sprites.iter().take(5).enumerate() {
                let size_str = sprite.custom_size
                    .map(|s| format!("{:.0}x{:.0}", s.x, s.y))
                    .unwrap_or_else(|| "default".to_string());
                ui.label(format!(
                    "#{}: ({:.0},{:.0}) Z={:.1} [{}]",
                    i,
                    transform.translation.x,
                    transform.translation.y, 
                    transform.translation.z,
                    size_str
                ));
            }
        });
}
