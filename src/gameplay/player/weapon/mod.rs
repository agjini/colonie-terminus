use crate::{AppSystems, PausableSystems};
use bevy::prelude::*;
use bevy::render::render_resource::{TextureDimension, TextureFormat};

mod asset;
mod bullet;
mod reticle;
mod slot;

use crate::gameplay::player::asset::PlayerAssets;
use crate::gameplay::player::weapon::asset::WeaponLevel;
use crate::gameplay::player::weapon::slot::WeaponSlots;
pub use asset::WeaponAssets;
pub use reticle::reticle;
pub use slot::weapon_slots;

pub fn plugin(app: &mut App) {
    app.add_plugins((asset::plugin, reticle::plugin, bullet::plugin));
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
pub struct WeaponDirection(pub Vec2);

fn update_timers(time: Res<Time>, mut slots: Single<&mut WeaponSlots>) {
    slots.tick(time.delta());
}

fn auto_fire(mut commands: Commands, slots: Single<&WeaponSlots>, dir: Single<&WeaponDirection>) {
    for slot in slots.just_finished() {
        let Some(bullet) = slot.level.attack.bullet(dir.0) else {
            return;
        };
        commands.spawn(bullet);
    }
}
