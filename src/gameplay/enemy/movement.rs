use crate::gameplay::enemy::asset::Enemy;
use crate::gameplay::movement::MovementController;
use crate::gameplay::player::Player;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, move_enemies);
    app.insert_resource(EnemyMoveTimer::default());
}

#[derive(Resource)]
struct EnemyMoveTimer(Timer);

impl Default for EnemyMoveTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Repeating))
    }
}

fn move_enemies(
    time: Res<Time>,
    mut timer: ResMut<EnemyMoveTimer>,
    enemies: Query<(&Transform, &mut MovementController), With<Enemy>>,
    player: Single<&Transform, With<Player>>,
) {
    timer.0.tick(time.delta());
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
