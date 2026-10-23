use bevy::prelude::*;

use crate::{
    animation::components::AnimationTimer,
    constants::SPRITE_SHEET_W,
    player::component::{Player, PlayerState},
};

pub fn animation_timer_tick(
    mut query: Query<&mut AnimationTimer, With<AnimationTimer>>,
    time: Res<Time>,
) {
    for mut timer in query.iter_mut() {
        timer.tick(time.delta());
    }
}

pub fn animate_player(
    mut player_query: Query<(&mut Sprite, &AnimationTimer, &PlayerState), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }
    let Ok((mut sprite, timer, player_state)) = player_query.single_mut() else {
        return;
    };
    if timer.just_finished() {
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            let base_sprite_index = match player_state {
                PlayerState::Idle => 0,
                PlayerState::Moving => 4,
            };
            atlas.index = base_sprite_index + (atlas.index + 1) % SPRITE_SHEET_W as usize;
        }
    }
}
