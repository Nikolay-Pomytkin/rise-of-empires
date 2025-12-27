//! Resources UI panel
//!
//! Top-left: Resources with amounts and income
//! Top-center: Age and nation
//! Top-right: Population

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::input::SelectionState;

/// Custom dark theme colors
const PANEL_BG: egui::Color32 = egui::Color32::from_rgba_premultiplied(20, 20, 30, 230);
const RESOURCE_TEXT: egui::Color32 = egui::Color32::from_rgb(255, 255, 220);
const INCOME_POSITIVE: egui::Color32 = egui::Color32::from_rgb(100, 255, 100);
const INCOME_NEGATIVE: egui::Color32 = egui::Color32::from_rgb(255, 100, 100);

// Resource colors
const FOOD_COLOR: egui::Color32 = egui::Color32::from_rgb(150, 255, 150);
const WOOD_COLOR: egui::Color32 = egui::Color32::from_rgb(180, 140, 90);
const GOLD_COLOR: egui::Color32 = egui::Color32::from_rgb(255, 215, 0);
const STONE_COLOR: egui::Color32 = egui::Color32::from_rgb(160, 160, 180);
const KNOWLEDGE_COLOR: egui::Color32 = egui::Color32::from_rgb(100, 180, 255);
const METAL_COLOR: egui::Color32 = egui::Color32::from_rgb(200, 200, 220);

/// Display player resources
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

    // Configure dark style
    let mut style = (*ctx.style()).clone();
    style.visuals.window_fill = PANEL_BG;
    style.visuals.panel_fill = PANEL_BG;
    ctx.set_style(style);

    // === TOP LEFT: Resources Panel ===
    egui::Area::new(egui::Id::new("resources_area"))
        .fixed_pos(egui::pos2(10.0, 10.0))
        .show(ctx, |ui| {
            egui::Frame::none()
                .fill(PANEL_BG)
                .rounding(egui::Rounding::same(4))
                .inner_margin(egui::Margin::same(8))
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 60, 80)))
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;

                        // Food
                        resource_row(ui, "🍖", "Food", player.resources.food, 0, FOOD_COLOR);

                        // Wood
                        resource_row(ui, "🪵", "Wood", player.resources.wood, 0, WOOD_COLOR);

                        // Gold
                        resource_row(ui, "🪙", "Gold", player.resources.gold, 0, GOLD_COLOR);

                        // Stone
                        resource_row(ui, "🪨", "Stone", player.resources.stone, 0, STONE_COLOR);
                    });
                });
        });

    // === TOP CENTER: Age Banner ===
    let screen_width = ctx.screen_rect().width();
    egui::Area::new(egui::Id::new("age_banner"))
        .fixed_pos(egui::pos2(screen_width / 2.0 - 100.0, 10.0))
        .show(ctx, |ui| {
            egui::Frame::none()
                .fill(PANEL_BG)
                .rounding(egui::Rounding::same(4))
                .inner_margin(egui::Margin::same(8))
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(80, 80, 100)))
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.set_min_width(180.0);

                        // Nation/Player name
                        ui.horizontal(|ui| {
                            ui.add_space(40.0);
                            ui.colored_label(
                                egui::Color32::from_rgb(200, 180, 140),
                                egui::RichText::new("Player 1").size(14.0),
                            );
                        });

                        // Current Age (prominently displayed)
                        ui.horizontal(|ui| {
                            ui.add_space(20.0);
                            ui.colored_label(
                                egui::Color32::from_rgb(255, 220, 150),
                                egui::RichText::new(format!("{}", player.current_age.0))
                                    .size(18.0)
                                    .strong(),
                            );
                        });
                    });
                });
        });

    // === TOP RIGHT: Population ===
    egui::Area::new(egui::Id::new("population_area"))
        .fixed_pos(egui::pos2(screen_width - 120.0, 10.0))
        .show(ctx, |ui| {
            egui::Frame::none()
                .fill(PANEL_BG)
                .rounding(egui::Rounding::same(4))
                .inner_margin(egui::Margin::same(8))
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 60, 80)))
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        // Population icon and count
                        ui.horizontal(|ui| {
                            ui.colored_label(
                                egui::Color32::from_rgb(200, 200, 255),
                                egui::RichText::new("👥").size(16.0),
                            );
                            ui.colored_label(
                                RESOURCE_TEXT,
                                egui::RichText::new(format!(
                                    "{}/{}",
                                    player.population, player.population_cap
                                ))
                                .size(16.0)
                                .strong(),
                            );
                        });
                    });
                });
        });
}

/// Draw a single resource row with icon, amount, and income
fn resource_row(
    ui: &mut egui::Ui,
    icon: &str,
    _name: &str,
    amount: u32,
    income: i32,
    color: egui::Color32,
) {
    ui.horizontal(|ui| {
        // Icon
        ui.colored_label(color, egui::RichText::new(icon).size(14.0));

        // Amount (right-aligned, fixed width)
        let amount_text = format!("{}", amount);
        ui.add_space(5.0);
        ui.colored_label(
            RESOURCE_TEXT,
            egui::RichText::new(&amount_text).size(14.0).strong(),
        );

        // Income rate (if non-zero)
        if income != 0 {
            let income_color = if income > 0 {
                INCOME_POSITIVE
            } else {
                INCOME_NEGATIVE
            };
            let income_text = if income > 0 {
                format!("+{}", income)
            } else {
                format!("{}", income)
            };
            ui.colored_label(income_color, egui::RichText::new(&income_text).size(11.0));
        }
    });
}
