use crate::gameplay::health::Health;
use crate::gameplay::player::{Player, Xp};
use crate::hud::panel::{PanelPosition, panel};
use crate::screen::Screen;
use crate::PausableSystems;
use bevy::prelude::*;

const HP_BAR_COLOR: Color = Color::srgb(0.85, 0.1, 0.1);
const XP_BAR_COLOR: Color = Color::srgb(0.7, 0.1, 0.85);
const BAR_BG_COLOR: Color = Color::srgba(0.2, 0.2, 0.25, 0.8);

const BORDER_WIDTH: Val = Val::Px(1.);
const BAR_WIDTH: f32 = 140.0;
const BAR_HEIGHT: f32 = 14.0;
const BAR_RADIUS: Val = Val::Px(2.0);

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay(false)), spawn_player_panel);
    app.add_systems(
        Update,
        (update_hp_bar, update_xp_bar).in_set(PausableSystems),
    );
}

#[derive(Component)]
struct HpBarFill;

#[derive(Component)]
struct HpText;

#[derive(Component)]
struct XpBarFill;

#[derive(Component)]
struct LevelText;

fn spawn_player_panel(mut commands: Commands) {
    commands.spawn((
        panel("Player Status", PanelPosition::TopLeft),
        DespawnOnExit(Screen::Gameplay(false)),
        GlobalZIndex(10),
        children![hp_row(), xp_row()],
    ));
}

fn hp_row() -> impl Bundle {
    (
        Name::new("HP Row"),
        Node {
            align_items: AlignItems::Center,
            column_gap: Val::Px(8.0),
            ..default()
        },
        children![hp_label(), hp_bar(), hp_text()],
    )
}

fn hp_label() -> impl Bundle {
    (
        Name::new("HP Label"),
        Text("HP".into()),
        TextFont::from_font_size(14.0),
        TextColor(HP_BAR_COLOR),
    )
}

fn hp_bar() -> impl Bundle {
    (
        Name::new("HP Bar"),
        Node {
            width: Val::Px(BAR_WIDTH),
            height: Val::Px(BAR_HEIGHT),
            border: UiRect::all(BORDER_WIDTH),
            border_radius: BorderRadius::all(BAR_RADIUS),
            ..default()
        },
        BackgroundColor(BAR_BG_COLOR),
        BorderColor::from(HP_BAR_COLOR),
        children![(
            HpBarFill,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                border_radius: BorderRadius::all(BAR_RADIUS),
                ..default()
            },
            BackgroundColor(HP_BAR_COLOR),
        )],
    )
}

fn hp_text() -> impl Bundle {
    (
        HpText,
        Name::new("HP Text"),
        Text("0 / 0".into()),
        TextFont::from_font_size(16.0),
        TextColor(HP_BAR_COLOR),
    )
}

fn xp_row() -> impl Bundle {
    (
        Name::new("XP Row"),
        Node {
            align_items: AlignItems::Center,
            column_gap: Val::Px(8.0),
            ..default()
        },
        children![xp_label(), xp_bar(), level_text()],
    )
}

fn xp_label() -> impl Bundle {
    (
        Name::new("XP Label"),
        Text("XP".into()),
        TextFont::from_font_size(14.0),
        TextColor(XP_BAR_COLOR),
    )
}

fn xp_bar() -> impl Bundle {
    (
        Name::new("XP Bar"),
        Node {
            width: Val::Px(BAR_WIDTH),
            height: Val::Px(BAR_HEIGHT),
            border: UiRect::all(BORDER_WIDTH),
            border_radius: BorderRadius::all(BAR_RADIUS),
            ..default()
        },
        BackgroundColor(BAR_BG_COLOR),
        BorderColor::from(XP_BAR_COLOR),
        children![(
            XpBarFill,
            Node {
                width: Val::Percent(0.0),
                height: Val::Percent(100.0),
                border_radius: BorderRadius::all(BAR_RADIUS),
                ..default()
            },
            BackgroundColor(XP_BAR_COLOR),
        )],
    )
}

fn level_text() -> impl Bundle {
    (
        LevelText,
        Name::new("Level Text"),
        Text("Lv. 01".into()),
        TextFont::from_font_size(16.0),
        TextColor(XP_BAR_COLOR),
    )
}

fn update_hp_bar(
    mut bar: Single<&mut Node, With<HpBarFill>>,
    mut text: Single<&mut Text, With<HpText>>,
    health: Single<&Health, With<Player>>,
) {
    let ratio = if health.max > 0.0 {
        (health.current / health.max).clamp(0.0, 1.0)
    } else {
        0.0
    };
    bar.width = Val::Percent(ratio * 100.0);
    text.0 = format!("{} / {}", health.current as u32, health.max as u32);
}

fn update_xp_bar(
    mut bar: Single<&mut Node, With<XpBarFill>>,
    mut text: Single<&mut Text, With<LevelText>>,
    xp: Single<&Xp, With<Player>>,
) {
    let ratio = if xp.next_level > 0.0 {
        (xp.current / xp.next_level).clamp(0.0, 1.0)
    } else {
        0.0
    };
    bar.width = Val::Percent(ratio * 100.0);
    text.0 = format!("Lv. {:02}", xp.level);
}
