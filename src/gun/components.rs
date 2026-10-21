use bevy::{prelude::*, time::Stopwatch};
#[derive(Component)]
pub struct Gun;

#[derive(Component)]
pub struct GunTimer(pub Stopwatch);
