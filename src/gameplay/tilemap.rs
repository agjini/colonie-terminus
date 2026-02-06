use crate::asset_tracking::LoadResource;
use bevy::math::U16Vec2;
use bevy::math::ops::{cos, sin};
use bevy::prelude::*;
use bevy::render::render_resource::Extent3d;
use bevy::render::render_resource::TextureDimension::D2;
use bevy::sprite_render::{TileData, TilemapChunk, TilemapChunkTileData};
use noise::{NoiseFn, Perlin};
use ron_asset_manager::{Shandle, prelude::RonAsset};
use serde::Deserialize;
use std::collections::HashMap;
use std::f32::consts::PI;

pub fn plugin(app: &mut App) {
    app.load_resource::<TilesetAssets>("tileset.ron")
        .add_systems(Update, prepare_tileset_texture);
}

#[derive(Deserialize, Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Ground {
    DirtRed,
    DirtBrown,
}

#[derive(Deserialize, Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GroundVariant {
    Rocks,
}

#[derive(Resource, Asset, RonAsset, TypePath, Deserialize, Clone, Debug)]
pub struct TilesetAssets {
    #[asset]
    pub texture: Shandle<Image>,
    pub tile_size: u32,
    pub chunk_size: u32,
    pub width: u32,
    pub grounds: HashMap<Ground, GroundTilesetWithVariant>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GroundTilesetWithVariant {
    pub tiles: Tileset,
    pub variants: HashMap<GroundVariant, Tileset>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Tileset {
    pub from: U16Vec2,
    pub to: U16Vec2,
}

#[derive(Component)]
pub struct ChunkPlanetPos(pub IVec2);

#[derive(Clone, Debug)]
pub struct RandomContext {
    pub perlin: Perlin,
    pub seed: u32,
    planet_width: f32,
    planet_height: f32,
}

impl RandomContext {
    pub fn new(seed: u32, planet_width: f32, planet_height: f32) -> Self {
        Self {
            perlin: Perlin::new(seed),
            seed,
            planet_width,
            planet_height,
        }
    }

    fn wrapping_noise(&self, pos: IVec2, scale: f64, offset: f64) -> f64 {
        let x = pos.x as f32;
        let y = pos.y as f32;

        let nx = cos(2.0 * PI * x / self.planet_width) as f64 * scale;
        let ny = sin(2.0 * PI * x / self.planet_width) as f64 * scale;
        let nz = cos(2.0 * PI * y / self.planet_height) as f64 * scale;
        let nw = sin(2.0 * PI * y / self.planet_height) as f64 * scale;

        self.perlin
            .get([nx + offset, ny + offset, nz + offset, nw + offset])
    }

    pub fn ground_type(&self, pos: IVec2) -> Ground {
        if self.wrapping_noise(pos, 1.0, 0.0) > 0.0 {
            Ground::DirtRed
        } else {
            Ground::DirtBrown
        }
    }

    pub fn has_variant(&self, pos: IVec2) -> bool {
        self.wrapping_noise(pos, 80.0, 500.0) > 0.4
    }

    pub fn tile_variant(&self, pos: IVec2, from: U16Vec2, to: U16Vec2) -> U16Vec2 {
        let h = hash(self.seed, pos.x, pos.y);
        let range_x = (to.x - from.x + 1) as u32;
        let range_y = (to.y - from.y + 1) as u32;
        let x = from.x + (h % range_x) as u16;
        let y = from.y + ((h / range_x) % range_y) as u16;
        U16Vec2::new(x, y)
    }
}

fn hash(seed: u32, x: i32, y: i32) -> u32 {
    let mut h = seed;
    h = h.wrapping_mul(0x9E3779B9).wrapping_add(x as u32);
    h ^= h >> 16;
    h = h.wrapping_mul(0x85EBCA6B).wrapping_add(y as u32);
    h ^= h >> 13;
    h = h.wrapping_mul(0xC2B2AE35);
    h ^= h >> 16;
    h
}

impl TilesetAssets {
    pub fn get_tile(&self, ctx: &RandomContext, pos: IVec2) -> Option<TileData> {
        let ground = ctx.ground_type(pos);
        let has_rocks = ctx.has_variant(pos);

        self.grounds
            .get(&ground)
            .map(|g| {
                if has_rocks {
                    g.variants.get(&GroundVariant::Rocks).unwrap_or(&g.tiles)
                } else {
                    &g.tiles
                }
            })
            .map(|tileset| {
                let tile = ctx.tile_variant(pos, tileset.from, tileset.to);
                let row = (self.width / self.tile_size) as u16;
                TileData::from_tileset_index(tile.x + tile.y * row)
            })
    }
}

pub fn chunk_pixel_size(tileset_assets: &TilesetAssets) -> f32 {
    tileset_assets.chunk_size as f32 * tileset_assets.tile_size as f32
}

pub fn chunk_tile_data(
    seed: u32,
    planet_size: UVec2,
    chunk_origin: IVec2,
    tileset_assets: &TilesetAssets,
) -> TilemapChunkTileData {
    let chunk_size = tileset_assets.chunk_size;
    let ctx = RandomContext::new(seed + 33, planet_size.x as f32, planet_size.y as f32);

    let tile_data: Vec<Option<TileData>> = (0..chunk_size)
        .flat_map(|x| (0..chunk_size).map(move |y| IVec2::new(x as i32, y as i32)))
        .map(|local| {
            let planet_pos = IVec2::new(
                (chunk_origin.x + local.x).rem_euclid(planet_size.x as i32),
                (chunk_origin.y + local.y).rem_euclid(planet_size.y as i32),
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

fn prepare_tileset_texture(
    tileset_assets: Option<Res<TilesetAssets>>,
    mut events: MessageReader<AssetEvent<Image>>,
    mut images: ResMut<Assets<Image>>,
) {
    let Some(tileset_assets) = tileset_assets else {
        return;
    };

    for event in events.read() {
        if event.is_loaded_with_dependencies(tileset_assets.texture.handle.id()) {
            let source_image = images.get(&tileset_assets.texture.handle).unwrap();
            let tile_size = tileset_assets.tile_size;

            let tiles_x = source_image.width() / tile_size;
            let tiles_y = source_image.height() / tile_size;

            let total_tiles = tiles_x * tiles_y;

            let source_data = source_image.data.as_ref().unwrap();
            let mut tile_data = Vec::new();

            let mut extracted = 0;
            'outer: for tile_y in 0..tiles_y {
                for tile_x in 0..tiles_x {
                    if extracted >= total_tiles {
                        break 'outer;
                    }

                    for pixel_y in 0..tile_size {
                        for pixel_x in 0..tile_size {
                            let src_x = tile_x * tile_size + pixel_x;
                            let src_y = tile_y * tile_size + pixel_y;
                            let pixel_index = ((src_y * source_image.width() + src_x) * 4) as usize;

                            tile_data.push(source_data[pixel_index]);
                            tile_data.push(source_data[pixel_index + 1]);
                            tile_data.push(source_data[pixel_index + 2]);
                            tile_data.push(source_data[pixel_index + 3]);
                        }
                    }

                    extracted += 1;
                }
            }

            let array_image = Image::new_fill(
                Extent3d {
                    width: tile_size,
                    height: tile_size,
                    depth_or_array_layers: total_tiles,
                },
                D2,
                &tile_data,
                source_image.texture_descriptor.format,
                source_image.asset_usage,
            );

            *images.get_mut(&tileset_assets.texture.handle).unwrap() = array_image;
        }
    }
}
