use crate::asset_tracking::LoadResource;
use bevy::prelude::*;
use bevy_seedling::prelude::*;
use ron_asset_manager::prelude::*;
use serde::Deserialize;

pub fn plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>("player.ron");
}

#[derive(Resource, Asset, RonAsset, TypePath, Deserialize, Debug, Clone)]
pub struct PlayerAssets {
    pub name: String,
    pub max_speed: f32,
    #[asset]
    pub sprite: Shandle<Image>,
    #[asset]
    pub steps: Vec<Shandle<AudioSample>>,
}
