// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

mod asset_tracking;
mod audio;
mod gameplay;
mod menu;
mod screen;
mod theme;
mod utils;

#[cfg(feature = "dev")]
mod dev_tools;

use crate::screen::Screen;
use crate::screen::Screen::{Gameplay, Title};
use avian2d::PhysicsPlugins;
use avian2d::prelude::{Gravity, Physics, PhysicsTime};
use bevy::input::common_conditions::input_just_pressed;
use bevy::window::{CursorOptions, PresentMode, PrimaryWindow, WindowMode, WindowResolution};
use bevy::{asset::AssetMetaCheck, prelude::*};

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Colonie Terminus".to_string(),
                        fit_canvas_to_parent: true,
                        resolution: WindowResolution::new(1024, 768),
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }
                    .into(),
                    primary_cursor_options: CursorOptions {
                        visible: false,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            asset_tracking::plugin,
            audio::plugin,
            gameplay::plugin,
            PhysicsPlugins::default(),
            menu::plugin,
            screen::plugin,
            theme::plugin,
            #[cfg(feature = "dev")]
            dev_tools::plugin,
        ))
        .insert_resource(Gravity::ZERO);

        app.configure_sets(
            Update,
            (
                AppSystems::TickTimers,
                AppSystems::RecordInput,
                AppSystems::Update,
            )
                .chain(),
        );

        app.init_state::<Pause>();
        app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));
        app.add_computed_state::<MetaState>();

        app.add_systems(OnEnter(Pause(true)), pause);
        app.add_systems(OnEnter(Pause(false)), unpause);
        app.add_systems(Startup, spawn_camera);
        app.add_systems(
            Update,
            change_window_mode.run_if(input_just_pressed(KeyCode::F3)),
        );
    }
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSystems {
    TickTimers,
    RecordInput,
    Update,
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
struct Pause(pub bool);

#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct PausableSystems;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera2d));
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
enum MetaState {
    #[default]
    InMenu,
    InGame,
    Loading,
}

impl ComputedStates for MetaState {
    type SourceStates = Screen;

    fn compute(screen: Screen) -> Option<Self> {
        match screen {
            Title(loading) => Some(if loading {
                MetaState::Loading
            } else {
                MetaState::InMenu
            }),
            Gameplay(loading) => Some(if loading {
                MetaState::Loading
            } else {
                MetaState::InGame
            }),
            _ => Some(MetaState::InMenu),
        }
    }
}

fn change_window_mode(mut window: Single<&mut Window, With<PrimaryWindow>>) {
    window.mode = if window.mode == WindowMode::Windowed {
        WindowMode::BorderlessFullscreen(MonitorSelection::Current)
    } else {
        WindowMode::Windowed
    };
}

fn pause(mut time: ResMut<Time<Physics>>) {
    time.pause();
}

fn unpause(mut time: ResMut<Time<Physics>>) {
    time.unpause();
}
