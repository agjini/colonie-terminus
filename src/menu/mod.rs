mod credits;
mod game_over;
mod level_up;
mod main;
mod pause;
mod settings;

use crate::asset_tracking::LoadResource;
use bevy::prelude::*;
use ron_asset_manager::Shandle;
use ron_asset_manager::prelude::RonAsset;
use serde::Deserialize;

pub fn plugin(app: &mut App) {
    app.init_state::<Menu>();
    app.init_resource::<MenuStack>();
    app.add_observer(navigate);

    app.load_resource::<MenuAssets>("menu.ron");

    app.add_systems(OnEnter(Menu::Pause), spawn_overlay);
    app.add_systems(OnEnter(Menu::LevelUp), spawn_overlay);
    app.add_systems(OnEnter(Menu::GameOver), spawn_overlay);

    app.add_plugins((
        credits::plugin,
        main::plugin,
        settings::plugin,
        pause::plugin,
        level_up::plugin,
        game_over::plugin,
    ));
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Menu {
    #[default]
    None,
    Main,
    Credits,
    Settings,
    Pause,
    LevelUp,
    GameOver,
}

#[derive(Event, Copy, Clone)]
pub enum Nav {
    Open(Menu),
    Back,
}

#[derive(Resource, Default)]
struct MenuStack(Vec<Menu>);

fn navigate(
    nav: On<Nav>,
    current: Res<State<Menu>>,
    mut stack: ResMut<MenuStack>,
    mut next: ResMut<NextState<Menu>>,
) {
    match *nav {
        Nav::Open(menu) => {
            stack.0.push(*current.get());
            next.set(menu);
        }
        Nav::Back => {
            next.set(stack.0.pop().unwrap_or(Menu::None));
        }
    }
}

fn spawn_overlay(mut commands: Commands, menu: Res<State<Menu>>) {
    commands.spawn((
        Name::new("Overlay"),
        Node {
            width: percent(100),
            height: percent(100),
            ..default()
        },
        GlobalZIndex(1),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        DespawnOnExit(*menu.get()),
    ));
}

#[derive(Resource, TypePath, Asset, RonAsset, Deserialize, Debug, Clone)]
pub struct MenuAssets {
    #[asset]
    pub music: Shandle<AudioSource>,
    #[asset]
    pub font: Shandle<Font>,
    pub font_size_base: f32,
    pub created_by: Vec<(String, String)>,
    pub assets_by: Vec<(String, String)>,
}
