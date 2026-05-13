use crate::gameplay::GameLayer;
use crate::gameplay::enemy::EnemyDeathEvent;
use crate::gameplay::loot::LootRoot;
use crate::gameplay::loot::asset::LootAssets;
use avian2d::debug_render::DebugRender;
use avian2d::prelude::{
    Collider, CollidingEntities, CollisionLayers, LockedAxes, RigidBody, Sensor,
};
use bevy::app::App;
use bevy::color::palettes::tailwind::CYAN_500;
use bevy::prelude::*;
use ron_asset_manager::Shandle;

#[derive(Component, Reflect, Default)]
pub struct XpAmount(pub(crate) f32);

pub fn plugin(app: &mut App) {
    app.world_mut().add_observer(spawn_gem);
}

fn spawn_gem(
    on: On<EnemyDeathEvent>,
    root: Single<Entity, With<LootRoot>>,
    loot_assets: Res<LootAssets>,
    mut commands: Commands,
) {
    let Some(mut root) = commands.get_entity(*root).ok() else {
        return;
    };
    root.with_child(xp_gem(&loot_assets.xp, on.pos));
}

fn xp_gem(sprite: &Shandle<Image>, position: Vec2) -> impl Bundle {
    let mut sprite = Sprite::from_image(sprite.handle.clone());
    sprite.color = Color::linear_rgb(10., 10., 10.);
    (
        Name::new("Xp Gem"),
        XpAmount(2.),
        GameLayer::Loot,
        sprite,
        Transform::from_translation(position.extend(1.0)),
        (
            RigidBody::Static,
            Collider::circle(30.),
            Sensor,
            LockedAxes::ROTATION_LOCKED,
            CollisionLayers::new(GameLayer::Loot, [GameLayer::Player]),
            CollidingEntities::default(),
        ),
        DebugRender::default().with_collider_color(CYAN_500.into()),
    )
}
