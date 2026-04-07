use crate::gameplay::player::Xp;
use crate::screen::Screen;
use bevy::color::palettes::tailwind::BLUE_300;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay(false)), spawn_xp_bar);
    app.add_systems(Update, update_xp_bar);
}

#[derive(Component)]
struct XpBar;

fn spawn_xp_bar(mut commands: Commands) {
    commands.spawn((
        Name::new("XP Bar"),
        DespawnOnExit(Screen::Gameplay(false)),
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(20.0),
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            ..default()
        },
        BackgroundColor(Color::WHITE),
        GlobalZIndex(10),
        children![(
            XpBar,
            Node {
                width: Val::Percent(0.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(BLUE_300.into()),
        )],
    ));
}

fn update_xp_bar(
    mut xp_bar: Single<(&mut Node, &mut BackgroundColor), With<XpBar>>,
    xp: Single<&Xp>,
) {
    let ratio = if xp.next_level > 0.0 {
        (xp.current / xp.next_level).clamp(0.0, 1.0)
    } else {
        0.0
    };
    xp_bar.0.width = Val::Percent(ratio * 100.0);
}
