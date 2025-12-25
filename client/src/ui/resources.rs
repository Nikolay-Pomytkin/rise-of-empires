//! Resources UI panel

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::input::SelectionState;

/// Display player resources at the top of the screen
pub fn ui_resources_panel(
    mut contexts: EguiContexts,
    sim_world: Res<sim::SimWorld>,
    selection_state: Res<SelectionState>,
) {
    let Some(player) = sim_world.get_player(selection_state.active_player) else {
        return;
    };

    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };
    egui::TopBottomPanel::top("resources_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 20.0;

            // Food
            ui.horizontal(|ui| {
                ui.colored_label(egui::Color32::from_rgb(100, 200, 100), "🍖");
                ui.label(format!("{}", player.resources.food));
            });

            // Wood
            ui.horizontal(|ui| {
                ui.colored_label(egui::Color32::from_rgb(139, 90, 43), "🪵");
                ui.label(format!("{}", player.resources.wood));
            });

            // Gold
            ui.horizontal(|ui| {
                ui.colored_label(egui::Color32::from_rgb(255, 215, 0), "🪙");
                ui.label(format!("{}", player.resources.gold));
            });

            // Stone
            ui.horizontal(|ui| {
                ui.colored_label(egui::Color32::from_rgb(128, 128, 128), "🪨");
                ui.label(format!("{}", player.resources.stone));
            });

            ui.separator();

            // Population
            ui.horizontal(|ui| {
                ui.label("👥");
                ui.label(format!("{}/{}", player.population, player.population_cap));
            });

            ui.separator();

            // Age
            ui.label(format!("Age: {}", player.current_age.0));
        });
    });
}
