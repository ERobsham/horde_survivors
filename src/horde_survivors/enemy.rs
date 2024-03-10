use bevy::prelude::*;
use bevy::utils::Duration;

use crate::{
    AnimationPlayerMapping, 
    AnimationType, 
    GameLoopSchedules, 
    GameState, 
    MeshAssets, 
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
                start_idle_animation
                .in_set(GameLoopSchedules::EntityUpdates),
            )
            .add_systems(Update, 
                follow_player
                .in_set(GameLoopSchedules::EntityUpdates),
            )
            ;
    }
}


fn spawn_enemy_wave() {}

fn start_idle_animation() {}

fn follow_player() {}