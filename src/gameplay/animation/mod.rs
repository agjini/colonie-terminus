use crate::gameplay::player::Player;
use crate::{AppSystems, PausableSystems, audio::sound_fx, gameplay::movement::MovementController};
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use rand::prelude::*;
use std::time::Duration;

mod asset;

use crate::audio::AudioSettings;
pub use asset::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(asset::plugin).add_systems(
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
    mut step_query: Query<&CharacterAnimation, With<Player>>,
    audio_settings: Res<AudioSettings>,
) {
    for animation in &mut step_query {
        let Some(steps) = animation.current.frames.steps.as_ref() else {
            continue;
        };

        if animation.changed() && steps.frames.contains(&animation.current.frame) {
            let rng = &mut rand::rng();
            let random_step = steps.samples.choose(rng).unwrap().clone();
            commands.spawn(sound_fx(random_step.handle, &audio_settings));
        }
    }
}

#[derive(Component)]
pub struct CharacterAnimation {
    frames: HashMap<CharacterAnimationState, AnimationFrames>,
    columns: usize,
    current: CurrentAnimation,
}

impl CharacterAnimation {
    pub fn init(
        animations: &mut Assets<Animation>,
        texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
        animation: &Handle<Animation>,
    ) -> (Sprite, Self) {
        let animation = animations.get(animation).unwrap();
        let layout = TextureAtlasLayout::from_grid(
            animation.size,
            animation.columns,
            animation.rows,
            animation.padding,
            animation.offset,
        );
        let image = animation.sheet.handle.clone();
        let layout = texture_atlas_layouts.add(layout);
        let animation = Self::idle(animation);
        let index = animation.get_atlas_index();
        (
            Sprite::from_atlas_image(image, TextureAtlas { layout, index }),
            animation,
        )
    }

    fn idle(animation: &Animation) -> Self {
        let frame = animation
            .frames
            .get(&CharacterAnimationState::Idle)
            .unwrap();
        let current = CurrentAnimation::new(CharacterAnimationState::Idle, frame.clone());
        Self {
            frames: animation.frames.clone(),
            columns: animation.columns as usize,
            current,
        }
    }

    pub fn update_timer(&mut self, delta: Duration) {
        self.current.timer.tick(delta);
        if !self.current.timer.is_finished() {
            return;
        }
        self.current.frame = (self.current.frame + 1) % self.current.frames.count;
    }

    pub fn update_state(&mut self, state: CharacterAnimationState) {
        if self.current.state == state {
            return;
        }
        let Some(frame) = self.frames.get(&state) else {
            return;
        };
        self.current = CurrentAnimation::new(state, frame.clone());
    }

    pub fn changed(&self) -> bool {
        self.current.timer.is_finished()
    }

    pub fn get_atlas_index(&self) -> usize {
        self.current.frames.row * self.columns + self.current.frame
    }
}

struct CurrentAnimation {
    timer: Timer,
    frame: usize,
    frames: AnimationFrames,
    state: CharacterAnimationState,
}

impl CurrentAnimation {
    fn new(state: CharacterAnimationState, frames: AnimationFrames) -> Self {
        let timer = Timer::new(Duration::from_millis(frames.interval), TimerMode::Repeating);
        Self {
            timer,
            frame: 0,
            frames,
            state,
        }
    }
}
