use bevy::prelude::*;

use crate::GameLoopSchedules;

#[derive(States, Debug, Default, Clone, Hash, Eq, PartialEq)]
pub enum GameState {
    #[default]
    Loading,
    // MainMenu,
    Initialize,
    Playing,
    PauseMenu,
    // GameOverMenu,
}

pub struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::Initialize), advance_initialization)
            .add_systems(Update, 
                process_pause_events
                .run_if(in_state(GameState::Playing))
                .before(GameLoopSchedules::ProcessInput)
            )
            ;
    }
}

fn process_pause_events(
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    match state.get() {
        GameState::Playing => {
            if keyboard_input.just_pressed(KeyCode::Escape) {
                next_state.set(GameState::PauseMenu);
            }
        },
        GameState::PauseMenu => {
            if keyboard_input.just_pressed(KeyCode::Escape) {
                next_state.set(GameState::Playing);
            }
        },
        _ => (),
    }
}

fn advance_initialization(
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::Playing);
}
