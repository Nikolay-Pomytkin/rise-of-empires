//! Game state management
//!
//! Handles main menu, game setup, in-game, and pause states.

use bevy::prelude::*;
use shared::{EmpireId, LeaderId, PlayerSetup};

/// Main game states
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    /// Play menu - choose new game or load game
    PlayMenu,
    /// Game setup - empire and leader selection
    GameSetup,
    InGame,
    Paused,
}

/// Sub-states for game setup flow
#[derive(SubStates, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[source(GameState = GameState::GameSetup)]
pub enum SetupState {
    #[default]
    EmpireSelect,
    LeaderSelect,
    MapSettings,
}

/// Resource to store the player's game setup choices
#[derive(Resource, Debug, Clone, Default)]
pub struct GameSetupData {
    pub player_setup: Option<PlayerSetup>,
    pub selected_empire: Option<EmpireId>,
    pub selected_leader: Option<LeaderId>,
}

impl GameSetupData {
    pub fn select_empire(&mut self, empire: EmpireId) {
        self.selected_empire = Some(empire);
        self.selected_leader = None; // Reset leader when empire changes
    }

    pub fn select_leader(&mut self, leader: LeaderId) {
        self.selected_leader = Some(leader.clone());
        // Create the final setup
        if let Some(empire) = &self.selected_empire {
            self.player_setup = Some(PlayerSetup::new(empire.clone(), leader));
        }
    }

    pub fn is_ready(&self) -> bool {
        self.player_setup.is_some()
    }

    pub fn reset(&mut self) {
        self.player_setup = None;
        self.selected_empire = None;
        self.selected_leader = None;
    }
}

/// Plugin for game state management
pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_sub_state::<SetupState>()
            .init_resource::<GameSetupData>()
            .add_systems(OnEnter(GameState::InGame), on_enter_game)
            .add_systems(OnExit(GameState::InGame), on_exit_game)
            .add_systems(OnEnter(GameState::GameSetup), on_enter_setup)
            .add_systems(OnExit(GameState::GameSetup), on_exit_setup)
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

fn on_enter_setup(
    mut setup_data: ResMut<GameSetupData>,
    mut next_setup_state: ResMut<NextState<SetupState>>,
) {
    info!("Entering game setup...");
    setup_data.reset();
    // Explicitly set the sub-state to EmpireSelect
    next_setup_state.set(SetupState::EmpireSelect);
}

fn on_exit_setup() {
    info!("Exiting game setup...");
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
