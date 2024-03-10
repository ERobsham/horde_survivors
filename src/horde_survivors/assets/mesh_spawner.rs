use bevy::prelude::*;

use crate::GameLoopSchedules;
use super::types::*;

pub struct MeshSpawnerPlugin;
impl Plugin for MeshSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app
            // resources
            .insert_resource(EntityAssetMapping::default())

            // events
            .add_event::<SpawnMesh>()

            //systems
            .add_systems(Update, 
                spawn_mesh
                .in_set(GameLoopSchedules::Spawn)
        );
    }
}

fn spawn_mesh(
    mut commands: Commands,
    mut events: EventReader<SpawnMesh>,
    assets: Res<MeshAssetMap>,
    mut asset_map: ResMut<EntityAssetMapping>,
) {
    for event in events.read() {
        let (entity, asset_key, initial_transform) = (event.0, event.1.clone(), event.2);

        let mesh_assets = if let Some(res) = assets.0.get(&asset_key.0) { 
            info!("associating mesh asset for entity: {:?} =uses=> {:?}", entity, asset_key);
            asset_map.0.insert(entity, asset_key.clone());
            res
        } else { 
            warn!("no assets for AssetKey: {:?}  (Entity: {:?})", asset_key, entity);
            continue;
        };

        let mesh_id = commands.spawn(SceneBundle { 
            scene: mesh_assets.mesh.clone_weak(), 
            transform: initial_transform,
            ..default()
        }).id();
        commands.entity(entity).add_child(mesh_id);
    }
}
