use bevy::prelude::*;

pub mod asset;

pub fn plugin(app: &mut App) {
    app.add_plugins((asset::plugin));
}
