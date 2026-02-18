use crate::gameplay::enemy::spawner::EnemyRoot;
use bevy::prelude::*;

pub mod asset;
pub mod movement;
mod spawner;

pub fn plugin(app: &mut App) {
    app.add_plugins((asset::plugin, movement::plugin, spawner::plugin));
}

pub fn enemy_root() -> impl Bundle {
    (EnemyRoot, Transform::default(), Visibility::default())
}
