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
            (check_damage, blink_when_hurt).in_set(AppSystems::Update),
        )
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay(false))),
    );
}

#[derive(Component, Reflect)]
pub struct Hurt(Timer);

fn check_damage(mut commands: Commands, enemies: Query<Entity, (With<Enemy>, Changed<Health>)>) {
    for entity in &enemies {
        commands
            .entity(entity)
            .insert(Hurt(Timer::from_seconds(1., TimerMode::Once)));
    }
}

fn update_damage_timer(
    mut commands: Commands,
    time: Res<Time>,
    hurts: Query<(Entity, &mut Hurt, &Health), With<Enemy>>,
) {
    for (e, mut hurt, health) in hurts.into_iter() {
        hurt.0.tick(time.delta());
        if hurt.0.just_finished() {
            if health.current <= 0. {
                commands.entity(e).despawn();
            } else {
                commands
                    .entity(e)
                    .remove::<Hurt>()
                    .insert(Visibility::Inherited);
            }
        }
    }
}

fn blink_when_hurt(mut enemies: Query<(&Hurt, &mut Visibility), With<Enemy>>) {
    for (hurt, mut visibility) in &mut enemies {
        let elapsed = hurt.0.elapsed_secs();
        let blink_on = (elapsed * 10.0) as u32 % 2 == 0;
        *visibility = if blink_on {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}
