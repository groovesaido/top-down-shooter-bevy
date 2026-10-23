use bevy::prelude::*;
use rand::RngExt;

use crate::{
    animation::components::AnimationTimer, constants::*, enemies::components::Enemy,
    player::component::Player, resources::*,
};

pub fn spawn_enemies(
    mut commands: Commands,
    texture_atlas: ResMut<GlobalTextureAtlasHandle>,
    image_handler: ResMut<GlobalSpriteSheetHandle>,

    enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
) {
    let num_enemies = enemy_query.iter().len();
    let enemy_spawn_count = (MAX_OF_ENEMIES - num_enemies).min(10);
    if num_enemies > MAX_OF_ENEMIES {
        return;
    }
    let mut rng = rand::rng();
    let x = rng.random_range(-WORLD_W..WORLD_W);
    let y = rng.random_range(-WORLD_H..WORLD_H);

    for _ in 0..enemy_spawn_count {
        commands.spawn((
            Sprite::from_atlas_image(
                image_handler.0.clone().unwrap(),
                TextureAtlas {
                    layout: texture_atlas.0.clone().unwrap(),
                    index: 8,
                },
            ),
            Transform::from_translation(Vec3::new(x, y, 1.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            Enemy::default(),
            AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
        ));
    }
}

pub fn update_enemy_transform(
    mut enemy_query: Query<(&mut Transform, &mut Sprite), (With<Enemy>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let player_pos = player_transform.translation;

    for (mut transform, mut sprite) in enemy_query.iter_mut() {
        let dir = (player_pos - transform.translation).normalize();
        transform.translation += dir * ENEMY_SPEED * time.delta_secs();

        //flip sprite on x-axis
        if transform.translation.x < player_pos.x {
            sprite.flip_x = false;
        } else {
            sprite.flip_x = true;
        }
    }
}

pub fn despawn_dead_enemies(mut commands: Commands, enemy_query: Query<(&Enemy, Entity)>) {
    if enemy_query.is_empty() {
        return;
    }
    for (enemy, entity) in enemy_query.iter() {
        if enemy.health <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
