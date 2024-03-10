use bevy::{asset::{LoadState, UntypedAssetId}, prelude::*, utils::hashbrown::HashMap};

use crate::GameState;
use super::types::*;

#[derive(Resource, Debug, Default)]
pub struct LoadingTimer(Timer);

// to track the loading state of all assets
#[derive(Resource, Default)]
pub struct LoadingAssets(pub HashMap<UntypedAssetId,LoadState>);
impl LoadingAssets {
    pub fn num_loaded(&self) -> usize {
        self.0.iter().filter_map(|(_, state)| {
            if *state == LoadState::Loaded { Some(()) } else { None } 
        }).count()
    }
    // pub fn num_failed(&self) -> usize {
    //     self.0.iter().filter_map(|(_, state)| {
    //         if *state == LoadState::Failed { Some(()) } else { None } 
    //     }).count()
    // }
}


pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .insert_resource(MeshAssetMap::default())
            .insert_resource(LoadingAssets::default())
            .insert_resource(LoadingTimer(Timer::from_seconds(0.5, TimerMode::Once)))

            // Events
            .add_event::<LoadingUpdate>()

            // Systems
            .add_systems(Startup, load_meshes)
            .add_systems(Update, update_loading_progress
                .run_if(in_state(GameState::Loading)))
            .add_systems(OnExit(GameState::Loading), cleanup_loading_resources)
        ;
    }
}


fn load_meshes(
    mut assets: ResMut<MeshAssetMap>, 
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

    let player_assets = MeshAssets{
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
    let enemy_assets = MeshAssets{
        mesh: asset_server.load("Skeleton.glb#Scene0"),
        animations: Some(Animations(enemy_animations)),
    };
    loading_assets.0.insert(enemy_assets.mesh.clone_weak().untyped().id(), LoadState::NotLoaded);
    
    //  Throwing Dagger
    let projectile_assets = MeshAssets{
        mesh:asset_server.load("Dagger.glb#Scene0"),
        animations: None,
    };
    loading_assets.0.insert(projectile_assets.mesh.clone_weak().untyped().id(), LoadState::NotLoaded);

    let destructible_assets = MeshAssets{
        mesh:asset_server.load("Torch.glb#Scene0"),
        animations: None,
    };
    loading_assets.0.insert(destructible_assets.mesh.clone_weak().untyped().id(), LoadState::NotLoaded);
    
    
    assets.0.insert(ASSET_KEY_PLAYER.into(), player_assets);
    assets.0.insert(ASSET_KEY_ENEMY.into(), enemy_assets);
    assets.0.insert(ASSET_KEY_PROJECTILE.into(), projectile_assets);
    assets.0.insert(ASSET_KEY_DESTRUCTIBLE.into(), destructible_assets);
}

fn update_loading_progress(
    server: Res<AssetServer>,
    mut loading: ResMut<LoadingAssets>,
    
    mut curr_loaded: Local<usize>,
    mut events: EventWriter<LoadingUpdate>,
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

    let num_loaded = loading.num_loaded();
    let total_loading = loading.0.len();
    if num_loaded != *curr_loaded {
        *curr_loaded = num_loaded;
        events.send(LoadingUpdate(num_loaded, total_loading));
    }

    timer.0.tick(time.delta());
    if !timer.0.finished() { return; }

    if num_loaded == total_loading {
        info!("{:?} assets loaded", total_loading);

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


