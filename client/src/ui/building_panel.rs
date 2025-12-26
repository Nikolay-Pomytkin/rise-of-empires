use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::input::BuildingPlacementState;

pub fn ui_building_panel(
    mut contexts: EguiContexts,
    mut placement: ResMut<BuildingPlacementState>,
) {
    let Ok(ctx) = contexts.ctx_mut() else { return };

    egui::TopBottomPanel::bottom("building_panel").show(ctx, |ui| {
        ui.horizontal_centered(|ui| {
            ui.label("Build:");

            if ui.button("Town Center").clicked() {
                placement.placing = Some(shared::BuildingType::TownCenter);
            }

            if ui.button("Barracks").clicked() {
                placement.placing = Some(shared::BuildingType::Barracks);
            }
            
            if placement.placing.is_some() {
                ui.separator();
                if ui.button("Cancel [Esc]").clicked() {
                    placement.placing = None;
                }
            }
        });
    });
}