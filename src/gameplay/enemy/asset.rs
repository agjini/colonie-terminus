use crate::asset_tracking::LoadResource;
use crate::gameplay::animation::Animation;
use bevy::prelude::*;
use ron_asset_manager::Shandle;
use ron_asset_manager::prelude::RonAsset;
use serde::Deserialize;

pub fn plugin(app: &mut App) {
    app.load_resource::<EnemyAssets>("enemy.ron");
}

#[derive(Component, Reflect)]
pub struct Enemy {
    pub weight: u32,
}

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
    #[asset]
    pub hit_sounds: Vec<Shandle<AudioSource>>,
    pub rarity_step_secs: u32,
    pub spawn_interval_secs: f32,
    pub max_weight: u32,
    pub max_weight_per_sec: f32,
    pub max_weight_per_step: u32,
}

#[derive(RonAsset, Deserialize, Debug, Clone)]
pub struct EnemyType {
    pub name: String,
    pub rarity: u32,
    pub rarity_growth: u32,
    pub max_speed: f32,
    pub max_health: f32,
    pub weight: u32,
    pub damage: f32,
    pub cooldown: f32,
    #[asset]
    pub sprite: Shandle<Animation>,
}

impl EnemyType {
    pub fn rarity_at(&self, step: u32) -> u32 {
        self.rarity + self.rarity_growth * step
    }
}

impl EnemyAssets {
    pub fn max_weight_at(&self, elapsed: f32, step: u32) -> u32 {
        self.max_weight
            + (self.max_weight_per_sec * elapsed) as u32
            + self.max_weight_per_step * step
    }
}
