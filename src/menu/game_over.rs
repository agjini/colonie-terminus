use bevy::prelude::*;

use crate::menu::MenuAssets;
use crate::{menu::Menu, screen::Screen, theme::widget};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::GameOver), spawn_game_over);
}

fn spawn_game_over(mut commands: Commands, assets: Res<MenuAssets>) {
    commands.spawn((
        widget::ui_root("Game Over"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::GameOver),
        children![
            widget::header(&assets, "Game over"),
            widget::header(&assets, "Play again ?"),
            widget::button(&assets, "Yes", play_again),
            widget::button(&assets, "No", quit_to_title),
        ],
    ));
}

fn play_again(
    _: On<Pointer<Click>>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut next: ResMut<NextState<Menu>>,
) {
    next.set(Menu::None);
    next_screen.set(Screen::Gameplay(true));
}

fn quit_to_title(_: On<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title(true));
}
