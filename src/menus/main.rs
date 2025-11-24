//! The main menu (seen on the title screen).

use bevy::prelude::*;

use crate::asset_tracking::LoadResource;
use crate::audio::music;
use crate::{asset_tracking::ResourceHandles, menus::Menu, screens::Screen, theme::widget};

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct MenuAssets {
    #[dependency]
    pub(crate) music: Handle<AudioSource>,
    #[dependency]
    pub(crate) background: Handle<Image>,
}

impl FromWorld for MenuAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/splash_screen.ogg"),
            background: assets.load("images/menu_background.png"),
        }
    }
}

fn start_menu_music(mut commands: Commands, menu_assets: Res<MenuAssets>) {
    commands.spawn((
        Name::new("Menu Music"),
        DespawnOnEnter(Screen::Gameplay),
        music(menu_assets.music.clone()),
    ));
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), (spawn_main_menu, start_menu_music));
    app.load_resource::<MenuAssets>();
}

fn spawn_main_menu(mut commands: Commands, menu_assets: Res<MenuAssets>) {
    commands.spawn((
        widget::ui_root("Main Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Main),
        ImageNode::new(menu_assets.background.clone()),
        #[cfg(not(target_family = "wasm"))]
        children![
            widget::button("Play", enter_loading_or_gameplay_screen),
            widget::button("Settings", open_settings_menu),
            widget::button("Credits", open_credits_menu),
            widget::button("Exit", exit_app),
        ],
        #[cfg(target_family = "wasm")]
        children![
            widget::button("Play", enter_loading_or_gameplay_screen),
            widget::button("Settings", open_settings_menu),
            widget::button("Credits", open_credits_menu),
        ],
    ));
}

fn enter_loading_or_gameplay_screen(
    _: On<Pointer<Click>>,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if resource_handles.is_all_done() {
        next_screen.set(Screen::Gameplay);
    } else {
        next_screen.set(Screen::Loading);
    }
}

fn open_settings_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn open_credits_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Credits);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: On<Pointer<Click>>, mut app_exit: MessageWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
