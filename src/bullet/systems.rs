use bevy::prelude::*;

use super::componets::*;
use crate::constants::*;

pub fn update_bullets(
    mut bullet_query: Query<(&mut Transform, &BulletDirection), With<Bullet>>,
    time: Res<Time>,
) {
    if bullet_query.is_empty() {
        return;
    }
    for (mut t, dir) in bullet_query.iter_mut() {
        t.translation += dir.0.normalize() * BULLET_SPEED * time.delta_secs();
        t.translation.z = 10.0;
    }
}

pub fn despawn_old_bullets(mut commands: Commands, bullet_query: Query<(&SpawnInstant, Entity)>) {
    for (instant, entity) in bullet_query.iter() {
        if instant.0.elapsed().as_secs_f32() > BULLET_TIME {
            commands.entity(entity).despawn();
        }
    }
}
