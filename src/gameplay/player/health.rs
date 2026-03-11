use crate::gameplay::enemy::asset::{Damage, DamageCooldown, Enemy};
use crate::gameplay::health::Health;
use crate::gameplay::player::Player;
use crate::{AppSystems, PausableSystems};
use avian2d::prelude::CollidingEntities;
use bevy::app::App;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            update_cooldown.in_set(AppSystems::TickTimers),
            apply_damage.in_set(AppSystems::Update),
        )
            .in_set(PausableSystems),
    );
}

fn update_cooldown(
    mut commands: Commands,
    time: Res<Time>,
    enemies: Query<(Entity, &mut DamageCooldown)>,
) {
    for (entity, mut damage_cooldown) in enemies.into_iter() {
        damage_cooldown.timer.tick(time.delta());
        if damage_cooldown.timer.just_finished() {
            commands.entity(entity).remove::<DamageCooldown>();
        }
    }
}

fn apply_damage(
    mut commands: Commands,
    player: Single<(&mut Health, &CollidingEntities), With<Player>>,
    enemies: Query<&Damage, (With<Enemy>, Without<DamageCooldown>)>,
) {
    let (mut health, colliding_entities) = player.into_inner();
    for e in colliding_entities.iter() {
        let Ok(damage) = enemies.get(*e) else {
            continue;
        };
        health.current -= damage.damage;
        commands.entity(*e).insert(DamageCooldown {
            timer: Timer::from_seconds(damage.cooldown, TimerMode::Once),
        });
    }
}
