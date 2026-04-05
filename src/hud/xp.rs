use crate::gameplay::player::Xp;
use crate::hud::{ProgressBar, progress_bar};
use bevy::color::palettes::tailwind::BLUE_300;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, update_xp_bar);
}

#[derive(Component)]
struct XpBar;

pub fn xp_bar(
    owner: Entity,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) -> impl Bundle {
    (
        progress_bar(
            owner,
            meshes,
            materials,
            "XP Bar",
            1000.0,
            20.0,
            Color::WHITE,
            BLUE_300.into(),
        ),
        XpBar,
    )
}

fn update_xp_bar(mut xp_bar: Single<&mut ProgressBar, With<XpBar>>, xp: Single<&Xp>) {
    xp_bar.length = xp.next_level;
    xp_bar.value = xp.current;
}
