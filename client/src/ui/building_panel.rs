//! Building placement panel
//!
//! Bottom bar with building buttons

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::input::BuildingPlacementState;

const PANEL_BG: egui::Color32 = egui::Color32::from_rgba_premultiplied(20, 20, 30, 230);
const BUTTON_SIZE: egui::Vec2 = egui::vec2(70.0, 50.0);

pub fn ui_building_panel(
    mut contexts: EguiContexts,
    mut placement: ResMut<BuildingPlacementState>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    let screen_rect = ctx.screen_rect();

    // Bottom-left command panel
    egui::Area::new(egui::Id::new("building_panel"))
        .fixed_pos(egui::pos2(10.0, screen_rect.height() - 70.0))
        .show(ctx, |ui| {
            egui::Frame::none()
                .fill(PANEL_BG)
                .rounding(egui::Rounding::same(4))
                .inner_margin(egui::Margin::same(8))
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 60, 80)))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        // Build label
                        ui.colored_label(
                            egui::Color32::from_rgb(200, 180, 140),
                            egui::RichText::new("Build:").size(12.0),
                        );

                        ui.add_space(10.0);

                        // Town Center button
                        let tc_selected = placement.placing == Some(shared::BuildingType::TownCenter);
                        let tc_button = egui::Button::new("🏛️\nTC")
                            .min_size(BUTTON_SIZE)
                            .fill(if tc_selected {
                                egui::Color32::from_rgb(60, 80, 60)
                            } else {
                                egui::Color32::from_rgb(40, 40, 50)
                            });

                        if ui
                            .add(tc_button)
                            .on_hover_text("Town Center\n400 Wood, 200 Stone")
                            .clicked()
                        {
                            if tc_selected {
                                placement.placing = None;
                            } else {
                                placement.placing = Some(shared::BuildingType::TownCenter);
                            }
                        }

                        // Barracks button
                        let br_selected = placement.placing == Some(shared::BuildingType::Barracks);
                        let br_button = egui::Button::new("⚔️\nBarracks")
                            .min_size(BUTTON_SIZE)
                            .fill(if br_selected {
                                egui::Color32::from_rgb(60, 80, 60)
                            } else {
                                egui::Color32::from_rgb(40, 40, 50)
                            });

                        if ui
                            .add(br_button)
                            .on_hover_text("Barracks\n200 Wood, 100 Stone")
                            .clicked()
                        {
                            if br_selected {
                                placement.placing = None;
                            } else {
                                placement.placing = Some(shared::BuildingType::Barracks);
                            }
                        }

                        // Cancel button (only show when placing)
                        if placement.placing.is_some() {
                            ui.add_space(20.0);
                            if ui
                                .add(
                                    egui::Button::new("❌\nCancel")
                                        .min_size(BUTTON_SIZE)
                                        .fill(egui::Color32::from_rgb(80, 40, 40)),
                                )
                                .on_hover_text("Cancel placement (Esc)")
                                .clicked()
                            {
                                placement.placing = None;
                            }
                        }
                    });
                });
        });

    // Show placement mode indicator
    if let Some(building_type) = placement.placing {
        egui::Area::new(egui::Id::new("placement_indicator"))
            .fixed_pos(egui::pos2(screen_rect.width() / 2.0 - 100.0, screen_rect.height() - 100.0))
            .show(ctx, |ui| {
                egui::Frame::none()
                    .fill(egui::Color32::from_rgba_premultiplied(40, 60, 40, 200))
                    .rounding(egui::Rounding::same(4))
                    .inner_margin(egui::Margin::same(10))
                    .show(ui, |ui| {
                        ui.colored_label(
                            egui::Color32::from_rgb(150, 255, 150),
                            egui::RichText::new(format!("Placing {:?} - Click to place", building_type))
                                .size(14.0),
                        );
                    });
            });
    }
}
