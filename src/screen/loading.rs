use bevy::prelude::*;

use crate::screen::Screen::{Gameplay, Title};
use crate::{MetaState, asset_tracking::ResourceHandles, screen::Screen, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(MetaState::Loading), spawn_loading_screen);

    app.add_systems(
        Update,
        enter_next_screen.run_if(in_state(MetaState::Loading).and(all_assets_loaded)),
    );
}

fn spawn_loading_screen(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Loading Screen"),
        DespawnOnExit(MetaState::Loading),
        children![widget::label("Loading...")],
    ));
}

fn enter_next_screen(screen: Res<State<Screen>>, mut next_screen: ResMut<NextState<Screen>>) {
    let next = match &screen.get() {
        Title(_) => Title(false),
        Gameplay(_) => Gameplay(false),
        _ => Title(true),
    };

    next_screen.set(next);
}

fn all_assets_loaded(resource_handles: Res<ResourceHandles>) -> bool {
    resource_handles.is_all_done()
}
