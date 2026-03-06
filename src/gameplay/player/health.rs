use crate::gameplay::enemy::asset::{Damage, Enemy};
use crate::gameplay::health::Health;
use crate::gameplay::player::Player;
use crate::{AppSystems, PausableSystems};
use avian2d::prelude::CollidingEntities;
use bevy::app::App;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        apply_damage
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

fn apply_damage(
    time: Res<Time>,
    player: Single<(&mut Health, &CollidingEntities), With<Player>>,
    enemies: Query<&Damage, With<Enemy>>,
) {
    let (mut health, colliding_entities) = player.into_inner();
    for dps in colliding_entities.iter().flat_map(|e| enemies.get(*e)) {
        health.current -= dps.0 * time.delta_secs();
    }
}
