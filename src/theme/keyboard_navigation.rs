use bevy::math::CompassOctant;
use bevy::prelude::*;
use bevy_input_focus::directional_navigation::{
    DirectionalNavigationMap, DirectionalNavigationPlugin,
};
use bevy_input_focus::{InputDispatchPlugin, InputFocus, InputFocusVisible};

use crate::theme::interaction::InteractionPalette;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((InputDispatchPlugin, DirectionalNavigationPlugin));
    app.insert_resource(InputFocusVisible(true));
    app.add_systems(Update, setup_navigation_for_new_buttons);
    app.add_systems(
        PreUpdate,
        (handle_keyboard_navigation, handle_keyboard_activation).chain(),
    );
    app.add_systems(Update, apply_focus_visual);
}

/// Component to mark buttons that can be focused with keyboard navigation
#[derive(Component)]
pub struct Focusable;

fn setup_navigation_for_new_buttons(
    new_buttons: Query<Entity, (Added<Focusable>, With<Button>)>,
    all_buttons: Query<Entity, (With<Focusable>, With<Button>)>,
    mut nav_map: ResMut<DirectionalNavigationMap>,
    mut input_focus: ResMut<InputFocus>,
) {
    if new_buttons.is_empty() {
        return;
    }

    // Collect all focusable buttons
    let buttons: Vec<Entity> = all_buttons.iter().collect();

    if buttons.is_empty() {
        return;
    }

    // Clear previous navigation map
    nav_map.clear();

    // Create vertical looping navigation (North/South)
    nav_map.add_looping_edges(&buttons, CompassOctant::South);

    // Always set focus on first button when new buttons are added
    input_focus.set(buttons[0]);
}

fn handle_keyboard_navigation(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut directional_navigation: bevy_input_focus::directional_navigation::DirectionalNavigation,
) {
    let direction = if keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::KeyW)
    {
        Some(CompassOctant::North)
    } else if keyboard.just_pressed(KeyCode::ArrowDown) || keyboard.just_pressed(KeyCode::KeyS) {
        Some(CompassOctant::South)
    } else {
        None
    };

    if let Some(direction) = direction {
        let _ = directional_navigation.navigate(direction);
    }
}

fn handle_keyboard_activation(
    keyboard: Res<ButtonInput<KeyCode>>,
    input_focus: Res<InputFocus>,
    mut commands: Commands,
    buttons: Query<(), With<Button>>,
    windows: Query<Entity, With<Window>>,
) {
    if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
        if let Some(focused_entity) = input_focus.0 {
            if buttons.contains(focused_entity) {
                // Get the primary window
                let Some(window_entity) = windows.iter().next() else {
                    warn!("No window found, cannot activate button");
                    return;
                };

                // Normalize the window reference
                let Some(normalized_window) = bevy::window::WindowRef::Primary.normalize(Some(window_entity)) else {
                    warn!("Failed to normalize window reference");
                    return;
                };

                // Trigger a Pointer<Click> event
                use bevy::picking::events::{Click, Pointer};
                use bevy::picking::pointer::{PointerId, PointerButton};
                use bevy::picking::backend::HitData;

                commands.entity(focused_entity).trigger(|entity| {
                    Pointer::new(
                        PointerId::Mouse,
                        bevy::picking::pointer::Location {
                            target: normalized_window.into(),
                            position: Vec2::ZERO,
                        },
                        Click {
                            button: PointerButton::Primary,
                            hit: HitData {
                                camera: Entity::PLACEHOLDER,
                                depth: 0.0,
                                position: None,
                                normal: None,
                            },
                            duration: std::time::Duration::from_millis(0),
                        },
                        entity,
                    )
                });
            }
        }
    }
}

fn apply_focus_visual(
    input_focus: Res<InputFocus>,
    input_focus_visible: Res<InputFocusVisible>,
    mut query: Query<(
        Entity,
        &InteractionPalette,
        &Interaction,
        &mut BackgroundColor,
    ), With<Button>>,
) {
    for (entity, palette, interaction, mut background) in query.iter_mut() {
        if input_focus.0 == Some(entity) && input_focus_visible.0 && *interaction == Interaction::None
        {
            // Focused button uses hovered style
            *background = palette.hovered.into();
        } else {
            // Non-focused buttons use normal interaction palette
            *background = match interaction {
                Interaction::None => palette.none,
                Interaction::Hovered => palette.hovered,
                Interaction::Pressed => palette.pressed,
            }
            .into();
        }
    }
}
