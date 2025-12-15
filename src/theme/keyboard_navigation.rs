use bevy::camera::NormalizedRenderTarget;
use bevy::math::CompassOctant;
use bevy::picking::backend::HitData;
use bevy::picking::pointer::{Location, PointerId};
use bevy::prelude::*;
use bevy_input_focus::directional_navigation::{
    DirectionalNavigation, DirectionalNavigationMap, DirectionalNavigationPlugin,
};
use bevy_input_focus::{InputDispatchPlugin, InputFocus, InputFocusVisible};
use std::time::Duration;

use crate::theme::widget::highlight_focused_element;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((InputDispatchPlugin, DirectionalNavigationPlugin));
    app.insert_resource(InputFocusVisible(true));
    app.insert_resource(StickNavigationCooldown::default());
    app.add_systems(Update, setup_navigation_for_new_buttons);
    app.add_systems(
        PreUpdate,
        (handle_keyboard_navigation, handle_keyboard_activation).chain(),
    );
    app.add_systems(Update, highlight_focused_element);
}

/// Component to mark buttons that can be focused with keyboard navigation
#[derive(Component)]
pub struct Focusable;

#[derive(Resource)]
struct StickNavigationCooldown {
    timer: Timer,
}

impl Default for StickNavigationCooldown {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.2, TimerMode::Once),
        }
    }
}

fn setup_navigation_for_new_buttons(
    new_buttons: Query<Entity, (Added<Focusable>, With<Button>)>,
    all_buttons: Query<Entity, (With<Focusable>, With<Button>)>,
    mut nav_map: ResMut<DirectionalNavigationMap>,
    mut input_focus: ResMut<InputFocus>,
) {
    if new_buttons.is_empty() {
        return;
    }

    // Collect all focusable buttons
    let buttons: Vec<Entity> = all_buttons.iter().collect();

    if buttons.is_empty() {
        return;
    }

    // Clear previous navigation map
    nav_map.clear();

    // Create vertical and horizontal looping navigation
    nav_map.add_looping_edges(&buttons, CompassOctant::South);
    nav_map.add_looping_edges(&buttons, CompassOctant::East);

    // Always set focus on first button when new buttons are added
    input_focus.set(buttons[0]);
}

fn handle_keyboard_navigation(
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    time: Res<Time>,
    mut cooldown: ResMut<StickNavigationCooldown>,
    mut directional_navigation: DirectionalNavigation,
) {
    cooldown.timer.tick(time.delta());

    let mut direction = None;
    let mut from_stick = false;

    if keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::KeyW) {
        direction = Some(CompassOctant::North);
    } else if keyboard.just_pressed(KeyCode::ArrowDown) || keyboard.just_pressed(KeyCode::KeyS) {
        direction = Some(CompassOctant::South);
    } else if keyboard.just_pressed(KeyCode::ArrowLeft) || keyboard.just_pressed(KeyCode::KeyA) {
        direction = Some(CompassOctant::West);
    } else if keyboard.just_pressed(KeyCode::ArrowRight) || keyboard.just_pressed(KeyCode::KeyD) {
        direction = Some(CompassOctant::East);
    }

    if let Some(gamepad) = gamepads.iter().next() {
        if gamepad.just_pressed(GamepadButton::DPadUp) {
            direction = Some(CompassOctant::North);
        } else if gamepad.just_pressed(GamepadButton::DPadDown) {
            direction = Some(CompassOctant::South);
        } else if gamepad.just_pressed(GamepadButton::DPadLeft) {
            direction = Some(CompassOctant::West);
        } else if gamepad.just_pressed(GamepadButton::DPadRight) {
            direction = Some(CompassOctant::East);
        }

        if cooldown.timer.is_finished() {
            let left_stick = gamepad.left_stick();
            const STICK_THRESHOLD: f32 = 0.5;
            if left_stick.y > STICK_THRESHOLD && direction.is_none() {
                direction = Some(CompassOctant::North);
                from_stick = true;
            } else if left_stick.y < -STICK_THRESHOLD && direction.is_none() {
                direction = Some(CompassOctant::South);
                from_stick = true;
            } else if left_stick.x < -STICK_THRESHOLD && direction.is_none() {
                direction = Some(CompassOctant::West);
                from_stick = true;
            } else if left_stick.x > STICK_THRESHOLD && direction.is_none() {
                direction = Some(CompassOctant::East);
                from_stick = true;
            }
        }
    }

    if let Some(direction) = direction {
        let _ = directional_navigation.navigate(direction);
        if from_stick {
            cooldown.timer.reset();
        }
    }
}

fn handle_keyboard_activation(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    input_focus: Res<InputFocus>,
) {
    let mut should_activate =
        keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space);

    if let Some(gamepad) = gamepads.iter().next()
        && gamepad.just_pressed(GamepadButton::South)
    {
        should_activate = true;
    }

    if !should_activate {
        return;
    }

    let Some(focused_entity) = input_focus.0 else {
        return;
    };

    commands.trigger(Pointer::<Click> {
        entity: focused_entity,
        pointer_id: PointerId::Mouse,
        pointer_location: Location {
            target: NormalizedRenderTarget::None {
                width: 0,
                height: 0,
            },
            position: Vec2::ZERO,
        },
        event: Click {
            button: PointerButton::Primary,
            hit: HitData {
                camera: Entity::PLACEHOLDER,
                depth: 0.0,
                position: None,
                normal: None,
            },
            duration: Duration::from_secs_f32(0.1),
        },
    })
}
