use crate::hud::panel::{PanelPosition, panel};
use crate::screen::Screen;
use bevy::prelude::*;

const SLOT_SIZE: f32 = 56.0;
const SLOT_BG: Color = Color::srgba(0.15, 0.15, 0.2, 0.6);
const SLOT_BORDER_COLOR: Color = Color::srgba(0.3, 0.3, 0.4, 0.5);
const SLOT_BORDER: Val = Val::Px(1.5);
const INDEX_COLOR: Color = Color::srgba(0.5, 0.5, 0.6, 0.8);
const LOCK_COLOR: Color = Color::srgba(0.5, 0.5, 0.6, 0.6);
const LOCK_ICON_SIZE: f32 = 24.0;
const SLOT_COUNT: usize = 5;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay(false)), spawn_weapon_panel);
}

fn spawn_weapon_panel(mut commands: Commands, asset_server: Res<AssetServer>) {
    let lock = asset_server.load("images/hud/lock.png");

    commands.spawn((
        panel("Weapon", PanelPosition::TopRight),
        DespawnOnExit(Screen::Gameplay(false)),
        GlobalZIndex(10),
        children![weapon_slot_row(lock)],
    ));
}

fn weapon_slot_row(lock: Handle<Image>) -> impl Bundle {
    (
        Name::new("Weapon Slot Row"),
        Node {
            column_gap: Val::Px(6.0),
            ..default()
        },
        Children::spawn(SpawnIter(
            (0..SLOT_COUNT).map(move |i| weapon_slot(i, lock.clone())),
        )),
    )
}

fn weapon_slot(index: usize, lock: Handle<Image>) -> impl Bundle {
    (
        Name::new(format!("Slot {}", index + 1)),
        Node {
            width: Val::Px(SLOT_SIZE),
            height: Val::Px(SLOT_SIZE),
            border: UiRect::all(SLOT_BORDER),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(SLOT_BG),
        BorderColor::from(SLOT_BORDER_COLOR),
        children![lock_icon(lock), slot_index(index)],
    )
}

fn lock_icon(lock: Handle<Image>) -> impl Bundle {
    (
        Name::new("Lock"),
        ImageNode {
            image: lock,
            color: LOCK_COLOR,
            ..default()
        },
        Node {
            width: Val::Px(LOCK_ICON_SIZE),
            height: Val::Px(LOCK_ICON_SIZE),
            ..default()
        },
    )
}

fn slot_index(index: usize) -> impl Bundle {
    (
        Name::new("Index"),
        Text(format!("{}", index + 1)),
        TextFont::from_font_size(11.0),
        TextColor(INDEX_COLOR),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(2.0),
            right: Val::Px(4.0),
            ..default()
        },
    )
}
