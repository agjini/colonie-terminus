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
pub struct Damage {
    pub damage: f32,
    pub cooldown: f32,
}

#[derive(Component, Reflect, Debug)]
pub struct DamageCooldown {
    pub timer: Timer,
}

#[derive(Resource, Asset, RonAsset, TypePath, Deserialize, Debug, Clone)]
pub struct EnemyAssets {
    #[asset]
    pub types: Vec<EnemyType>,
}

#[derive(RonAsset, Deserialize, Debug, Clone)]
pub struct EnemyType {
    pub name: String,
    pub max_speed: f32,
    pub max_health: f32,
    pub damage: f32,
    pub cooldown: f32,
    #[asset]
    pub sprite: Shandle<Image>,
}
