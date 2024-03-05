use bevy::prelude::*;

use crate::{GameLoopSchedules, PlayerComponent};

const CAMERA_DISTANCE: f32 = 20.0;
const CAMERA_FOLLOW_SPEED: f32 = 1.5;

#[derive(Component, Debug)]
pub struct MainCamera;
fn default_camera_transform() -> Transform {
    let start_loc = Vec3{x:1.0, y:-1.0, z:4.0}.normalize() * CAMERA_DISTANCE;
    let t = Transform::from_translation(start_loc)
        .looking_at(Vec3::ZERO, Vec3::Y);

    // info!("cam up      : {:?}", t.up());
    // info!("cam forward : {:?}", t.forward());
    // info!("cam position: {:?}", t.translation);

    t
}
fn default_spotlight_transform() -> Transform {
    let start_loc = Vec3::Z * CAMERA_DISTANCE * 0.25;
    let t = Transform::from_translation(start_loc)
        .looking_at(Vec3::ZERO, Vec3::Y);

    // info!("spotlight up      : {:?}", t.up());
    // info!("spotlight forward : {:?}", t.forward());
    // info!("spotlight position: {:?}", t.translation);

    t
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, 
                follow_player
                .after_ignore_deferred(GameLoopSchedules::EntityUpdates)
            )
            ;
    }
}

fn spawn_camera(mut commands: Commands) {    
    commands.spawn((
        SpatialBundle::default(),
        MainCamera, 
    )).with_children(|parent|{
        // spawning the 'real' camera as a child allows us to use a default transform 
        // (centered at 0,0,0) to do all movement / follow calculations.
        parent.spawn( Camera3dBundle {
            transform: default_camera_transform(),
            ..default()
        });
        parent.spawn(PointLightBundle {
            transform: default_spotlight_transform(),
            ..default()
        });
    });
}

fn follow_player(
    time: Res<Time>,
    player: Query<&Transform, (With<PlayerComponent>, Without<MainCamera>)>,
    mut camera: Query<&mut Transform, (With<MainCamera>, Without<PlayerComponent>)>,
) {
    let player = if player.get_single().is_ok() { player.single() } else { return; };
    let mut camera = if camera.get_single_mut().is_ok() { camera.single_mut() } else { return; };

    let p_loc = Vec2{ x:player.translation.x, y:player.translation.y };
    let c_loc = Vec2{ x:camera.translation.x, y:camera.translation.y };
    let dist = p_loc.distance(c_loc);


    if dist > 2.0 {
        let dir2d = p_loc-c_loc;
        let dir = Vec3{ x: dir2d.x, y:dir2d.y, z:0.0};

        camera.translation += dir*CAMERA_FOLLOW_SPEED*time.delta_seconds();
    }
}
