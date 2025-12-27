//! Main menu UI
//!
//! Title screen with game options

use bevy::ecs::message::MessageWriter;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::game_state::GameState;

// Colors matching the dark bronze/gold theme
const MENU_BG: egui::Color32 = egui::Color32::from_rgba_premultiplied(15, 12, 8, 250);
const BUTTON_BG: egui::Color32 = egui::Color32::from_rgb(30, 50, 80);
const BUTTON_HOVER: egui::Color32 = egui::Color32::from_rgb(50, 80, 120);
const BUTTON_TEXT: egui::Color32 = egui::Color32::from_rgb(220, 200, 160);
const TITLE_COLOR: egui::Color32 = egui::Color32::from_rgb(255, 200, 100);
const SUBTITLE_COLOR: egui::Color32 = egui::Color32::from_rgb(200, 160, 100);
const BORDER_COLOR: egui::Color32 = egui::Color32::from_rgb(120, 90, 50);

pub fn ui_main_menu(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    let screen_rect = ctx.screen_rect();

    // Full screen dark background
    egui::Area::new(egui::Id::new("menu_bg"))
        .fixed_pos(egui::pos2(0.0, 0.0))
        .show(ctx, |ui| {
            ui.painter().rect_filled(
                egui::Rect::from_min_size(egui::pos2(0.0, 0.0), screen_rect.size()),
                0.0,
                egui::Color32::from_rgb(20, 15, 10),
            );

            // Add some decorative gradient overlay
            let gradient_rect = egui::Rect::from_min_size(
                egui::pos2(0.0, 0.0),
                egui::vec2(screen_rect.width(), screen_rect.height() * 0.4),
            );
            ui.painter().rect_filled(
                gradient_rect,
                0.0,
                egui::Color32::from_rgba_premultiplied(60, 40, 20, 100),
            );
        });

    // Center menu panel
    let panel_width = 400.0;
    let panel_height = 500.0;
    let panel_x = (screen_rect.width() - panel_width) / 2.0;
    let panel_y = (screen_rect.height() - panel_height) / 2.0;

    egui::Area::new(egui::Id::new("main_menu"))
        .fixed_pos(egui::pos2(panel_x, panel_y))
        .show(ctx, |ui| {
            egui::Frame::none()
                .fill(MENU_BG)
                .rounding(egui::Rounding::same(8))
                .inner_margin(egui::Margin::same(30))
                .stroke(egui::Stroke::new(2.0, BORDER_COLOR))
                .show(ui, |ui| {
                    ui.set_min_width(panel_width - 60.0);

                    // Title
                    ui.vertical_centered(|ui| {
                        ui.add_space(10.0);

                        // Game logo/title
                        ui.colored_label(
                            TITLE_COLOR,
                            egui::RichText::new("RISE")
                                .size(64.0)
                                .strong(),
                        );

                        ui.colored_label(
                            SUBTITLE_COLOR,
                            egui::RichText::new("of Empires")
                                .size(28.0)
                                .italics(),
                        );

                        ui.add_space(10.0);

                        // Decorative line
                        ui.horizontal(|ui| {
                            ui.add_space(50.0);
                            let line_width = panel_width - 160.0;
                            ui.painter().line_segment(
                                [
                                    ui.cursor().min + egui::vec2(0.0, 5.0),
                                    ui.cursor().min + egui::vec2(line_width, 5.0),
                                ],
                                egui::Stroke::new(2.0, BORDER_COLOR),
                            );
                        });

                        ui.add_space(30.0);

                        // Menu buttons
                        let button_size = egui::vec2(280.0, 45.0);

                        if menu_button(ui, "Play Game", button_size).clicked() {
                            next_state.set(GameState::InGame);
                        }

                        ui.add_space(10.0);

                        if menu_button(ui, "Quick Battle", button_size).clicked() {
                            next_state.set(GameState::InGame);
                        }

                        ui.add_space(10.0);

                        if menu_button(ui, "Multiplayer", button_size).clicked() {
                            // TODO: Multiplayer lobby
                        }

                        ui.add_space(10.0);

                        if menu_button(ui, "Options", button_size).clicked() {
                            // TODO: Options menu
                        }

                        ui.add_space(10.0);

                        if menu_button(ui, "Exit", button_size).clicked() {
                            exit.write(AppExit::Success);
                        }

                        ui.add_space(20.0);

                        // Version info
                        ui.colored_label(
                            egui::Color32::from_rgb(100, 90, 70),
                            egui::RichText::new("Version 0.1.0").size(11.0),
                        );
                    });
                });
        });
}

fn menu_button(ui: &mut egui::Ui, text: &str, size: egui::Vec2) -> egui::Response {
    let button = egui::Button::new(
        egui::RichText::new(text)
            .size(18.0)
            .color(BUTTON_TEXT),
    )
    .min_size(size)
    .fill(BUTTON_BG)
    .stroke(egui::Stroke::new(1.0, BORDER_COLOR))
    .rounding(egui::Rounding::same(4));

    let response = ui.add(button);

    // Custom hover effect
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
