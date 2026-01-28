use crate::asset_tracking::LoadResource;
use crate::gameplay::player::Player;
use crate::gameplay::tilemap::{TilesetAssets, tilemap_data, world_size};
use crate::{
    audio::music,
    gameplay::player::{PlayerAssets, player},
    screen::Screen,
};
use bevy::prelude::*;
use ron_asset_manager::Shandle;
use ron_asset_manager::prelude::RonAsset;
use serde::Deserialize;

const CAMERA_DECAY_RATE: f32 = 2.;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>("level.ron");
    app.add_systems(Update, (update_camera, loop_translations));
}

#[derive(Resource, Asset, RonAsset, TypePath, Deserialize, Clone, Debug)]
pub struct LevelAssets {
    #[asset]
    pub music: Shandle<AudioSource>,
    pub seed: Option<u32>,
}

pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    tileset_assets: Res<TilesetAssets>,
    player_assets: Res<PlayerAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let tilemap = tilemap_data(level_assets.seed.unwrap_or(32), &tileset_assets);

    commands
        .spawn((
            Name::new("Level"),
            Transform::default(),
            Visibility::default(),
            DespawnOnExit(Screen::Gameplay),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Name::new("Tilemap"),
                    Transform::default(),
                    Visibility::default(),
                ))
                .with_children(|tilemap_parent| {
                    for (ox, oy) in [
                        (-1., -1.),
                        (0., -1.),
                        (1., -1.),
                        (-1., 0.),
                        (0., 0.),
                        (1., 0.),
                        (-1., 1.),
                        (0., 1.),
                        (1., 1.),
                    ] {
                        tilemap_parent.spawn((
                            Transform::from_xyz(
                                ox * tilemap.world_size,
                                oy * tilemap.world_size,
                                0.,
                            ),
                            tilemap.chunk.clone(),
                            tilemap.tile_data.clone(),
                        ));
                    }
                });

            parent.spawn(player(400.0, &player_assets, &mut texture_atlas_layouts));

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
    camera.translation = direction;
    //.smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
}

fn loop_translations(
    mut query: Query<&mut Transform, With<Player>>,
    tileset_assets: Option<Res<TilesetAssets>>,
) {
    let Some(tileset_assets) = tileset_assets else {
        return;
    };
    let size = world_size(&tileset_assets);

    for mut t in &mut query {
        t.translation.x = t.translation.x.rem_euclid(size);
        t.translation.y = t.translation.y.rem_euclid(size);
    }
}
