use bevy::prelude::*;

mod animation;
pub mod level;
mod movement;
pub mod player;
mod tilemap;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        level::plugin,
        movement::plugin,
        player::plugin,
        tilemap::plugin,
    ));
}
