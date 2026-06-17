use bevy::prelude::*;

use crate::audio::music;
use crate::menu::MenuAssets;
use crate::{
    MetaState,
    menu::{Menu, Nav},
    screen::Screen,
    theme::widget,
};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(MetaState::InMenu), start_menu_music);
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
}

fn spawn_main_menu(assets: Res<MenuAssets>, mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Main Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Main),
        #[cfg(not(target_family = "wasm"))]
        children![
            widget::button(&assets, "Play", enter_gameplay_screen),
            widget::button(&assets, "Settings", open_settings_menu),
            widget::button(&assets, "Credits", open_credits_menu),
            widget::button(&assets, "Exit", exit_app),
        ],
        #[cfg(target_family = "wasm")]
        children![
            widget::button(&assets, "Play", enter_gameplay_screen),
            widget::button(&assets, "Settings", open_settings_menu),
            widget::button(&assets, "Credits", open_credits_menu),
        ],
    ));
}

fn enter_gameplay_screen(_: On<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Gameplay(true));
}

fn open_settings_menu(_: On<Pointer<Click>>, mut commands: Commands) {
    commands.trigger(Nav::Open(Menu::Settings));
}

fn open_credits_menu(_: On<Pointer<Click>>, mut commands: Commands) {
    commands.trigger(Nav::Open(Menu::Credits));
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: On<Pointer<Click>>, mut app_exit: MessageWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}

fn start_menu_music(mut commands: Commands, menu_assets: Option<Res<MenuAssets>>) {
    let Some(assets) = menu_assets else { return };
    commands.spawn((
        Name::new("Menu Music"),
        DespawnOnExit(MetaState::InMenu),
        music(assets.music.handle.clone()),
    ));
}
