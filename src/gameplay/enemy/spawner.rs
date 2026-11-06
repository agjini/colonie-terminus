use crate::gameplay::animation::Animation;
use crate::gameplay::enemy::asset::{Damage, Enemy, EnemyAssets, EnemyType};
use crate::gameplay::health::Health;
use crate::gameplay::layer::GameLayer;
use crate::gameplay::level::{RandomSeed, WorldEntity};
use crate::gameplay::{animation::CharacterAnimation, movement::MovementController};
use crate::hud::timer::SurvivalTimer;
use crate::screen::Screen;
use crate::{AppSystems, PausableSystems};
use avian2d::prelude::{CollisionLayers, DebugRender, LockedAxes, Mass, RigidBody};
use bevy::color::palettes::tailwind::AMBER_400;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::RngExt;
use rand::seq::IndexedRandom;
use std::f32::consts::PI;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay(false)), init_spawn_timer);
    app.add_systems(
        Update,
        (
            update_spawn_timer.in_set(AppSystems::TickTimers),
            spawn_enemies.in_set(AppSystems::Update),
        )
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay(false))),
    );
}

#[derive(Resource)]
struct SpawnTimer(Timer);
fn init_spawn_timer(mut commands: Commands, enemy_assets: Res<EnemyAssets>) {
    commands.insert_resource(SpawnTimer(Timer::from_seconds(
        enemy_assets.spawn_interval_secs,
        TimerMode::Repeating,
    )));
}

fn update_spawn_timer(time: Res<Time>, mut timer: ResMut<SpawnTimer>) {
    timer.0.tick(time.delta());
}

fn spawn_enemies(
    mut commands: Commands,
    timer: Res<SpawnTimer>,
    mut rng: ResMut<RandomSeed>,
    enemy_assets: Res<EnemyAssets>,
    mut animations: ResMut<Assets<Animation>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    root: Single<Entity, With<EnemyRoot>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    survival: Res<SurvivalTimer>,
    enemies: Query<&Enemy>,
) {
    if !timer.0.just_finished() {
        return;
    }
    let elapsed = survival.0.elapsed_secs();
    let step = elapsed as u32 / enemy_assets.rarity_step_secs;

    let Ok(enemy_type) = enemy_assets
        .types
        .choose_weighted(&mut rng.0, |t| t.rarity_at(step))
    else {
        return;
    };
    let used: u32 = enemies.iter().map(|e| e.weight).sum();
    if used + enemy_type.weight > enemy_assets.max_weight_at(elapsed, step) {
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

    let angle = rng.0.random_range(0.0..2.0 * PI);
    let position = center + Vec2::new(angle.cos(), angle.sin()) * radius;
    root.with_children(|p| {
        p.spawn(enemy(
            position,
            enemy_type,
            &mut animations,
            &mut texture_atlas_layouts,
        ));
    });
}

#[derive(Component, Reflect)]
pub struct EnemyRoot;

pub fn enemy(
    position: Vec2,
    enemy: &EnemyType,
    animations: &mut Assets<Animation>,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    let (sprite, animation) =
        CharacterAnimation::init(animations, texture_atlas_layouts, &enemy.sprite);

    let collider = animation.hit_box.as_collider();
    let anchor = animation.anchor;
    (
        Name::new(enemy.name.to_string()),
        WorldEntity,
        (
            Enemy {
                weight: enemy.weight,
            },
            Damage {
                damage: enemy.damage,
                cooldown: enemy.cooldown,
            },
            Health::new(enemy.max_health),
            GameLayer::Enemy,
        ),
        sprite,
        animation,
        Anchor(anchor),
        Transform::from_xyz(position.x, position.y, 0.0),
        MovementController {
            max_speed: enemy.max_speed,
            ..default()
        },
        (
            RigidBody::Dynamic,
            Mass(1.0),
            collider,
            LockedAxes::ROTATION_LOCKED,
            CollisionLayers::new(
                GameLayer::Enemy,
                [
                    GameLayer::Enemy,
                    GameLayer::Player,
                    GameLayer::Bullet,
                    GameLayer::AimZone,
                ],
            ),
        ),
        DebugRender::default().with_collider_color(AMBER_400.into()),
    )
}
