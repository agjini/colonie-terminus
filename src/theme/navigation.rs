use bevy::input_focus::directional_navigation::DirectionalNavigationPlugin;
use bevy::input_focus::{FocusCause, InputFocus, InputFocusVisible};
use bevy::math::CompassOctant;
use bevy::prelude::*;
use bevy::ui::UiGlobalTransform;
use bevy::ui::auto_directional_navigation::{AutoDirectionalNavigation, AutoDirectionalNavigator};
use bevy::ui_widgets::Activate;

pub fn plugin(app: &mut App) {
    app.add_plugins(DirectionalNavigationPlugin);
    app.insert_resource(InputFocusVisible(true));
    app.insert_resource(StickNavigationCooldown::default());
    app.add_systems(Update, focus_first_button);
    app.add_systems(
        PreUpdate,
        (handle_directional_input, handle_gamepad_activation).chain(),
    );
}

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

type NavigableButtons<'w, 's> =
    Query<'w, 's, (Entity, &'static UiGlobalTransform), With<AutoDirectionalNavigation>>;

fn top_left(buttons: &NavigableButtons) -> Option<Entity> {
    buttons
        .iter()
        .min_by_key(|(_, t)| (t.translation.y as i32, t.translation.x as i32))
        .map(|(entity, _)| entity)
}

fn focus_first_button(
    moved: Query<(), (With<AutoDirectionalNavigation>, Changed<UiGlobalTransform>)>,
    buttons: NavigableButtons,
    mut input_focus: ResMut<InputFocus>,
) {
    if moved.is_empty() || input_focus.get().is_some_and(|e| buttons.contains(e)) {
        return;
    }
    if let Some(first) = top_left(&buttons) {
        input_focus.set(first, FocusCause::Navigated);
    }
}

fn handle_directional_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    time: Res<Time>,
    mut cooldown: ResMut<StickNavigationCooldown>,
    mut navigator: AutoDirectionalNavigator,
    mut input_focus_visible: ResMut<InputFocusVisible>,
    buttons: NavigableButtons,
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
            const STICK_THRESHOLD: f32 = 0.3;
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

    let Some(direction) = direction else {
        return;
    };

    input_focus_visible.0 = true;
    if navigator.input_focus().is_some_and(|e| buttons.contains(e)) {
        let _ = navigator.navigate(direction);
    } else if let Some(first) = top_left(&buttons) {
        navigator
            .manual_directional_navigation
            .focus
            .set(first, FocusCause::Navigated);
    }
    if from_stick {
        cooldown.timer.reset();
    }
}

fn handle_gamepad_activation(
    mut commands: Commands,
    gamepads: Query<&Gamepad>,
    input_focus: Res<InputFocus>,
) {
    let pressed = gamepads
        .iter()
        .next()
        .is_some_and(|gamepad| gamepad.just_pressed(GamepadButton::South));
    if pressed && let Some(entity) = input_focus.get() {
        commands.trigger(Activate { entity });
    }
}
