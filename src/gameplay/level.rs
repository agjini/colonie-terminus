use crate::asset_tracking::LoadResource;
use crate::gameplay::enemy::{EnemyAssets, enemy};
use crate::gameplay::player::Player;
use crate::gameplay::tilemap::{
    ChunkPlanetPos, TilesetAssets, chunk_pixel_size, chunk_tile_data, tilemap_chunk,
};
use crate::{
    audio::music,
    gameplay::player::{PlayerAssets, player},
    screen::Screen,
};
use bevy::prelude::*;
use bevy::sprite_render::TilemapChunkTileData;
use ron_asset_manager::Shandle;
use ron_asset_manager::prelude::RonAsset;
use serde::Deserialize;

const CAMERA_DECAY_RATE: f32 = 2.;

#[derive(Component)]
pub struct TilemapOrigin;

#[derive(Component)]
pub struct WorldEntity;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>("level.ron");
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

#[derive(Resource, Asset, RonAsset, TypePath, Deserialize, Clone, Debug)]
pub struct LevelAssets {
    #[asset]
    pub music: Shandle<AudioSource>,
    pub seed: Option<u32>,
    pub planet_width: u32,
    pub planet_height: u32,
}

pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    tileset_assets: Res<TilesetAssets>,
    player_assets: Res<PlayerAssets>,
    enemy_assets: Res<EnemyAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let seed = level_assets.seed.unwrap_or(32);
    let planet_size = UVec2::new(level_assets.planet_width, level_assets.planet_height);
    let chunk = tilemap_chunk(&tileset_assets);
    let chunk_px = chunk_pixel_size(&tileset_assets);
    let cs = tileset_assets.chunk_size as i32;

    commands
        .spawn((
            Name::new("Level"),
            Transform::default(),
            Visibility::default(),
            DespawnOnExit(Screen::Gameplay(false)),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Name::new("Tilemap"),
                    TilemapOrigin,
                    Transform::default(),
                    Visibility::default(),
                ))
                .with_children(|tilemap_parent| {
                    for ox in -1..=1 {
                        for oy in -1..=1 {
                            let planet_pos = IVec2::new(ox * cs, oy * cs);
                            let tile_data =
                                chunk_tile_data(seed, planet_size, planet_pos, &tileset_assets);

                            tilemap_parent.spawn((
                                Transform::from_xyz(ox as f32 * chunk_px, oy as f32 * chunk_px, 0.),
                                chunk.clone(),
                                tile_data,
                                ChunkPlanetPos(planet_pos),
                            ));
                        }
                    }
                });

            parent.spawn(player(&player_assets, &mut texture_atlas_layouts));

            parent.spawn(enemy(&enemy_assets, &mut texture_atlas_layouts));

            parent.spawn((
                Name::new("Gameplay Music"),
                music(level_assets.music.handle.clone()),
            ));
        });
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
    let new_center = IVec2::new(
        (tilemap.translation.x / chunk_px).round() as i32,
        (tilemap.translation.y / chunk_px).round() as i32,
    );

    if new_center == *last_center {
        return;
    }
    *last_center = new_center;

    let seed = level_assets.seed.unwrap_or(32);
    let planet_size = UVec2::new(level_assets.planet_width, level_assets.planet_height);

    let expected: Vec<IVec2> = (-1..=1)
        .flat_map(|ox| {
            (-1..=1).map(move |oy| IVec2::new((new_center.x + ox) * cs, (new_center.y + oy) * cs))
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
        if !expected.contains(&planet_pos.0) {
            if let Some(new_pos) = uncovered_iter.next() {
                planet_pos.0 = new_pos;
                *tile_data = chunk_tile_data(seed, planet_size, new_pos, &tileset_assets);
            }
        }

        let gx = planet_pos.0.x / cs - new_center.x;
        let gy = planet_pos.0.y / cs - new_center.y;
        transform.translation.x = gx as f32 * chunk_px;
        transform.translation.y = gy as f32 * chunk_px;
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
