use bevy::prelude::{ButtonInput, Gamepad, GamepadButton, KeyCode, Query, Res};
use itertools::Itertools;

pub fn escape_just_pressed(keyboard: Res<ButtonInput<KeyCode>>, gamepads: Query<&Gamepad>) -> bool {
    if keyboard.just_pressed(KeyCode::KeyP) || keyboard.just_pressed(KeyCode::Escape) {
        true
    } else if let Some(gamepad) = gamepads.iter().next() {
        gamepad.just_pressed(GamepadButton::Start)
    } else {
        false
    }
}

pub fn group_by<I, C: Ord, S: Ord>(
    iterator: impl IntoIterator<Item = I>,
    chunker: impl Fn(&I) -> C,
    sorter: impl Fn(&I) -> S,
) -> Vec<Vec<I>> {
    iterator
        .into_iter()
        .sorted_by_key(|item| chunker(item))
        .chunk_by(|item| chunker(item))
        .into_iter()
        .map(|(_, chunk)| chunk.sorted_by_key(|item| sorter(item)).collect())
        .collect()
}
