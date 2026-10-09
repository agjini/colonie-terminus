use super::DebugState;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy::reflect::serde::TypedReflectSerializer;
use itertools::Itertools;
use ron::ser::{PrettyConfig, to_string_pretty};

const LABEL_OFFSET: Vec3 = Vec3::new(0.0, 25.0, 50.0);

pub fn watch<T: Component + Reflect + TypePath>(app: &mut App) {
    app.add_observer(attach_lines::<T>);
    app.add_observer(detach_line::<T>);
    app.add_systems(
        Update,
        collect_line::<T>
            .run_if(debug_enabled)
            .before(render_labels),
    );
}

pub fn plugin(app: &mut App) {
    app.add_observer(spawn_label);
    app.add_systems(Update, render_labels);
}

#[derive(Component, Default)]
struct WatchLines(HashMap<&'static str, String>);

#[derive(Component)]
struct WatchLabel;

fn debug_enabled(debug_state: Res<DebugState>) -> bool {
    debug_state.enabled
}

fn attach_lines<T: Component>(add: On<Add, T>, mut commands: Commands) {
    commands
        .entity(add.entity)
        .insert_if_new(WatchLines::default());
}

fn detach_line<T: Component + TypePath>(remove: On<Remove, T>, mut lines: Query<&mut WatchLines>) {
    if let Ok(mut lines) = lines.get_mut(remove.entity) {
        lines.0.remove(T::short_type_path());
    }
}

fn spawn_label(add: On<Add, WatchLines>, mut commands: Commands) {
    commands.entity(add.entity).with_child((
        WatchLabel,
        Text2d::default(),
        TextFont {
            font_size: FontSize::Px(6.0),
            ..default()
        },
        Transform::from_translation(LABEL_OFFSET),
        Visibility::Hidden,
    ));
}

fn collect_line<T: Component + Reflect + TypePath>(
    registry: Res<AppTypeRegistry>,
    mut query: Query<(&T, &mut WatchLines)>,
) {
    let registry = registry.read();
    for (component, mut lines) in &mut query {
        let serializer = TypedReflectSerializer::new(component.as_partial_reflect(), &registry);
        let value = to_string_pretty(&serializer, inline_config())
            .unwrap_or_else(|error| error.to_string());
        lines.0.insert(
            T::short_type_path(),
            format!("{} {value}", T::short_type_path()),
        );
    }
}

fn inline_config() -> PrettyConfig {
    PrettyConfig::new()
        .compact_structs(false)
        .compact_arrays(true)
        .compact_maps(true)
}

fn render_labels(
    debug_state: Res<DebugState>,
    lines: Query<&WatchLines>,
    mut labels: Query<(&mut Text2d, &mut Visibility, &ChildOf), With<WatchLabel>>,
) {
    for (mut text, mut visibility, child_of) in &mut labels {
        visibility.set_if_neq(if debug_state.enabled {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        });
        if !debug_state.enabled {
            continue;
        }
        if let Ok(lines) = lines.get(child_of.parent()) {
            let content = lines.0.iter().sorted().map(|(_, line)| line).join("\n");
            if text.0 != content {
                text.0 = content;
            }
        }
    }
}
