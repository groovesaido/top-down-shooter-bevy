use bevy::prelude::*;

pub mod componets;
mod systems;
use crate::states::*;
use systems::*;
pub struct BulletPlugins;
impl Plugin for BulletPlugins {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(
            Update,
            (update_bullets, despawn_old_bullets).run_if(in_state(GameState::InGame)),
        );
    }
}
