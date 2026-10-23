pub mod animation;
mod bullet;
mod constants;
mod enemies;
mod gun;
mod player;
mod resources;
mod states;
mod world;
use crate::enemies::EnemyPlugin;
use crate::states::*;
use crate::world::WorldPlugins;
use crate::{
    animation::AnimationPlugin, bullet::BulletPlugins, gun::GunPlugin, player::PlayerPlugins,
};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_pancam::{PanCam, PanCamPlugin};
use constants::*;
use player::component::Player;
use resources::*;
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: true,
                        focused: true,
                        resolution: (WW, WH).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        //Build plugins and External plugins
        .add_plugins((
            PlayerPlugins,
            GunPlugin,
            BulletPlugins,
            WorldPlugins,
            ResourePlugin,
            EnemyPlugin,
            AnimationPlugin,
            PanCamPlugin::default(),
        ))
        //states
        .init_state::<GameState>()
        //systems
        .add_systems(OnEnter(GameState::GameInit), setup_camera)
        .add_systems(OnEnter(GameState::Loading), load_assets)
        .add_systems(
            Update,
            (update_cursor_position, camera_follow_player).run_if(in_state(GameState::InGame)),
        )
        //resources
        .insert_resource(ClearColor(Color::srgb_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
        )))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Msaa::Off)).insert(PanCam {
        // Panning position limits (World coordinates)
        min_x: -WORLD_W,
        max_x: WORLD_H,
        min_y: -WORLD_H,
        max_y: WORLD_H,

        // Zoom scaling limits
        min_scale: 1.0,  // How far you can zoom IN (lower numbers = closer view)
        max_scale: 10.0, // How far you can zoom OUT (higher numbers = further view)

        // Other typical settings
        grab_buttons: vec![MouseButton::Right],
        ..default()
    });
}
pub fn load_assets(
    assets_server: Res<AssetServer>,
    mut texture_atlas: ResMut<GlobalTextureAtlasHandle>,
    mut image_handle: ResMut<GlobalSpriteSheetHandle>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    image_handle.0 = Some(assets_server.load(SPRITE_SHEET_PATH));
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(TILE_W, TILE_H),
        SPRITE_SHEET_W,
        SPRITE_SHEET_H,
        None,
        None,
    );
    texture_atlas.0 = Some(texture_atlas_layout.add(layout));
    next_state.set(GameState::GameInit);
}

fn update_cursor_position(
    mut cursor_pos: ResMut<CursorPos>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok(window) = window_query.single() else {
        cursor_pos.0 = None;
        return;
    };
    let Ok((camera, camera_transform)) = camera_query.single() else {
        cursor_pos.0 = None;
        return;
    };
    cursor_pos.0 = window.cursor_position().and_then(|cursor| {
        camera
            .viewport_to_world(camera_transform, cursor)
            .ok()
            .map(|ray| ray.origin.truncate())
    });
}

fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,

    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };
    let target = player_transform.translation.truncate();

    // Frame-rate-independent smoothing
    let smoothing = 8.0;
    let t = 1.0 - (-smoothing * time.delta_secs()).exp();

    let current = camera_transform.translation.truncate();

    let new_position = current.lerp(target, t);

    camera_transform.translation.x = new_position.x;
    camera_transform.translation.y = new_position.y;
}
