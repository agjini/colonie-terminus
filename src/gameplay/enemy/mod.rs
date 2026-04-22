use crate::gameplay::enemy::spawner::EnemyRoot;
use bevy::prelude::*;

pub mod asset;
mod damage;
pub mod movement;
mod spawner;

pub use damage::EnemyDeathEvent;
pub use damage::Hurt;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        asset::plugin,
        movement::plugin,
        spawner::plugin,
        damage::plugin,
    ));
}

pub fn enemy_root() -> impl Bundle {
    (EnemyRoot, Transform::default(), Visibility::default())
}
