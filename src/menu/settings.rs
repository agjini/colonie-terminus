use crate::audio::{
    CONVERTER, lower_music_volume, lower_sfx_volume, raise_music_volume, raise_sfx_volume,
};
use crate::theme::widget;
use crate::utils::escape_just_pressed;
use crate::{MetaState, menu::Menu};
use bevy::prelude::*;
use bevy_seedling::prelude::{MusicPool, SamplerPool, SoundEffectsBus, VolumeNode};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Settings), spawn_settings_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Settings).and(escape_just_pressed)),
    );

    app.add_systems(
        Update,
        (update_music_volume_label, update_sfx_volume_label).run_if(in_state(Menu::Settings)),
    );
}

fn spawn_settings_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Settings Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Settings),
        children![
            widget::header("Settings"),
            settings_grid(),
            widget::button("Back", go_back_on_click),
        ],
    ));
}

fn settings_grid() -> impl Bundle {
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
                widget::label("Music Volume"),
                Node {
                    justify_self: JustifySelf::End,
                    ..default()
                }
            ),
            music_volume_widget(),
            (
                widget::label("Sfx Volume"),
                Node {
                    justify_self: JustifySelf::End,
                    ..default()
                }
            ),
            sfx_volume_widget(),
        ],
    )
}

fn music_volume_widget() -> impl Bundle {
    (
        Name::new("Music Volume Widget"),
        Node {
            justify_self: JustifySelf::Start,
            ..default()
        },
        children![
            widget::button_small("-", lower_music_volume),
            (
                Name::new("Current Volume"),
                Node {
                    padding: UiRect::horizontal(px(10)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![(widget::label(""), MusicVolumeLabel)],
            ),
            widget::button_small("+", raise_music_volume),
        ],
    )
}

fn sfx_volume_widget() -> impl Bundle {
    (
        Name::new("Sfx Volume Widget"),
        Node {
            justify_self: JustifySelf::Start,
            ..default()
        },
        children![
            widget::button_small("-", lower_sfx_volume),
            (
                Name::new("Current Sfx Volume"),
                Node {
                    padding: UiRect::horizontal(px(10)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![(widget::label(""), SfxVolumeLabel)],
            ),
            widget::button_small("+", raise_sfx_volume),
        ],
    )
}

#[derive(Component, Reflect)]
struct MusicVolumeLabel;

#[derive(Component, Reflect)]
struct SfxVolumeLabel;

fn update_music_volume_label(
    mut label: Single<&mut Text, With<MusicVolumeLabel>>,
    music: Single<&VolumeNode, (With<SamplerPool<MusicPool>>, Changed<VolumeNode>)>,
) {
    let percent = CONVERTER.volume_to_perceptual(music.volume) * 100.0;
    let text = format!("{}%", percent.round());
    label.0 = text;
}

fn update_sfx_volume_label(
    mut label: Single<&mut Text, With<SfxVolumeLabel>>,
    sfx: Single<&VolumeNode, (With<SoundEffectsBus>, Changed<VolumeNode>)>,
) {
    let percent = CONVERTER.volume_to_perceptual(sfx.volume) * 100.0;
    let text = format!("{}%", percent.round());
    label.0 = text;
}

fn go_back_on_click(
    _: On<Pointer<Click>>,
    screen: Res<State<MetaState>>,
    mut next_menu: ResMut<NextState<Menu>>,
) {
    next_menu.set(if screen.get() == &MetaState::InMenu {
        Menu::Main
    } else {
        Menu::Pause
    });
}

fn go_back(screen: Res<State<MetaState>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(if screen.get() == &MetaState::InMenu {
        Menu::Main
    } else {
        Menu::Pause
    });
}
