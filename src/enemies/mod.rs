use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
pub mod components;
mod systems;
use systems::*;

use crate::{constants::ENEMY_SPAWN_INTERVAL, states::GameState};
pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_enemies.run_if(on_timer(Duration::from_secs_f32(ENEMY_SPAWN_INTERVAL))),
                update_enemy_transform,
                despawn_dead_enemies,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
