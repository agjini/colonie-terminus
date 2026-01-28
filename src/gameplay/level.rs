use crate::asset_tracking::LoadResource;
use crate::gameplay::player::Player;
use crate::gameplay::tilemap::{TilesetAssets, tilemap};
use crate::{
    audio::music,
    gameplay::player::{PlayerAssets, player},
    screen::Screen,
};
use bevy::prelude::*;
use ron_asset_manager::Shandle;
use ron_asset_manager::prelude::RonAsset;
use serde::Deserialize;

const CAMERA_DECAY_RATE: f32 = 2.;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>("level.ron");
    app.add_systems(Update, update_camera);
}

#[derive(Resource, Asset, RonAsset, TypePath, Deserialize, Clone, Debug)]
pub struct LevelAssets {
    #[asset]
    pub music: Shandle<AudioSource>,
    pub seed: Option<u32>,
}

pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    tileset_assets: Res<TilesetAssets>,
    player_assets: Res<PlayerAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        DespawnOnExit(Screen::Gameplay),
        children![
            tilemap(level_assets.seed.unwrap_or(32), &tileset_assets),
            player(400.0, &player_assets, &mut texture_atlas_layouts),
            (
                Name::new("Gameplay Music"),
                music(level_assets.music.handle.clone())
            ),
        ],
    ));
}

fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);
    camera
        .translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
}
