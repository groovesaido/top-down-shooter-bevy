use std::f32::consts::PI;

use bevy::{prelude::*, time::Stopwatch, window::PrimaryWindow};
use bevy_pancam::{PanCam, PanCamPlugin};
use rand::RngExt;
const WW: u32 = 1200;
const WH: u32 = 700;
//sprite
const SPRITE_SHEET_PATH: &str = "assets.png";
const TILE_W: u32 = 16;
const TILE_H: u32 = 16;
const SPRITE_SHEET_W: u32 = 4;
const SPRITE_SHEET_H: u32 = 4;
const SPRITE_SCALE_FACTOR: f32 = 3.0;
//bullet
const BULLET_SPAWN_INTERVAL: f32 = 0.1;
const BULLET_SPEED: f32 = 300.0;
// WORLD
const NUM_WORLD_DECORS: u32 = 1000;
const WORLD_W: f32 = 3000.0;
const WORLD_H: f32 = 2500.0;
//colors
const BG_COLOR: (u8, u8, u8) = (197, 204, 184);
//player
const PLAYER_SPEED: f32 = 200.0;
//Game State
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum GameState {
    #[default]
    Loading,
    GameInit,
    InGame,
}
//Resources
#[derive(Resource)]
struct GlobalTextureAtlasHandle(Option<Handle<TextureAtlasLayout>>);

#[derive(Resource)]
struct GlobalSpriteSheetHandle(Option<Handle<Image>>);

#[derive(Resource)]
struct CursorPos(Option<Vec2>);
//componets
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Gun;
#[derive(Component)]
struct Bullet;
#[derive(Component)]
struct GunTimer(Stopwatch);
#[derive(Component)]
struct BulletDirection(Vec3);
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
        //state
        .init_state::<GameState>()
        //External plugin
        .add_plugins(PanCamPlugin::default())
        //system
        .add_systems(
            OnEnter(GameState::GameInit),
            (setup_camera, init_world, spawn_world_decorations),
        )
        .add_systems(OnEnter(GameState::Loading), load_assets)
        .add_systems(
            Update,
            (
                handle_player_input,
                update_gun_transform,
                update_cursor_position,
                handle_gun_input,
                update_bullets,
                camera_follow_player,
            )
                .run_if(in_state(GameState::InGame)),
        )
        //custom resources
        .insert_resource(GlobalTextureAtlasHandle(None))
        .insert_resource(GlobalSpriteSheetHandle(None))
        .insert_resource(CursorPos(None))
        //resources
        .insert_resource(ClearColor(Color::srgb_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
        )))
        .run();
}

fn load_assets(
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
        grab_buttons: vec![MouseButton::Left],
        enabled: true,
        zoom_to_cursor: true,
        ..default()
    });
}

fn init_world(
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

fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let Ok(mut transform) = player_query.single_mut() else {
        return;
    };
    let w_key = keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp);
    let a_key = keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft);
    let s_key = keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown);
    let d_key =
        keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight);
    let mut delta = Vec2::ZERO;
    if w_key {
        delta.y += 1.0;
    }
    if a_key {
        delta.x -= 1.0;
    }
    if s_key {
        delta.y -= 1.0;
    }
    if d_key {
        delta.x += 1.0;
    }
    //normalize vector
    if delta.length() > 0.0 {
        delta = delta.normalize();
    }
    if delta.is_finite() && (w_key || s_key || a_key || d_key) {
        transform.translation +=
            Vec3::new(delta.x, delta.y, 0.0) * PLAYER_SPEED * time.delta_secs();
        transform.translation.z = 10.0;
    }
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
fn update_gun_transform(
    player_query: Query<&Transform, With<Player>>,
    mut gun_query: Query<&mut Transform, (With<Gun>, Without<Player>)>,
    cursor_pos: Res<CursorPos>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let Ok(mut gun_transform) = gun_query.single_mut() else {
        return;
    };
    let player_pos = player_transform.translation.truncate();
    let cursor_pos = match cursor_pos.0 {
        Some(pos) => pos,
        None => player_pos,
    };
    //gun rotate to player
    let angle = (player_pos.y - cursor_pos.y).atan2(player_pos.x - cursor_pos.x) + PI;
    gun_transform.rotation = Quat::from_rotation_z(angle);

    let offset = 20.0;

    gun_transform.translation = Vec3::new(
        player_pos.x + offset * angle.cos() + 10.0,
        player_pos.y + offset * angle.sin() - 5.0,
        gun_transform.translation.z,
    );
}

fn handle_gun_input(
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
    let gun_pos = gun_transform.translation.truncate();
    gun_timer.0.tick(time.delta());
    if !mouse_btn_input.pressed(MouseButton::Left) {
        return;
    }
    let bullet_direction = gun_transform.local_x();

    if gun_timer.0.elapsed_secs() >= BULLET_SPAWN_INTERVAL {
        gun_timer.0.reset();
        commands.spawn((
            Sprite::from_atlas_image(
                image_handler.0.clone().unwrap(),
                TextureAtlas {
                    layout: texture_atlas.0.clone().unwrap(),
                    index: 3,
                },
            ),
            Transform::from_translation(Vec3::new(gun_pos.x, gun_pos.y, 1.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            Bullet,
            BulletDirection(*bullet_direction),
        ));
    }
}

fn update_bullets(
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

fn spawn_world_decorations(
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

fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };
    let (x, y) = (
        player_transform.translation.x,
        player_transform.translation.y,
    );
    camera_transform.translation = camera_transform.translation.lerp(Vec3::new(x, y, 0.0), 0.1);
}
