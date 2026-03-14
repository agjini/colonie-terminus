use avian2d::prelude::{Physics, PhysicsTime};
use bevy::prelude::*;

use crate::audio::music;
use crate::gameplay::level::{GameplayMusic, LevelAssets};
use crate::menu::Menu;
use crate::screen::Screen;
use crate::utils::escape_just_pressed;
use bevy_seedling::prelude::PlaybackSettings;

pub fn plugin(app: &mut App) {
    app.add_sub_state::<GameState>();
    app.add_systems(
        Update,
        enter_pause.run_if(in_state(GameState::InGame).and(escape_just_pressed)),
    );
    app.add_systems(
        OnEnter(Menu::None),
        resume.run_if(in_state(GameState::Pause)),
    );
    app.add_systems(
        OnEnter(GameState::Pause),
        (open_pause_menu, spawn_overlay),
    );
    app.add_systems(
        OnEnter(GameState::GameOver),
        (open_game_over_menu, spawn_overlay, start_game_over_music),
    );
    app.add_systems(OnEnter(GameState::LevelUp), spawn_overlay);
    app.add_systems(OnEnter(GameState::InGame), (close_menu, unpause, resume_music));
    app.add_systems(OnExit(GameState::InGame), (pause, pause_music));
    app.add_systems(OnExit(Screen::Gameplay(false)), close_menu);
}

#[derive(SubStates, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[source(Screen = Screen::Gameplay(false))]
pub enum GameState {
    #[default]
    InGame,
    Pause,
    GameOver,
    LevelUp,
}

fn enter_pause(mut next: ResMut<NextState<GameState>>) {
    next.set(GameState::Pause);
}

fn resume(mut next: ResMut<NextState<GameState>>) {
    next.set(GameState::InGame);
}

fn open_pause_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pause);
}

fn open_game_over_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::GameOver);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

fn spawn_overlay(mut commands: Commands, state: Res<State<GameState>>) {
    commands.spawn((
        Name::new("Overlay"),
        Node {
            width: percent(100),
            height: percent(100),
            ..default()
        },
        GlobalZIndex(1),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        DespawnOnExit(*state.get()),
    ));
}

fn start_game_over_music(mut commands: Commands, level_assets: Res<LevelAssets>) {
    commands.spawn((
        Name::new("Game Over Music"),
        DespawnOnExit(GameState::GameOver),
        music(level_assets.game_over.handle.clone()),
    ));
}

fn pause_music(mut music: Query<&mut PlaybackSettings, With<GameplayMusic>>) {
    for mut settings in &mut music {
        settings.pause();
    }
}

fn resume_music(mut music: Query<&mut PlaybackSettings, With<GameplayMusic>>) {
    for mut settings in &mut music {
        settings.play();
    }
}

fn pause(mut time: ResMut<Time<Physics>>) {
    time.pause();
}

fn unpause(mut time: ResMut<Time<Physics>>) {
    time.unpause();
}
