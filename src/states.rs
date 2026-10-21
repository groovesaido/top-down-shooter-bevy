use bevy::prelude::*;

//Game State
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    GameInit,
    InGame,
}
