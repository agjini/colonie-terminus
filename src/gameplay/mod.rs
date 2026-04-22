use bevy::prelude::*;

mod animation;
mod enemy;
pub mod health;
mod layer;
pub mod level;
mod loot;
mod movement;
pub mod player;
mod tilemap;

pub use layer::GameLayer;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        level::plugin,
        movement::plugin,
        player::plugin,
        enemy::plugin,
        tilemap::plugin,
        health::plugin,
        layer::plugin,
        loot::plugin,
    ));
}
