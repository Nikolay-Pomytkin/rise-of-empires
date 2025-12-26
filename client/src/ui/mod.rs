//! UI systems using bevy_egui

mod building_panel;
mod resources;
mod selection;
mod tech_panel;

pub use building_panel::*;
pub use resources::*;
pub use selection::*;
pub use tech_panel::*;

use bevy::prelude::*;
use bevy_egui::EguiPrimaryContextPass;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            EguiPrimaryContextPass,
            (
                ui_resources_panel,
                ui_selection_panel,
                ui_tech_panel,
                ui_building_panel,
            ),
        );
    }
}
