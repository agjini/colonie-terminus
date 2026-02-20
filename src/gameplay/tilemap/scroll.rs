use crate::gameplay::level::{LevelAssets, TilemapOrigin, WorldEntity};
use crate::gameplay::player::Player;
use crate::gameplay::tilemap::chunk::{chunk_pixel_size, chunk_tile_data};
use crate::gameplay::tilemap::{ChunkPlanetPos, TilesetAssets};
use bevy::prelude::*;
use bevy::sprite_render::TilemapChunkTileData;

const CAMERA_DECAY_RATE: f32 = 2.;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            update_tilemap_origin,
            recycle_chunks,
            update_camera,
            recenter_world,
        )
            .chain(),
    );
}

fn update_tilemap_origin(
    player: Single<&Transform, With<Player>>,
    mut tilemap: Single<&mut Transform, (With<TilemapOrigin>, Without<Player>)>,
    tileset_assets: Res<TilesetAssets>,
) {
    let size = chunk_pixel_size(&tileset_assets);

    let px = player.translation.x;
    let py = player.translation.y;

    tilemap.translation.x = (px / size).round() * size;
    tilemap.translation.y = (py / size).round() * size;
}

fn recycle_chunks(
    tilemap: Single<&Transform, With<TilemapOrigin>>,
    mut last_center: Local<IVec2>,
    mut chunks: Query<
        (
            &mut ChunkPlanetPos,
            &mut TilemapChunkTileData,
            &mut Transform,
        ),
        Without<TilemapOrigin>,
    >,
    tileset_assets: Res<TilesetAssets>,
    level_assets: Res<LevelAssets>,
) {
    let chunk_px = chunk_pixel_size(&tileset_assets);
    let cs = tileset_assets.chunk_size as i32;
    let planet_size = UVec2::new(level_assets.planet_width, level_assets.planet_height);
    let pw = planet_size.x as i32;
    let ph = planet_size.y as i32;
    let chunks_x = pw / cs;
    let chunks_y = ph / cs;

    let raw_center = IVec2::new(
        (tilemap.translation.x / chunk_px).round() as i32,
        (tilemap.translation.y / chunk_px).round() as i32,
    );
    let wrapped_center = IVec2::new(
        raw_center.x.rem_euclid(chunks_x),
        raw_center.y.rem_euclid(chunks_y),
    );

    if wrapped_center == *last_center {
        return;
    }
    *last_center = wrapped_center;

    let expected: Vec<IVec2> = (-1..=1)
        .flat_map(|ox| {
            (-1..=1).map(move |oy| {
                IVec2::new(
                    (wrapped_center.x + ox).rem_euclid(chunks_x) * cs,
                    (wrapped_center.y + oy).rem_euclid(chunks_y) * cs,
                )
            })
        })
        .collect();

    let uncovered: Vec<IVec2> = {
        let existing: Vec<IVec2> = chunks.iter().map(|(pos, _, _)| pos.0).collect();
        expected
            .iter()
            .filter(|p| !existing.contains(p))
            .copied()
            .collect()
    };

    let mut uncovered_iter = uncovered.into_iter();

    for (mut planet_pos, mut tile_data, mut transform) in &mut chunks {
        if !expected.contains(&planet_pos.0)
            && let Some(new_pos) = uncovered_iter.next()
        {
            planet_pos.0 = new_pos;
            *tile_data = chunk_tile_data(level_assets.seed, planet_size, new_pos, &tileset_assets);
        }

        let chunk_x = planet_pos.0.x / cs;
        let chunk_y = planet_pos.0.y / cs;

        let mut dx = chunk_x - wrapped_center.x;
        if dx > chunks_x / 2 {
            dx -= chunks_x;
        } else if dx < -(chunks_x / 2) {
            dx += chunks_x;
        }

        let mut dy = chunk_y - wrapped_center.y;
        if dy > chunks_y / 2 {
            dy -= chunks_y;
        } else if dy < -(chunks_y / 2) {
            dy += chunks_y;
        }

        transform.translation.x = dx as f32 * chunk_px;
        transform.translation.y = dy as f32 * chunk_px;
    }
}

fn recenter_world(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>, Without<TilemapOrigin>)>,
    mut player: Single<&mut Transform, (With<Player>, Without<Camera2d>, Without<TilemapOrigin>)>,
    mut tilemap: Single<&mut Transform, (With<TilemapOrigin>, Without<Player>, Without<Camera2d>)>,
    mut entities: Query<
        &mut Transform,
        (
            With<WorldEntity>,
            Without<Camera2d>,
            Without<Player>,
            Without<TilemapOrigin>,
        ),
    >,
    tileset_assets: Option<Res<TilesetAssets>>,
    level_assets: Option<Res<LevelAssets>>,
) {
    let Some(tileset_assets) = tileset_assets else {
        return;
    };
    let Some(level_assets) = level_assets else {
        return;
    };

    let size = (level_assets.planet_width * tileset_assets.tile_size) as f32;

    let px = player.translation.x;
    let py = player.translation.y;

    let threshold = size / 2.;
    if px.abs() < threshold && py.abs() < threshold {
        return;
    }

    let offset_x = (px / size).round() * size;
    let offset_y = (py / size).round() * size;

    player.translation.x -= offset_x;
    player.translation.y -= offset_y;

    camera.translation.x -= offset_x;
    camera.translation.y -= offset_y;

    tilemap.translation.x -= offset_x;
    tilemap.translation.y -= offset_y;

    for mut t in &mut entities {
        t.translation.x -= offset_x;
        t.translation.y -= offset_y;
    }
}

fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);
    camera
        .translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
}
