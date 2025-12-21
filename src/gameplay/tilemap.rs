use crate::asset_tracking::LoadResource;
use bevy::prelude::*;
use bevy::render::render_resource::Extent3d;
use bevy::render::render_resource::TextureDimension::D2;
use bevy::sprite_render::{TileData, TilemapChunk, TilemapChunkTileData};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
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
    pub from: (u16, u16),
    pub to: (u16, u16),
    pub variants: HashMap<GroundVariant, GroundTileset>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GroundTileset {
    pub from: (u16, u16),
    pub to: (u16, u16),
}

impl TilesetAssets {
    pub fn get_tile(&self, ground: Ground) -> Option<TileData> {
        let rnd = ChaCha8Rng::seed_from_u64(19878367467712);
        self.grounds.get(&ground).map(|g| {
            let (from_x, from_y) = g.from;
            let row = self.width / self.tile_size;
            TileData::from_tileset_index(from_x + from_y * row)
        })
    }
}

pub fn tilemap(tileset_assets: &TilesetAssets) -> impl Bundle {
    let chunk_size = UVec2::splat(32);
    let tile_display_size = UVec2::splat(32);

    let tile_data: Vec<Option<TileData>> = (0..chunk_size.element_product())
        .map(|_| tileset_assets.get_tile(Ground::DirtRed))
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
