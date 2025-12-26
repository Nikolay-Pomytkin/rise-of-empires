//! Tech tree UI panel

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::input::SelectionState;

/// Display available technologies
pub fn ui_tech_panel(
    mut contexts: EguiContexts,
    sim_world: Res<sim::SimWorld>,
    selection_state: Res<SelectionState>,
) {
    let Some(player) = sim_world.get_player(selection_state.active_player) else {
        return;
    };

    // Get default tech tree
    let tech_tree = sim::TechTree::default();

    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };
    egui::SidePanel::left("tech_panel")
        .min_width(180.0)
        .show(ctx, |ui| {
            ui.heading("Technologies");
            ui.separator();

            ui.label(format!("Current Age: {}", player.current_age.0));
            ui.separator();

            ui.label("Available Techs:");

            // Show available techs
            for tech in &tech_tree.techs {
                let can_research = tech_tree.can_research(
                    &tech.id,
                    &player.current_age.0,
                    &player.researched_techs,
                );

                let already_researched = player.researched_techs.contains(&tech.id);

                ui.horizontal(|ui| {
                    if already_researched {
                        ui.label(format!("✓ {}", tech.name));
                    } else {
                        let button = ui.add_enabled(can_research, egui::Button::new(&tech.name));

                        if button.clicked() {
                            // TODO: Queue tech research
                            info!("Research {} clicked", tech.name);
                        }

                        if button.hovered() {
                            egui::show_tooltip(
                                ui.ctx(),
                                ui.layer_id(),
                                egui::Id::new(&tech.id),
                                |ui| {
                                    ui.label(&tech.description);
                                    ui.label(format!(
                                        "Cost: {}F {}W {}G {}S",
                                        tech.cost.food,
                                        tech.cost.wood,
                                        tech.cost.gold,
                                        tech.cost.stone
                                    ));
                                    if let Some(ref age) = tech.required_age {
                                        ui.label(format!("Requires: {}", age));
                                    }
                                },
                            );
                        }
                    }
                });
            }

            ui.separator();

            // Age advancement
            ui.label("Ages:");
            for age in &tech_tree.ages {
                let is_current = player.current_age.0 == age.id;
                let can_advance = !is_current
                    && age
                        .requires
                        .as_ref()
                        .map(|r| r == &player.current_age.0)
                        .unwrap_or(true);

                ui.horizontal(|ui| {
                    if is_current {
                        ui.label(format!("→ {}", age.name));
                    } else if can_advance && age.cost.food > 0 {
                        if ui.button(format!("Advance to {}", age.name)).clicked() {
                            // TODO: Queue age advancement
                            info!("Advance to {} clicked", age.name);
                        }
                    } else {
                        ui.label(&age.name);
                    }
                });
            }
        });
}
