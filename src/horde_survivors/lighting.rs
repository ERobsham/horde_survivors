use bevy::prelude::*;

const BG_COLOR: Color = Color::BLACK;

pub struct LightingPlugin;
impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BG_COLOR))
            .insert_resource(AmbientLight::default());
    }
}

// TODO: maybe add systems to mess with the scene lighting?
//  * fade in from back / dark on 'start'?
//  * fade out to back / dark on 'game over'?
