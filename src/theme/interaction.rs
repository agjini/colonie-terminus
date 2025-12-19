use crate::{asset_tracking::LoadResource, audio::sound_effect};
use bevy::input_focus::InputFocus;
use bevy::prelude::*;
use ron_asset_manager::Shandle;
use ron_asset_manager::prelude::RonAsset;
use serde::Deserialize;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, apply_interaction_palette);
    app.load_resource::<InteractionAssets>("interaction.ron");
    app.add_systems(Update, play_on_focus_sound_effect);
    app.add_observer(play_on_click_sound_effect);
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionPalette {
    pub none: Color,
    pub hovered: Color,
    pub pressed: Color,
}

fn apply_interaction_palette(
    mut palette_query: Query<
        (&Interaction, &InteractionPalette, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    for (interaction, palette, mut background) in &mut palette_query {
        *background = match interaction {
            Interaction::None => palette.none,
            Interaction::Hovered => palette.hovered,
            Interaction::Pressed => palette.pressed,
        }
        .into();
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
    res: Res<InputFocus>,
    mut commands: Commands,
    interaction_assets: Option<Res<InteractionAssets>>,
    interaction_query: Query<(), With<Interaction>>,
) {
    if !res.is_changed() {
        return;
    }

    let Some(input_focus) = res.0 else {
        return;
    };

    let Some(interaction_assets) = interaction_assets else {
        return;
    };

    if interaction_query.contains(input_focus.entity()) {
        commands.spawn(sound_effect(interaction_assets.hover.handle.clone()));
    }
}

fn play_on_click_sound_effect(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    interaction_assets: Option<Res<InteractionAssets>>,
    interaction_query: Query<(), With<Interaction>>,
) {
    let Some(interaction_assets) = interaction_assets else {
        return;
    };

    if interaction_query.contains(trigger.entity) {
        commands.spawn(sound_effect(interaction_assets.click.handle.clone()));
    }
}
