use crate::gameplay::enemy::asset::Enemy;
use crate::gameplay::health::Health;
use crate::screen::Screen;
use crate::{AppSystems, PausableSystems};
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
    enemies: Query<(Entity, &Health), (With<Enemy>, Changed<Health>)>,
) {
    for (entity, health) in &enemies {
        if health.current == health.max {
            continue;
        }
        commands.entity(entity).insert(Hurt {
            timer: Timer::from_seconds(0.2, TimerMode::Once),
            dead: health.is_dead(),
        });
    }
}

fn update_damage_timer(
    mut commands: Commands,
    time: Res<Time>,
    hurts: Query<(Entity, &mut Hurt, &Health), With<Enemy>>,
) {
    for (e, mut hurt, health) in hurts.into_iter() {
        hurt.timer.tick(time.delta());
        if hurt.timer.just_finished() {
            if health.is_dead() {
                commands.entity(e).despawn();
            } else {
                commands.entity(e).remove::<Hurt>();
            }
        }
    }
}

fn flash_when_hurt(mut enemies: Query<(&Hurt, &mut Sprite), With<Enemy>>) {
    for (hurt, mut sprite) in &mut enemies {
        let t = hurt.timer.fraction();
        let f = EasingCurve::new(20.0, 1.0, EaseFunction::CubicIn);
        let intensity = f.sample_clamped(t);
        sprite.color = Color::linear_rgb(intensity, intensity, intensity);
    }
}
