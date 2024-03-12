use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    GameLoopSchedules, GameState, MovableObjectBundle, PlayerComponent, Velocity
};

use super::types::{AnimationType, AssetKey, SpawnMesh, TriggerAnimation, ASSET_KEY_ENEMY};

const WAVE_TIME: f32 = 5.0;
const WAVE_SPAWNS_PER: usize = 8;

const ENEMY_SPAWN_DIST: f32 = 15.0;
const ENEMY_MOVE_SPEED: f32 = 2.25;

#[derive(Component, Debug, Default)]
pub struct EnemyComponent;

#[derive(Resource, Debug, Default)]
struct WaveTimer(Timer);

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .insert_resource(WaveTimer(Timer::from_seconds(WAVE_TIME, TimerMode::Repeating)))
            
            // Systems
            .add_systems(Update, 
                (tick_timer, spawn_enemy_wave)
                .run_if(in_state(GameState::Playing))
                .in_set(GameLoopSchedules::Spawn)
            )
            .add_systems(Update, 
                follow_player
                .run_if(in_state(GameState::Playing))
                .in_set(GameLoopSchedules::EntityUpdates)
            )
        ;
    }
}

fn tick_timer(mut wave_timer: ResMut<WaveTimer>, time: Res<Time>) {
    wave_timer.0.tick(time.delta());
}


fn spawn_enemy_wave(
    wave_timer: Res<WaveTimer>,
    q_center: Query<&Transform, With<PlayerComponent>>,
    
    mut commands: Commands,
    mut events: EventWriter<SpawnMesh>,
) {
    if !wave_timer.0.just_finished() { return; }

    let center = if let Ok(ctr) = q_center.get_single() {
        ctr.translation
    } else { return; };

    let angle = (PI * 2.0) / (WAVE_SPAWNS_PER as f32);
    for n in 0..WAVE_SPAWNS_PER {
        let mut next_spawn_pt: Transform = Transform::from_translation(center);
        next_spawn_pt = next_spawn_pt.with_translation(Vec3::X*ENEMY_SPAWN_DIST);
        
        let rot = Quat::from_rotation_z(angle * (n as f32));
        next_spawn_pt.rotate_around(center, rot);

        spawn_enemy(next_spawn_pt, &mut commands, &mut events);
    }
}

fn spawn_enemy(
    spawn_pt: Transform,
    commands: &mut Commands,
    events: &mut EventWriter<SpawnMesh>,
) {
    info!("spawn enemy at: {:?}", spawn_pt.translation);
    
    let enemy_asset_key = AssetKey(ASSET_KEY_ENEMY.into());

    let enemy = commands.spawn((
        MovableObjectBundle{
            transform:SpatialBundle { transform: spawn_pt, ..default() },
            ..default()
        },
        EnemyComponent,
        enemy_asset_key.clone(),
    )).id();

    let t = Transform::default()
    .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
    .looking_at(-Vec3::Y, Vec3::Z);

    events.send(SpawnMesh(enemy, enemy_asset_key, t));
}

fn follow_player(
    mut q_enemy: Query<(Entity, &Transform, &mut Velocity), With<EnemyComponent>>,
    q_player: Query<&Transform, With<PlayerComponent>>,
    
    mut events: EventWriter<TriggerAnimation>,
) {

    let player_loc = if let Ok(t) = q_player.get_single() {
        t
    } else { return; };

    for (entity, enemy_loc, mut enemy_velocity) in q_enemy.iter_mut() {
        let mut move_vec = player_loc.translation - enemy_loc.translation;

        let next_animation: AnimationType;
        let dist_sq = move_vec.length_squared();
        if dist_sq > 0.25 {
            move_vec = move_vec.normalize() * ENEMY_MOVE_SPEED;
    
            enemy_velocity.0.x = move_vec.x;
            enemy_velocity.0.y = move_vec.y;

            next_animation = AnimationType::Walk;
        } else {
            enemy_velocity.0 = Vec3::ZERO;
            next_animation = AnimationType::Idle;
        }

        events.send(TriggerAnimation(entity, next_animation));
    }
}
