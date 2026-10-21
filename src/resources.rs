use crate::load_assets;
use crate::states::*;
use bevy::prelude::*;
#[derive(Resource)]
pub struct GlobalTextureAtlasHandle(pub Option<Handle<TextureAtlasLayout>>);

#[derive(Resource)]
pub struct GlobalSpriteSheetHandle(pub Option<Handle<Image>>);

#[derive(Resource)]
pub struct CursorPos(pub Option<Vec2>);

pub struct ResourePlugin;

impl Plugin for ResourePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GlobalTextureAtlasHandle(None))
            .insert_resource(GlobalSpriteSheetHandle(None))
            .insert_resource(CursorPos(None))
            .add_systems(OnEnter(GameState::Loading), load_assets);
    }
}
