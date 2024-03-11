use bevy::{
    prelude::*,  
    utils::Duration,
    utils::hashbrown::HashMap,
};

use crate::GameLoopSchedules;
use super::types::*;

/// when an `Entity` with an `AssetKey` and has a child `AnimationPlayer` added,
/// this resource is updated to track which `Entity` is the `AnimationPlayer` for a given root level `Entity`.
/// ie. `get(root_entity)` => produces `AnimationPlayer`'s `Entity`
#[derive(Resource, Default)]
pub struct AnimationPlayerMapping(pub HashMap<Entity, Entity>);

/// The Reverse of `AnimationPlayerMapping`.  Mapping from an `AnimationPlayer`'s `Entity` to the root `Entity`
/// ie. `get(animator_entity)` => produces root `Entity`
#[derive(Resource, Default)]
pub struct AnimationPlayerReverseMapping(pub HashMap<Entity, Entity>);


pub struct MeshAnimatorPlugin;
impl Plugin for MeshAnimatorPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .insert_resource(AnimationPlayerMapping::default())
            .insert_resource(AnimationPlayerReverseMapping::default())

            // Events
            .add_event::<TriggerAnimation>()
            
            // Systems
            .add_systems(Update, 
                associate_animation_players_to_root_entities
                .in_set(GameLoopSchedules::PostSpawn))

            .add_systems(Update, start_idle_animation
                .in_set(GameLoopSchedules::EntityUpdates))
            .add_systems(Update, trigger_animation
                .in_set(GameLoopSchedules::EntityUpdates))
        ;
    }
}

fn associate_animation_players_to_root_entities(
    mut mapping: ResMut<AnimationPlayerMapping>,
    mut rev_mapping: ResMut<AnimationPlayerReverseMapping>,
    players:Query<Entity, Added<AnimationPlayer>>,
    parents:Query<&Parent>,
) {
    for entity in players.iter() {
        let root_entity = get_root(entity, &parents);
        mapping.0.insert(root_entity, entity);
        rev_mapping.0.insert(entity, root_entity);

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

fn start_idle_animation(
    assets: Res<MeshAssetMap>,
    animator_rev_map: Res<AnimationPlayerReverseMapping>,
    asset_map: Res<EntityAssetMapping>,
    mut q_animators: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (animator_entity, mut animator) in q_animators.iter_mut() {
        let root_entity = if let Some(entity) = animator_rev_map.0.get(&animator_entity) { 
            *entity 
        } else { continue; };
        let asset_key: &AssetKey = if let Some(key) = asset_map.0.get(&root_entity) {
            key
        } else { continue; };
        let entity_assets = if let Some(e_assets) = assets.0.get(&asset_key.0) { 
            e_assets
        } else { continue; };
        let animations = if let Some(animations) = &entity_assets.animations { 
            animations
        } else { continue; };

        info!("starting AnimationPlayer for entity: {:?} | {:?}", root_entity, asset_key.0);
        animator.play(
            animations.0
            .get(&AnimationType::Idle)
            .expect("idle animation type must be set")
            .clone_weak()
        ).repeat();
    }
}

fn trigger_animation(
    mut events: EventReader<TriggerAnimation>,
    assets: Res<MeshAssetMap>,
    asset_map: Res<EntityAssetMapping>,
    animator_map: Res<AnimationPlayerMapping>,
    mut q_animators: Query<&mut AnimationPlayer>,
) {
    for event in events.read() {
        let animator_entity = if let Some(entity) = animator_map.0.get(&event.0) { 
            *entity 
        } else { continue; };
        let asset_key = if let Some(key) = asset_map.0.get(&event.0) { 
            key 
        } else { continue; };
        let entity_assets = if let Some(key) = assets.0.get(&asset_key.0) { 
            key 
        } else { continue; };
        let animations = if let Some(animations) = &entity_assets.animations { 
            animations
        } else { continue; };


        if let Ok(mut animator) = q_animators.get_mut(animator_entity) {
            animator.play_with_transition(
                animations.0.get(&event.1).expect("all animation types must be set").clone_weak(),
                Duration::from_millis(250),
                )
                .repeat();
        } else {
            warn!("no AnimationPlayer for this entity!");
        }
    }
}
