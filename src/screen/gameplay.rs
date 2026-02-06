use bevy::prelude::*;

use crate::screen::Screen::Gameplay;
use crate::utils::escape_just_pressed;
use crate::{Pause, gameplay::level::spawn_level, menu::Menu, screen::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Gameplay(false)), spawn_level);
    app.add_computed_state::<InGame>();
    app.add_systems(
        Update,
        (
            (pause, spawn_pause_overlay, open_pause_menu).run_if(
                in_state(Gameplay(false))
                    .and(in_state(Menu::None))
                    .and(escape_just_pressed),
            ),
            close_menu.run_if(
                in_state(Gameplay(false))
                    .and(not(in_state(Menu::None)))
                    .and(escape_just_pressed),
            ),
        ),
    );
    app.add_systems(OnExit(Gameplay(false)), (close_menu, unpause));
    app.add_systems(
        OnEnter(Menu::None),
        unpause.run_if(in_state(Gameplay(false))),
    );
}

fn unpause(mut next_pause: ResMut<NextState<Pause>>) {
    next_pause.set(Pause(false));
}

fn pause(mut next_pause: ResMut<NextState<Pause>>) {
    next_pause.set(Pause(true));
}

fn spawn_pause_overlay(mut commands: Commands) {
    commands.spawn((
        Name::new("Pause Overlay"),
        Node {
            width: percent(100),
            height: percent(100),
            ..default()
        },
        GlobalZIndex(1),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        DespawnOnExit(Pause(true)),
    ));
}

fn open_pause_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pause);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct InGame;

impl ComputedStates for InGame {
    type SourceStates = Screen;

    fn compute(screen: Screen) -> Option<Self> {
        match screen {
            Gameplay(loading) => {
                if loading {
                    Some(InGame)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
