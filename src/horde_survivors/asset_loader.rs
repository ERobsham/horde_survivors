use bevy::{asset::{LoadState, UntypedAssetId}, prelude::*, utils::hashbrown::HashMap};

use crate::{
    GameLoopSchedules, 
    GameState,
};

use super::ui::loading;


pub const ASSET_KEY_PLAYER: &str = "player";
pub const ASSET_KEY_ENEMY: &str = "enemy";
pub const ASSET_KEY_DESTRUCTIBLE: &str = "destructible";
pub const ASSET_KEY_PROJECTILE: &str = "projectile";


pub struct CharacterAssets {
    pub mesh: Handle<Scene>,
    pub animations: Option<Animations>,
}

#[derive(Resource, Default)]
pub struct MeshAssets(pub HashMap<String, CharacterAssets>);

#[derive(Component, Default, Clone)]
pub struct AssetKey(pub String);

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum AnimationType {
    Idle = 0,
    Walk,
    Run,
    TakeHit,
    Die,
}

#[derive(Resource, Default)]
pub struct Animations(pub HashMap<AnimationType, Handle<AnimationClip>>);


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
    //  Player
    let mut player_animations = HashMap::new();
    player_animations.insert(AnimationType::Idle, asset_server.load("Anne.glb#Animation3"));    // idle 4 (idx 3)
    player_animations.insert(AnimationType::Walk, asset_server.load("Anne.glb#Animation11"));   // walk 12
    player_animations.insert(AnimationType::Run, asset_server.load("Anne.glb#Animation9"));     // run 10
    player_animations.insert(AnimationType::TakeHit, asset_server.load("Anne.glb#Animation2")); // take hit 3
    player_animations.insert(AnimationType::Die, asset_server.load("Anne.glb#Animation0"));     // die 1
    player_animations.iter().for_each(|(_, a)| { 
        loading_assets.0.insert(a.clone_weak().untyped().id(), LoadState::NotLoaded); 
    });

    let player_assets = CharacterAssets{
        mesh: asset_server.load("Anne.glb#Scene0"),
        animations: Some(Animations(player_animations)),
    };
    loading_assets.0.insert(player_assets.mesh.clone_weak().untyped().id(), LoadState::NotLoaded);
    

    //  Skeleton
    let mut enemy_animations = HashMap::new();
    enemy_animations.insert(AnimationType::Idle, asset_server.load("Skeleton.glb#Animation3"));    // idle 4 (idx 3)
    enemy_animations.insert(AnimationType::Walk, asset_server.load("Skeleton.glb#Animation12"));   // walk 13
    enemy_animations.insert(AnimationType::Run, asset_server.load("Skeleton.glb#Animation10"));    // run 11
    enemy_animations.insert(AnimationType::TakeHit, asset_server.load("Skeleton.glb#Animation2")); // take hit 3
    enemy_animations.insert(AnimationType::Die, asset_server.load("Skeleton.glb#Animation0"));     // die 1
    enemy_animations.iter().for_each(|(_, a)| { 
        loading_assets.0.insert(a.clone_weak().untyped().id(), LoadState::NotLoaded); 
    });
    let enemy_assets = CharacterAssets{
        mesh: asset_server.load("Skeleton.glb#Scene0"),
        animations: Some(Animations(enemy_animations)),
    };
    loading_assets.0.insert(enemy_assets.mesh.clone_weak().untyped().id(), LoadState::NotLoaded);
    
    //  Throwing Dagger
    let projectile_assets = CharacterAssets{
        mesh:asset_server.load("Dagger.glb#Scene0"),
        animations: None,
    };
    loading_assets.0.insert(projectile_assets.mesh.clone_weak().untyped().id(), LoadState::NotLoaded);

    let destructible_assets = CharacterAssets{
        mesh:asset_server.load("Torch.glb#Scene0"),
        animations: None,
    };
    loading_assets.0.insert(destructible_assets.mesh.clone_weak().untyped().id(), LoadState::NotLoaded);
    
    
    assets.0.insert(ASSET_KEY_PLAYER.into(), player_assets);
    assets.0.insert(ASSET_KEY_ENEMY.into(), enemy_assets);
    assets.0.insert(ASSET_KEY_PROJECTILE.into(), projectile_assets);
    assets.0.insert(ASSET_KEY_DESTRUCTIBLE.into(), destructible_assets);
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
