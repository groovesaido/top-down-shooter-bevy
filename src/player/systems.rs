use crate::constants::*;
use crate::player::component::Player;
use bevy::prelude::*;
pub fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let Ok(mut transform) = player_query.single_mut() else {
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
    }
    if s_key {
        delta.y -= 1.0;
    }
    if d_key {
        delta.x += 1.0;
    }
    //normalize vector
    if delta.length() > 0.0 {
        delta = delta.normalize();
    }
    if delta.is_finite() && (w_key || s_key || a_key || d_key) {
        transform.translation +=
            Vec3::new(delta.x, delta.y, 0.0) * PLAYER_SPEED * time.delta_secs();
        transform.translation.z = 2.0;
    }
}
