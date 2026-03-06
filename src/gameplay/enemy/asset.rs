use crate::asset_tracking::LoadResource;
use bevy::prelude::*;
use ron_asset_manager::Shandle;
use ron_asset_manager::prelude::RonAsset;
use serde::Deserialize;

pub fn plugin(app: &mut App) {
    app.load_resource::<EnemyAssets>("enemy.ron");
}

#[derive(Component, Reflect)]
pub struct Enemy;

#[derive(Component, Reflect, Debug)]
pub struct Damage(pub f32);

#[derive(Resource, Asset, RonAsset, TypePath, Deserialize, Debug, Clone)]
pub struct EnemyAssets {
    #[asset]
    pub types: Vec<EnemyType>,
}

#[derive(RonAsset, Deserialize, Debug, Clone)]
pub struct EnemyType {
    pub name: String,
    pub max_speed: f32,
    pub dps: f32,
    #[asset]
    pub sprite: Shandle<Image>,
}
