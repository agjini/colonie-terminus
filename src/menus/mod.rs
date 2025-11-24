//! The game's menus and transitions between them.

mod credits;
mod main;
mod pause;
mod settings;

use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Menu>();

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

fn setup_menu<Marker>(
    app: &mut App,
    menu_state: Menu,
    spawn_fn: impl IntoSystem<(), (), Marker> + Send + Sync + 'static,
) {
    app.add_systems(OnEnter(menu_state), spawn_fn);
    app.add_systems(
        Update,
        (move |mut next_menu: ResMut<NextState<Menu>>| {
            next_menu.set(Menu::None);
        })
        .run_if(in_state(menu_state).and(input_just_pressed(KeyCode::Escape))),
    );
}
