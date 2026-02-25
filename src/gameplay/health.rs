use bevy::color::palettes::tailwind::RED_500;
use bevy::prelude::*;

const BAR_LENGTH: f32 = 20.0;
const BAR_THICKNESS: f32 = 2.0;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, update_health_bar);
}

pub fn health_bar(meshes: &mut Assets<Mesh>, materials: &mut Assets<ColorMaterial>) -> impl Bundle {
    (
        Name::new("Health Bar"),
        Transform::from_translation(Vec3::new(0.0, -10.0, -1.0)),
        children![
            (
                Name::new("Health Bar Background"),
                Mesh2d(meshes.add(Rectangle::new(BAR_LENGTH, BAR_THICKNESS))),
                MeshMaterial2d(materials.add(ColorMaterial {
                    color: Color::WHITE,
                    ..default()
                })),
                Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
            ),
            (
                Name::new("Health Bar Foreground"),
                HealthBar,
                Mesh2d(meshes.add(Rectangle::new(BAR_LENGTH, BAR_THICKNESS))),
                MeshMaterial2d(materials.add(ColorMaterial {
                    color: RED_500.into(),
                    ..default()
                })),
                Transform::from_translation(Vec3::ZERO),
            )
        ],
    )
}

#[derive(Component, Reflect)]
struct HealthBar;

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

fn update_health_bar(health: Single<&Health>, mut bar: Single<&mut Transform, With<HealthBar>>) {
    let ratio = (health.current / health.max).clamp(0.0, 1.0);
    bar.scale.x = ratio;
    bar.translation.x = -(BAR_LENGTH * (1.0 - ratio)) / 2.0;
}
