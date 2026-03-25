use crate::{AppSystems, PausableSystems};
use bevy::prelude::*;

mod aim_zone;
mod asset;
mod bullet;
mod slot;

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
    dir: Single<&WeaponDirection>,
) {
    let Some(mut root) = commands.get_entity(*root).ok() else {
        return;
    };

    root.with_children(|parent| {
        for bullet in slots.just_finished().flat_map(|s| {
            s.level
                .attack
                .bullet(s.level.damage, origin.translation().truncate(), dir.0)
        }) {
            parent.spawn(bullet);
        }
    });
}
