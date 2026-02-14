use crate::gameplay::player::Player;
use crate::{AppSystems, PausableSystems, gameplay::movement::MovementController};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        record_player_directional_input
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );
}

const UP: [KeyCode; 2] = [KeyCode::KeyW, KeyCode::ArrowUp];
const DOWN: [KeyCode; 2] = [KeyCode::KeyS, KeyCode::ArrowDown];
const LEFT: [KeyCode; 2] = [KeyCode::KeyA, KeyCode::ArrowLeft];
const RIGHT: [KeyCode; 2] = [KeyCode::KeyD, KeyCode::ArrowRight];

fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    mut controller_query: Query<&mut MovementController, With<Player>>,
) {
    let mut intent = Vec2::ZERO;

    if input.any_pressed(UP) {
        intent.y += 1.0;
    }
    if input.any_pressed(DOWN) {
        intent.y -= 1.0;
    }
    if input.any_pressed(LEFT) {
        intent.x -= 1.0;
    }
    if input.any_pressed(RIGHT) {
        intent.x += 1.0;
    }

    if let Some(gamepad) = gamepads.iter().next() {
        let left_stick_x = gamepad.left_stick().x;
        let left_stick_y = gamepad.left_stick().y;

        const DEADZONE: f32 = 0.2;
        if left_stick_x.abs() > DEADZONE || left_stick_y.abs() > DEADZONE {
            intent = Vec2::new(left_stick_x, left_stick_y);
        }
    }

    let intent = if intent.length() > 1.0 {
        intent.normalize()
    } else {
        intent
    };

    for mut controller in &mut controller_query {
        controller.direction = intent;
    }
}
