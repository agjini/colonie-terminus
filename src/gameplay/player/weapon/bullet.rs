use crate::gameplay::layer::GameLayer;
use crate::gameplay::player::weapon::asset::WeaponAttack;
use crate::{AppSystems, PausableSystems};
use avian2d::math::PI;
use avian2d::prelude::*;
use bevy::color::palettes::tailwind::CYAN_500;
use bevy::prelude::*;
use ron_asset_manager::Shandle;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        despawn_bullets
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

#[derive(Component)]
pub struct BulletRoot;

#[derive(Component)]
pub struct FireOrigin;

#[derive(Component)]
struct Bullet;

impl WeaponAttack {
    pub fn bullet(&self, from: Vec2, direction: Dir2) -> Option<impl Bundle> {
        match self {
            WeaponAttack::Projectile {
                sprite,
                speed,
                lifetime,
                ..
            } => Some(bullet(sprite, *speed, *lifetime, from, direction)),
            _ => None,
        }
    }
}

pub fn bullet_root() -> impl Bundle {
    (BulletRoot, Transform::default(), Visibility::default())
}

pub fn fire_origin() -> impl Bundle {
    (FireOrigin, Transform::default(), Visibility::default())
}

#[derive(Component)]
struct BulletLifetime {
    timer: Timer,
}

impl BulletLifetime {
    fn new(lifetime: f32) -> Self {
        Self {
            timer: Timer::from_seconds(lifetime, TimerMode::Once),
        }
    }
}

pub fn bullet(
    sprite: &Shandle<Image>,
    speed: f32,
    lifetime: f32,
    from: Vec2,
    direction: Dir2,
) -> impl Bundle {
    (
        Name::new("Bullet"),
        Bullet,
        GameLayer::Bullet,
        BulletLifetime::new(lifetime),
        Sprite::from_image(sprite.handle.clone()),
        Transform::from_scale(Vec2::splat(0.2).extend(1.0))
            .with_translation(from.extend(0.0))
            .with_rotation(Quat::from_rotation_z(
                direction.y.atan2(direction.x) - PI / 2.,
            )),
        (
            RigidBody::Dynamic,
            Collider::circle(7.),
            LinearVelocity::from(direction * speed),
            LockedAxes::ROTATION_LOCKED,
            CollisionEventsEnabled,
            CollisionLayers::new(GameLayer::Bullet, [GameLayer::Enemy]),
        ),
        DebugRender::default().with_collider_color(CYAN_500.into()),
    )
}

fn despawn_bullets(
    mut commands: Commands,
    time: Res<Time>,
    bullets: Query<(Entity, &mut BulletLifetime)>,
) {
    for (entity, mut timer) in bullets {
        timer.timer.tick(time.delta());
        if timer.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
