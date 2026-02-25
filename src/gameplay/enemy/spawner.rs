use crate::gameplay::enemy::asset::{Enemy, EnemyAssets, EnemyType};
use crate::gameplay::layer::GameLayer;
use crate::gameplay::level::{RandomSeed, WorldEntity};
use crate::gameplay::{animation::CharacterAnimation, movement::MovementController};
use crate::screen::Screen;
use crate::{AppSystems, PausableSystems};
use avian2d::prelude::{
    Collider, CollisionEventsEnabled, CollisionLayers, DebugRender, LockedAxes, RigidBody,
};
use bevy::color::palettes::tailwind::AMBER_400;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::Rng;
use std::f32::consts::PI;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            update_timer.in_set(AppSystems::TickTimers),
            spawn_enemies.in_set(AppSystems::Update),
        )
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay(false))),
    );
    app.insert_resource(SpawnTimer::default());
}

#[derive(Resource)]
struct SpawnTimer(Timer);

impl Default for SpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1., TimerMode::Repeating))
    }
}

fn update_timer(time: Res<Time>, mut timer: ResMut<SpawnTimer>) {
    timer.0.tick(time.delta());
}

fn spawn_enemies(
    mut commands: Commands,
    timer: Res<SpawnTimer>,
    mut rng: ResMut<RandomSeed>,
    enemy_assets: Res<EnemyAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    root: Single<Entity, With<EnemyRoot>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
) {
    if !timer.0.just_finished() {
        return;
    }

    let Some(mut root) = commands.get_entity(*root).ok() else {
        return;
    };

    let resolution = Vec2::new(window.resolution.width(), window.resolution.height());
    let (camera, camera_transform) = *camera;
    let un = camera
        .viewport_to_world_2d(camera_transform, Vec2::ZERO)
        .unwrap();
    let deux = camera
        .viewport_to_world_2d(camera_transform, resolution)
        .unwrap();

    let half_size = (deux - un).abs() / 2.0;
    let center = (un + deux) / 2.0;
    let radius = half_size.length() + 50.0;

    root.with_children(|parent| {
        for enemy_type in enemy_assets.types.iter() {
            let angle = rng.0.random_range(0.0..2.0 * PI);
            let position = center + Vec2::new(angle.cos(), angle.sin()) * radius;
            parent.spawn(enemy(position, enemy_type, &mut texture_atlas_layouts));
        }
    });
}

#[derive(Component, Reflect)]
pub struct EnemyRoot;

pub fn enemy(
    position: Vec2,
    enemy: &EnemyType,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let enemy_animation = CharacterAnimation::new();

    (
        Name::new(enemy.name.to_string()),
        WorldEntity,
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
        Transform::from_xyz(position.x, position.y, 0.0).with_scale(Vec2::splat(2.0).extend(1.0)),
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
