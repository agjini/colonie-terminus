use bevy::color::palettes::tailwind::RED_500;
use bevy::prelude::*;

use crate::hud::{ProgressBar, progress_bar};

const BAR_LENGTH: f32 = 20.0;
const BAR_THICKNESS: f32 = 2.0;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, update_health_bar);
}

pub fn health_bar(
    owner: Entity,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) -> impl Bundle {
    progress_bar(
        owner,
        meshes,
        materials,
        "Health",
        BAR_LENGTH,
        BAR_THICKNESS,
        Color::WHITE,
        RED_500.into(),
    )
}

#[derive(Component, Reflect)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: 80., max }
    }
}

fn update_health_bar(mut bars: Query<&mut ProgressBar>, health: Query<&Health>) {
    for mut bar in &mut bars {
        let Ok(health) = health.get(bar.related) else {
            continue;
        };
        bar.value = health.current / health.max;
    }
}
