//! UI systems using bevy_egui

mod building_panel;
mod main_menu;
mod pause_menu;
mod resources;
mod selection;
mod tech_panel;

pub use building_panel::*;
pub use main_menu::*;
pub use pause_menu::*;
pub use resources::*;
pub use selection::*;
pub use tech_panel::*;

use bevy::prelude::*;
use bevy_egui::EguiPrimaryContextPass;

use crate::game_state::GameState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TechPanelState>()
            // Main menu (only in MainMenu state)
            .add_systems(
                EguiPrimaryContextPass,
                ui_main_menu.run_if(in_state(GameState::MainMenu)),
            )
            // Pause menu (only in Paused state)
            .add_systems(
                EguiPrimaryContextPass,
                ui_pause_menu.run_if(in_state(GameState::Paused)),
            )
            // In-game UI (in InGame or Paused state - so it shows behind pause menu)
            .add_systems(
                EguiPrimaryContextPass,
                (
                    ui_resources_panel,
                    ui_selection_panel,
                    ui_tech_panel,
                    ui_building_panel,
                )
                    .run_if(in_state(GameState::InGame).or(in_state(GameState::Paused))),
            );
    }
}
