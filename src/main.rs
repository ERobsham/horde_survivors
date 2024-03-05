use bevy::prelude::*;
// use bevy::log::LogPlugin;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod horde_survivors;
use horde_survivors::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
    // app.add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
    //     .add_plugins(WorldInspectorPlugin::new())
        .add_plugins((
            StatePlugin,
            SchedulesPlugin,
            LightingPlugin,
            CameraPlugin,
            AssetLoaderPlugin,
            MovementPlugin,
            PlayerPlugin,
        ))
        .add_systems(Startup, setup_test_scene);
    
    // bevy_mod_debugdump::print_schedule_graph(&mut app, Update);
    app.run();
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