use crate::asset_tracking::LoadResource;
use crate::gameplay::tilemap::generator::TilemapGenerator;
use bevy::math::U16Vec2;
use bevy::prelude::*;
use bevy::render::render_resource::Extent3d;
use bevy::render::render_resource::TextureDimension::D2;
use bevy::sprite_render::TileData;
use ron_asset_manager::Shandle;
use ron_asset_manager::prelude::RonAsset;
use serde::Deserialize;
use std::collections::HashMap;

pub fn plugin(app: &mut App) {
    app.load_resource::<TilesetAssets>("tileset.ron");
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

impl TilesetAssets {
    pub fn get_tile(&self, ctx: &TilemapGenerator, pos: IVec2) -> Option<TileData> {
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

pub fn convert_tileset_to_array(tileset_assets: &TilesetAssets, images: &mut Assets<Image>) {
    let source_image = images.get(&tileset_assets.texture.handle).unwrap();
    if source_image.texture_descriptor.size.depth_or_array_layers > 1 {
        return;
    }
    let tile_size = tileset_assets.tile_size;
    let tiles_x = source_image.width() / tile_size;
    let tiles_y = source_image.height() / tile_size;
    let total_tiles = tiles_x * tiles_y;
    let source_data = source_image.data.as_ref().unwrap();

    let mut tile_data = Vec::new();
    for tile_y in 0..tiles_y {
        for tile_x in 0..tiles_x {
            for pixel_y in 0..tile_size {
                for pixel_x in 0..tile_size {
                    let src_x = tile_x * tile_size + pixel_x;
                    let src_y = tile_y * tile_size + pixel_y;
                    let i = ((src_y * source_image.width() + src_x) * 4) as usize;
                    tile_data.extend_from_slice(&source_data[i..i + 4]);
                }
            }
        }
    }

    let format = source_image.texture_descriptor.format;
    let usage = source_image.asset_usage;
    let array_image = Image::new_fill(
        Extent3d {
            width: tile_size,
            height: tile_size,
            depth_or_array_layers: total_tiles,
        },
        D2,
        &tile_data,
        format,
        usage,
    );

    *images.get_mut(&tileset_assets.texture.handle).unwrap() = array_image;
}
