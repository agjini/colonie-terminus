use crate::gameplay::enemy::asset::Enemy;
use crate::gameplay::movement::MovementController;
use crate::gameplay::player::Player;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, move_enemies);
}

fn move_enemies(
    enemies: Query<(&Transform, &mut MovementController), With<Enemy>>,
    player: Single<&Transform, With<Player>>,
) {
    let player_pos = player.translation.truncate();
    for (transform, mut mov) in enemies {
        let diff = player_pos - transform.translation.truncate();
        mov.direction = if diff.length() > 20.0 {
            diff.normalize()
        } else {
            Vec2::ZERO
        };
    }
}
