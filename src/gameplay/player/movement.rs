use crate::gameplay::player::Player;
use crate::gameplay::player::weapon::WeaponDirection;
use crate::{AppSystems, PausableSystems, gameplay::movement::MovementController};
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

fn record_weapon_direction_with_mouse(
    gamepads: Query<&Gamepad>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut player_query: Query<(&GlobalTransform, &mut WeaponDirection), With<Player>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();
    let decay_rate = f32::ln(400.0);

    let (cam, cam_transform) = *camera;

    for (player_transform, mut weapon_dir) in &mut player_query {
        if let Some(gamepad) = gamepads.iter().next() {
            let stick = gamepad.right_stick();
            if stick.x.abs() > AIM_DEADZONE || stick.y.abs() > AIM_DEADZONE {
                weapon_dir.0.smooth_nudge(
                    &Vec2::new(stick.x, stick.y).normalize(),
                    decay_rate,
                    delta_time,
                );
                continue;
            }
        }

        // if let Some(cursor_pos) = window.cursor_position()
        //     && let Ok(world_pos) = cam.viewport_to_world_2d(cam_transform, cursor_pos)
        // {
        //     let dir = world_pos - player_transform.translation().truncate();
        //     if dir.length() > 1.0 {
        //         weapon_dir.0 = dir.normalize();
        //     }
        // }
    }
}

fn record_weapon_direction(
    gamepads: Query<&Gamepad>,
    mut weapon_dir: Single<&mut WeaponDirection>,
    time: Res<Time>,
) {
    let Some(gamepad) = gamepads.iter().next() else {
        return;
    };

    let delta_time = time.delta_secs();
    let decay_rate = f32::ln(400.0);

    let stick = gamepad.right_stick();
    if stick.x.abs() > AIM_DEADZONE || stick.y.abs() > AIM_DEADZONE {
        weapon_dir.0.smooth_nudge(
            &Vec2::new(stick.x, stick.y).normalize(),
            decay_rate,
            delta_time,
        );
    }
}
