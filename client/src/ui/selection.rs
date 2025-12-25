//! Selection UI panel

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::input::SelectionState;

/// Display selection info and unit commands
pub fn ui_selection_panel(
    mut contexts: EguiContexts,
    selection_state: Res<SelectionState>,
    tick: Res<sim::TickScheduler>,
    mut command_buffer: ResMut<sim::CommandBuffer>,
    units: Query<(&sim::SimEntity, &sim::Owner, Option<&sim::Unit>, Option<&sim::Health>, Option<&sim::Gatherer>)>,
    buildings: Query<(&sim::SimEntity, &sim::Owner, &sim::Building, Option<&sim::ProductionQueue>, Option<&sim::Health>)>,
) {
    if selection_state.is_empty() {
        return;
    }

    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };
    egui::SidePanel::right("selection_panel")
        .min_width(200.0)
        .show(ctx, |ui| {
            ui.heading("Selection");
            ui.label(format!("{} selected", selection_state.selected.len()));
            ui.separator();

            // Show info for first selected entity
            if let Some(&first) = selection_state.selected.first() {
                // Check if it's a unit
                if let Ok((sim_entity, _owner, unit, health, gatherer)) = units.get(first) {
                    ui.label(format!("ID: {}", sim_entity.id.0));

                    if let Some(unit) = unit {
                        ui.label(format!("Type: {:?}", unit.unit_type));
                    }

                    if let Some(health) = health {
                        ui.horizontal(|ui| {
                            ui.label("HP:");
                            let progress = health.current as f32 / health.max as f32;
                            ui.add(egui::ProgressBar::new(progress).text(format!("{}/{}", health.current, health.max)));
                        });
                    }

                    if let Some(gatherer) = gatherer {
                        if gatherer.carry_amount > 0 {
                            ui.label(format!(
                                "Carrying: {} {:?}",
                                gatherer.carry_amount, gatherer.carrying_type
                            ));
                        }
                        ui.label(format!("State: {:?}", gatherer.state));
                    }
                }

                // Check if it's a building
                if let Ok((sim_entity, _owner, building, queue, health)) = buildings.get(first) {
                    ui.label(format!("ID: {}", sim_entity.id.0));
                    ui.label(format!("Type: {:?}", building.building_type));

                    if let Some(health) = health {
                        ui.horizontal(|ui| {
                            ui.label("HP:");
                            let progress = health.current as f32 / health.max as f32;
                            ui.add(egui::ProgressBar::new(progress).text(format!("{}/{}", health.current, health.max)));
                        });
                    }

                    // Production queue
                    if let Some(queue) = queue {
                        ui.separator();
                        ui.label("Production Queue:");

                        if queue.is_empty() {
                            ui.label("(empty)");
                        } else {
                            for (i, item) in queue.items.iter().enumerate() {
                                ui.horizontal(|ui| {
                                    ui.label(format!("{}. {:?}", i + 1, item.unit_type));
                                    if i == 0 {
                                        let progress = item.progress();
                                        ui.add(egui::ProgressBar::new(progress).desired_width(80.0));
                                    }
                                });
                            }
                        }

                        // Train buttons
                        ui.separator();
                        ui.label("Train:");
                        ui.horizontal(|ui| {
                            if ui.button("Villager (50F)").clicked() {
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
                                if ui.button("Soldier (60F 20G)").clicked() {
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
                }
            }
        });
}
