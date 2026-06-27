use crate::asset_tracking::LoadResource;
use crate::gameplay::animation::Animation;
use bevy::prelude::*;
use ron_asset_manager::prelude::*;
use serde::Deserialize;

pub fn plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>("player.ron");
}

#[derive(Resource, Asset, RonAsset, TypePath, Deserialize, Debug, Clone)]
pub struct PlayerAssets {
    pub name: String,
    pub max_speed: f32,
    pub max_health: f32,
    pub auto_aim_angle: f32,
    pub fire_origin: Vec2,
    #[asset]
    pub sprite: Shandle<Animation>,
    #[asset]
    pub pickup_xp: Shandle<AudioSource>,
}
