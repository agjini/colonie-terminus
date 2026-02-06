use crate::menu::MenuAssets;
use crate::utils::escape_just_pressed;
use crate::{menu::Menu, theme::prelude::*};
use bevy::{ecs::spawn::SpawnIter, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Credits), spawn_credits_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Credits).and(escape_just_pressed)),
    );
}

fn spawn_credits_menu(mut commands: Commands, assets: Res<MenuAssets>) {
    commands.spawn((
        widget::ui_root("Credits Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Credits),
        children![
            widget::header("Created by"),
            grid(assets.created_by.clone()),
            widget::header("Assets"),
            grid(assets.assets_by.clone()),
            widget::button("Back", go_back_on_click),
        ],
    ));
}

fn grid(content: Vec<(String, String)>) -> impl Bundle {
    (
        Name::new("Grid"),
        Node {
            display: Display::Grid,
            row_gap: px(10),
            column_gap: px(30),
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        Children::spawn(SpawnIter(content.into_iter().flat_map(|(left, right)| {
            [
                (
                    widget::label(left),
                    Node {
                        justify_self: JustifySelf::End,
                        ..default()
                    },
                ),
                (
                    widget::label(right),
                    Node {
                        justify_self: JustifySelf::Start,
                        ..default()
                    },
                ),
            ]
        }))),
    )
}

fn go_back_on_click(_: On<Pointer<Click>>, next_menu: ResMut<NextState<Menu>>) {
    go_back(next_menu);
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}
