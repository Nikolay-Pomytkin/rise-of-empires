//! Empire selection UI
//!
//! Grid of empire cards for the player to choose from.

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::game_state::{GameSetupData, GameState, SetupState};
use sim::EmpireData;
use shared::EmpireId;

// UI Colors
const PANEL_BG: egui::Color32 = egui::Color32::from_rgba_premultiplied(20, 20, 30, 245);
const CARD_BG: egui::Color32 = egui::Color32::from_rgb(35, 40, 55);
const CARD_HOVER: egui::Color32 = egui::Color32::from_rgb(50, 55, 75);
const CARD_SELECTED: egui::Color32 = egui::Color32::from_rgb(60, 70, 100);
const TITLE_COLOR: egui::Color32 = egui::Color32::from_rgb(255, 215, 100);
const TEXT_COLOR: egui::Color32 = egui::Color32::from_rgb(220, 210, 180);
const SUBTITLE_COLOR: egui::Color32 = egui::Color32::from_rgb(150, 145, 130);
const BORDER_COLOR: egui::Color32 = egui::Color32::from_rgb(100, 85, 60);
const HIGHLIGHT_COLOR: egui::Color32 = egui::Color32::from_rgb(180, 150, 80);

pub fn ui_empire_select(
    mut contexts: EguiContexts,
    empire_data: Res<EmpireData>,
    mut setup_data: ResMut<GameSetupData>,
    mut next_setup_state: ResMut<NextState<SetupState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    current_setup_state: Option<Res<State<SetupState>>>,
) {
    // Only show when in EmpireSelect sub-state
    // If sub-state isn't available yet (first frame), show the UI anyway
    if let Some(state) = &current_setup_state {
        if *state.get() != SetupState::EmpireSelect {
            return;
        }
    }
    // If current_setup_state is None, we're in the first frame after state transition
    // In that case, show the empire select UI since EmpireSelect is the default
    
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };
    let screen_rect = ctx.screen_rect();

    // Full screen dark background
    egui::Area::new(egui::Id::new("empire_select_bg"))
        .fixed_pos(egui::pos2(0.0, 0.0))
        .show(ctx, |ui| {
            ui.painter().rect_filled(
                screen_rect,
                0.0,
                egui::Color32::from_rgb(12, 12, 18),
            );
        });

    // Main content area - use Window instead of CentralPanel for better visibility
    egui::Window::new("Choose Your Empire")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .min_width(screen_rect.width() * 0.9)
        .min_height(screen_rect.height() * 0.8)
        .title_bar(false)
        .frame(egui::Frame::none().fill(PANEL_BG).rounding(12.0).inner_margin(30.0))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                // Title
                ui.label(
                    egui::RichText::new("Choose Your Empire")
                        .size(42.0)
                        .color(TITLE_COLOR)
                        .strong(),
                );
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new("Each empire has unique units, buildings, and bonuses")
                        .size(16.0)
                        .color(SUBTITLE_COLOR),
                );
                ui.add_space(30.0);

                // Empire grid
                let empires = empire_data.all_empires();
                let card_width = 280.0;
                let card_height = 200.0;
                let cards_per_row = ((screen_rect.width() - 100.0) / (card_width + 20.0)) as usize;
                let cards_per_row = cards_per_row.max(1).min(4);

                egui::ScrollArea::vertical()
                    .max_height(screen_rect.height() - 200.0)
                    .show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.spacing_mut().item_spacing = egui::vec2(20.0, 20.0);

                            for empire in &empires {
                                let is_selected = setup_data.selected_empire.as_ref()
                                    .map(|e| e.0 == empire.id.0)
                                    .unwrap_or(false);

                                let card_response = draw_empire_card(ui, empire, is_selected, card_width, card_height);

                                if card_response.clicked() {
                                    setup_data.select_empire(empire.id.clone());
                                }
                            }
                        });
                    });

                ui.add_space(20.0);

                // Bottom buttons
                ui.horizontal(|ui| {
                    // Back button
                    if ui.add(
                        egui::Button::new(
                            egui::RichText::new("← Back")
                                .size(16.0)
                                .color(SUBTITLE_COLOR),
                        )
                        .fill(egui::Color32::TRANSPARENT)
                        .min_size(egui::vec2(100.0, 40.0)),
                    ).clicked() {
                        // Go back to play menu
                        next_game_state.set(GameState::PlayMenu);
                    }

                    ui.add_space(ui.available_width() - 220.0);

                    // Continue button (only enabled if empire selected)
                    let continue_enabled = setup_data.selected_empire.is_some();
                    let continue_btn = ui.add_enabled(
                        continue_enabled,
                        egui::Button::new(
                            egui::RichText::new("Continue →")
                                .size(18.0)
                                .color(if continue_enabled { TEXT_COLOR } else { SUBTITLE_COLOR }),
                        )
                        .fill(if continue_enabled { CARD_SELECTED } else { CARD_BG })
                        .stroke(egui::Stroke::new(2.0, if continue_enabled { HIGHLIGHT_COLOR } else { BORDER_COLOR }))
                        .min_size(egui::vec2(140.0, 45.0))
                        .rounding(egui::Rounding::same(6)),
                    );

                    if continue_btn.clicked() {
                        next_setup_state.set(SetupState::LeaderSelect);
                    }
                });
            });
        });
}

fn draw_empire_card(
    ui: &mut egui::Ui,
    empire: &shared::EmpireDef,
    is_selected: bool,
    width: f32,
    height: f32,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::click());

    if ui.is_rect_visible(rect) {
        let bg_color = if is_selected {
            CARD_SELECTED
        } else if response.hovered() {
            CARD_HOVER
        } else {
            CARD_BG
        };

        let border_color = if is_selected {
            HIGHLIGHT_COLOR
        } else if response.hovered() {
            egui::Color32::from_rgb(120, 100, 70)
        } else {
            BORDER_COLOR
        };

        // Card background
        ui.painter().rect(
            rect,
            egui::Rounding::same(8),
            bg_color,
            egui::Stroke::new(if is_selected { 3.0 } else { 1.5 }, border_color),
            egui::StrokeKind::Inside,
        );

        // Card content
        let inner_rect = rect.shrink(15.0);
        let mut cursor = inner_rect.min;

        // Empire name
        ui.painter().text(
            cursor,
            egui::Align2::LEFT_TOP,
            &empire.name,
            egui::FontId::proportional(22.0),
            TITLE_COLOR,
        );
        cursor.y += 28.0;

        // Theme
        ui.painter().text(
            cursor,
            egui::Align2::LEFT_TOP,
            &empire.theme,
            egui::FontId::proportional(14.0),
            HIGHLIGHT_COLOR,
        );
        cursor.y += 22.0;

        // Description (truncated)
        let desc = if empire.description.len() > 120 {
            format!("{}...", &empire.description[..117])
        } else {
            empire.description.clone()
        };

        // Wrap description text
        let galley = ui.painter().layout(
            desc,
            egui::FontId::proportional(12.0),
            SUBTITLE_COLOR,
            inner_rect.width(),
        );
        ui.painter().galley(cursor, galley, SUBTITLE_COLOR);
        cursor.y += 60.0;

        // Bonuses summary
        let mut bonus_text = String::new();

        // Resource bonuses
        for (resource, bonus) in &empire.resource_bonuses.gather_rate {
            if *bonus != 0.0 {
                bonus_text.push_str(&format!("+{:.0}% {:?} ", bonus * 100.0, resource));
            }
        }

        // Unique units
        if !empire.unique_units.is_empty() {
            let unit_names: Vec<_> = empire.unique_units.iter().map(|u| u.name.as_str()).collect();
            bonus_text.push_str(&format!("• {} ", unit_names.join(", ")));
        }

        if !bonus_text.is_empty() {
            ui.painter().text(
                egui::pos2(cursor.x, inner_rect.max.y - 20.0),
                egui::Align2::LEFT_TOP,
                bonus_text,
                egui::FontId::proportional(11.0),
                egui::Color32::from_rgb(130, 180, 130),
            );
        }

        // Leaders count
        let leaders_text = format!("{} Leader{}", empire.leaders.len(), if empire.leaders.len() != 1 { "s" } else { "" });
        ui.painter().text(
            egui::pos2(inner_rect.max.x, inner_rect.min.y),
            egui::Align2::RIGHT_TOP,
            leaders_text,
            egui::FontId::proportional(12.0),
            SUBTITLE_COLOR,
        );
    }

    response
}
