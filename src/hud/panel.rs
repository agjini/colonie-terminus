use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::srgba(0.05, 0.05, 0.12, 0.85);

pub fn panel(name: impl Into<String>, position: PanelPosition) -> impl Bundle {
    let (top, right, bottom, left) = match position {
        PanelPosition::TopLeft => (Val::Px(16.0), Val::Auto, Val::Auto, Val::Px(16.0)),
        PanelPosition::TopRight => (Val::Px(16.0), Val::Px(16.0), Val::Auto, Val::Auto),
        PanelPosition::BottomLeft => (Val::Auto, Val::Auto, Val::Px(16.0), Val::Px(16.0)),
        PanelPosition::BottomRight => (Val::Auto, Val::Px(16.0), Val::Px(16.0), Val::Auto),
    };

    (
        Name::new(name.into()),
        Node {
            position_type: PositionType::Absolute,
            top,
            right,
            bottom,
            left,
            padding: UiRect::all(Val::Px(12.0)),
            border_radius: BorderRadius::new(Val::Px(0.), Val::Px(20.), Val::Px(0.), Val::Px(20.)),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            ..default()
        },
        BackgroundColor(BACKGROUND_COLOR),
    )
}

pub enum PanelPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}
