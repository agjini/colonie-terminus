use crate::gameplay::enemy::asset::{Damage, Enemy};
use crate::gameplay::health::Health;
use crate::gameplay::layer::GameLayer;
use crate::gameplay::player::weapon::slot::Weapon;
use crate::hud::spawn_damage_popup;
use crate::{AppSystems, PausableSystems};
use avian2d::math::PI;
use avian2d::prelude::*;
use bevy::color::palettes::tailwind::CYAN_500;
use bevy::prelude::*;
use ron_asset_manager::Shandle;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        update_bullets
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

impl Weapon {
    pub fn bullet(&self, from: Vec2, direction: Dir2) -> Option<impl Bundle> {
        let stats = self.stats();
        Some(bullet(
            &self.weapon.bullet,
            stats.damage,
            stats.speed,
            stats.lifetime,
            from,
            direction,
        ))
    }
}

pub fn bullet_root() -> impl Bundle {
    (BulletRoot, Transform::default(), Visibility::default())
}

pub fn fire_origin(fire_origin: Vec2) -> impl Bundle {
    (
        FireOrigin,
        Transform::from_xyz(fire_origin.x, fire_origin.y, 0.),
        Visibility::default(),
    )
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
    damage: f32,
    speed: f32,
    lifetime: f32,
    from: Vec2,
    direction: Dir2,
) -> impl Bundle {
    let mut sprite = Sprite::from_image(sprite.handle.clone());
    sprite.color = Color::linear_rgb(10., 10., 10.);
    (
        Name::new("Bullet"),
        Bullet,
        GameLayer::Bullet,
        BulletLifetime::new(lifetime),
        Damage {
            damage,
            cooldown: 0.,
        },
        sprite,
        Transform::from_scale(Vec2::splat(0.2).extend(1.0))
            .with_translation(from.extend(0.0))
            .with_rotation(Quat::from_rotation_z(
                direction.y.atan2(direction.x) - PI / 2.,
            )),
        (
            RigidBody::Kinematic,
            Collider::circle(7.),
            Sensor,
            LinearVelocity::from(direction * speed),
            LockedAxes::ROTATION_LOCKED,
            CollisionLayers::new(GameLayer::Bullet, [GameLayer::Enemy]),
            CollidingEntities::default(),
        ),
        DebugRender::default().with_collider_color(CYAN_500.into()),
    )
}

fn update_bullets(
    mut commands: Commands,
    time: Res<Time>,
    bullets: Query<(Entity, &Damage, &CollidingEntities, &mut BulletLifetime), With<Bullet>>,
    mut enemies: Query<(&mut Health, &GlobalTransform), With<Enemy>>,
) {
    for (bullet, damage, colliding_entities, mut lifetime) in bullets {
        let mut hit = false;
        for e in colliding_entities.iter() {
            let Ok((mut health, t)) = enemies.get_mut(*e) else {
                continue;
            };
            health.current -= damage.damage;
            spawn_damage_popup(&mut commands, t.translation(), damage.damage);
            hit = true;
        }

        lifetime.timer.tick(time.delta());

        if hit || lifetime.timer.is_finished() {
            commands.entity(bullet).despawn();
        }
    }
}
