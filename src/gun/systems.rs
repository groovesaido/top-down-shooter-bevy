use std::f32::consts::PI;

use super::components::*;
use crate::bullet::componets::*;
use crate::constants::*;
use crate::player::component::Player;
use crate::resources::*;
use bevy::prelude::*;

pub fn update_gun_transform(
    player_query: Query<&Transform, With<Player>>,
    mut gun_query: Query<&mut Transform, (With<Gun>, Without<Player>)>,
    cursor_pos: Res<CursorPos>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let Ok(mut gun_transform) = gun_query.single_mut() else {
        return;
    };
    let player_pos = player_transform.translation.truncate();
    let cursor_pos = match cursor_pos.0 {
        Some(pos) => pos,
        None => player_pos,
    };
    //gun rotate to player
    let angle = (player_pos.y - cursor_pos.y).atan2(player_pos.x - cursor_pos.x) + PI;
    gun_transform.rotation = Quat::from_rotation_z(angle);

    let offset = 20.0;

    gun_transform.translation = Vec3::new(
        player_pos.x + offset * angle.cos() + 10.0,
        player_pos.y + offset * angle.sin() - 5.0,
        gun_transform.translation.z,
    );
}

pub fn handle_gun_input(
    mut commands: Commands,
    mouse_btn_input: Res<ButtonInput<MouseButton>>,
    texture_atlas: ResMut<GlobalTextureAtlasHandle>,
    image_handler: ResMut<GlobalSpriteSheetHandle>,
    time: Res<Time>,
    mut gun_query: Query<(&Transform, &mut GunTimer), (With<Gun>, Without<Player>)>,
) {
    let Ok((gun_transform, mut gun_timer)) = gun_query.single_mut() else {
        return;
    };
    let gun_pos = gun_transform.translation.truncate();
    gun_timer.0.tick(time.delta());
    if !mouse_btn_input.pressed(MouseButton::Left) {
        return;
    }
    let bullet_direction = gun_transform.local_x();

    if gun_timer.0.elapsed_secs() >= BULLET_SPAWN_INTERVAL {
        gun_timer.0.reset();
        commands.spawn((
            Sprite::from_atlas_image(
                image_handler.0.clone().unwrap(),
                TextureAtlas {
                    layout: texture_atlas.0.clone().unwrap(),
                    index: 3,
                },
            ),
            Transform::from_translation(Vec3::new(gun_pos.x, gun_pos.y, 1.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            Bullet,
            BulletDirection(*bullet_direction),
        ));
    }
}
