use crate::gameplay::player::weapon::WeaponAssets;
use crate::gameplay::player::weapon::asset::{WeaponAttack, WeaponLevel, WeaponType};
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component, Debug, Clone, Reflect)]
pub struct WeaponSlots {
    pub slots: [Option<WeaponSlot>; 2],
}

impl WeaponSlots {
    pub fn tick(&mut self, delta: Duration) {
        for slot in self.slots.iter_mut().flatten() {
            slot.timer.tick(delta);
        }
    }

    pub fn just_finished(&self) -> impl Iterator<Item = &WeaponSlot> {
        self.slots
            .iter()
            .flatten()
            .filter(|slot| slot.timer.just_finished())
    }
}

#[derive(Debug, Clone, Reflect)]
pub struct WeaponSlot {
    pub timer: Timer,
    #[reflect(ignore)]
    pub weapon: WeaponType,
    #[reflect(ignore)]
    pub level: WeaponLevel,
}

impl WeaponSlot {
    pub fn try_level(weapon: WeaponType, level: usize) -> Option<Self> {
        let level = weapon.levels.get(level)?.clone();
        let WeaponAttack::Projectile { fire_rate, .. } = level.attack else {
            return None;
        };
        Some(Self {
            timer: Timer::from_seconds(fire_rate, TimerMode::Repeating),
            weapon,
            level,
        })
    }
}

pub fn weapon_slots(weapon_assets: &WeaponAssets) -> impl Bundle {
    let weapon = weapon_assets.types.first().unwrap();
    (
        Name::new("WeaponSlots"),
        WeaponSlots {
            slots: [WeaponSlot::try_level(weapon.clone(), 0), None],
        },
    )
}
