//! Play menu UI
//!
//! Choose between starting a new game or loading a saved game.

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::game_state::GameState;

// UI Colors
const PANEL_BG: egui::Color32 = egui::Color32::from_rgba_premultiplied(25, 25, 35, 245);
const BUTTON_BG: egui::Color32 = egui::Color32::from_rgb(45, 55, 75);
const BUTTON_HOVER: egui::Color32 = egui::Color32::from_rgb(65, 75, 95);
const BUTTON_TEXT: egui::Color32 = egui::Color32::from_rgb(220, 210, 180);
const TITLE_COLOR: egui::Color32 = egui::Color32::from_rgb(255, 215, 100);
const SUBTITLE_COLOR: egui::Color32 = egui::Color32::from_rgb(180, 170, 150);
const BORDER_COLOR: egui::Color32 = egui::Color32::from_rgb(100, 85, 60);

pub fn ui_play_menu(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };
    let screen_rect = ctx.screen_rect();

    // Full screen dark background
    egui::Area::new(egui::Id::new("play_menu_bg"))
        .fixed_pos(egui::pos2(0.0, 0.0))
        .show(ctx, |ui| {
            ui.painter().rect_filled(
                screen_rect,
                0.0,
                egui::Color32::from_rgb(15, 15, 20),
            );
        });

    // Center panel
    let panel_width = 500.0;
    let panel_height = 400.0;
    let panel_x = (screen_rect.width() - panel_width) / 2.0;
    let panel_y = (screen_rect.height() - panel_height) / 2.0;

    egui::Area::new(egui::Id::new("play_menu_panel"))
        .fixed_pos(egui::pos2(panel_x, panel_y))
        .show(ctx, |ui| {
            egui::Frame::none()
                .fill(PANEL_BG)
                .rounding(egui::Rounding::same(12))
                .inner_margin(egui::Margin::same(40))
                .stroke(egui::Stroke::new(2.0, BORDER_COLOR))
                .show(ui, |ui| {
                    ui.set_min_width(panel_width - 80.0);

                    ui.vertical_centered(|ui| {
                        // Title
                        ui.add_space(10.0);
                        ui.label(
                            egui::RichText::new("Choose Your Path")
                                .size(36.0)
                                .color(TITLE_COLOR)
                                .strong(),
                        );
                        ui.add_space(5.0);
                        ui.label(
                            egui::RichText::new("Begin a new conquest or continue your legacy")
                                .size(14.0)
                                .color(SUBTITLE_COLOR),
                        );
                        ui.add_space(40.0);

                        // New Game button (large, prominent)
                        let new_game_size = egui::vec2(320.0, 80.0);
                        let new_game_btn = ui.add(
                            egui::Button::new(
                                egui::RichText::new("⚔  New Game")
                                    .size(24.0)
                                    .color(BUTTON_TEXT),
                            )
                            .min_size(new_game_size)
                            .fill(BUTTON_BG)
                            .stroke(egui::Stroke::new(2.0, BORDER_COLOR))
                            .rounding(egui::Rounding::same(8)),
                        );

                        if new_game_btn.clicked() {
                            next_state.set(GameState::GameSetup);
                        }

                        if new_game_btn.hovered() {
                            ui.painter().rect_stroke(
                                new_game_btn.rect,
                                egui::Rounding::same(8),
                                egui::Stroke::new(2.0, TITLE_COLOR),
                                egui::StrokeKind::Outside,
                            );
                        }

                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new("Select an empire and leader to forge your destiny")
                                .size(12.0)
                                .color(SUBTITLE_COLOR),
                        );

                        ui.add_space(30.0);

                        // Load Game button
                        let load_size = egui::vec2(320.0, 60.0);
                        let load_btn = ui.add(
                            egui::Button::new(
                                egui::RichText::new("📜  Load Game")
                                    .size(20.0)
                                    .color(BUTTON_TEXT),
                            )
                            .min_size(load_size)
                            .fill(BUTTON_BG)
                            .stroke(egui::Stroke::new(1.0, BORDER_COLOR))
                            .rounding(egui::Rounding::same(6)),
                        );

                        if load_btn.clicked() {
                            // TODO: Implement load game screen
                            info!("Load game clicked - not yet implemented");
                        }

                        if load_btn.hovered() {
                            ui.painter().rect_stroke(
                                load_btn.rect,
                                egui::Rounding::same(6),
                                egui::Stroke::new(2.0, egui::Color32::from_rgb(150, 140, 100)),
                                egui::StrokeKind::Outside,
                            );
                        }

                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new("Continue from a previous save")
                                .size(12.0)
                                .color(SUBTITLE_COLOR),
                        );

                        ui.add_space(40.0);

                        // Back button
                        let back_btn = ui.add(
                            egui::Button::new(
                                egui::RichText::new("← Back to Main Menu")
                                    .size(14.0)
                                    .color(SUBTITLE_COLOR),
                            )
                            .fill(egui::Color32::TRANSPARENT)
                            .stroke(egui::Stroke::NONE),
                        );

                        if back_btn.clicked() {
                            next_state.set(GameState::MainMenu);
                        }
                    });
                });
        });
}
