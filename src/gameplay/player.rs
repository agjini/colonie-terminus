use crate::asset_tracking::LoadResource;
use crate::{
    AppSystems, PausableSystems,
    gameplay::{animation::PlayerAnimation, movement::MovementController},
};
use avian2d::prelude::{
    Collider, CollisionEventsEnabled, DebugRender, LinearVelocity, LockedAxes, RigidBody,
};
use bevy::prelude::*;
use bevy::reflect::DynamicTypePath;
use ron_asset_manager::Shandle;
use ron_asset_manager::prelude::RonAsset;
use serde::Deserialize;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>("player.ron");
    app.add_systems(
        Update,
        record_player_directional_input
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );
}

pub fn player(
    player_assets: &PlayerAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = PlayerAnimation::new();

    (
        Name::new(player_assets.name.to_string()),
        Player,
        Sprite::from_atlas_image(
            player_assets.sprite.handle.clone(),
            TextureAtlas {
                layout: texture_atlas_layout,
                index: player_animation.get_atlas_index(),
            },
        ),
        Transform::from_scale(Vec2::splat(2.0).extend(1.0)),
        MovementController {
            max_speed: player_assets.max_speed,
            ..default()
        },
        player_animation,
        RigidBody::Dynamic,
        Collider::rectangle(28.0, 28.0),
        LinearVelocity::ZERO,
        LockedAxes::ROTATION_LOCKED,
        CollisionEventsEnabled,
        DebugRender::default().with_collider_color(Color::WHITE),
    )
}

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

const UP: [KeyCode; 2] = [KeyCode::KeyW, KeyCode::ArrowUp];
const DOWN: [KeyCode; 2] = [KeyCode::KeyS, KeyCode::ArrowDown];
const LEFT: [KeyCode; 2] = [KeyCode::KeyA, KeyCode::ArrowLeft];
const RIGHT: [KeyCode; 2] = [KeyCode::KeyD, KeyCode::ArrowRight];

fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    mut controller_query: Query<&mut MovementController, With<Player>>,
) {
    let mut intent = Vec2::ZERO;

    if input.any_pressed(UP) {
        intent.y += 1.0;
    }
    if input.any_pressed(DOWN) {
        intent.y -= 1.0;
    }
    if input.any_pressed(LEFT) {
        intent.x -= 1.0;
    }
    if input.any_pressed(RIGHT) {
        intent.x += 1.0;
    }

    if let Some(gamepad) = gamepads.iter().next() {
        let left_stick_x = gamepad.left_stick().x;
        let left_stick_y = gamepad.left_stick().y;

        const DEADZONE: f32 = 0.2;
        if left_stick_x.abs() > DEADZONE || left_stick_y.abs() > DEADZONE {
            intent = Vec2::new(left_stick_x, left_stick_y);
        }
    }

    let intent = if intent.length() > 1.0 {
        intent.normalize()
    } else {
        intent
    };

    for mut controller in &mut controller_query {
        controller.intent = intent;
    }
}

#[derive(Resource, Asset, RonAsset, TypePath, Deserialize, Debug, Clone)]
pub struct PlayerAssets {
    name: String,
    max_speed: f32,
    #[asset]
    sprite: Shandle<Image>,
    #[asset]
    pub steps: Vec<Shandle<AudioSource>>,
}
