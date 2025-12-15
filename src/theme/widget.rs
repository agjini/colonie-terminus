//! Helper functions for creating common widgets.

use std::borrow::Cow;

use crate::theme::{interaction::InteractionPalette, keyboard_navigation::Focusable, palette::*};
use bevy::{
    ecs::{spawn::SpawnWith, system::IntoObserverSystem},
    prelude::*,
};
use bevy_input_focus::{InputFocus, InputFocusVisible};

pub fn ui_root(name: impl Into<Cow<'static, str>>) -> impl Bundle {
    (
        Name::new(name),
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(20),
            ..default()
        },
        // Don't block picking events for other UI roots.
        Pickable::IGNORE,
    )
}

pub fn header(text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Header"),
        Text(text.into()),
        TextFont::from_font_size(40.0),
        TextColor(HEADER_TEXT.into()),
    )
}

pub fn label(text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Label"),
        Text(text.into()),
        TextFont::from_font_size(24.0),
        TextColor(LABEL_TEXT.into()),
    )
}

pub fn button<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    button_base(
        text,
        action,
        (
            Node {
                width: px(380),
                height: px(80),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BorderRadius::MAX,
        ),
    )
}

pub fn button_small<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    button_base(
        text,
        action,
        Node {
            width: px(30),
            height: px(30),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
    )
}

fn button_base<E, B, M, I>(
    text: impl Into<String>,
    action: I,
    button_bundle: impl Bundle,
) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let text = text.into();
    let action = IntoObserverSystem::into_system(action);
    (
        Name::new("Button"),
        Node::default(),
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            let mut entity = parent.spawn((
                Name::new("Button Inner"),
                Button,
                BackgroundColor(BUTTON_BACKGROUND.into()),
                InteractionPalette {
                    none: BUTTON_BACKGROUND.into(),
                    hovered: BUTTON_HOVERED_BACKGROUND.into(),
                    pressed: BUTTON_PRESSED_BACKGROUND.into(),
                },
                Focusable,
                children![(
                    Name::new("Button Text"),
                    Text(text),
                    TextFont::from_font_size(40.0),
                    TextColor(BUTTON_TEXT.into()),
                    // Don't bubble picking events from the text up to the button.
                    Pickable::IGNORE,
                )],
            ));
            entity.insert(button_bundle);
            entity.observe(action);
        })),
    )
}

pub fn highlight_focused_element(
    input_focus: Res<InputFocus>,
    // While this isn't strictly needed for the example,
    // we're demonstrating how to be a good citizen by respecting the `InputFocusVisible` resource.
    input_focus_visible: Res<InputFocusVisible>,
    mut query: Query<(Entity, &InteractionPalette, &mut BackgroundColor)>,
) {
    for (entity, interaction, mut background_color) in query.iter_mut() {
        if input_focus.0 == Some(entity) && input_focus_visible.0 {
            // Don't change the background size / radius here,
            // as it would result in wiggling buttons when they are focused
            background_color.0 = interaction.hovered;
        } else {
            background_color.0 = interaction.none;
        }
    }
}
