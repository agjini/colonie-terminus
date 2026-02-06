mod credits;
mod main;
mod pause;
mod settings;

use crate::asset_tracking::LoadResource;
use bevy::prelude::*;
use bevy_seedling::prelude::AudioSample;
use ron_asset_manager::Shandle;
use ron_asset_manager::prelude::RonAsset;
use serde::Deserialize;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Menu>();

    app.load_resource::<MenuAssets>("menu.ron");

    app.add_plugins((
        credits::plugin,
        main::plugin,
        settings::plugin,
        pause::plugin,
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
}

#[derive(Resource, TypePath, Asset, RonAsset, Deserialize, Debug, Clone)]
struct MenuAssets {
    #[asset]
    music: Shandle<AudioSample>,
    created_by: Vec<(String, String)>,
    assets_by: Vec<(String, String)>,
}
