use crate::screen::Screen;
use avian2d::prelude::{PhysicsDebugPlugin, PhysicsGizmos};
use bevy::color::palettes::tailwind::GREEN_500;
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig};
use bevy::text::FontSmoothing;
use bevy::{
    dev_tools::states::log_transitions, input::common_conditions::input_just_pressed, prelude::*,
};

#[derive(Resource, Default)]
struct DebugState {
    enabled: bool,
}

pub fn plugin(app: &mut App) {
    app.init_resource::<DebugState>();

    app.add_plugins((
        PhysicsDebugPlugin,
        FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: 32.0,
                    font: default(),
                    font_smoothing: FontSmoothing::default(),
                    ..default()
                },
                text_color: GREEN_500.into(),
                refresh_interval: core::time::Duration::from_millis(100),
                enabled: false,
                frame_time_graph_config: FrameTimeGraphConfig {
                    enabled: false,
                    min_fps: 30.0,
                    target_fps: 144.0,
                },
            },
        },
    ));

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
    mut overlay: ResMut<FpsOverlayConfig>,
) {
    if !debug_state.is_changed() {
        return;
    }

    ui_debug_options.enabled = debug_state.enabled;
    overlay.enabled = debug_state.enabled;
    overlay.frame_time_graph_config.enabled = debug_state.enabled;
    let (config, _) = gizmo_config_store.config_mut::<PhysicsGizmos>();
    config.enabled = debug_state.enabled;
}
