//! Pause menu UI
//!
//! In-game pause overlay with options

use bevy::ecs::message::MessageWriter;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::game_state::GameState;

const OVERLAY_BG: egui::Color32 = egui::Color32::from_rgba_premultiplied(0, 0, 0, 180);
const PANEL_BG: egui::Color32 = egui::Color32::from_rgba_premultiplied(20, 18, 15, 245);
const BUTTON_BG: egui::Color32 = egui::Color32::from_rgb(30, 50, 80);
const BUTTON_TEXT: egui::Color32 = egui::Color32::from_rgb(220, 200, 160);
const TITLE_COLOR: egui::Color32 = egui::Color32::from_rgb(255, 200, 100);
const BORDER_COLOR: egui::Color32 = egui::Color32::from_rgb(120, 90, 50);

pub fn ui_pause_menu(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    let screen_rect = ctx.screen_rect();

    // Dark overlay over the game
    egui::Area::new(egui::Id::new("pause_overlay"))
        .fixed_pos(egui::pos2(0.0, 0.0))
        .order(egui::Order::Middle)
        .show(ctx, |ui| {
            ui.painter().rect_filled(
                egui::Rect::from_min_size(egui::pos2(0.0, 0.0), screen_rect.size()),
                0.0,
                OVERLAY_BG,
            );
        });

    // Center pause panel
    let panel_width = 350.0;
    let panel_height = 380.0;
    let panel_x = (screen_rect.width() - panel_width) / 2.0;
    let panel_y = (screen_rect.height() - panel_height) / 2.0;

    egui::Area::new(egui::Id::new("pause_menu"))
        .fixed_pos(egui::pos2(panel_x, panel_y))
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            egui::Frame::none()
                .fill(PANEL_BG)
                .rounding(egui::Rounding::same(8))
                .inner_margin(egui::Margin::same(25))
                .stroke(egui::Stroke::new(2.0, BORDER_COLOR))
                .show(ui, |ui| {
                    ui.set_min_width(panel_width - 50.0);

                    ui.vertical_centered(|ui| {
                        // Title
                        ui.colored_label(
                            TITLE_COLOR,
                            egui::RichText::new("Game Paused")
                                .size(32.0)
                                .strong(),
                        );

                        ui.add_space(5.0);

                        // Decorative line
                        ui.horizontal(|ui| {
                            ui.add_space(30.0);
                            let line_width = panel_width - 110.0;
                            ui.painter().line_segment(
                                [
                                    ui.cursor().min + egui::vec2(0.0, 5.0),
                                    ui.cursor().min + egui::vec2(line_width, 5.0),
                                ],
                                egui::Stroke::new(1.0, BORDER_COLOR),
                            );
                        });

                        ui.add_space(25.0);

                        let button_size = egui::vec2(240.0, 40.0);

                        // Resume
                        if pause_button(ui, "Resume Game", button_size).clicked() {
                            next_state.set(GameState::InGame);
                        }

                        ui.add_space(10.0);

                        // Options
                        if pause_button(ui, "Options", button_size).clicked() {
                            // TODO: Options submenu
                        }

                        ui.add_space(10.0);

                        // Save Game
                        if pause_button(ui, "Save Game", button_size).clicked() {
                            // TODO: Save game
                        }

                        ui.add_space(10.0);

                        // Main Menu
                        if pause_button(ui, "Main Menu", button_size).clicked() {
                            next_state.set(GameState::MainMenu);
                        }

                        ui.add_space(10.0);

                        // Exit
                        if pause_button(ui, "Exit Game", button_size).clicked() {
                            exit.write(AppExit::Success);
                        }

                        ui.add_space(15.0);

                        // Hint
                        ui.colored_label(
                            egui::Color32::from_rgb(120, 110, 90),
                            egui::RichText::new("Press ESC to resume").size(12.0),
                        );
                    });
                });
        });
}

fn pause_button(ui: &mut egui::Ui, text: &str, size: egui::Vec2) -> egui::Response {
    let button = egui::Button::new(
        egui::RichText::new(text)
            .size(16.0)
            .color(BUTTON_TEXT),
    )
    .min_size(size)
    .fill(BUTTON_BG)
    .stroke(egui::Stroke::new(1.0, BORDER_COLOR))
    .rounding(egui::Rounding::same(4));

    let response = ui.add(button);

    if response.hovered() {
        ui.painter().rect_stroke(
            response.rect,
            egui::Rounding::same(4),
            egui::Stroke::new(2.0, egui::Color32::from_rgb(180, 150, 80)),
            egui::StrokeKind::Outside,
        );
    }

    response
}
