use crate::hud::panel::{PanelPosition, panel};
use crate::screen::Screen;
use crate::PausableSystems;
use bevy::prelude::*;

const LABEL_COLOR: Color = Color::srgba(0.5, 0.5, 0.6, 0.8);
const COUNT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const ICON_SIZE: f32 = 20.0;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay(false)), spawn_elimination_panel);
    app.add_systems(Update, update_elimination.in_set(PausableSystems));
}

#[derive(Component)]
struct EliminationText;

#[derive(Resource, Default)]
pub struct EliminationCount(pub u32);

fn spawn_elimination_panel(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(EliminationCount::default());

    let skull = asset_server.load("images/hud/skull.png");
    commands.spawn((
        panel("Elimination", PanelPosition::BottomRight),
        DespawnOnExit(Screen::Gameplay(false)),
        GlobalZIndex(10),
        children![elimination_row(skull)],
    ));
}

fn elimination_row(skull: Handle<Image>) -> impl Bundle {
    (
        Name::new("Elimination Row"),
        Node {
            align_items: AlignItems::Center,
            column_gap: Val::Px(10.0),
            ..default()
        },
        children![elim_label(), elim_icon(skull), elim_count()],
    )
}

fn elim_label() -> impl Bundle {
    (
        Name::new("Elim Label"),
        Text("ELIMINATIONS".into()),
        TextFont::from_font_size(12.0),
        TextColor(LABEL_COLOR),
    )
}

fn elim_icon(skull: Handle<Image>) -> impl Bundle {
    (
        Name::new("Elim Icon"),
        ImageNode {
            image: skull,
            color: LABEL_COLOR,
            ..default()
        },
        Node {
            width: Val::Px(ICON_SIZE),
            height: Val::Px(ICON_SIZE),
            ..default()
        },
    )
}

fn elim_count() -> impl Bundle {
    (
        EliminationText,
        Name::new("Elim Count"),
        Text("0".into()),
        TextFont::from_font_size(28.0),
        TextColor(COUNT_COLOR),
    )
}

fn update_elimination(
    count: Res<EliminationCount>,
    mut text: Single<&mut Text, With<EliminationText>>,
) {
    text.0 = format!("{}", count.0);
}
