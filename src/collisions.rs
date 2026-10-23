use bevy::prelude::*;

use crate::{
    bullet::componets::Bullet, constants::BULLET_DAMAGE, enemies::components::Enemy,
    states::GameState,
};
pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_enemy_bullet_collisions).run_if(in_state(GameState::InGame)),
        );
    }
}

fn handle_enemy_bullet_collisions(
    bullet_query: Query<&Transform, With<Bullet>>,
    mut enemy_query: Query<(&Transform, &mut Enemy), With<Enemy>>,
) {
    for bul_transform in bullet_query.iter() {
        for (ene_transform, mut enemy) in enemy_query.iter_mut() {
            if bul_transform
                .translation
                .distance_squared(ene_transform.translation)
                <= 500.0
            {
                enemy.health -= BULLET_DAMAGE;
            }
        }
    }
}
