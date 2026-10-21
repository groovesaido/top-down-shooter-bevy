use crate::constants::*;
use crate::gun::components::*;
use crate::player::component::Player;
use crate::resources::*;
use crate::states::GameState;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use rand::RngExt;

pub fn spawn_world_decorations(
    mut commands: Commands,
    texture_atlas: Res<GlobalTextureAtlasHandle>,
    image_handler: Res<GlobalSpriteSheetHandle>,
) {
    let mut rng = rand::rng();
    for _ in 0..NUM_WORLD_DECORS {
        let x = rng.random_range(-WORLD_W..WORLD_W);
        let y = rng.random_range(-WORLD_H..WORLD_H);

        commands.spawn((
            Sprite::from_atlas_image(
                image_handler.0.clone().unwrap(),
                TextureAtlas {
                    layout: texture_atlas.0.clone().unwrap(),
                    index: rng.random_range(4..=5),
                },
            ),
            Transform::from_translation(Vec3::new(x, y, 0.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
        ));
    }
}

pub fn init_world(
    mut commands: Commands,
    texture_atlas: Res<GlobalTextureAtlasHandle>,
    image_handler: Res<GlobalSpriteSheetHandle>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    //player
    commands.spawn((
        Sprite::from_atlas_image(
            image_handler.0.clone().unwrap(),
            TextureAtlas {
                layout: texture_atlas.0.clone().unwrap(),
                index: 0,
            },
        ),
        Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR))
            .with_translation(Vec3::new(0.0, 0.0, 1.0)),
        Player,
    ));
    //gun
    commands.spawn((
        Sprite::from_atlas_image(
            image_handler.0.clone().unwrap(),
            TextureAtlas {
                layout: texture_atlas.0.clone().unwrap(),
                index: 2,
            },
        ),
        Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR))
            .with_translation(Vec3::new(0.0, 0.0, 2.0)),
        Gun,
        GunTimer(Stopwatch::new()),
    ));
    next_state.set(GameState::InGame);
}
