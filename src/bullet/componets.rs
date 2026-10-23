use std::time::Instant;

use bevy::prelude::*;
#[derive(Component)]

pub struct Bullet;

#[derive(Component)]
pub struct BulletDirection(pub Vec3);
#[derive(Component)]

pub struct SpawnInstant(pub Instant);
