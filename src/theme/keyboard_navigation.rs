use bevy::math::CompassOctant;
use bevy::prelude::*;
use bevy_input_focus::directional_navigation::{
    DirectionalNavigationMap, DirectionalNavigationPlugin,
};
use bevy_input_focus::{InputDispatchPlugin, InputFocus, InputFocusVisible};

use crate::theme::interaction::InteractionPalette;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((InputDispatchPlugin, DirectionalNavigationPlugin));
    app.insert_resource(InputFocusVisible(true));
    app.insert_resource(StickNavigationCooldown::default());
    app.add_systems(Update, setup_navigation_for_new_buttons);
    app.add_systems(
        PreUpdate,
        (handle_keyboard_navigation, handle_keyboard_activation).chain(),
    );
    app.add_systems(Update, apply_focus_visual);
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
    mut directional_navigation: bevy_input_focus::directional_navigation::DirectionalNavigation,
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
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    input_focus: Res<InputFocus>,
    mut commands: Commands,
    buttons: Query<(), With<Button>>,
    windows: Query<Entity, With<Window>>,
) {
    let mut should_activate = keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space);

    if let Some(gamepad) = gamepads.iter().next() {
        if gamepad.just_pressed(GamepadButton::South) {
            should_activate = true;
        }
    }

    if should_activate
        && let Some(focused_entity) = input_focus.0
            && buttons.contains(focused_entity) {
                let Some(window_entity) = windows.iter().next() else {
                    warn!("No window found, cannot activate button");
                    return;
                };

                let Some(normalized_window) = bevy::window::WindowRef::Primary.normalize(Some(window_entity)) else {
                    warn!("Failed to normalize window reference");
                    return;
                };

                use bevy::picking::events::{Click, Pointer};
                use bevy::picking::pointer::{PointerId, PointerButton};
                use bevy::picking::backend::HitData;

                commands.entity(focused_entity).trigger(|entity| {
                    Pointer::new(
                        PointerId::Mouse,
                        bevy::picking::pointer::Location {
                            target: normalized_window.into(),
                            position: Vec2::ZERO,
                        },
                        Click {
                            button: PointerButton::Primary,
                            hit: HitData {
                                camera: Entity::PLACEHOLDER,
                                depth: 0.0,
                                position: None,
                                normal: None,
                            },
                            duration: std::time::Duration::from_millis(0),
                        },
                        entity,
                    )
                });
            }
}

fn apply_focus_visual(
    input_focus: Res<InputFocus>,
    input_focus_visible: Res<InputFocusVisible>,
    mut query: Query<(
        Entity,
        &InteractionPalette,
        &Interaction,
        &mut BackgroundColor,
    ), With<Button>>,
) {
    for (entity, palette, interaction, mut background) in query.iter_mut() {
        if input_focus.0 == Some(entity) && input_focus_visible.0 && *interaction == Interaction::None
        {
            // Focused button uses hovered style
            *background = palette.hovered.into();
        } else {
            // Non-focused buttons use normal interaction palette
            *background = match interaction {
                Interaction::None => palette.none,
                Interaction::Hovered => palette.hovered,
                Interaction::Pressed => palette.pressed,
            }
            .into();
        }
    }
}
