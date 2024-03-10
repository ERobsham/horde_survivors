use bevy::{prelude::*,  utils::Duration};

use crate::GameLoopSchedules;
use super::types::*;


pub struct MeshAnimatorPlugin;
impl Plugin for MeshAnimatorPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .insert_resource(AnimationPlayerMapping::default())

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

fn start_idle_animation(
    assets: Res<MeshAssetMap>,
    mut asset_map: ResMut<EntityAssetMapping>,
    animator_map: Res<AnimationPlayerMapping>,
    q_entities: Query<(Entity, &AssetKey)>,
    mut q_animators: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    if q_animators.is_empty() { return; }
    for (entity, asset_key) in q_entities.iter() {
        info!("starting AnimationPlayer for entity: {:?} | {:?}", entity, asset_key.0);

        let animator_entity = if let Some(entity) = animator_map.0.get(&entity) { 
            *entity 
        } else { continue; };
        let entity_assets = if let Some(e_assets) = assets.0.get(&asset_key.0) { 
            e_assets
        } else { continue; };
        let animations = if let Some(animations) = &entity_assets.animations { 
            animations
        } else { continue; };

        asset_map.0.insert(entity, asset_key.clone());

        if let Ok(mut animator) = q_animators.get_mut(animator_entity) {
            animator.play(animations.0.get(&AnimationType::Idle).expect("idle animation type must be set").clone_weak())
                .repeat();
        } else {
            warn!("no AnimationPlayer for this entity!");
        }
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

