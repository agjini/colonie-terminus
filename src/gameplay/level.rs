use crate::asset_tracking::LoadResource;
use crate::gameplay::enemy::{EnemyAssets, enemy};
use crate::gameplay::tilemap::asset::TilesetAssets;
use crate::gameplay::tilemap::spawn_tilemap;
use crate::{
    audio::music,
    gameplay::player::{PlayerAssets, player},
    screen::Screen,
};
use bevy::prelude::*;
use bevy_seedling::prelude::AudioSample;
use rand::Rng;
use rand::rngs::ThreadRng;
use ron_asset_manager::Shandle;
use ron_asset_manager::prelude::RonAsset;
use serde::Deserialize;

#[derive(Component)]
pub struct TilemapOrigin;

#[derive(Component)]
pub struct WorldEntity;

pub fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>("level.ron");
}

#[derive(Resource, Asset, RonAsset, TypePath, Deserialize, Clone, Debug)]
pub struct LevelAssets {
    #[asset]
    pub music: Shandle<AudioSample>,
    #[serde(default = "random_seed")]
    pub seed: u32,
    pub planet_width: u32,
    pub planet_height: u32,
}

fn random_seed() -> u32 {
    ThreadRng::default().next_u32()
}

pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    tileset_assets: Res<TilesetAssets>,
    player_assets: Res<PlayerAssets>,
    enemy_assets: Res<EnemyAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    info!("Loading level with seed: {}", level_assets.seed);

    commands
        .spawn((
            Name::new("Level"),
            Transform::default(),
            Visibility::default(),
            DespawnOnExit(Screen::Gameplay(false)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Gameplay Music"),
                music(level_assets.music.handle.clone()),
            ));

            parent.spawn(player(&player_assets, &mut texture_atlas_layouts));
            parent.spawn(enemy(&enemy_assets, &mut texture_atlas_layouts));

            spawn_tilemap(parent, &level_assets, &tileset_assets);
        });
}
