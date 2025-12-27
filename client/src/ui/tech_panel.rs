//! Tech tree UI panel
//!
//! Collapsible panel at bottom-left showing available techs and ages

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::input::SelectionState;

const PANEL_BG: egui::Color32 = egui::Color32::from_rgba_premultiplied(20, 20, 30, 230);
const HEADER_COLOR: egui::Color32 = egui::Color32::from_rgb(255, 220, 150);
const TEXT_COLOR: egui::Color32 = egui::Color32::from_rgb(220, 220, 220);
const AVAILABLE_COLOR: egui::Color32 = egui::Color32::from_rgb(150, 255, 150);
const RESEARCHED_COLOR: egui::Color32 = egui::Color32::from_rgb(100, 200, 255);
const UNAVAILABLE_COLOR: egui::Color32 = egui::Color32::from_rgb(120, 120, 140);

/// Resource to track if tech panel is expanded
#[derive(Resource, Default)]
pub struct TechPanelState {
    pub expanded: bool,
}

/// Display available technologies in a collapsible bottom panel
pub fn ui_tech_panel(
    mut contexts: EguiContexts,
    sim_world: Res<sim::SimWorld>,
    selection_state: Res<SelectionState>,
    mut panel_state: ResMut<TechPanelState>,
) {
    let Some(player) = sim_world.get_player(selection_state.active_player) else {
        return;
    };

    // Get default tech tree
    let tech_tree = sim::TechTree::default();

    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    let screen_rect = ctx.screen_rect();
    let panel_x = 200.0; // To the right of build panel
    let collapsed_height = 30.0;
    let expanded_height = 180.0;
    let panel_width = 300.0;

    let current_height = if panel_state.expanded {
        expanded_height
    } else {
        collapsed_height
    };

    egui::Area::new(egui::Id::new("tech_panel"))
        .fixed_pos(egui::pos2(panel_x, screen_rect.height() - current_height - 10.0))
        .show(ctx, |ui| {
            egui::Frame::none()
                .fill(PANEL_BG)
                .rounding(egui::Rounding::same(4))
                .inner_margin(egui::Margin::same(8))
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 60, 80)))
                .show(ui, |ui| {
                    ui.set_min_width(panel_width);

                    // Header with collapse button
                    ui.horizontal(|ui| {
                        let arrow = if panel_state.expanded { "▼" } else { "▶" };
                        if ui
                            .add(egui::Button::new(
                                egui::RichText::new(format!("{} Technologies", arrow))
                                    .color(HEADER_COLOR)
                                    .size(14.0),
                            ).frame(false))
                            .clicked()
                        {
                            panel_state.expanded = !panel_state.expanded;
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            // Current age badge
                            ui.colored_label(
                                RESEARCHED_COLOR,
                                egui::RichText::new(&player.current_age.0).size(12.0),
                            );
                        });
                    });

                    // Expanded content
                    if panel_state.expanded {
                        ui.add_space(5.0);
                        ui.separator();

                        // Scrollable area for techs
                        egui::ScrollArea::vertical()
                            .max_height(expanded_height - 60.0)
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    // Left column: Technologies
                                    ui.vertical(|ui| {
                                        ui.colored_label(TEXT_COLOR, "Techs:");
                                        for tech in &tech_tree.techs {
                                            let can_research = tech_tree.can_research(
                                                &tech.id,
                                                &player.current_age.0,
                                                &player.researched_techs,
                                            );
                                            let already_researched =
                                                player.researched_techs.contains(&tech.id);

                                            if already_researched {
                                                ui.colored_label(
                                                    RESEARCHED_COLOR,
                                                    format!("✓ {}", tech.name),
                                                );
                                            } else if can_research {
                                                let btn = ui.add(
                                                    egui::Button::new(
                                                        egui::RichText::new(&tech.name)
                                                            .color(AVAILABLE_COLOR),
                                                    )
                                                    .min_size(egui::vec2(100.0, 20.0)),
                                                );
                                                if btn.clicked() {
                                                    info!("Research {} clicked", tech.name);
                                                }
                                                if btn.hovered() {
                                                    show_tech_tooltip(ui, tech);
                                                }
                                            } else {
                                                ui.colored_label(UNAVAILABLE_COLOR, &tech.name);
                                            }
                                        }
                                    });

                                    ui.add_space(20.0);

                                    // Right column: Ages
                                    ui.vertical(|ui| {
                                        ui.colored_label(TEXT_COLOR, "Ages:");
                                        for age in &tech_tree.ages {
                                            let is_current = player.current_age.0 == age.id;
                                            let can_advance = !is_current
                                                && age
                                                    .requires
                                                    .as_ref()
                                                    .map(|r| r == &player.current_age.0)
                                                    .unwrap_or(true);

                                            if is_current {
                                                ui.colored_label(
                                                    RESEARCHED_COLOR,
                                                    format!("▶ {}", age.name),
                                                );
                                            } else if can_advance && age.cost.food > 0 {
                                                let btn = ui.add(
                                                    egui::Button::new(
                                                        egui::RichText::new(format!(
                                                            "→ {}",
                                                            age.name
                                                        ))
                                                        .color(AVAILABLE_COLOR),
                                                    )
                                                    .min_size(egui::vec2(100.0, 20.0)),
                                                );
                                                if btn.clicked() {
                                                    info!("Advance to {} clicked", age.name);
                                                }
                                                if btn.hovered() {
                                                    egui::show_tooltip(
                                                        ui.ctx(),
                                                        ui.layer_id(),
                                                        egui::Id::new(&age.id),
                                                        |ui| {
                                                            ui.label(format!(
                                                                "Cost: {}F {}W {}G {}S",
                                                                age.cost.food,
                                                                age.cost.wood,
                                                                age.cost.gold,
                                                                age.cost.stone
                                                            ));
                                                        },
                                                    );
                                                }
                                            } else {
                                                ui.colored_label(UNAVAILABLE_COLOR, &age.name);
                                            }
                                        }
                                    });
                                });
                            });
                    }
                });
        });
}

fn show_tech_tooltip(ui: &egui::Ui, tech: &sim::TechDef) {
    egui::show_tooltip(
        ui.ctx(),
        ui.layer_id(),
        egui::Id::new(&tech.id),
        |ui| {
            ui.label(&tech.description);
            ui.label(format!(
                "Cost: {}F {}W {}G {}S",
                tech.cost.food, tech.cost.wood, tech.cost.gold, tech.cost.stone
            ));
            if let Some(ref age) = tech.required_age {
                ui.label(format!("Requires: {}", age));
            }
        },
    );
}
