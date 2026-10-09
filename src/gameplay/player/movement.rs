use crate::gameplay::player::Player;
use crate::gameplay::player::weapon::{FireOrigin, WeaponDirection};
use crate::{AppSystems, PausableSystems, gameplay::movement::MovementController};
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

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
const AIM_HALF_LIFE: f32 = 0.04;

fn record_weapon_direction(
    gamepads: Query<&Gamepad>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    fire_origin: Single<&GlobalTransform, With<FireOrigin>>,
    mut weapon_dir: Single<&mut WeaponDirection>,
    time: Res<Time>,
) {
    if let Some(stick) = active_right_stick(gamepads.iter().next()) {
        let decay_rate = f32::ln(2.0) / AIM_HALF_LIFE;
        let strength = stick.length().min(1.0);
        if let Ok(dir) = Dir2::new(stick) {
            weapon_dir
                .0
                .smooth_nudge(&dir, decay_rate * strength, time.delta_secs());
        }
        return;
    }

    let (camera, camera_transform) = *camera;
    if mouse_motion.delta != Vec2::ZERO
        && let Some(cursor) = window.cursor_position()
        && let Ok(target) = camera.viewport_to_world_2d(camera_transform, cursor)
        && let Ok(dir) = Dir2::new(target - fire_origin.translation().truncate())
    {
        weapon_dir.0 = dir;
    }
}

fn active_right_stick(gamepad: Option<&Gamepad>) -> Option<Vec2> {
    let stick = gamepad?.right_stick();
    (stick.x.abs() > AIM_DEADZONE || stick.y.abs() > AIM_DEADZONE).then_some(stick)
}
