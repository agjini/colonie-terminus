use bevy::prelude::{App, Component, Reflect};
use std::ops::Add;

pub fn plugin(_app: &mut App) {}

#[derive(Component, Reflect)]
pub struct Xp {
    pub level: u32,
    pub current: f32,
    pub next_level: f32,
}

impl Default for Xp {
    fn default() -> Self {
        Self {
            level: 1,
            current: 0.,
            next_level: 5.,
        }
    }
}

impl Add<f32> for Xp {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        let current = self.current + rhs;
        let diff = current - self.next_level;
        if diff > 0. {
            Self {
                current: diff,
                next_level: self.next_level + 10.,
                level: self.level + 1,
            }
        } else {
            Self { current, ..self }
        }
    }
}
