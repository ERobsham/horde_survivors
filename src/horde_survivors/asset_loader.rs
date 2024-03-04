use bevy::{prelude::*, utils::HashMap};

use crate::GameLoopSchedules;

#[derive(Resource, Debug, Default)]
pub struct MeshAssets {
    pub player: Handle<Scene>,
    pub player_animations: Animations,
    pub enemy: Handle<Scene>,
    pub projectile: Handle<Scene>,
    pub destructible: Handle<Scene>,
}

pub enum AnimationType {
    Idle = 0,
    Walk,
    Run,
    TakeHit,
    Die,
}

#[derive(Resource, Debug, Default)]
pub struct Animations(pub Vec<Handle<AnimationClip>>);

/// a mapping from `root entity` -> `animation player's entity`
#[derive(Resource, Default)]
pub struct AnimationPlayerMapping(pub HashMap<Entity, Entity>);



pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MeshAssets::default())
            .insert_resource(AnimationPlayerMapping::default())
            .add_systems(Startup, load_meshes)
            .add_systems(Update, 
                associate_animation_players_to_root_entities
                    .in_set(GameLoopSchedules::PostSpawn),
            );
    }
}


fn load_meshes(mut assets: ResMut<MeshAssets>, asset_server: Res<AssetServer>) {
    *assets = MeshAssets {
        player_animations: Animations(vec![
            asset_server.load("Anne.glb#Animation3"), // idle 4 (idx 3)
            asset_server.load("Anne.glb#Animation11"), // walk 12
            asset_server.load("Anne.glb#Animation9"), // run 10
            asset_server.load("Anne.glb#Animation2"), // take hit 3
            asset_server.load("Anne.glb#Animation0"), // die 1
        ]),
        player: asset_server.load("Anne.glb#Scene0"),

        enemy: asset_server.load("Skeleton.glb#Scene0"),
        projectile: asset_server.load("Dagger.glb#Scene0"),
        destructible: asset_server.load("Torch.glb#Scene0"),
    };
}

pub(crate) fn associate_animation_players_to_root_entities(
    mut mapping: ResMut<AnimationPlayerMapping>,
    players:Query<Entity, Added<AnimationPlayer>>,
    parents:Query<&Parent>,
) {
    for entity in players.iter() {
        let root_entity = get_root(entity, &parents);
        mapping.0.insert(root_entity, entity);

        info!("entity {:?} with animation player added --> mapped to root entity {:?}", entity, root_entity);
    }
}

fn get_root(entity: Entity, q_parents: &Query<&Parent>) -> Entity {
    let mut cur_entity = entity;
    loop {
        if let Ok(parent) = q_parents.get(cur_entity) {
            cur_entity = parent.get();
        } else {
            break cur_entity
        }
    }
}
