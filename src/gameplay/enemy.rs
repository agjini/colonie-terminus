use crate::asset_tracking::LoadResource;
use crate::gameplay::{animation::PlayerAnimation, movement::MovementController};
use bevy::prelude::*;
use ron_asset_manager::Shandle;
use ron_asset_manager::prelude::RonAsset;
use serde::Deserialize;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<EnemyAssets>("enemy.ron");
}

pub fn enemy(
    enemy_assets: &EnemyAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    let enemy = enemy_assets.enemies.first().unwrap();
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(32),
        enemy.cols,
        enemy.rows,
        Some(UVec2::splat(1)),
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let enemy_animation = PlayerAnimation::new();

    (
        Name::new(enemy.name.to_string()),
        Enemy,
        Sprite::from_atlas_image(
            enemy.sprite.handle.clone(),
            TextureAtlas {
                layout: texture_atlas_layout,
                index: enemy_animation.get_atlas_index(),
            },
        ),
        Transform::from_scale(Vec2::splat(2.0).extend(1.0)),
        MovementController {
            max_speed: enemy.max_speed,
            ..default()
        },
        enemy_animation,
    )
}

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub struct Enemy;

#[derive(Resource, Asset, RonAsset, TypePath, Deserialize, Debug, Clone)]
pub struct EnemyAssets {
    pub enemies: Vec<EnemyDef>,
}

#[derive(RonAsset, TypePath, Deserialize, Debug, Clone)]
pub struct EnemyDef {
    name: String,
    rows: u32,
    cols: u32,
    max_speed: f32,
    #[asset]
    sprite: Shandle<Image>,
}
