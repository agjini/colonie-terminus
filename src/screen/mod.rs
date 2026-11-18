mod gameplay;
#[cfg(not(target_arch = "wasm32"))] // pour tout sauf web, au build on charge keep_awake
mod keep_awake;
mod loading;
mod splash;
mod title;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_state::<Screen>();

    app.add_plugins((
        gameplay::plugin,
        #[cfg(not(target_arch = "wasm32"))]
        keep_awake::plugin,
        loading::plugin,
        splash::plugin,
        title::plugin,
    ));
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Screen {
    #[default]
    Splash,
    Title(bool),
    Gameplay(bool),
}
