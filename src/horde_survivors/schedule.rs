use bevy::prelude::*;


#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub enum GameLoopSchedules {
    ProcessInput,
    Spawn,
    // flush commands here to ensure all things
    // are created before continuing
    PostSpawn,
    // another flush, to ensure this system set runs all alone...?
    EntityUpdates,
    CollisionDetection,
    Despawn,
}

pub struct SchedulesPlugin;
impl Plugin for SchedulesPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, (
            GameLoopSchedules::ProcessInput,
            GameLoopSchedules::Spawn,
            GameLoopSchedules::PostSpawn,
            GameLoopSchedules::EntityUpdates,
            GameLoopSchedules::CollisionDetection,
            GameLoopSchedules::Despawn,
        ).chain())
        .add_systems(Update, 
            apply_deferred
            .after(GameLoopSchedules::Spawn)
            .before(GameLoopSchedules::PostSpawn)
        );
    }
}
