use super::components::*;
use crate::bullet::componets::*;
use crate::constants::*;
use crate::player::component::Player;
use crate::resources::*;
use bevy::prelude::*;

pub fn update_gun_transform(
    player_query: Query<&Transform, With<Player>>,
    mut gun_query: Query<(&mut Transform, &mut Sprite), (With<Gun>, Without<Player>)>,
    cursor_pos: Res<CursorPos>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let Ok((mut gun_transform, mut gun_sprite)) = gun_query.single_mut() else {
        return;
    };

    let player_pos = player_transform.translation.truncate();

    let Some(cursor_pos) = cursor_pos.0 else {
        return;
    };

    // Determine which side of the player the cursor is on
    let facing_left = cursor_pos.x < player_pos.x;

    // Keep the gun beside the player
    let offset_x = if facing_left { -15.0 } else { 20.0 };
    let offset_y = -10.0;

    let gun_pos = player_pos + Vec2::new(offset_x, offset_y);

    gun_transform.translation = Vec3::new(gun_pos.x, gun_pos.y, 9.0);

    // Calculate rotation from the gun's position to the cursor
    let direction = cursor_pos - gun_pos;

    let angle = direction.y.atan2(direction.x);

    gun_transform.rotation = Quat::from_rotation_z(angle);

    // Flip the gun vertically when aiming left
    gun_sprite.flip_y = facing_left;
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

    gun_timer.0.tick(time.delta());
    if !mouse_btn_input.pressed(MouseButton::Left) {
        return;
    }
    let bullet_direction = gun_transform.local_x();
    let barrel_lenght = 16.0;
    let muzzle_position = gun_transform.translation + bullet_direction * barrel_lenght;

    if gun_timer.0.elapsed_secs() >= BULLET_SPAWN_INTERVAL {
        gun_timer.0.reset();
        commands.spawn((
            Sprite::from_atlas_image(
                image_handler.0.clone().unwrap(),
                TextureAtlas {
                    layout: texture_atlas.0.clone().unwrap(),
                    index: 15,
                },
            ),
            Transform::from_translation(Vec3::new(muzzle_position.x, muzzle_position.y, 1.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            Bullet,
            BulletDirection(*bullet_direction),
        ));
    }
}
