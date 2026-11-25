use crate::asset_tracking::LoadResource;
use bevy::prelude::*;
use ron_asset_manager::prelude::RonAsset;
use serde::Deserialize;

pub fn plugin(app: &mut App) {
    app.load_resource::<AffixAssets>("affix.ron");
}

#[derive(Component, Reflect)]
pub struct Affixes(pub Vec<AffixAsset>);

#[derive(Resource, Asset, RonAsset, TypePath, Deserialize, Debug, Clone)]
pub struct AffixAssets(Vec<AffixAsset>);

#[derive(Deserialize, Debug, Clone, Reflect)]
pub struct AffixAsset {
    pub name: String,
    pub weight_multiplier: f32,
    pub modifiers: Vec<Modifier>,
}

#[derive(Deserialize, Debug, Clone, Reflect)]
pub enum Modifier {
    Scale(f32),
    Health(f32),
    Damage(f32),
    Speed(f32),
}
