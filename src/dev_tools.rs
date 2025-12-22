//! Development tools for the game. This plugin is only enabled in dev builds.

use avian2d::prelude::{PhysicsDebugPlugin, PhysicsGizmos};
use bevy::{
    dev_tools::states::log_transitions, input::common_conditions::input_just_pressed, prelude::*,
};

use crate::screen::Screen;

#[derive(Resource, Default)]
struct DebugState {
    enabled: bool,
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<DebugState>();

    app.add_plugins((PhysicsDebugPlugin,));

    app.insert_gizmo_config(
        PhysicsGizmos::default(),
        GizmoConfig {
            enabled: false,
            ..default()
        },
    );

    app.add_systems(Update, log_transitions::<Screen>);
    app.add_systems(Update, toggle_debug.run_if(input_just_pressed(TOGGLE_KEY)));
    app.add_systems(Update, apply_debug_state);
}

const TOGGLE_KEY: KeyCode = KeyCode::F2;

fn toggle_debug(mut debug_state: ResMut<DebugState>) {
    debug_state.enabled = !debug_state.enabled;
    info!(
        "Debug tools {}",
        if debug_state.enabled {
            "enabled"
        } else {
            "disabled"
        }
    );
}

fn apply_debug_state(
    debug_state: Res<DebugState>,
    mut ui_debug_options: ResMut<UiDebugOptions>,
    mut gizmo_config_store: ResMut<GizmoConfigStore>,
) {
    if !debug_state.is_changed() {
        return;
    }

    ui_debug_options.enabled = debug_state.enabled;

    let (config, _) = gizmo_config_store.config_mut::<PhysicsGizmos>();
    config.enabled = debug_state.enabled;
}
