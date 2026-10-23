use bevy::prelude::{ButtonInput, Gamepad, GamepadButton, KeyCode, Query, Res};

pub fn escape_just_pressed(keyboard: Res<ButtonInput<KeyCode>>, gamepads: Query<&Gamepad>) -> bool {
    if keyboard.just_pressed(KeyCode::KeyP) || keyboard.just_pressed(KeyCode::Escape) {
        true
    } else if let Some(gamepad) = gamepads.iter().next() {
        gamepad.just_pressed(GamepadButton::Start)
    } else {
        false
    }
}
