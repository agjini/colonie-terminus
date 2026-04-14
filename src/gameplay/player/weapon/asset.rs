use crate::asset_tracking::LoadResource;
use bevy::asset::LoadContext;
use bevy::prelude::*;
use bevy_seedling::prelude::AudioSample;
use ron_asset_manager::Shandle;
use ron_asset_manager::prelude::RonAsset;
use serde::Deserialize;

pub fn plugin(app: &mut App) {
    app.load_resource::<WeaponAssets>("weapon.ron");
}

#[derive(Resource, Asset, TypePath, Deserialize, Debug, Clone)]
pub struct WeaponAssets {
    pub types: Vec<WeaponType>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct WeaponType {
    pub name: String,
    pub trigger_sounds: Vec<Shandle<AudioSample>>,
    pub levels: Vec<WeaponLevel>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct WeaponLevel {
    pub damage: f32,
    pub attack: WeaponAttack,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub enum WeaponAttack {
    #[default]
    None,
    Projectile {
        sprite: Shandle<Image>,
        speed: f32,
        fire_rate: f32,
        lifetime: f32,
        trajectories: Vec<Trajectory>,
        on_hit: Option<HitEffect>,
    },
}

#[derive(Deserialize, Debug, Clone, Default)]
pub enum HitEffect {
    #[default]
    Explode,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub enum Trajectory {
    #[default]
    Straight,
    Spread {
        angle: f32,
    },
    Homing {
        turn_rate: f32,
    },
}

impl RonAsset for WeaponAssets {
    fn load_assets(&mut self, context: &mut LoadContext) {
        self.types.load_assets(context);
    }
}

impl RonAsset for WeaponType {
    fn load_assets(&mut self, context: &mut LoadContext) {
        self.trigger_sounds.load_assets(context);
        self.levels.load_assets(context);
    }
}

impl RonAsset for WeaponLevel {
    fn load_assets(&mut self, context: &mut LoadContext) {
        self.attack.load_assets(context);
    }
}

impl RonAsset for WeaponAttack {
    fn load_assets(&mut self, context: &mut LoadContext) {
        match self {
            WeaponAttack::Projectile { sprite, .. } => sprite.load_assets(context),
            _ => {}
        }
    }
}
