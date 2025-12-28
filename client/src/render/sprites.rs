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
pub fn load_sprite_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let assets = SpriteAssets {
        // Buildings
        town_center: Some(asset_server.load("sprites/buildings/town_center.png")),
        barracks: Some(asset_server.load("sprites/buildings/barracks.png")),

        // Units
        villager: Some(asset_server.load("sprites/units/villager.png")),
        soldier: Some(asset_server.load("sprites/units/soldier.png")),

        // Resources
        tree: Some(asset_server.load("sprites/resources/tree.png")),
        gold_mine: Some(asset_server.load("sprites/resources/gold_mine.png")),
        stone_quarry: Some(asset_server.load("sprites/resources/stone_quarry.png")),
        berry_bush: Some(asset_server.load("sprites/resources/berry_bush.png")),
    };

    commands.insert_resource(assets);
}
