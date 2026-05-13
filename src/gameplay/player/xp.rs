use crate::audio::sound_effect;
use crate::gameplay::loot::XpAmount;
use crate::gameplay::player::Player;
use crate::gameplay::player::asset::PlayerAssets;
use crate::screen::Screen;
use crate::{AppSystems, PausableSystems};
use avian2d::prelude::CollidingEntities;
use bevy::app::Update;
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

impl Xp {
    fn add(&mut self, rhs: f32) {
        let current = self.current + rhs;
        let diff = current - self.next_level;
        if diff > 0. {
            self.current = diff;
            self.next_level = self.next_level + 10.;
            self.level = self.level + 1;
        } else {
            self.current = current;
        }
    }
}
