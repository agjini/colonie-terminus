use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, update_progress_bar);
}

pub fn progress_bar(
    owner: Entity,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    name: impl Into<String>,
    length: f32,
    thickness: f32,
    bg: Color,
    fg: Color,
) -> impl Bundle {
    (
        Name::new(format!("{} Bar", &name.into())),
        Visibility::default(),
        Transform::from_translation(Vec3::new(0.0, -10.0, -1.0)),
        children![
            (
                Mesh2d(meshes.add(Rectangle::new(length, thickness))),
                MeshMaterial2d(materials.add(ColorMaterial {
                    color: bg,
                    ..default()
                })),
                Visibility::default(),
                Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
            ),
            (
                ProgressBar::new(owner, length, 0.),
                Mesh2d(meshes.add(Rectangle::new(length, thickness))),
                MeshMaterial2d(materials.add(ColorMaterial {
                    color: fg,
                    ..default()
                })),
                Visibility::default(),
                Transform::from_translation(Vec3::ZERO),
            )
        ],
    )
}

#[derive(Component, Reflect)]
pub struct ProgressBar {
    pub related: Entity,
    pub value: f32,
    pub length: f32,
}

impl ProgressBar {
    pub fn new(related: Entity, length: f32, value: f32) -> Self {
        Self {
            related,
            length,
            value,
        }
    }
}

fn update_progress_bar(mut bars: Query<(&ProgressBar, &mut Transform), Changed<ProgressBar>>) {
    for (bar, mut transform) in &mut bars {
        let ratio = bar.value.clamp(0.0, 1.0);
        transform.scale.x = ratio;
        transform.translation.x = -(bar.length * (1.0 - ratio)) / 2.0;
    }
}
