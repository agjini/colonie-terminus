use crate::audio::sound_effect;
use crate::gameplay::loot::XpAmount;
use crate::gameplay::player::Player;
use crate::gameplay::player::asset::PlayerAssets;
use crate::screen::Screen;
use crate::{AppSystems, PausableSystems};
use avian2d::prelude::CollidingEntities;
use bevy::app::Update;
use bevy::math::ops::round;
use bevy::prelude::{
    App, Commands, Component, IntoScheduleConfigs, Query, Reflect, Res, Single, With, in_state,
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (apply_xp.in_set(AppSystems::Update),)
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay(false))),
    );
}

fn apply_xp(
    mut commands: Commands,
    player: Single<(&mut Xp, &CollidingEntities), With<Player>>,
    xp_gems: Query<&XpAmount>,
    player_assets: Res<PlayerAssets>,
) {
    let (mut xp, colliding_entities) = player.into_inner();
    for e in colliding_entities.iter() {
        let Ok(amount) = xp_gems.get(*e) else {
            continue;
        };
        xp.add(amount.0);
        commands.entity(*e).despawn();
        commands.spawn(sound_effect(player_assets.pickup_xp.handle.clone()));
    }
}

#[derive(Component, Reflect)]
pub struct Xp {
    pub level: u32,
    pub current: f32,
}

impl Default for Xp {
    fn default() -> Self {
        Self {
            level: 1,
            current: 0.,
        }
    }
}

impl Xp {
    fn new(level: u32) -> Self {
        Self { level, current: 0. }
    }

    pub fn add(&mut self, rhs: f32) -> bool {
        self.current = self.current + rhs;
        self.current <= self.next_level()
    }

    pub fn level_up(&mut self) {
        let diff = self.current - self.next_level();
        if diff <= 0. {
            self.current = diff;
            self.level = self.level + 1;
        }
    }

    pub fn skip(&mut self) {
        self.current = self.next_level() / 2.;
    }

    pub fn next_level(&self) -> f32 {
        self.level as f32 * 5.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_at_5() {
        assert_eq!(Xp::default().next_level(), 5.0);
    }

    #[test]
    fn level_2_at_15() {
        assert_eq!(Xp::new(2).next_level(), 10.0);
    }

    #[test]
    fn level_3_at_30() {
        assert_eq!(Xp::new(3).next_level(), 15.0);
    }

    #[test]
    fn level_4_at_45() {
        assert_eq!(Xp::new(4).next_level(), 20.0);
    }
}
