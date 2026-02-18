use crate::gameplay::enemy::asset::Enemy;
use crate::gameplay::movement::MovementController;
use crate::gameplay::player::Player;
use crate::{AppSystems, PausableSystems};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            update_move_timer.in_set(AppSystems::TickTimers),
            move_enemies.in_set(AppSystems::Update),
        )
            .in_set(PausableSystems),
    );
    app.insert_resource(EnemyMoveTimer::default());
}

#[derive(Resource)]
struct EnemyMoveTimer(Timer);

impl Default for EnemyMoveTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Repeating))
    }
}

fn update_move_timer(time: Res<Time>, mut timer: ResMut<EnemyMoveTimer>) {
    timer.0.tick(time.delta());
}

fn move_enemies(
    timer: Res<EnemyMoveTimer>,
    enemies: Query<(&Transform, &mut MovementController), With<Enemy>>,
    player: Single<&Transform, With<Player>>,
) {
    if !timer.0.just_finished() {
        return;
    }

    let player_pos = player.translation.truncate();
    for (transform, mut mov) in enemies {
        let diff = player_pos - transform.translation.truncate();
        mov.direction = if diff.length() > 32.0 {
            diff.normalize()
        } else {
            Vec2::ZERO
        };
    }
}
