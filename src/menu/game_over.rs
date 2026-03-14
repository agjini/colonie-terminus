use bevy::prelude::*;

use crate::{menu::Menu, screen::Screen, theme::widget};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::GameOver), spawn_game_over);
}

fn spawn_game_over(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Game Over"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::GameOver),
        children![
            widget::header("Game over"),
            widget::header("Play again ?"),
            widget::button("Yes", play_again),
            widget::button("No", quit_to_title),
        ],
    ));
}

fn play_again(_: On<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Gameplay(true));
}

fn quit_to_title(_: On<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title(true));
}
