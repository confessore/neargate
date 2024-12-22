use bevy::prelude::*;

use crate::GameState;

pub struct Cell {
    pub height: f32,
}

impl Cell {
    pub fn load_cell_scene(asset_server: Res<AssetServer>) -> Handle<Scene> {
        let cell_scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset("tile.glb"));
        cell_scene
    }
}
