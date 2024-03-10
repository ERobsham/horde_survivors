use bevy::prelude::*;
use bevy::utils::Duration;

use crate::{
    AnimationPlayerMapping, AnimationType, AssetKey, GameLoopSchedules, GameState, MeshAssets, MovableObjectBundle, TriggerAnimation, Velocity, ASSET_KEY_PLAYER
};

#[derive(Component, Debug, Default)]
pub struct PlayerComponent;

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
    asset_key: AssetKey,
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
            ;
    }
}

pub(crate) fn spawn_player(
    mut commands: Commands, 
    assets: Res<MeshAssets>,
) {
    info!("spawning player");
    commands.spawn(PlayerBundle{
        asset_key: AssetKey(ASSET_KEY_PLAYER.into()),
        ..default()
    })
        .with_children(|parent|{
            let t = Transform::default()
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
                .looking_at(-Vec3::Y, Vec3::Z);

            let player_assets = assets.0.get(ASSET_KEY_PLAYER).expect("hardcoded asset");
            
            parent.spawn(SceneBundle { 
                scene: player_assets.mesh.clone_weak(), 
                transform: t,
                ..default()
            });
        });
}

fn handle_move_ctl(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut events: EventWriter<TriggerAnimation>,
    mut query: Query<(&mut Velocity, &PlayerModifiers, Entity), With<PlayerComponent>>,
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

    if move_dir.length_squared() > 0.05 {
        move_dir = move_dir.normalize();
    }

    move_dir *= modifiers.move_speed * modifiers.move_speed_mod;
    velocity.0 = move_dir;


    let length = move_dir.length_squared();
    if length < 0.05 {
        events.send(TriggerAnimation(player_entity, AnimationType::Idle));
    } else {
        events.send(TriggerAnimation(player_entity, AnimationType::Run));
    }
}
