use bevy::{prelude::*, utils::hashbrown::HashMap};
use bevy::utils::Duration;

use crate::{AnimationPlayerMapping, AnimationType, AssetKey, GameLoopSchedules, MeshAssets};


#[derive(Event, Debug)]
pub struct TriggerAnimation(pub Entity, pub AnimationType);

#[derive(Resource, Default)]
struct EntityAssetMapping(pub HashMap<Entity, AssetKey>);

pub struct CharacterAnimatorPlugin;
impl Plugin for CharacterAnimatorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EntityAssetMapping::default())
            .add_event::<TriggerAnimation>()
            .add_systems(Update, start_idle_animation
                .in_set(GameLoopSchedules::EntityUpdates))
            .add_systems(Update, trigger_animation
                .in_set(GameLoopSchedules::EntityUpdates))
            ;
    }
}

fn start_idle_animation(
    assets: Res<MeshAssets>,
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
            animator.play(animations.0.get(&AnimationType::Idle).expect("idle animation must be set").clone_weak())
                .repeat();
        } else {
            warn!("no AnimationPlayer for this entity!");
        }
    }
}

fn trigger_animation(
    mut events: EventReader<TriggerAnimation>,
    assets: Res<MeshAssets>,
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
                animations.0.get(&event.1).expect("all animation must be set").clone_weak(),
                Duration::from_millis(250),
                )
                .repeat();
        } else {
            warn!("no AnimationPlayer for this entity!");
        }
    }
}

