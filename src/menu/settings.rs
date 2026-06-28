use crate::audio::{
    AudioSettings, lower_music_volume, lower_sfx_volume, raise_music_volume, raise_sfx_volume,
};
use crate::menu::{Menu, MenuAssets, Nav};
use crate::theme::widget;
use crate::utils::escape_just_pressed;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Settings), spawn_settings_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Settings).and_then(escape_just_pressed)),
    );

    app.add_systems(
        Update,
        (update_music_volume_label, update_sfx_volume_label).run_if(in_state(Menu::Settings)),
    );
}

fn spawn_settings_menu(assets: Res<MenuAssets>, mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Settings Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Settings),
        children![
            widget::header(&assets, "Settings"),
            settings_grid(&assets),
            widget::button(&assets, "Back", go_back_on_click),
        ],
    ));
}

fn settings_grid(assets: &Res<MenuAssets>) -> impl Bundle {
    (
        Name::new("Settings Grid"),
        Node {
            display: Display::Grid,
            row_gap: px(10),
            column_gap: px(30),
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        children![
            (
                widget::label(
                    assets.font.handle.clone(),
                    assets.font_size_base,
                    "Music Volume"
                ),
                Node {
                    justify_self: JustifySelf::End,
                    ..default()
                }
            ),
            music_volume_widget(assets),
            (
                widget::label(
                    assets.font.handle.clone(),
                    assets.font_size_base,
                    "Sfx Volume"
                ),
                Node {
                    justify_self: JustifySelf::End,
                    ..default()
                }
            ),
            sfx_volume_widget(assets),
        ],
    )
}

fn music_volume_widget(assets: &Res<MenuAssets>) -> impl Bundle {
    (
        Name::new("Music Volume Widget"),
        Node {
            justify_self: JustifySelf::Start,
            ..default()
        },
        children![
            widget::button_small(assets, "-", lower_music_volume),
            (
                Name::new("Current Volume"),
                Node {
                    padding: UiRect::horizontal(px(10)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![(
                    widget::label(assets.font.handle.clone(), assets.font_size_base, ""),
                    MusicVolumeLabel
                )],
            ),
            widget::button_small(assets, "+", raise_music_volume),
        ],
    )
}

fn sfx_volume_widget(assets: &Res<MenuAssets>) -> impl Bundle {
    (
        Name::new("Sfx Volume Widget"),
        Node {
            justify_self: JustifySelf::Start,
            ..default()
        },
        children![
            widget::button_small(assets, "-", lower_sfx_volume),
            (
                Name::new("Current Sfx Volume"),
                Node {
                    padding: UiRect::horizontal(px(10)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![(
                    widget::label(assets.font.handle.clone(), assets.font_size_base, ""),
                    SfxVolumeLabel
                )],
            ),
            widget::button_small(assets, "+", raise_sfx_volume),
        ],
    )
}

#[derive(Component, Reflect)]
struct MusicVolumeLabel;

#[derive(Component, Reflect)]
struct SfxVolumeLabel;

fn update_music_volume_label(
    mut label: Single<&mut Text, With<MusicVolumeLabel>>,
    audio_settings: Res<AudioSettings>,
) {
    let percent = audio_settings.music_volume * 100.0;
    let text = format!("{}%", percent.round());
    label.0 = text;
}

fn update_sfx_volume_label(
    mut label: Single<&mut Text, With<SfxVolumeLabel>>,
    audio_settings: Res<AudioSettings>,
) {
    let percent = audio_settings.sound_fx_volume * 100.0;
    let text = format!("{}%", percent.round());
    label.0 = text;
}

fn go_back_on_click(_: On<Pointer<Click>>, mut commands: Commands) {
    commands.trigger(Nav::Back);
}

fn go_back(mut commands: Commands) {
    commands.trigger(Nav::Back);
}
