use bevy::prelude::*;

use crate::{GameLoopSchedules, GameState};

const MOVEMENT_ROTATION_SPEED:f32 = 5.0;


#[derive(Component, Debug, Default)]
pub struct Velocity(pub Vec3);

#[derive(Component, Debug, Default)]
pub struct Acceleration(pub Vec3);

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, 
                (
                    update_velocity, 
                    update_position,
                    update_facing,
                )
                .run_if(in_state(GameState::Playing))
                .in_set(GameLoopSchedules::EntityUpdates)
            )
        ;
    }
}

fn update_velocity(
    time: Res<Time>,
    mut query: Query<(&Acceleration, &mut Velocity)>, 
) {
    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.0 += acceleration.0 * time.delta_seconds();
    }
}

fn update_position(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform)>, 
) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();
    }
}

fn update_facing(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform)>,
) {
    for (velocity, mut transform) in query.iter_mut() {
        let dist: f32 = velocity.0.length_squared();
        if  -0.05 < dist && dist < 0.05 { continue; }

        let target_angle = (f64::atan2(velocity.0.x as f64, velocity.0.y as f64) * -1.0) as f32;
        let target = Quat::from_rotation_z(target_angle);

        transform.rotation = transform.rotation.lerp(target, time.delta_seconds() * MOVEMENT_ROTATION_SPEED);
    }
}
