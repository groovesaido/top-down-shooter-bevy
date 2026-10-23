use bevy::prelude::*;

use crate::{animation::systems::*, states::GameState};

pub mod components;
mod systems;

pub struct AnimationPlugin;
impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (animate_player, animation_timer_tick, animate_enemy)
                .run_if(in_state(GameState::InGame)),
        );
    }
}
