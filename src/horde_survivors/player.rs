use bevy::prelude::*;
use bevy::utils::Duration;

use crate::{
    AnimationPlayerMapping, AnimationType, GameLoopSchedules, GameState, MeshAssets, MovableObjectBundle, Velocity
};

#[derive(Component, Debug, Default)]
pub struct PlayerComponent;
#[derive(Component, Debug, Default)]
pub struct PlayerMesh;

#[derive(Component, Debug)]
struct PlayerModifiers {
    move_speed: f32,
    move_speed_mod: f32,
}
impl Default for PlayerModifiers {
    fn default() -> Self {
        Self {
            move_speed: 5.0,
            move_speed_mod: 1.0,
        }
    }
}

#[derive(Bundle, Default)]
struct PlayerBundle {
    movement: MovableObjectBundle,
    modifiers: PlayerModifiers,
    marker: PlayerComponent,
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Initialize), 
                spawn_player
                .in_set(GameLoopSchedules::Spawn),
            )
            .add_systems(Update, 
                handle_move_ctl
                .in_set(GameLoopSchedules::ProcessInput),
            )
            .add_systems(Update, 
                start_idle_animation
                .in_set(GameLoopSchedules::EntityUpdates),
            );
    }
}

pub(crate) fn spawn_player(
    mut commands: Commands, 
    assets: Res<MeshAssets>,
) {
    info!("spawning player");
    commands.spawn(PlayerBundle::default())
        .with_children(|parent|{
            let t = Transform::default()
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
                .looking_at(-Vec3::Y, Vec3::Z);
            
            parent.spawn(SceneBundle { 
                scene: assets.player.clone_weak(), 
                transform: t,
                ..default()
            });
        });
}

fn start_idle_animation(
    assets: Res<MeshAssets>,
    animator_map: Res<AnimationPlayerMapping>,
    q_players: Query<Entity, (With<PlayerComponent>, Added<PlayerComponent>)>,
    mut q_animators: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for player_entity in q_players.iter() {
        let animator_entity = if let Some(entity) = animator_map.0.get(&player_entity) { 
            *entity 
        } else { continue; };
        
        if let Ok(mut animator) = q_animators.get_mut(animator_entity) {
            animator.play(assets.player_animations.0[AnimationType::Idle as usize].clone_weak())
                .repeat();
        } else {
            warn!("no AnimationPlayer for this player entity!");
        }
    }
}

fn handle_move_ctl(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &PlayerModifiers, Entity), With<PlayerComponent>>,
    assets: Res<MeshAssets>,
    animator_map: Res<AnimationPlayerMapping>,
    mut q_animators: Query<&mut AnimationPlayer>,
) {
    let (mut velocity, modifiers, player_entity) = if let Ok(res) = query.get_single_mut() {
        res
    } else { return; };
    
    let mut move_dir = Vec3::ZERO;

    // TODO: abstract this a bit more so we can handle multiple sets of inputs
    //       maybe even configurable sets for keyboard / game pads etc?
    if keyboard_input.pressed(KeyCode::KeyA) {
        move_dir -= Vec3::X;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        move_dir += Vec3::X;
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        move_dir += Vec3::Y;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        move_dir -= Vec3::Y;
    }

    move_dir *= modifiers.move_speed * modifiers.move_speed_mod;
    velocity.0 = move_dir;


    let animator_entity = if let Some(res) = animator_map.0.get(&player_entity) {
        *res
    } else { return; };
    let mut animator = if let Ok(res) = q_animators.get_mut(animator_entity) {
        res
    } else { return; };

    let length = move_dir.length_squared();
    if length < 0.05 {
        animator.play_with_transition(
            assets.player_animations.0[AnimationType::Idle as usize].clone_weak(),
                Duration::from_millis(500),
            )
            .repeat();
    } else {
        animator.play_with_transition(
            assets.player_animations.0[AnimationType::Run as usize].clone_weak(),
                Duration::from_millis(250),
            )
            .repeat();
    }
}
