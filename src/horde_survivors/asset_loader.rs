use bevy::{asset::{LoadState, UntypedAssetId}, prelude::*, utils::HashMap};

use crate::{
    GameLoopSchedules, 
    GameState,
};

use super::ui::loading;

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
    // Walk,
    Run,
    // TakeHit,
    // Die,
}

#[derive(Resource, Debug, Default)]
pub struct Animations(pub Vec<Handle<AnimationClip>>);


#[derive(Resource, Debug, Default)]
pub struct LoadingTimer(Timer);

/// a mapping from `root entity` -> `animation player's entity`
#[derive(Resource, Default)]
pub struct AnimationPlayerMapping(pub HashMap<Entity, Entity>);

// to track the loading state of all assets
#[derive(Resource, Default)]
pub struct LoadingAssets(pub HashMap<UntypedAssetId,LoadState>);
impl LoadingAssets {
    pub fn num_loaded(&self) -> usize {
        self.0.iter().filter_map(|(_, state)| {
            if *state == LoadState::Loaded { Some(()) } else { None } 
        }).count()
    }
    pub fn num_failed(&self) -> usize {
        self.0.iter().filter_map(|(_, state)| {
            if *state == LoadState::Failed { Some(()) } else { None } 
        }).count()
    }
}


pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MeshAssets::default())
            .insert_resource(AnimationPlayerMapping::default())
            
            // 'loading' only resources / systems
            .insert_resource(LoadingAssets::default())
            .insert_resource(LoadingTimer(Timer::from_seconds(0.5, TimerMode::Once)))
            .add_systems(Startup, load_meshes)
            .add_systems(Update, load_progress
                .run_if(in_state(GameState::Loading)))
            .add_systems(OnExit(GameState::Loading), cleanup_loading_resources)
            .add_plugins(loading::UIPlugin) // add the loading UI
            
            // in-game systems
            .add_systems(Update, 
                associate_animation_players_to_root_entities
                    .in_set(GameLoopSchedules::PostSpawn),
            );
    }
}


fn load_meshes(
    mut assets: ResMut<MeshAssets>, 
    mut loading_assets: ResMut<LoadingAssets>,
    asset_server: Res<AssetServer>,
) {
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

    loading_assets.0.insert(assets.player.clone_weak().untyped().id(), LoadState::NotLoaded);
    assets.player_animations.0.iter()
        .for_each(|a| { loading_assets.0.insert(a.clone_weak().untyped().id(), LoadState::NotLoaded); } );
    
    loading_assets.0.insert(assets.enemy.clone_weak().untyped().id(), LoadState::NotLoaded);
    loading_assets.0.insert(assets.projectile.clone_weak().untyped().id(), LoadState::NotLoaded);
    loading_assets.0.insert(assets.destructible.clone_weak().untyped().id(), LoadState::NotLoaded);
}

fn load_progress(
    server: Res<AssetServer>,
    mut loading: ResMut<LoadingAssets>,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<LoadingTimer>,
) {
    // update our cached progress:
    for (id, state) in loading.0.iter_mut() {
        match server.get_load_state(*id) {
            Some(LoadState::Failed) => {
                warn!("asset {:?} failed to load!", id);
                *state = LoadState::Failed;
            },
            Some(LoadState::Loaded) => {
                *state = LoadState::Loaded;
            },
            _ => {},
        }
    }

    timer.0.tick(time.delta());
    if !timer.0.finished() { return; }

    if loading.num_loaded() == loading.0.len() {
        info!("{:?} assets loaded", loading.0.len());

        // set next state
        next_state.set(GameState::Initialize);
    }
}

fn cleanup_loading_resources(
    mut commands: Commands,
) {
    // remove the resource to drop the handles used for tracking the 'loading' state.
    commands.remove_resource::<LoadingAssets>();
    commands.remove_resource::<LoadingTimer>();
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
