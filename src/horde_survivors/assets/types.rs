use bevy::{
    prelude::*, 
    utils::hashbrown::HashMap
};



pub const ASSET_KEY_PLAYER: &str = "player";
pub const ASSET_KEY_ENEMY: &str = "enemy";
pub const ASSET_KEY_DESTRUCTIBLE: &str = "destructible";
pub const ASSET_KEY_PROJECTILE: &str = "projectile";



/// The `Component` used to associate a root level `Entity` with the mesh
/// assets in the `MeshAssetMap` resource.
#[derive(Component, Debug, Default, Clone)]
pub struct AssetKey(pub String);


#[derive(PartialEq, Eq, Hash, Debug, Default, Clone, Copy)]
pub enum AnimationType {
    #[default]
    Idle = 0,
    Walk,
    Run,
    TakeHit,
    Die,
}

#[derive(Event, Debug)]
pub struct LoadingUpdate(pub usize, pub usize);


#[derive(Event, Debug)]
pub struct TriggerAnimation(pub Entity, pub AnimationType);


#[derive(Event, Debug)]
pub struct SpawnMesh(pub Entity, pub AssetKey, pub Transform);


// =================================
//  Asset Handle Tracking Resources
// =================================

pub(super) struct MeshAssets {
    pub mesh: Handle<Scene>,
    pub animations: Option<Animations>,
}

#[derive(Resource, Default)]
pub(super) struct MeshAssetMap(pub HashMap<String, MeshAssets>);

#[derive(Resource, Default)]
pub(super) struct Animations(pub HashMap<AnimationType, Handle<AnimationClip>>);



// =================================
//  Entity Tracking Resources
// =================================


/// When a `SpawnMesh` event is handled, it updates this Resource to associates an `Entity` with
/// the `AssetKey` needed to lookup its mesh (and animation) assets.
#[derive(Resource, Default)]
pub(super) struct EntityAssetMapping(pub HashMap<Entity, AssetKey>);

