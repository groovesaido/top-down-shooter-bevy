use bevy::prelude::*;
pub mod component;

pub mod systems;

use crate::states::*;
use systems::*;
pub struct PlayerPlugins;

impl Plugin for PlayerPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_player_input).run_if(in_state(GameState::InGame)),
        );
    }
}
