use bevy::prelude::*;

use crate::{MeshAssets, MovableObjectBundle, Velocity};

#[derive(Component, Debug, Default)]
pub(crate) struct PlayerComponent;

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
        app.add_systems(PostStartup, spawn_player)
            .add_systems(Update, handle_move_ctl);
    }
}

fn spawn_player(mut commands: Commands, assets: Res<MeshAssets>) {
    commands.spawn(PlayerBundle::default())
        .with_children(|parent|{
            let t = Transform::from_translation(Vec3::default())
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
                .looking_at(-Vec3::Y, Vec3::Z);
            
            parent.spawn(SceneBundle { 
                scene: assets.player.clone(), 
                transform: t,
                ..default()
            });
        });
}

fn handle_move_ctl(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &PlayerModifiers), With<PlayerComponent>>,
) {
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

    let (mut velocity, modifiers) = query.single_mut();
    move_dir *= modifiers.move_speed * modifiers.move_speed_mod;
    
    velocity.0 = move_dir;
}
