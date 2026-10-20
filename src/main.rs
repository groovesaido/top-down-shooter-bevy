use bevy::prelude::*;
const WW: u32 = 1200;
const WH: u32 = 700;
//sprite
const SPRITE_SHEET_PATH: &str = "assets.png";
const TILE_W: u32 = 16;
const TILE_H: u32 = 16;
const SPRITE_SHEET_W: u32 = 4;
const SPRITE_SHEET_H: u32 = 4;
const SPRITE_SCALE_FACTOR: f32 = 3.0;

//colors
const BG_COLOR: (u8, u8, u8) = (197, 204, 184);
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
        .add_systems(Startup, (setup_camera, spawn_player))
        .insert_resource(ClearColor(Color::srgb_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
        )))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Msaa::Off));
}

fn spawn_player(
    assets_server: Res<AssetServer>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
) {
    let texture: Handle<Image> = assets_server.load(SPRITE_SHEET_PATH);
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(TILE_W, TILE_H),
        SPRITE_SHEET_W,
        SPRITE_SHEET_H,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layout.add(layout);
    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        ),
        Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
    ));
}
