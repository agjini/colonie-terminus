use std::borrow::Cow;

use crate::menu::MenuAssets;
use crate::theme::{interaction::InteractionPalette, navigation::Focusable, palette::*};
use bevy::input_focus::{InputFocus, InputFocusVisible};
use bevy::{
    ecs::{spawn::SpawnWith, system::IntoObserverSystem},
    prelude::*,
};

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
    )
}

pub fn header(assets: &MenuAssets, text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Header"),
        Text(text.into()),
        TextFont {
            font: assets.font.handle.clone(),
            font_size: assets.font_size_base + 10.,
            ..default()
        },
        TextColor(HEADER_TEXT.into()),
    )
}

pub fn label(font: Handle<Font>, font_size_base: f32, text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Label"),
        Text(text.into()),
        TextFont {
            font,
            font_size: font_size_base,
            ..default()
        },
        TextColor(LABEL_TEXT.into()),
    )
}

pub fn button<E, B, M, I>(assets: &MenuAssets, text: impl Into<String>, action: I) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let text = text.into();
    let width = text.len() as f32 * 12. + 20.;
    button_base(
        assets,
        text,
        action,
        (Node {
            width: px(width),
            height: px(40),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            border_radius: BorderRadius::MAX,
            ..default()
        },),
    )
}

pub fn button_small<E, B, M, I>(
    assets: &MenuAssets,
    text: impl Into<String>,
    action: I,
) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    button_base(
        assets,
        text.into(),
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
    assets: &MenuAssets,
    text: String,
    action: I,
    button_bundle: impl Bundle,
) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let action = IntoObserverSystem::into_system(action);
    let text_font = TextFont {
        font: assets.font.handle.clone(),
        font_size: assets.font_size_base,
        ..default()
    };
    (
        Name::new("Button"),
        Node {
            padding: UiRect::all(Val::Px(5.0)),
            border_radius: BorderRadius::new(Val::Px(0.), Val::Px(20.), Val::Px(0.), Val::Px(20.)),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(4.0),
            ..default()
        },
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            let mut entity = parent.spawn((
                Name::new(text.clone()),
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
                    text_font,
                    TextColor(BUTTON_TEXT.into()),
                )],
            ));
            entity.insert(button_bundle);
            entity.observe(action);
        })),
    )
}

pub fn highlight_focused_element(
    input_focus: Res<InputFocus>,
    input_focus_visible: Res<InputFocusVisible>,
    mut query: Query<(Entity, &InteractionPalette, &mut BackgroundColor)>,
) {
    for (entity, interaction, mut background_color) in query.iter_mut() {
        if input_focus.0 == Some(entity) && input_focus_visible.0 {
            background_color.0 = interaction.hovered;
        } else {
            background_color.0 = interaction.none;
        }
    }
}
