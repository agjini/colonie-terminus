use avian2d::prelude::{Physics, PhysicsTime};
use bevy::prelude::*;

use crate::MetaState;
use crate::audio::{AudioSettings, music};
use crate::gameplay::level::{GameplayMusic, LevelAssets};
use crate::menu::Menu;
use crate::utils::escape_just_pressed;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        enter_pause.run_if(
            in_state(MetaState::InGame)
                .and_then(in_state(Menu::None))
                .and_then(escape_just_pressed),
        ),
    );
    app.add_systems(OnEnter(MetaState::InGame), start_simulation);
    app.add_systems(OnExit(MetaState::InGame), stop_simulation);
    app.add_systems(
        OnEnter(Menu::None),
        start_simulation.run_if(in_state(MetaState::InGame)),
    );
    app.add_systems(
        OnExit(Menu::None),
        stop_simulation.run_if(in_state(MetaState::InGame)),
    );
    app.add_systems(OnEnter(Menu::GameOver), start_game_over_music);
}

fn enter_pause(mut next: ResMut<NextState<Menu>>) {
    next.set(Menu::Pause);
}

fn start_simulation(
    mut time: ResMut<Time<Physics>>,
    mut music: Query<&mut AudioSink, With<GameplayMusic>>,
) {
    time.unpause();
    for settings in &mut music {
        settings.play();
    }
}

fn stop_simulation(
    mut time: ResMut<Time<Physics>>,
    mut music: Query<&mut AudioSink, With<GameplayMusic>>,
) {
    time.pause();
    for settings in &mut music {
        settings.pause();
    }
}

fn start_game_over_music(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    audio_settings: Res<AudioSettings>,
) {
    commands.spawn((
        Name::new("Game Over Music"),
        DespawnOnExit(Menu::GameOver),
        music(level_assets.game_over.handle.clone(), &audio_settings),
    ));
}
