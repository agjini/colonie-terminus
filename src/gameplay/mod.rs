use bevy::prelude::*;

mod animation;
mod enemy;
mod layer;
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
        enemy::plugin,
        tilemap::plugin,
        layer::plugin,
    ));
}
