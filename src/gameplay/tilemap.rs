use crate::asset_tracking::LoadResource;
use bevy::math::U16Vec2;
use bevy::math::ops::{cos, sin};
use bevy::prelude::*;
use bevy::render::render_resource::Extent3d;
use bevy::render::render_resource::TextureDimension::D2;
use bevy::sprite_render::{TileData, TilemapChunk, TilemapChunkTileData};
use noise::{NoiseFn, Perlin};
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use rand_chacha::rand_core::SeedableRng;
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
    pub tile_size: u16,
    pub width: u16,
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

#[derive(Clone, Debug)]
pub struct RandomContext {
    pub perlin: Perlin,
    pub rng: ChaCha8Rng,
    width: f32,
    height: f32,
}

impl RandomContext {
    pub fn new(seed: u32, width: f32, height: f32) -> Self {
        Self {
            perlin: Perlin::new(seed),
            rng: ChaCha8Rng::seed_from_u64(seed as u64),
            width,
            height,
        }
    }

    fn noise(&self, pos: U16Vec2) -> f64 {
        let x = pos.x as f32;
        let y = pos.y as f32;

        let nx = cos(2.0 * PI * x / self.width) as f64;
        let ny = sin(2.0 * PI * x / self.width) as f64;
        let nz = cos(2.0 * PI * y / self.height) as f64;
        let nw = sin(2.0 * PI * y / self.height) as f64;

        self.perlin.get([nx, ny, nz, nw])
    }

    pub fn ground_type(&self, pos: U16Vec2) -> Ground {
        let noise_value = self.noise(pos);
        if noise_value > 0.0 {
            Ground::DirtRed
        } else {
            Ground::DirtBrown
        }
    }

    pub fn has_variant(&self, pos: U16Vec2) -> bool {
        let noise_value = self
            .perlin
            .get([pos.x as f64 * 0.3 + 500.0, pos.y as f64 * 0.3]);
        noise_value > 0.4
    }

    pub fn lerp(&mut self, from: U16Vec2, to: U16Vec2) -> U16Vec2 {
        let x = self.rng.random_range(from.x..=to.x);
        let y = self.rng.random_range(from.y..=to.y);

        U16Vec2::new(x, y)
    }
}

impl TilesetAssets {
    pub fn get_tile(&self, ctx: &mut RandomContext, pos: U16Vec2) -> Option<TileData> {
        let ground = ctx.ground_type(pos);
        let has_rocks = ctx.has_variant(pos);

        self.grounds.get(&ground).map(|g| {
            let tileset = if has_rocks {
                g.variants.get(&GroundVariant::Rocks).unwrap_or(&g.tiles)
            } else {
                &g.tiles
            };

            let tile = ctx.lerp(tileset.from, tileset.to);
            let row = self.width / self.tile_size;
            TileData::from_tileset_index(tile.x + tile.y * row)
        })
    }
}

pub const CHUNK_SIZE: u32 = 100;

pub fn world_size(tileset_assets: &TilesetAssets) -> f32 {
    CHUNK_SIZE as f32 * tileset_assets.tile_size as f32
}

pub struct TilemapData {
    pub chunk: TilemapChunk,
    pub tile_data: TilemapChunkTileData,
    pub world_size: f32,
}

pub fn tilemap_data(
    seed: u32,
    width: f32,
    height: f32,
    tileset_assets: &TilesetAssets,
) -> TilemapData {
    let chunk_size = UVec2::splat(CHUNK_SIZE);
    let tile_display_size = UVec2::splat(tileset_assets.tile_size as u32);

    let mut random_context = RandomContext::new(seed + 33, width, height);

    let tile_data: Vec<Option<TileData>> = (0..chunk_size.x)
        .flat_map(|x| (0..chunk_size.y).map(move |y| U16Vec2::new(x as u16, y as u16)))
        .map(|pos| tileset_assets.get_tile(&mut random_context, pos))
        .collect();

    TilemapData {
        chunk: TilemapChunk {
            chunk_size,
            tile_display_size,
            tileset: tileset_assets.texture.handle.clone(),
            ..default()
        },
        tile_data: TilemapChunkTileData(tile_data),
        world_size: world_size(tileset_assets),
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
            let tile_size = tileset_assets.tile_size as u32;

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
