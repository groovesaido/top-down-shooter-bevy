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
        //system
        .add_systems(OnEnter(GameState::GameInit), (setup_camera, init_world))
        .add_systems(OnEnter(GameState::Loading), load_assets)
        //custom resources
        .insert_resource(GlobalTextureAtlasHandle(None))
        .insert_resource(GlobalSpriteSheetHandle(None))
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
    commands.spawn((Camera2d, Msaa::Off));
}

fn init_world(
    mut commands: Commands,
    texture_atlas: Res<GlobalTextureAtlasHandle>,
    image_handler: Res<GlobalSpriteSheetHandle>,
) {
    commands.spawn((
        Sprite::from_atlas_image(
            image_handler.0.clone().unwrap(),
            TextureAtlas {
                layout: texture_atlas.0.clone().unwrap(),
                index: 0,
            },
        ),
        Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
    ));
}
