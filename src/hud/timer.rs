use crate::hud::panel::{PanelPosition, panel};
use crate::screen::Screen;
use crate::PausableSystems;
use bevy::prelude::*;
use bevy::time::Stopwatch;

const LABEL_COLOR: Color = Color::srgba(0.5, 0.5, 0.6, 0.8);
const TIME_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay(false)), spawn_timer_panel);
    app.add_systems(Update, update_timer.in_set(PausableSystems));
}

#[derive(Component)]
struct TimerText;

#[derive(Resource)]
pub struct SurvivalTimer(pub Stopwatch);

fn spawn_timer_panel(mut commands: Commands) {
    commands.insert_resource(SurvivalTimer(Stopwatch::new()));

    commands.spawn((
        panel("Timer", PanelPosition::BottomLeft),
        DespawnOnExit(Screen::Gameplay(false)),
        GlobalZIndex(10),
        children![timer_row()],
    ));
}

fn timer_row() -> impl Bundle {
    (
        Name::new("Timer Row"),
        Node {
            align_items: AlignItems::Center,
            column_gap: Val::Px(10.0),
            ..default()
        },
        children![timer_label(), timer_value()],
    )
}

fn timer_label() -> impl Bundle {
    (
        Name::new("Timer Label"),
        Text("TEMPS SURVECU".into()),
        TextFont::from_font_size(12.0),
        TextColor(LABEL_COLOR),
    )
}

fn timer_value() -> impl Bundle {
    (
        TimerText,
        Name::new("Timer Value"),
        Text("00:00".into()),
        TextFont::from_font_size(28.0),
        TextColor(TIME_COLOR),
    )
}

fn update_timer(
    time: Res<Time>,
    mut survival: ResMut<SurvivalTimer>,
    mut text: Single<&mut Text, With<TimerText>>,
) {
    survival.0.tick(time.delta());
    let secs = survival.0.elapsed_secs() as u32;
    let minutes = secs / 60;
    let seconds = secs % 60;
    text.0 = format!("{:02}:{:02}", minutes, seconds);
}
