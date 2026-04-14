use crate::{AppSystems, PausableSystems};
use avian2d::prelude::CollidingEntities;
use bevy::prelude::*;
use rand::prelude::IndexedRandom;

mod aim_zone;
mod asset;
mod bullet;
mod slot;

use crate::audio::sound_effect;
use crate::gameplay::player::weapon::aim_zone::AimZone;
use crate::gameplay::player::weapon::bullet::FireOrigin;
use crate::gameplay::player::weapon::slot::WeaponSlots;
pub use aim_zone::aim_zone;
pub use asset::WeaponAssets;
pub use bullet::{BulletRoot, bullet_root, fire_origin};
pub use slot::weapon_slots;

pub fn plugin(app: &mut App) {
    app.add_plugins((asset::plugin, aim_zone::plugin, bullet::plugin));
    app.add_systems(
        Update,
        (
            update_timers.in_set(AppSystems::TickTimers),
            auto_fire.in_set(AppSystems::Update),
        )
            .in_set(PausableSystems),
    );
}

#[derive(Component, Reflect)]
pub struct WeaponDirection(pub Dir2);

fn update_timers(time: Res<Time>, mut slots: Single<&mut WeaponSlots>) {
    slots.tick(time.delta());
}

fn auto_fire(
    mut commands: Commands,
    origin: Single<&GlobalTransform, With<FireOrigin>>,
    root: Single<Entity, With<BulletRoot>>,
    slots: Single<&WeaponSlots>,
    aim_zone: Single<&CollidingEntities, With<AimZone>>,
    enemies: Query<&GlobalTransform>,
) {
    let Some(mut root) = commands.get_entity(*root).ok() else {
        return;
    };

    let origin_pos = origin.translation().truncate();

    let closest = aim_zone
        .iter()
        .flat_map(|e| enemies.get(*e).ok().map(|g| g.translation().truncate()))
        .min_by(|a, b| {
            let da = a.distance(origin_pos);
            let db = b.distance(origin_pos);
            da.partial_cmp(&db).unwrap()
        });

    let Some(enemy_pos) = closest else {
        return;
    };

    let direction = Dir2::new(enemy_pos - origin_pos).unwrap_or(Dir2::X);

    root.with_children(|parent| {
        for s in slots.just_finished() {
            let Some(bullet) =
                s.level
                    .attack
                    .bullet(s.level.damage, origin.translation().truncate(), direction)
            else {
                continue;
            };
            parent.spawn(bullet);
            let sound = s.weapon.trigger_sounds.choose(&mut rand::rng()).unwrap();
            parent.spawn(sound_effect(sound.handle.clone()));
        }
    });
}
