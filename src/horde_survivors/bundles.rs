use bevy::prelude::*;

use crate::{Acceleration, Velocity};


#[derive(Bundle, Default)]
pub struct MovableObjectBundle {
    pub transform: SpatialBundle,
    pub velocity: Velocity,
    pub acceleration: Acceleration,

    // TODO: collider / bounding box?
}

#[derive(Bundle, Default)]
pub struct StaticObjectBundle {
    pub transform: SpatialBundle,
    // TODO: collider / bounding box?
}
