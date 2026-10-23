use bevy::prelude::*;

#[derive(Component)]
pub struct Player;
#[derive(Component, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    Moving,
}
