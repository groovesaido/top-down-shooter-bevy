use bevy::prelude::*;

use crate::{
    animation::components::AnimationTimer, gun::components::Gun, player::component::Player,
    resources::CursorPos,
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
    mut player_query: Query<(&mut Sprite, &AnimationTimer, &Transform), With<Player>>,
    cursor_pos: Res<CursorPos>,
) {
    if player_query.is_empty() {
        return;
    }
    let Ok((mut sprite, timer, player_transform)) = player_query.single_mut() else {
        return;
    };
    if timer.just_finished() {
        if let Some(mut atlas) = sprite.texture_atlas.as_mut() {
            if let Some(cursor_pos) = cursor_pos.0 {
                if cursor_pos.x < player_transform.translation.x {
                    sprite.flip_x = true;
                } else {
                    sprite.flip_x = false;
                }
            }
        }
    }
}
