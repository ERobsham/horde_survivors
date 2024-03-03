use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct MeshAssets {
    pub player: Handle<Scene>,
    pub enemy: Handle<Scene>,
    pub projectile: Handle<Scene>,
    pub destructible: Handle<Scene>,
}

pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MeshAssets::default())
            .add_systems(Startup, load_meshes);
    }
}


fn load_meshes(mut assets: ResMut<MeshAssets>, asset_server: Res<AssetServer>) {
    *assets = MeshAssets {
        player: asset_server.load("Anne.glb#Scene0"),
        enemy: asset_server.load("Skeleton.glb#Scene0"),
        projectile: asset_server.load("Dagger.glb#Scene0"),
        destructible: asset_server.load("Torch.glb#Scene0"),
    };
}