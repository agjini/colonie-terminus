use crate::asset_tracking::LoadResource;
use crate::gameplay::enemy::enemy_root;
use crate::gameplay::player::asset::PlayerAssets;
use crate::gameplay::player::weapon::reticle;
use crate::gameplay::tilemap::asset::TilesetAssets;
use crate::gameplay::tilemap::spawn_tilemap;
use crate::{audio::music, gameplay::player::player, screen::Screen};
use bevy::prelude::*;
use bevy_seedling::prelude::AudioSample;
use rand::prelude::StdRng;
use rand::rngs::ThreadRng;
use rand::{RngCore, SeedableRng};
use ron_asset_manager::Shandle;
use ron_asset_manager::prelude::RonAsset;
use serde::Deserialize;

#[derive(Component)]
pub struct TilemapOrigin;

#[derive(Component)]
pub struct WorldEntity;

#[derive(Resource)]
pub struct RandomSeed(pub(crate) StdRng);

pub fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>("level.ron");
    app.add_systems(OnEnter(Screen::Gameplay(false)), spawn_level);
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

fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    tileset_assets: Res<TilesetAssets>,
    player_assets: Res<PlayerAssets>,
    mut camera: Single<&mut Transform, With<Camera2d>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    info!("Loading level with seed: {}", level_assets.seed);
    camera.translation = Vec3::ZERO;

    let rng = StdRng::seed_from_u64(level_assets.seed as u64);

    commands.insert_resource(RandomSeed(rng));

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

            parent
                .spawn(player(&player_assets, &mut texture_atlas_layouts))
                .with_children(|player| {
                    player.spawn(reticle(&mut meshes, &mut materials, &mut images));
                });

            parent.spawn(enemy_root());

            spawn_tilemap(parent, &level_assets, &tileset_assets, &mut images);
        });
}
