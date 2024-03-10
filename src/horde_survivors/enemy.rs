use bevy::prelude::*;

use crate::{
    GameLoopSchedules, 
    GameState, 
    MovableObjectBundle, 
    Velocity
};


pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Initialize), 
                spawn_enemy_wave
                .in_set(GameLoopSchedules::Spawn),
            )
            .add_systems(Update, 
                follow_player
                .in_set(GameLoopSchedules::EntityUpdates),
            )
            ;
    }
}


fn spawn_enemy_wave() {}

fn follow_player() {}
