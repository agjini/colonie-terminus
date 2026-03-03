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
struct Bullet;

impl WeaponAttack {
    pub fn bullet(&self, direction: Vec2) -> Option<impl Bundle> {
        match self {
            WeaponAttack::Projectile { sprite, speed, .. } => {
                Some(bullet(sprite, *speed, direction))
            }
            _ => None,
        }
    }
}

pub fn bullet(sprite: &Shandle<Image>, speed: f32, direction: Vec2) -> impl Bundle {
    (
        Name::new("Bullet"),
        Bullet,
        GameLayer::Bullet,
        Sprite::from_image(sprite.handle.clone()),
        Transform::from_scale(Vec2::splat(-0.1).extend(1.0))
            .with_rotation(Quat::from_rotation_z(direction.y.atan2(direction.x) + PI / 2.)),
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

fn despawn_bullets(mut commands: Commands, bullets: Query<(Entity, &Transform), With<Bullet>>) {
    for (entity, transform) in &bullets {
        if transform.translation.x.abs() > 1000. {
            commands.entity(entity).despawn();
        }
    }
}
