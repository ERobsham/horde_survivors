use bevy::prelude::*;

use super::{
    animator::MeshAnimatorPlugin,
    loader::AssetLoaderPlugin,
    mesh_spawner::MeshSpawnerPlugin,
    ui,
};


pub struct AssetHandlerPlugin;
impl Plugin for AssetHandlerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(AssetLoaderPlugin)
            .add_plugins(ui::loading::UIPlugin)
            .add_plugins(MeshSpawnerPlugin)
            .add_plugins(MeshAnimatorPlugin)
        ;
    }
}
