use bevy::app::App;
use bevy::prelude::*;

mod asset;
mod xp;

pub use xp::XpAmount;

#[derive(Component, Reflect, Default)]
struct LootRoot;

pub fn plugin(app: &mut App) {
    app.add_plugins((asset::plugin, xp::plugin));
}

pub fn loot_root() -> impl Bundle {
    (LootRoot, Transform::default(), Visibility::default())
}
