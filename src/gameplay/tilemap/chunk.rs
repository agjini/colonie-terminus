use crate::gameplay::tilemap::TilesetAssets;
use crate::gameplay::tilemap::generator::TilemapGenerator;
use bevy::prelude::*;
use bevy::sprite_render::{TileData, TilemapChunk, TilemapChunkTileData};
use itertools::Itertools;

pub fn chunk_pixel_size(tileset_assets: &TilesetAssets) -> f32 {
    tileset_assets.chunk_size as f32 * tileset_assets.tile_size as f32
}

pub fn chunk_tile_data(
    seed: u32,
    planet_size: UVec2,
    chunk_origin: IVec2,
    tileset_assets: &TilesetAssets,
) -> TilemapChunkTileData {
    let chunk_size = tileset_assets.chunk_size as i32;
    let ctx = TilemapGenerator::new(seed + 33, planet_size.x as f32, planet_size.y as f32);

    let tile_data: Vec<Option<TileData>> = (0..chunk_size)
        .cartesian_product(0..chunk_size)
        .map(|(y, x)| {
            let planet_pos = IVec2::new(
                (chunk_origin.x + x).rem_euclid(planet_size.x as i32),
                (chunk_origin.y + y).rem_euclid(planet_size.y as i32),
            );
            tileset_assets.get_tile(&ctx, planet_pos)
        })
        .collect();

    TilemapChunkTileData(tile_data)
}

pub fn tilemap_chunk(tileset_assets: &TilesetAssets) -> TilemapChunk {
    let chunk_size = UVec2::splat(tileset_assets.chunk_size);
    let tile_display_size = UVec2::splat(tileset_assets.tile_size);

    TilemapChunk {
        chunk_size,
        tile_display_size,
        tileset: tileset_assets.texture.handle.clone(),
        ..default()
    }
}
