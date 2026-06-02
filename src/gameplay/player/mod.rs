use crate::gameplay::health::Health;
use crate::gameplay::layer::GameLayer;
use crate::gameplay::player::asset::PlayerAssets;
use crate::gameplay::player::weapon::{
    WeaponAssets, WeaponDirection, aim_zone, fire_origin, weapon_slots,
};
use crate::gameplay::{animation::CharacterAnimation, movement::MovementController};
use avian2d::prelude::{
    CenterOfMass, Collider, CollidingEntities, CollisionLayers, DebugRender, LinearVelocity,
    LockedAxes, Mass, RigidBody, Sensor,
};
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub mod asset;
mod health;
mod movement;
pub mod weapon;
mod xp;

use crate::gameplay::animation::Animation;
pub use xp::Xp;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        asset::plugin,
        movement::plugin,
        weapon::plugin,
        health::plugin,
        xp::plugin,
    ));
}

pub fn spawn_player(
    commands: &mut RelatedSpawnerCommands<ChildOf>,
    player_assets: &PlayerAssets,
    weapon_assets: &WeaponAssets,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    animations: &mut Assets<Animation>,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) {
    commands
        .spawn(player(player_assets, animations, texture_atlas_layouts))
        .with_children(|player| {
            player.spawn(aim_zone(
                meshes,
                materials,
                player_assets.fire_origin,
                player_assets.auto_aim_angle,
            ));
            player.spawn(weapon_slots(weapon_assets));
            player.spawn(fire_origin(player_assets.fire_origin));
        });
}

fn player(
    player_assets: &PlayerAssets,
    animations: &mut Assets<Animation>,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    let (sprite, animation) =
        CharacterAnimation::init(animations, texture_atlas_layouts, &player_assets.sprite);

    (
        Name::new(player_assets.name.to_string()),
        Player,
        Health::new(player_assets.max_health),
        GameLayer::Player,
        Anchor(Vec2::new(0., -0.1)),
        Transform::default(),
        (
            sprite,
            animation,
            MovementController {
                max_speed: player_assets.max_speed,
                ..default()
            },
        ),
        (
            RigidBody::Dynamic,
            Collider::capsule(12., 70.),
            Mass(10.0),
            CenterOfMass::new(0.0, -0.1),
            Sensor,
            LinearVelocity::ZERO,
            LockedAxes::ROTATION_LOCKED,
            CollisionLayers::new(GameLayer::Player, [GameLayer::Loot, GameLayer::Enemy]),
            CollidingEntities::default(),
        ),
        WeaponDirection(Dir2::X),
        DebugRender::default().with_collider_color(Color::WHITE),
    )
}

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Default, Reflect)]
#[require(Health, Transform, Visibility, Xp)]
pub struct Player;
