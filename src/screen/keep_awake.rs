use bevy::prelude::*;
use keepawake::{Builder, KeepAwake};

use crate::MetaState;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(MetaState::InGame), acquire);
    app.add_systems(OnExit(MetaState::InGame), release);
}

// Windows ties the request to the calling thread, so it must stay on the main thread.
fn acquire(world: &mut World) {
    let awake = Builder::default()
        .display(true)
        .idle(true)
        .reason("Game in progress")
        .app_name("Colonie Terminus")
        .app_reverse_domain("colonie-terminus")
        .create();

    if let Ok(awake) = awake {
        world.insert_non_send(awake)
    };
}

fn release(world: &mut World) {
    world.remove_non_send::<KeepAwake>();
}
