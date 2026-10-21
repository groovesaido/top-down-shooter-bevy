use bevy::prelude::*;
pub mod components;
mod systems;
use crate::states::*;
use systems::*;
pub struct GunPlugin;
impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_gun_transform, handle_gun_input).run_if(in_state(GameState::InGame)),
        );
    }
}
