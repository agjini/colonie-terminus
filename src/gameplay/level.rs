use crate::asset_tracking::LoadResource;
use crate::{
    audio::music,
    gameplay::player::{PlayerAssets, player},
    screens::Screen,
};
use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use ron_asset_manager::Shandle;
use ron_asset_manager::prelude::RonAsset;
use serde::Deserialize;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>("level.ron");
    app.add_systems(First, prepare_tileset_texture);
}

#[derive(Resource, Asset, RonAsset, TypePath, Deserialize, Clone, Debug)]
pub struct LevelAssets {
    #[asset]
    pub music: Shandle<AudioSource>,

    #[asset]
    pub tileset: Shandle<Image>,
}

pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    player_assets: Res<PlayerAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        DespawnOnExit(Screen::Gameplay),
        children![
            tiles(&level_assets.tileset.handle),
            player(400.0, &player_assets, &mut texture_atlas_layouts),
            (
                Name::new("Gameplay Music"),
                music(level_assets.music.handle.clone())
            ),
            wall(
                &mut meshes,
                &mut materials,
                Vec2::new(0.0, -100.0),
                Vec2::new(500.0, 25.0)
            ),
            wall(
                &mut meshes,
                &mut materials,
                Vec2::new(0.0, 100.0),
                Vec2::new(500.0, 25.0)
            ),
            wall(
                &mut meshes,
                &mut materials,
                Vec2::new(-250.0, 0.0),
                Vec2::new(25.0, 200.0)
            ),
            wall(
                &mut meshes,
                &mut materials,
                Vec2::new(250.0, 0.0),
                Vec2::new(25.0, 200.0)
            ),
            wall(
                &mut meshes,
                &mut materials,
                Vec2::new(100.0, 0.0),
                Vec2::new(100.0, 25.0)
            ),
        ],
    ));
}

fn wall(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec2,
    size: Vec2,
) -> impl Bundle {
    (
        Name::new("Wall"),
        Mesh2d(meshes.add(Rectangle::new(size.x, size.y))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::WHITE))),
        Transform::from_translation(position.extend(0.0)),
        RigidBody::Static,
        Collider::rectangle(size.x, size.y),
    )
}

fn tiles(tileset: &Handle<Image>) -> impl Bundle {
    // We're seeding the PRNG here to make this example deterministic for testing purposes.
    // This isn't strictly required in practical use unless you need your app to be deterministic.
    let mut rng = ChaCha8Rng::seed_from_u64(42);

    let chunk_size = UVec2::splat(32);
    let tile_display_size = UVec2::splat(32);
    let tile_data: Vec<Option<TileData>> = (0..chunk_size.element_product())
        .map(|_| rng.random_range(0..5))
        .map(|i| {
            if i == 0 {
                None
            } else {
                Some(TileData::from_tileset_index(i - 1))
            }
        })
        .collect();

    (
        TilemapChunk {
            chunk_size,
            tile_display_size,
            tileset: tileset.clone(),
            ..default()
        },
        TilemapChunkTileData(tile_data),
    )
}

fn prepare_tileset_texture(
    level_assets: Option<Res<LevelAssets>>,
    mut images: ResMut<Assets<Image>>,
    mut converted: Local<bool>,
) {
    if *converted {
        return;
    }

    if let Some(level_assets) = level_assets {
        if let Some(image) = images.get_mut(&level_assets.tileset.handle) {
            image.reinterpret_stacked_2d_as_array(4);
            *converted = true;
        }
    }
}
