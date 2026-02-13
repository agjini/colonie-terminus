use crate::asset_tracking::LoadResource;
use crate::gameplay::tilemap::generator::TilemapGenerator;
use bevy::math::U16Vec2;
use bevy::prelude::*;
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
