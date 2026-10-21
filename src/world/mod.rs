use bevy::prelude::*;
mod componet;
mod systems;
use crate::states::GameState;
use systems::*;
pub struct WorldPlugins;
impl Plugin for WorldPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::GameInit),
            (init_world, spawn_world_decorations),
        );
    }
}
