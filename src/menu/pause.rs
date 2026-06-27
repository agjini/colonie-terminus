use bevy::prelude::*;

use crate::menu::MenuAssets;
use crate::utils::escape_just_pressed;
use crate::{
    menu::{Menu, Nav},
    screen::Screen,
    theme::widget,
};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Pause), spawn_pause_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Pause).and_then(escape_just_pressed)),
    );
}

fn spawn_pause_menu(assets: Res<MenuAssets>, mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Pause Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Pause),
        children![
            widget::header(&assets, "Game paused"),
            widget::button(&assets, "Continue", close_menu),
            widget::button(&assets, "Settings", open_settings_menu),
            widget::button(&assets, "Quit to title", quit_to_title),
        ],
    ));
}

fn open_settings_menu(_: On<Pointer<Click>>, mut commands: Commands) {
    commands.trigger(Nav::Open(Menu::Settings));
}

fn close_menu(_: On<Pointer<Click>>, mut next: ResMut<NextState<Menu>>) {
    next.set(Menu::None);
}

fn quit_to_title(_: On<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title(true));
}

fn go_back(mut next: ResMut<NextState<Menu>>) {
    next.set(Menu::None);
}
