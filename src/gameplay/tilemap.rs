use crate::asset_tracking::LoadResource;
use bevy::math::{DVec2, U16Vec2};
use bevy::prelude::*;
use bevy::render::render_resource::Extent3d;
use bevy::render::render_resource::TextureDimension::D2;
use bevy::sprite_render::{TileData, TilemapChunk, TilemapChunkTileData};
use noise::{NoiseFn, Perlin};
use ron_asset_manager::{Shandle, prelude::RonAsset};
use serde::Deserialize;
use std::collections::HashMap;

pub(super) fn plugin(app: &mut App) {
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

#[derive(Resource, Clone, Debug)]
pub struct RandomContext {
    pub seed: u32,
    pub scale: f64,
    pub perlin: Perlin,
}

impl RandomContext {
    pub fn new(seed: u32) -> Self {
        Self {
            seed,
            scale: 0.8,
            perlin: Perlin::new(seed),
        }
    }

    pub fn either<T>(&self, pos: U16Vec2, left: T, right: T) -> T {
        let val = self.noise(pos);
        if val.x > 0.5 { left } else { right }
    }

    fn noise(&self, pos: U16Vec2) -> DVec2 {
        let nx = pos.x as f64 * self.scale;
        let ny = pos.y as f64 * self.scale;

        let noise_x = self.perlin.get([nx, ny]); // [-1.0, 1.0]
        let noise_y = self.perlin.get([nx + 1000.0, ny]); // décalage pour éviter corrélation

        DVec2::new(noise_x, noise_y)
    }

    pub fn lerp(&self, pos: U16Vec2, from: U16Vec2, to: U16Vec2) -> U16Vec2 {
        let noise = self.noise(pos);

        U16Vec2::new(
            self.remap_noise_to_u16(noise.x, from.x, to.x),
            self.remap_noise_to_u16(noise.y, from.y, to.y),
        )
    }

    fn remap_noise_to_u16(&self, v: f64, min: u16, max: u16) -> u16 {
        let t = (v + 1.0) * 0.5;
        min + ((max - min) as f64 * t) as u16
    }
}

impl TilesetAssets {
    pub fn get_tile(&self, ctx: &RandomContext, pos: U16Vec2, ground: Ground) -> Option<TileData> {
        self.grounds.get(&ground).map(|g| {
            let tile = ctx.lerp(pos, g.tiles.from, g.tiles.to);
            let row = self.width / self.tile_size;
            TileData::from_tileset_index(tile.x + tile.y * row)
        })
    }
}

pub fn tilemap(seed: u32, tileset_assets: &TilesetAssets) -> impl Bundle {
    let chunk_size = UVec2::splat(32);
    let tile_display_size = UVec2::splat(32);

    let random_context = RandomContext::new(seed + 33);

    let tile_data: Vec<Option<TileData>> = (0..chunk_size.x)
        .flat_map(|x| (0..chunk_size.y).map(move |y| U16Vec2::new(x as u16, y as u16)))
        .map(|pos| tileset_assets.get_tile(&random_context, pos, Ground::DirtRed))
        .collect();

    (
        TilemapChunk {
            chunk_size,
            tile_display_size,
            tileset: tileset_assets.texture.handle.clone(),
            ..default()
        },
        TilemapChunkTileData(tile_data),
    )
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

            let total_tiles = (tiles_x * tiles_y);

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
