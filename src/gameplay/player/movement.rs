use crate::gameplay::player::Player;
use crate::gameplay::player::weapon::WeaponDirection;
use crate::{AppSystems, PausableSystems, gameplay::movement::MovementController};
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (record_player_directional_input, record_weapon_direction)
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

const AIM_DEADZONE: f32 = 0.1;
const MOUSE_SENSITIVITY: f32 = 0.05;

fn record_weapon_direction(
    gamepads: Query<&Gamepad>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut weapon_dir: Single<&mut WeaponDirection>,
    time: Res<Time>,
) {
    const AIM_HALF_LIFE: f32 = 0.08;
    let decay_rate = f32::ln(2.0) / AIM_HALF_LIFE;
    let (direction, strength) = get_new_direction(gamepads.iter().next(), &mouse_motion);
    if strength > 0.0 {
        weapon_dir
            .0
            .smooth_nudge(&direction, decay_rate * strength, time.delta_secs());
    }
}

fn get_new_direction(
    gamepad: Option<&Gamepad>,
    mouse_motion: &AccumulatedMouseMotion,
) -> (Vec2, f32) {
    if let Some(gamepad) = gamepad
        && let stick = gamepad.right_stick()
        && (stick.x.abs() > AIM_DEADZONE || stick.y.abs() > AIM_DEADZONE)
    {
        let stick = Vec2::new(stick.x, stick.y);
        (stick.normalize(), stick.length().min(1.0))
    } else {
        let delta = Vec2::new(mouse_motion.delta.x, -mouse_motion.delta.y);
        let strength = (delta.length() * MOUSE_SENSITIVITY).min(1.0);
        (delta.normalize_or_zero(), strength)
    }
}
