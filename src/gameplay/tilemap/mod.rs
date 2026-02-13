use crate::gameplay::layer::GameLayer;
use crate::gameplay::level::{LevelAssets, TilemapOrigin};
use crate::gameplay::tilemap::asset::{TilesetAssets, convert_tileset_to_array};
use crate::gameplay::tilemap::chunk::{chunk_pixel_size, chunk_tile_data, tilemap_chunk};
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;
use itertools::Itertools;

pub mod asset;
mod chunk;
mod generator;
mod scroll;

pub fn plugin(app: &mut App) {
    app.add_plugins((asset::plugin, scroll::plugin));
}

#[derive(Component)]
pub struct ChunkPlanetPos(pub IVec2);

pub fn spawn_tilemap(
    commands: &mut RelatedSpawnerCommands<ChildOf>,
    level_assets: &LevelAssets,
    tileset_assets: &TilesetAssets,
    images: &mut Assets<Image>,
) {
    convert_tileset_to_array(tileset_assets, images);

    let planet_size = UVec2::new(level_assets.planet_width, level_assets.planet_height);
    let chunk = tilemap_chunk(tileset_assets);
    let chunk_px = chunk_pixel_size(tileset_assets);
    let cs = tileset_assets.chunk_size as i32;

    let chunks = (-1..=1)
        .cartesian_product(-1..=1)
        .map(|(ox, oy)| {
            let planet_pos = IVec2::new(ox * cs, oy * cs);
            let tile_data =
                chunk_tile_data(level_assets.seed, planet_size, planet_pos, tileset_assets);

            (
                Transform::from_xyz(ox as f32 * chunk_px, oy as f32 * chunk_px, 0.),
                chunk.clone(),
                tile_data,
                ChunkPlanetPos(planet_pos),
            )
        })
        .collect_vec();

    commands
        .spawn((
            Name::new("Tilemap"),
            TilemapOrigin,
            Transform::default(),
            Visibility::default(),
            GameLayer::Ground,
        ))
        .with_children(|p| {
            for chunk in chunks {
                p.spawn(chunk);
            }
        });
}
