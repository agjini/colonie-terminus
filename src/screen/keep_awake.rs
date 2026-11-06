use bevy::prelude::*;
use keepawake::{Builder, KeepAwake};

use crate::MetaState;
use crate::menu::Menu;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(MetaState::InGame), acquire);
    app.add_systems(OnExit(MetaState::InGame), release);
    app.add_systems(
        OnEnter(Menu::None),
        acquire.run_if(in_state(MetaState::InGame)),
    );
    app.add_systems(
        OnExit(Menu::None),
        release.run_if(in_state(MetaState::InGame)),
    );
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

    match awake {
        Ok(awake) => world.insert_non_send(awake),
        Err(error) => warn!("Could not prevent screen sleep: {error}"),
    }
}

fn release(world: &mut World) {
    world.remove_non_send::<KeepAwake>();
}
