use crate::gameplay::player::weapon::WeaponAssets;
use crate::gameplay::player::weapon::asset::{WeaponStats, WeaponType};
use bevy::prelude::*;
use std::fmt::{Display, Formatter};
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
    pub weapon: WeaponType, // fire_rate = 0.5
    pub bonus: WeaponStats, // bonus = 0, 0.1, 0.2, 0.3
}

impl Weapon {
    pub fn new(weapon: WeaponType) -> Self {
        let delta = 1.0 / weapon.stats.fire_rate;
        Self {
            timer: Timer::from_seconds(delta, TimerMode::Repeating),
            weapon,
            bonus: WeaponStats::default(),
        }
    }

    pub fn stats(&self) -> WeaponStats {
        self.weapon.stats.apply(self.bonus)
    }

    pub fn inc_damage(&mut self, bonus_to_add: f32) {
        self.bonus.damage += bonus_to_add;
    }

    pub fn inc_speed(&mut self, bonus_to_add: f32) {
        self.bonus.speed += bonus_to_add;
    }

    #[allow(dead_code)]
    pub fn inc_lifetime(&mut self, bonus_to_add: f32) {
        self.bonus.lifetime += bonus_to_add;
    }

    pub fn inc_fire_rate(&mut self, bonus_to_add: f32) {
        self.bonus.fire_rate += bonus_to_add;
    }
}

impl Display for Weapon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let stats = self.stats();
        write!(
            f,
            "fire_rate = {} / {} => {} fire/s",
            self.weapon.stats.fire_rate, self.bonus.fire_rate, stats.fire_rate
        )
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
