use crate::gameplay::player::weapon::WeaponAssets;
use crate::gameplay::player::weapon::asset::{WeaponAttack, WeaponLevel, WeaponType};
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component, Debug, Clone, Reflect)]
pub struct WeaponSlots {
    pub slot_1: WeaponSlot,
    pub slot_2: Option<WeaponSlot>,
}

impl WeaponSlots {
    pub fn tick(&mut self, delta: Duration) {
        self.slot_1.timer.tick(delta);
    }

    pub fn just_finished(&self) -> Option<&WeaponSlot> {
        if self.slot_1.timer.just_finished() {
            Some(&self.slot_1)
        } else {
            None
        }
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
            slot_1: WeaponSlot::try_level(weapon.clone(), 0).unwrap(),
            slot_2: None,
        },
    )
}
