use crate::{asset_tracking::LoadResource, audio::sound_effect};
use bevy::prelude::*;
use bevy_input_focus::InputFocus;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, apply_interaction_palette);
    app.load_resource::<InteractionAssets>();
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

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct InteractionAssets {
    #[dependency]
    hover: Handle<AudioSource>,
    #[dependency]
    click: Handle<AudioSource>,
}

impl FromWorld for InteractionAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            hover: assets.load("audio/sound_effects/button_hover.ogg"),
            click: assets.load("audio/sound_effects/button_click.ogg"),
        }
    }
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
        commands.spawn(sound_effect(interaction_assets.hover.clone()));
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
        commands.spawn(sound_effect(interaction_assets.click.clone()));
    }
}
