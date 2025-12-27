//! Selection UI panel
//!
//! Bottom-right panel showing selected unit/building info

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::input::SelectionState;

const PANEL_BG: egui::Color32 = egui::Color32::from_rgba_premultiplied(20, 20, 30, 230);
const HEADER_COLOR: egui::Color32 = egui::Color32::from_rgb(255, 220, 150);
const TEXT_COLOR: egui::Color32 = egui::Color32::from_rgb(220, 220, 220);
const HP_BAR_BG: egui::Color32 = egui::Color32::from_rgb(60, 20, 20);
const HP_BAR_FG: egui::Color32 = egui::Color32::from_rgb(50, 200, 50);

/// Display selection info (bottom-right)
pub fn ui_selection_panel(
    mut contexts: EguiContexts,
    selection_state: Res<SelectionState>,
    tick: Res<sim::TickScheduler>,
    mut command_buffer: ResMut<sim::CommandBuffer>,
    units: Query<(
        &sim::SimEntity,
        &sim::Owner,
        Option<&sim::Unit>,
        Option<&sim::Health>,
        Option<&sim::Gatherer>,
    )>,
    buildings: Query<(
        &sim::SimEntity,
        &sim::Owner,
        &sim::Building,
        Option<&sim::ProductionQueue>,
        Option<&sim::Health>,
    )>,
) {
    if selection_state.is_empty() {
        return;
    }

    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    let screen_rect = ctx.screen_rect();
    let panel_width = 280.0;
    let panel_height = 200.0;

    // Position at bottom-right
    egui::Area::new(egui::Id::new("selection_panel"))
        .fixed_pos(egui::pos2(
            screen_rect.width() - panel_width - 10.0,
            screen_rect.height() - panel_height - 50.0, // Above building panel
        ))
        .show(ctx, |ui| {
            egui::Frame::none()
                .fill(PANEL_BG)
                .rounding(egui::Rounding::same(4))
                .inner_margin(egui::Margin::same(10))
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(80, 80, 100)))
                .show(ui, |ui| {
                    ui.set_min_width(panel_width - 20.0);
                    ui.set_min_height(panel_height - 20.0);

                    // Show info for first selected entity
                    if let Some(&first) = selection_state.selected.first() {
                        // Check if it's a unit
                        if let Ok((sim_entity, _owner, unit, health, gatherer)) = units.get(first) {
                            render_unit_panel(ui, sim_entity, unit, health, gatherer, &selection_state);
                        }
                        // Check if it's a building
                        else if let Ok((sim_entity, _owner, building, queue, health)) =
                            buildings.get(first)
                        {
                            render_building_panel(
                                ui,
                                sim_entity,
                                building,
                                queue,
                                health,
                                &tick,
                                &mut command_buffer,
                                &selection_state,
                            );
                        }
                    }
                });
        });
}

fn render_unit_panel(
    ui: &mut egui::Ui,
    sim_entity: &sim::SimEntity,
    unit: Option<&sim::Unit>,
    health: Option<&sim::Health>,
    gatherer: Option<&sim::Gatherer>,
    selection_state: &SelectionState,
) {
    // Header with unit type
    let unit_name = unit
        .map(|u| format!("{:?}", u.unit_type))
        .unwrap_or_else(|| "Unit".to_string());

    ui.horizontal(|ui| {
        ui.colored_label(HEADER_COLOR, egui::RichText::new(&unit_name).size(16.0).strong());
        
        // Show count if multiple selected
        if selection_state.selected.len() > 1 {
            ui.colored_label(
                TEXT_COLOR,
                egui::RichText::new(format!("({})", selection_state.selected.len())).size(12.0),
            );
        }
    });

    ui.add_space(5.0);

    // Health bar
    if let Some(health) = health {
        ui.horizontal(|ui| {
            let progress = health.current as f32 / health.max as f32;
            let hp_color = if progress > 0.6 {
                HP_BAR_FG
            } else if progress > 0.3 {
                egui::Color32::from_rgb(200, 200, 50)
            } else {
                egui::Color32::from_rgb(200, 50, 50)
            };

            ui.colored_label(TEXT_COLOR, "HP");
            ui.add(
                egui::ProgressBar::new(progress)
                    .fill(hp_color)
                    .text(format!("{} / {}", health.current, health.max))
                    .desired_width(180.0),
            );
        });
    }

    ui.add_space(5.0);

    // Gatherer info
    if let Some(gatherer) = gatherer {
        ui.horizontal(|ui| {
            ui.colored_label(TEXT_COLOR, "Status:");
            ui.colored_label(
                egui::Color32::from_rgb(150, 200, 255),
                format!("{:?}", gatherer.state),
            );
        });

        if gatherer.carry_amount > 0 {
            ui.horizontal(|ui| {
                ui.colored_label(TEXT_COLOR, "Carrying:");
                let resource_color = match gatherer.carrying_type {
                    shared::ResourceType::Food => egui::Color32::from_rgb(150, 255, 150),
                    shared::ResourceType::Wood => egui::Color32::from_rgb(180, 140, 90),
                    shared::ResourceType::Gold => egui::Color32::from_rgb(255, 215, 0),
                    shared::ResourceType::Stone => egui::Color32::from_rgb(160, 160, 180),
                };
                ui.colored_label(
                    resource_color,
                    format!("{} {:?}", gatherer.carry_amount, gatherer.carrying_type),
                );
            });
        }
    }

    // ID (small, at bottom)
    ui.add_space(10.0);
    ui.colored_label(
        egui::Color32::from_rgb(100, 100, 120),
        egui::RichText::new(format!("ID: {}", sim_entity.id.0)).size(10.0),
    );
}

fn render_building_panel(
    ui: &mut egui::Ui,
    sim_entity: &sim::SimEntity,
    building: &sim::Building,
    queue: Option<&sim::ProductionQueue>,
    health: Option<&sim::Health>,
    tick: &sim::TickScheduler,
    command_buffer: &mut sim::CommandBuffer,
    selection_state: &SelectionState,
) {
    // Header
    ui.colored_label(
        HEADER_COLOR,
        egui::RichText::new(format!("{:?}", building.building_type))
            .size(16.0)
            .strong(),
    );

    ui.add_space(5.0);

    // Health bar
    if let Some(health) = health {
        ui.horizontal(|ui| {
            let progress = health.current as f32 / health.max as f32;
            ui.colored_label(TEXT_COLOR, "HP");
            ui.add(
                egui::ProgressBar::new(progress)
                    .fill(HP_BAR_FG)
                    .text(format!("{} / {}", health.current, health.max))
                    .desired_width(180.0),
            );
        });
    }

    // Production queue
    if let Some(queue) = queue {
        ui.add_space(10.0);
        ui.colored_label(TEXT_COLOR, "Production:");

        if queue.is_empty() {
            ui.colored_label(egui::Color32::from_rgb(120, 120, 140), "(idle)");
        } else {
            for (i, item) in queue.items.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.colored_label(TEXT_COLOR, format!("{:?}", item.unit_type));
                    if i == 0 {
                        let progress = item.progress();
                        ui.add(
                            egui::ProgressBar::new(progress)
                                .desired_width(100.0)
                                .text(format!("{}%", (progress * 100.0) as i32)),
                        );
                    }
                });
            }
        }

        ui.add_space(10.0);
        ui.separator();

        // Train buttons
        ui.horizontal(|ui| {
            if ui
                .add(egui::Button::new("🧑‍🌾 Villager").min_size(egui::vec2(80.0, 30.0)))
                .on_hover_text("Train Villager (50 Food)")
                .clicked()
            {
                command_buffer.push_command(
                    tick.tick() + 1,
                    selection_state.active_player,
                    shared::GameCommand::QueueUnit {
                        building: sim_entity.id,
                        unit_type: shared::UnitType::Villager,
                    },
                );
            }

            // Only show soldier button for barracks
            if matches!(building.building_type, shared::BuildingType::Barracks) {
                if ui
                    .add(egui::Button::new("⚔️ Soldier").min_size(egui::vec2(80.0, 30.0)))
                    .on_hover_text("Train Soldier (60 Food, 20 Gold)")
                    .clicked()
                {
                    command_buffer.push_command(
                        tick.tick() + 1,
                        selection_state.active_player,
                        shared::GameCommand::QueueUnit {
                            building: sim_entity.id,
                            unit_type: shared::UnitType::Soldier,
                        },
                    );
                }
            }
        });
    }

    // ID (small, at bottom)
    ui.add_space(5.0);
    ui.colored_label(
        egui::Color32::from_rgb(100, 100, 120),
        egui::RichText::new(format!("ID: {}", sim_entity.id.0)).size(10.0),
    );
}
