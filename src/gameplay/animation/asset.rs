use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use ron_asset_manager::prelude::*;
use serde::Deserialize;
use std::collections::HashSet;

pub fn plugin(app: &mut App) {
    app.add_plugins(RonAssetPlugin::<Animation>::default());
}

#[derive(Asset, RonAsset, TypePath, Deserialize, Debug, Clone)]
pub struct Animation {
    #[asset]
    pub sheet: Shandle<Image>,
    pub size: UVec2,
    #[serde(default)]
    pub padding: Option<UVec2>,
    #[serde(default)]
    pub offset: Option<UVec2>,
    pub rows: u32,
    pub columns: u32,
    #[asset]
    pub frames: HashMap<CharacterAnimationState, AnimationFrames>,
}

#[derive(Asset, RonAsset, TypePath, Deserialize, Debug, Clone)]
pub struct AnimationFrames {
    pub row: usize,
    pub count: usize,
    pub interval: u64,
    #[asset]
    pub steps: Option<AnimationSounds>,
}

#[derive(Asset, RonAsset, TypePath, Deserialize, Debug, Clone)]
pub struct AnimationSounds {
    pub frames: HashSet<usize>,
    #[asset]
    pub samples: Vec<Shandle<AudioSource>>,
}

#[derive(Deserialize, Debug, Clone, Reflect, Hash, PartialEq, Eq)]
pub enum CharacterAnimationState {
    Idle,
    Walk,
}
