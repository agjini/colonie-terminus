use crate::audio::AudioSettings;
use crate::{asset_tracking::LoadResource, audio::sound_fx};
use bevy::input_focus::{InputFocus, InputFocusVisible};
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::ui::Pressed;
use bevy::ui_widgets::Activate;
use bevy::ui_widgets::Button;
use ron_asset_manager::Shandle;
use ron_asset_manager::prelude::RonAsset;
use serde::Deserialize;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, apply_interaction_palette);
    app.load_resource::<InteractionAssets>("interaction.ron");
    app.add_systems(Update, play_on_focus_sound_effect);
    app.add_observer(play_on_click_sound_effect);
}

#[derive(Component, Debug, Reflect)]
pub struct InteractionPalette {
    pub none: Color,
    pub hovered: Color,
    pub pressed: Color,
}

fn apply_interaction_palette(
    input_focus: Res<InputFocus>,
    input_focus_visible: Res<InputFocusVisible>,
    mut palette_query: Query<(
        Entity,
        &Hovered,
        Has<Pressed>,
        &InteractionPalette,
        &mut BackgroundColor,
    )>,
) {
    for (entity, hovered, pressed, palette, mut background) in &mut palette_query {
        let focused = input_focus_visible.0 && input_focus.get() == Some(entity);
        background.0 = if pressed {
            palette.pressed
        } else if hovered.get() || focused {
            palette.hovered
        } else {
            palette.none
        };
    }
}

#[derive(Resource, Asset, TypePath, RonAsset, Deserialize, Debug, Clone)]
struct InteractionAssets {
    #[asset]
    hover: Shandle<AudioSource>,
    #[asset]
    click: Shandle<AudioSource>,
}

fn play_on_focus_sound_effect(
    input_focus: Res<InputFocus>,
    mut commands: Commands,
    interaction_assets: Option<Res<InteractionAssets>>,
    interaction_query: Query<(), With<Button>>,
    audio_settings: Res<AudioSettings>,
) {
    if !input_focus.is_changed() {
        return;
    }

    let Some(focused_entity) = input_focus.get() else {
        return;
    };

    let Some(interaction_assets) = interaction_assets else {
        return;
    };

    if interaction_query.contains(focused_entity) {
        commands.spawn(sound_fx(
            interaction_assets.hover.handle.clone(),
            &audio_settings,
        ));
    }
}

fn play_on_click_sound_effect(
    trigger: On<Activate>,
    mut commands: Commands,
    interaction_assets: Option<Res<InteractionAssets>>,
    interaction_query: Query<(), With<Button>>,
    audio_settings: Res<AudioSettings>,
) {
    let Some(interaction_assets) = interaction_assets else {
        return;
    };

    if interaction_query.contains(trigger.entity) {
        commands.spawn(sound_fx(
            interaction_assets.click.handle.clone(),
            &audio_settings,
        ));
    }
}
