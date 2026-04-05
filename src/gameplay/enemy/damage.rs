use crate::gameplay::animation::CharacterAnimation;
use crate::gameplay::enemy::asset::Enemy;
use crate::gameplay::health::Health;
use crate::gameplay::movement::MovementController;
use crate::screen::Screen;
use crate::{AppSystems, PausableSystems};
use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            update_damage_timer.in_set(AppSystems::TickTimers),
            (check_damage, flash_when_hurt).in_set(AppSystems::Update),
        )
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay(false))),
    );
}

#[derive(Component, Reflect)]
pub struct Hurt {
    pub timer: Timer,
    pub dead: bool,
}

fn check_damage(
    mut commands: Commands,
    enemies: Query<(Entity, &Health, &mut LinearVelocity), (With<Enemy>, Changed<Health>)>,
) {
    for (entity, health, mut vel) in enemies {
        if health.current == health.max {
            continue;
        }
        let dead = health.is_dead();
        let mut entity = commands.entity(entity);
        if dead {
            entity.remove::<Enemy>();
            entity.remove::<Health>();
            entity.remove::<MovementController>();
            entity.remove::<CharacterAnimation>();
            vel.0 = Vec2::ZERO;
        }
        entity.insert(Hurt {
            timer: Timer::from_seconds(0.15, TimerMode::Once),
            dead,
        });
    }
}

fn update_damage_timer(mut commands: Commands, time: Res<Time>, hurts: Query<(Entity, &mut Hurt)>) {
    for (e, mut hurt) in hurts {
        hurt.timer.tick(time.delta());
        if hurt.timer.just_finished() {
            if hurt.dead {
                commands.entity(e).despawn();
            } else {
                commands.entity(e).remove::<Hurt>();
            }
        }
    }
}

fn flash_when_hurt(mut enemies: Query<(&Hurt, &mut Sprite)>) {
    const STEPS: &[f32] = &[2.0, 2.0, 50.0, 50., 5.0, 3.0, 2.0, 1.0, 1.0];
    for (hurt, mut sprite) in &mut enemies {
        let t = hurt.timer.fraction();
        let index = ((t * STEPS.len() as f32) as usize).min(STEPS.len() - 1);
        let intensity = STEPS[index];
        sprite.color = Color::linear_rgb(intensity, intensity, intensity);
    }
}
