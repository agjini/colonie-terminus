use crate::menu::{Menu, MenuAssets, Nav};
use crate::theme::widget::*;
use crate::utils::escape_just_pressed;
use bevy::{ecs::spawn::SpawnIter, prelude::*};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Credits), spawn_credits_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Credits).and(escape_just_pressed)),
    );
}

fn spawn_credits_menu(mut commands: Commands, assets: Res<MenuAssets>) {
    commands.spawn((
        ui_root("Credits Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Credits),
        children![
            header(&assets, "Created by"),
            grid(
                assets.font.handle.clone(),
                assets.font_size_base,
                assets.created_by.clone()
            ),
            header(&assets, "Assets"),
            grid(
                assets.font.handle.clone(),
                assets.font_size_base,
                assets.assets_by.clone()
            ),
            button(&assets, "Back", go_back_on_click),
        ],
    ));
}

fn grid(font: Handle<Font>, font_size_base: f32, content: Vec<(String, String)>) -> impl Bundle {
    let children = Children::spawn(SpawnIter(content.into_iter().flat_map(
        move |(left, right)| {
            [
                (
                    label(font.clone(), font_size_base, left),
                    Node {
                        justify_self: JustifySelf::End,
                        ..default()
                    },
                ),
                (
                    label(font.clone(), font_size_base, right),
                    Node {
                        justify_self: JustifySelf::Start,
                        ..default()
                    },
                ),
            ]
        },
    )));
    (
        Name::new("Grid"),
        Node {
            display: Display::Grid,
            row_gap: px(10),
            column_gap: px(30),
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        children,
    )
}

fn go_back_on_click(_: On<Pointer<Click>>, mut commands: Commands) {
    commands.trigger(Nav::Back);
}

fn go_back(mut commands: Commands) {
    commands.trigger(Nav::Back);
}
