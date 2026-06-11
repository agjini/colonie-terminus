use crate::gameplay::player::weapon::WeaponAssets;
use crate::gameplay::player::weapon::asset::{WeaponStats, WeaponType};
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component, Debug, Clone, Reflect)]
pub struct WeaponSlots {
    pub slots: Vec<Weapon>,
}

impl WeaponSlots {
    pub fn tick(&mut self, delta: Duration) {
        for weapon in self.slots.iter_mut() {
            weapon.timer.tick(delta);
        }
    }

    pub fn just_finished(&self) -> impl Iterator<Item = &Weapon> {
        self.slots.iter().filter(|slot| slot.timer.just_finished())
    }
}

#[derive(Debug, Clone, Reflect)]
pub struct Weapon {
    pub timer: Timer,
    #[reflect(ignore)]
    pub weapon: WeaponType,
    pub upgrade: WeaponStats,
}

impl Weapon {
    pub fn new(weapon: WeaponType) -> Self {
        Self {
            timer: Timer::from_seconds(weapon.stats.fire_rate, TimerMode::Repeating),
            weapon,
            upgrade: WeaponStats::default(),
        }
    }

    pub fn stats(&self) -> WeaponStats {
        self.weapon.stats.upgrade(self.upgrade)
    }

    pub fn inc_damage(&mut self, bonus_to_add: f32) {
        self.upgrade.damage = self.upgrade.damage + bonus_to_add;
    }

    pub fn inc_speed(&mut self, bonus_to_add: f32) {
        self.upgrade.speed = self.upgrade.speed + bonus_to_add;
    }

    pub fn inc_lifetime(&mut self, bonus_to_add: f32) {
        self.upgrade.lifetime = self.upgrade.lifetime + bonus_to_add;
    }

    pub fn inc_fire_rate(&mut self, bonus_to_add: f32) {
        self.upgrade.fire_rate = self.upgrade.fire_rate + bonus_to_add;
    }
}

pub fn weapon_slots(weapon_assets: &WeaponAssets) -> impl Bundle {
    let weapon = weapon_assets.types.first().unwrap();
    (
        Name::new("WeaponSlots"),
        WeaponSlots {
            slots: vec![Weapon::new(weapon.clone())],
        },
    )
}
