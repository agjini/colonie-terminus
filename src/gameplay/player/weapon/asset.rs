#![allow(dead_code)]

use crate::asset_tracking::LoadResource;
use bevy::prelude::*;
use ron_asset_manager::Shandle;
use ron_asset_manager::prelude::RonAsset;
use serde::Deserialize;

pub fn plugin(app: &mut App) {
    app.load_resource::<WeaponAssets>("weapon.ron");
}

#[derive(Resource, Asset, RonAsset, TypePath, Deserialize, Debug, Clone)]
pub struct WeaponAssets {
    #[asset]
    pub types: Vec<WeaponType>,
}

#[derive(Deserialize, RonAsset, TypePath, Debug, Clone, Default)]
pub struct WeaponType {
    pub name: String,
    pub trigger_sounds: Vec<Shandle<AudioSource>>,
    pub stats: WeaponStats,
    #[asset]
    pub bullet: Shandle<Image>,
}

#[derive(Deserialize, Debug, Copy, Clone, Default, Reflect)]
pub struct WeaponStats {
    pub damage: f32,
    pub speed: f32,
    pub fire_rate: f32,
    pub lifetime: f32,
}

impl WeaponStats {
    pub fn upgrade(&self, upgrade: WeaponStats) -> Self {
        Self {
            damage: self.damage + (self.damage * upgrade.damage),
            speed: self.speed + (self.speed * upgrade.speed),
            fire_rate: self.fire_rate - (self.fire_rate * upgrade.fire_rate),
            lifetime: self.lifetime + (self.lifetime * upgrade.lifetime),
        }
    }
}
