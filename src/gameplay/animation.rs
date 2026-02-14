use bevy::prelude::*;
use rand::prelude::*;
use std::time::Duration;

use crate::gameplay::player::Player;
use crate::gameplay::player::asset::PlayerAssets;
use crate::{
    AppSystems, PausableSystems, audio::sound_effect, gameplay::movement::MovementController,
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            update_animation_timer.in_set(AppSystems::TickTimers),
            (
                update_animation_movement,
                update_animation_atlas,
                trigger_step_sound_effect,
            )
                .chain()
                .in_set(AppSystems::Update),
        )
            .in_set(PausableSystems),
    );
}

fn update_animation_timer(time: Res<Time>, mut query: Query<&mut CharacterAnimation>) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}

fn update_animation_movement(
    mut player_query: Query<(&MovementController, &mut Sprite, &mut CharacterAnimation)>,
) {
    for (controller, mut sprite, mut animation) in &mut player_query {
        let dx = controller.direction.x;
        if dx != 0.0 {
            sprite.flip_x = dx < 0.0;
        }

        let animation_state = if controller.direction == Vec2::ZERO {
            CharacterAnimationState::Idle
        } else {
            CharacterAnimationState::Walk
        };
        animation.update_state(animation_state);
    }
}

fn update_animation_atlas(mut query: Query<(&CharacterAnimation, &mut Sprite)>) {
    for (animation, mut sprite) in &mut query {
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };
        if animation.changed() {
            atlas.index = animation.get_atlas_index();
        }
    }
}

fn trigger_step_sound_effect(
    mut commands: Commands,
    player_assets: If<Res<PlayerAssets>>,
    mut step_query: Query<&CharacterAnimation, With<Player>>,
) {
    for animation in &mut step_query {
        if animation.state == CharacterAnimationState::Walk
            && animation.changed()
            && (animation.frame == 2 || animation.frame == 5)
        {
            let rng = &mut rand::rng();
            let random_step = player_assets.steps.choose(rng).unwrap().clone();
            commands.spawn(sound_effect(random_step.handle));
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct CharacterAnimation {
    timer: Timer,
    frame: usize,
    state: CharacterAnimationState,
}

#[derive(Reflect, PartialEq)]
pub enum CharacterAnimationState {
    Idle,
    Walk,
}

impl CharacterAnimation {
    const IDLE_FRAMES: usize = 2;
    const IDLE_INTERVAL: Duration = Duration::from_millis(500);
    const WALK_FRAMES: usize = 6;
    const WALKING_INTERVAL: Duration = Duration::from_millis(50);

    fn idling() -> Self {
        Self {
            timer: Timer::new(Self::IDLE_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: CharacterAnimationState::Idle,
        }
    }

    fn walking() -> Self {
        Self {
            timer: Timer::new(Self::WALKING_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: CharacterAnimationState::Walk,
        }
    }

    pub fn new() -> Self {
        Self::idling()
    }

    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if !self.timer.is_finished() {
            return;
        }
        self.frame = (self.frame + 1)
            % match self.state {
                CharacterAnimationState::Idle => Self::IDLE_FRAMES,
                CharacterAnimationState::Walk => Self::WALK_FRAMES,
            };
    }

    pub fn update_state(&mut self, state: CharacterAnimationState) {
        if self.state != state {
            match state {
                CharacterAnimationState::Idle => *self = Self::idling(),
                CharacterAnimationState::Walk => *self = Self::walking(),
            }
        }
    }

    pub fn changed(&self) -> bool {
        self.timer.is_finished()
    }

    pub fn get_atlas_index(&self) -> usize {
        match self.state {
            CharacterAnimationState::Idle => self.frame,
            CharacterAnimationState::Walk => 6 + self.frame,
        }
    }
}
