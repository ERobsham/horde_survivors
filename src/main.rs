use bevy::prelude::*;

mod horde_survivors;
use horde_survivors::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(LightingPlugin)
    .add_plugins(CameraPlugin)
    .add_plugins(AssetLoaderPlugin)
    .add_plugins(MovementPlugin)
    .add_plugins(PlayerPlugin)
    .add_systems(Startup, setup_test_scene)
    .run();
}


fn setup_test_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let t = Transform::from_rotation(Quat::from_rotation_x(0.0));
    // info!("base up      : {:?}", t.forward());
    // info!("base position: {:?}", t.translation);

    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(150.0)),
        material: materials.add(Color::WHITE),
        transform: t,
        ..default()
    });

    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::rgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(-2.0, -2.0, 0.0),
        ..default()
    });

    // light
    // commands.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // });
}