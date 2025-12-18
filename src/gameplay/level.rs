//! Spawn the main level.

use crate::{
    asset_tracking::LoadResource,
    audio::music,
    gameplay::player::{PlayerAssets, player},
    screens::Screen,
};
use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Fluffing A Duck.ogg"),
        }
    }
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    player_assets: Res<PlayerAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        DespawnOnExit(Screen::Gameplay),
        children![
            player(400.0, &player_assets, &mut texture_atlas_layouts),
            (
                Name::new("Gameplay Music"),
                music(level_assets.music.clone())
            ),
            wall(
                &mut meshes,
                &mut materials,
                Vec2::new(0.0, -100.0),
                Vec2::new(500.0, 25.0)
            ),
            wall(
                &mut meshes,
                &mut materials,
                Vec2::new(0.0, 100.0),
                Vec2::new(500.0, 25.0)
            ),
            wall(
                &mut meshes,
                &mut materials,
                Vec2::new(-250.0, 0.0),
                Vec2::new(25.0, 200.0)
            ),
            wall(
                &mut meshes,
                &mut materials,
                Vec2::new(250.0, 0.0),
                Vec2::new(25.0, 200.0)
            ),
            wall(
                &mut meshes,
                &mut materials,
                Vec2::new(100.0, 0.0),
                Vec2::new(100.0, 25.0)
            ),
        ],
    ));
}

fn wall(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec2,
    size: Vec2,
) -> impl Bundle {
    (
        Name::new("Wall"),
        Mesh2d(meshes.add(Rectangle::new(size.x, size.y))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::WHITE))),
        Transform::from_translation(position.extend(0.0)),
        RigidBody::Static,
        Collider::rectangle(size.x, size.y),
    )
}
