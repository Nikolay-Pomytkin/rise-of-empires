//! Sprite asset loading for 2D rendering

use bevy::prelude::*;

/// Loaded sprite assets
#[derive(Resource, Default)]
pub struct SpriteAssets {
    // Buildings
    pub town_center: Option<Handle<Image>>,
    pub barracks: Option<Handle<Image>>,

    // Units
    pub villager: Option<Handle<Image>>,
    pub soldier: Option<Handle<Image>>,

    // Resources
    pub tree: Option<Handle<Image>>,
    pub gold_mine: Option<Handle<Image>>,
    pub stone_quarry: Option<Handle<Image>>,
    pub berry_bush: Option<Handle<Image>>,
}

/// Load sprite assets from disk
/// Note: For WASM, we skip sprite loading to avoid .meta file issues
/// and use colored fallback sprites instead
pub fn load_sprite_assets(mut commands: Commands, _asset_server: Res<AssetServer>) {
    // For now, use colored fallback sprites instead of loading images
    // This avoids Bevy's asset meta file requirements which cause issues in WASM
    // TODO: Set up proper asset processing pipeline for WASM builds
    let assets = SpriteAssets::default();
    commands.insert_resource(assets);
}
