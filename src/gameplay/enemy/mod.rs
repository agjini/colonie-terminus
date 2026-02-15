use crate::gameplay::enemy::asset::{Enemy, EnemyAssets};
use crate::gameplay::layer::GameLayer;
use crate::gameplay::{animation::CharacterAnimation, movement::MovementController};
use avian2d::prelude::{
    Collider, CollisionEventsEnabled, CollisionLayers, DebugRender, LockedAxes, RigidBody,
};
use bevy::color::palettes::tailwind::AMBER_400;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::Rng;
use rand::prelude::StdRng;

pub mod asset;
pub mod movement;

pub fn plugin(app: &mut App) {
    app.add_plugins((asset::plugin, movement::plugin));
}

pub fn enemy(
    rng: &mut StdRng,
    enemy_assets: &EnemyAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    let enemy = enemy_assets.types.first().unwrap();

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let enemy_animation = CharacterAnimation::new();

    let x = rng.random_range(-2000.0..2000.0);
    let y = rng.random_range(-2000.0..2000.0);

    (
        Name::new(enemy.name.to_string()),
        Enemy,
        GameLayer::Enemy,
        Sprite::from_atlas_image(
            enemy.sprite.handle.clone(),
            TextureAtlas {
                layout: texture_atlas_layout,
                index: enemy_animation.get_atlas_index(),
            },
        ),
        Anchor(Vec2::new(0., -0.3)),
        Transform::from_xyz(x, y, 0.0).with_scale(Vec2::splat(2.0).extend(1.0)),
        MovementController {
            max_speed: enemy.max_speed,
            ..default()
        },
        enemy_animation,
        RigidBody::Dynamic,
        Collider::circle(7.),
        LockedAxes::ROTATION_LOCKED,
        CollisionEventsEnabled,
        CollisionLayers::new(GameLayer::Enemy, [GameLayer::Ground, GameLayer::Enemy]),
        DebugRender::default().with_collider_color(AMBER_400.into()),
    )
}
