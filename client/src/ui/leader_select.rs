//! Leader selection UI
//!
//! Shows available leaders for the selected empire.

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use shared::{LeaderDef, PassiveTrait, UnitCategory};

use crate::game_state::{GameSetupData, GameState, SetupState};
use sim::EmpireData;

// UI Colors
const PANEL_BG: egui::Color32 = egui::Color32::from_rgba_premultiplied(20, 20, 30, 245);
const CARD_BG: egui::Color32 = egui::Color32::from_rgb(35, 35, 50);
const CARD_SELECTED: egui::Color32 = egui::Color32::from_rgb(60, 50, 40);
const CARD_HOVER: egui::Color32 = egui::Color32::from_rgb(50, 50, 70);
const TITLE_COLOR: egui::Color32 = egui::Color32::from_rgb(255, 215, 100);
const TEXT_COLOR: egui::Color32 = egui::Color32::from_rgb(220, 210, 190);
const SUBTITLE_COLOR: egui::Color32 = egui::Color32::from_rgb(150, 140, 120);
const BONUS_COLOR: egui::Color32 = egui::Color32::from_rgb(100, 200, 100);
const BORDER_COLOR: egui::Color32 = egui::Color32::from_rgb(100, 85, 60);
const SELECTED_BORDER: egui::Color32 = egui::Color32::from_rgb(200, 170, 80);

pub fn ui_leader_select(
    mut contexts: EguiContexts,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_setup_state: ResMut<NextState<SetupState>>,
    mut setup_data: ResMut<GameSetupData>,
    empire_data: Res<EmpireData>,
    current_setup_state: Option<Res<State<SetupState>>>,
) {
    // Only show when in LeaderSelect state
    // If sub-state isn't available, don't show (wait for empire select first)
    match &current_setup_state {
        Some(state) if *state.get() == SetupState::LeaderSelect => {}
        _ => return,
    }
    
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };
    let screen_rect = ctx.screen_rect();

    // Get selected empire
    let selected_empire_id = setup_data.selected_empire.clone();
    let empire = selected_empire_id.as_ref().and_then(|id| {
        empire_data.get_empire(id)
    });

    // Full screen dark background
    egui::Area::new(egui::Id::new("leader_select_bg"))
        .fixed_pos(egui::pos2(0.0, 0.0))
        .show(ctx, |ui| {
            ui.painter().rect_filled(
                screen_rect,
                0.0,
                egui::Color32::from_rgb(12, 12, 18),
            );
        });

    // Main content area - use Window instead of CentralPanel for better WASM visibility
    egui::Window::new("Choose Your Leader")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .min_width(screen_rect.width() * 0.9)
        .min_height(screen_rect.height() * 0.85)
        .title_bar(false)
        .frame(egui::Frame::none().fill(PANEL_BG).rounding(12.0).inner_margin(30.0))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                // Title with empire name
                let empire_name = empire.map(|e| e.name.as_str()).unwrap_or("Unknown Empire");
                ui.label(
                    egui::RichText::new("Choose Your Leader")
                        .size(42.0)
                        .color(TITLE_COLOR)
                        .strong(),
                );
                ui.add_space(5.0);
                ui.label(
                    egui::RichText::new(format!("Leading the {}", empire_name))
                        .size(18.0)
                        .color(SUBTITLE_COLOR),
                );
                ui.add_space(40.0);

                // Leader cards
                if let Some(empire) = empire {
                    let leaders = &empire.leaders;

                    if leaders.is_empty() {
                        ui.label(
                            egui::RichText::new("No leaders available for this empire")
                                .size(16.0)
                                .color(egui::Color32::RED),
                        );
                    } else {
                        // Center the leader cards
                        let card_width = 350.0;
                        let card_height = 320.0;
                        let spacing = 40.0;
                        let total_width = leaders.len() as f32 * card_width + (leaders.len() - 1) as f32 * spacing;

                        ui.horizontal(|ui| {
                            ui.add_space((ui.available_width() - total_width).max(0.0) / 2.0);

                            for leader in leaders {
                                let is_selected = setup_data.selected_leader.as_ref() == Some(&leader.id);

                                let card_response = leader_card(
                                    ui,
                                    leader,
                                    is_selected,
                                    egui::vec2(card_width, card_height),
                                );

                                if card_response.clicked() {
                                    setup_data.select_leader(leader.id.clone());
                                }

                                ui.add_space(spacing);
                            }
                        });
                    }
                } else {
                    ui.label(
                        egui::RichText::new("No empire selected. Go back and select an empire.")
                            .size(16.0)
                            .color(egui::Color32::RED),
                    );
                }
            });
        });

    // Bottom buttons - separate window at bottom
    egui::Window::new("leader_buttons")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_BOTTOM, [0.0, -30.0])
        .title_bar(false)
        .frame(egui::Frame::none())
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                let button_width = 150.0;

                // Back button
                if ui
                    .add(
                        egui::Button::new(
                            egui::RichText::new("← Back")
                                .size(16.0)
                                .color(TEXT_COLOR),
                        )
                        .min_size(egui::vec2(button_width, 40.0))
                        .fill(CARD_BG)
                        .stroke(egui::Stroke::new(1.0, BORDER_COLOR))
                        .rounding(egui::Rounding::same(6)),
                    )
                    .clicked()
                {
                    next_setup_state.set(SetupState::EmpireSelect);
                }

                ui.add_space(20.0);

                // Start Game button (only enabled if leader selected)
                let start_enabled = setup_data.is_ready();
                let start_btn = ui.add_enabled(
                    start_enabled,
                    egui::Button::new(
                        egui::RichText::new("⚔ Start Game")
                            .size(18.0)
                            .color(if start_enabled { TITLE_COLOR } else { SUBTITLE_COLOR }),
                    )
                    .min_size(egui::vec2(button_width + 30.0, 45.0))
                    .fill(if start_enabled { CARD_SELECTED } else { CARD_BG })
                    .stroke(egui::Stroke::new(
                        if start_enabled { 2.0 } else { 1.0 },
                        if start_enabled { SELECTED_BORDER } else { BORDER_COLOR },
                    ))
                    .rounding(egui::Rounding::same(6)),
                );

                if start_btn.clicked() {
                    // Start the game!
                    next_game_state.set(GameState::InGame);
                }
            });
        });
}

fn leader_card(
    ui: &mut egui::Ui,
    leader: &LeaderDef,
    is_selected: bool,
    size: egui::Vec2,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

    if ui.is_rect_visible(rect) {
        let bg_color = if is_selected {
            CARD_SELECTED
        } else if response.hovered() {
            CARD_HOVER
        } else {
            CARD_BG
        };

        let border_color = if is_selected {
            SELECTED_BORDER
        } else if response.hovered() {
            egui::Color32::from_rgb(140, 120, 80)
        } else {
            BORDER_COLOR
        };

        let border_width = if is_selected { 3.0 } else { 1.0 };

        // Draw card background
        ui.painter().rect(
            rect,
            egui::Rounding::same(10),
            bg_color,
            egui::Stroke::new(border_width, border_color),
            egui::StrokeKind::Inside,
        );

        // Card content
        let content_rect = rect.shrink(20.0);
        let mut cursor_y = content_rect.top();

        // Leader name
        let name_galley = ui.painter().layout_no_wrap(
            leader.name.clone(),
            egui::FontId::proportional(26.0),
            TITLE_COLOR,
        );
        ui.painter().galley(
            egui::pos2(content_rect.left(), cursor_y),
            name_galley,
            TITLE_COLOR,
        );
        cursor_y += 35.0;

        // Title
        if !leader.title.is_empty() {
            let title_galley = ui.painter().layout_no_wrap(
                format!("\"{}\"", leader.title),
                egui::FontId::proportional(14.0),
                SUBTITLE_COLOR,
            );
            ui.painter().galley(
                egui::pos2(content_rect.left(), cursor_y),
                title_galley,
                SUBTITLE_COLOR,
            );
            cursor_y += 22.0;
        }

        // Separator
        cursor_y += 5.0;
        ui.painter().line_segment(
            [
                egui::pos2(content_rect.left(), cursor_y),
                egui::pos2(content_rect.right(), cursor_y),
            ],
            egui::Stroke::new(1.0, BORDER_COLOR),
        );
        cursor_y += 15.0;

        // Description
        let desc_galley = ui.painter().layout(
            leader.description.clone(),
            egui::FontId::proportional(12.0),
            TEXT_COLOR,
            content_rect.width(),
        );
        ui.painter().galley(
            egui::pos2(content_rect.left(), cursor_y),
            desc_galley,
            TEXT_COLOR,
        );
        cursor_y += 50.0;

        // Military bonuses section
        cursor_y += 10.0;
        let bonuses_label = ui.painter().layout_no_wrap(
            "Military Bonuses:".to_string(),
            egui::FontId::proportional(11.0),
            SUBTITLE_COLOR,
        );
        ui.painter().galley(
            egui::pos2(content_rect.left(), cursor_y),
            bonuses_label,
            SUBTITLE_COLOR,
        );
        cursor_y += 18.0;

        // Show attack bonuses
        for (category, bonus) in &leader.military_bonuses.attack_bonus {
            let bonus_text = format!("• +{} Attack ({})", bonus, format_category(category));
            let bonus_galley = ui.painter().layout_no_wrap(
                bonus_text,
                egui::FontId::proportional(11.0),
                BONUS_COLOR,
            );
            ui.painter().galley(
                egui::pos2(content_rect.left() + 10.0, cursor_y),
                bonus_galley,
                BONUS_COLOR,
            );
            cursor_y += 15.0;
        }

        // Show defense bonuses
        for (category, bonus) in &leader.military_bonuses.defense_bonus {
            let bonus_text = format!("• +{} Defense ({})", bonus, format_category(category));
            let bonus_galley = ui.painter().layout_no_wrap(
                bonus_text,
                egui::FontId::proportional(11.0),
                BONUS_COLOR,
            );
            ui.painter().galley(
                egui::pos2(content_rect.left() + 10.0, cursor_y),
                bonus_galley,
                BONUS_COLOR,
            );
            cursor_y += 15.0;
        }

        // Show speed bonuses
        for (category, bonus) in &leader.military_bonuses.speed_bonus {
            let bonus_text = format!("• +{:.0}% Speed ({})", bonus * 100.0, format_category(category));
            let bonus_galley = ui.painter().layout_no_wrap(
                bonus_text,
                egui::FontId::proportional(11.0),
                BONUS_COLOR,
            );
            ui.painter().galley(
                egui::pos2(content_rect.left() + 10.0, cursor_y),
                bonus_galley,
                BONUS_COLOR,
            );
            cursor_y += 15.0;
        }

        // Training speed
        if leader.military_bonuses.training_speed != 1.0 {
            let training_bonus = (1.0 - leader.military_bonuses.training_speed) * 100.0;
            let bonus_text = format!("• {:.0}% faster training", training_bonus);
            let bonus_galley = ui.painter().layout_no_wrap(
                bonus_text,
                egui::FontId::proportional(11.0),
                BONUS_COLOR,
            );
            ui.painter().galley(
                egui::pos2(content_rect.left() + 10.0, cursor_y),
                bonus_galley,
                BONUS_COLOR,
            );
            cursor_y += 15.0;
        }

        // Passive traits
        if !leader.passive_traits.is_empty() {
            cursor_y += 5.0;
            let traits_label = ui.painter().layout_no_wrap(
                "Passive Traits:".to_string(),
                egui::FontId::proportional(11.0),
                SUBTITLE_COLOR,
            );
            ui.painter().galley(
                egui::pos2(content_rect.left(), cursor_y),
                traits_label,
                SUBTITLE_COLOR,
            );
            cursor_y += 18.0;

            for trait_ in &leader.passive_traits {
                let trait_text = format!("• {}", trait_);
                let trait_galley = ui.painter().layout_no_wrap(
                    trait_text,
                    egui::FontId::proportional(11.0),
                    BONUS_COLOR,
                );
                ui.painter().galley(
                    egui::pos2(content_rect.left() + 10.0, cursor_y),
                    trait_galley,
                    BONUS_COLOR,
                );
                cursor_y += 15.0;
            }
        }
    }

    response
}

fn format_category(category: &UnitCategory) -> &'static str {
    match category {
        UnitCategory::Infantry => "Infantry",
        UnitCategory::Cavalry => "Cavalry",
        UnitCategory::Ranged => "Ranged",
        UnitCategory::Siege => "Siege",
        UnitCategory::Naval => "Naval",
        UnitCategory::Villager => "Villager",
    }
}
