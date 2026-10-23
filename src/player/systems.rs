use crate::constants::*;
use crate::player::component::{Player, PlayerState};
use crate::resources::CursorPos;
use bevy::prelude::*;
pub fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut PlayerState, &mut Sprite), With<Player>>,
    time: Res<Time>,
    cursor_pos: Res<CursorPos>,
) {
    let Ok((mut transform, mut player_state, mut sprite)) = player_query.single_mut() else {
        return;
    };
    let w_key = keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp);
    let a_key = keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft);
    let s_key = keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown);
    let d_key =
        keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight);
    let mut delta = Vec2::ZERO;
    if w_key {
        delta.y += 1.0;
    }
    if a_key {
        delta.x -= 1.0;
        sprite.flip_x = true;
    }
    if s_key {
        delta.y -= 1.0;
    }
    if d_key {
        delta.x += 1.0;
        sprite.flip_x = false;
    }
    //normalize vector
    if delta.length() > 0.0 {
        delta = delta.normalize();
    }
    if delta.is_finite() && (w_key || s_key || a_key || d_key) {
        transform.translation +=
            Vec3::new(delta.x, delta.y, 0.0) * PLAYER_SPEED * time.delta_secs();
        transform.translation.z = 2.0;
        *player_state = PlayerState::Moving;
    }
    //flip player based on cursor position
    if let Some(cursor_pos) = cursor_pos.0 {
        if cursor_pos.x < transform.translation.x {
            sprite.flip_x = true;
        } else {
            sprite.flip_x = false;
        }
    }

    //reset back to idle mode if no keyboard input
    if keyboard_input.get_pressed().next().is_none() {
        *player_state = PlayerState::Idle;
    }
}
