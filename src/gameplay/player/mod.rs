use crate::gameplay::layer::GameLayer;
use crate::gameplay::player::asset::PlayerAssets;
use crate::gameplay::{animation::CharacterAnimation, movement::MovementController};
use avian2d::prelude::{
    Collider, CollisionEventsEnabled, CollisionLayers, DebugRender, LinearVelocity, LockedAxes,
    RigidBody,
};
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub mod asset;
mod movement;

pub fn plugin(app: &mut App) {
    app.add_plugins((asset::plugin, movement::plugin));
}

pub fn player(
    player_assets: &PlayerAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation = CharacterAnimation::new();

    (
        Name::new(player_assets.name.to_string()),
        Player,
        GameLayer::Player,
        Sprite::from_atlas_image(
            player_assets.sprite.handle.clone(),
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation.get_atlas_index(),
            },
        ),
        Anchor(Vec2::new(0., -0.3)),
        Transform::from_scale(Vec2::splat(2.0).extend(1.0)),
        MovementController {
            max_speed: player_assets.max_speed,
            ..default()
        },
        animation,
        RigidBody::Dynamic,
        Collider::circle(7.),
        LinearVelocity::ZERO,
        LockedAxes::ROTATION_LOCKED,
        CollisionEventsEnabled,
        CollisionLayers::new(GameLayer::Player, [GameLayer::Ground]),
        DebugRender::default().with_collider_color(Color::WHITE),
    )
}

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;
