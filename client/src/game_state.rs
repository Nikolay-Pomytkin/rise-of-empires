//! Game state management
//!
//! Handles main menu, in-game, and pause states.

use bevy::prelude::*;

/// Game states
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    Paused,
}

/// Plugin for game state management
pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::InGame), on_enter_game)
            .add_systems(OnExit(GameState::InGame), on_exit_game)
            .add_systems(
                Update,
                handle_pause_input.run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                handle_unpause_input.run_if(in_state(GameState::Paused)),
            );
    }
}

fn on_enter_game() {
    info!("Entering game...");
}

fn on_exit_game() {
    info!("Exiting game...");
}

fn handle_pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
}

fn handle_unpause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::InGame);
    }
}
