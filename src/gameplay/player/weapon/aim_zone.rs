use crate::gameplay::layer::GameLayer;
use crate::gameplay::player::weapon::WeaponDirection;
use crate::{AppSystems, PausableSystems};
use avian2d::prelude::{Collider, CollidingEntities, CollisionLayers, Mass, Sensor};
use bevy::mesh::{Indices, PrimitiveTopology, VertexAttributeValues};
use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        update_aim_zone
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

const RETICLE_LENGTH: f32 = 200.0;
const BASE_HALF_WIDTH: f32 = 1.5;
const PLAIN_RETICLE_COLOR: [f32; 4] = [3.0, 0.15, 0.1, 0.1];
const RETICLE_COLOR: [f32; 4] = [1.0, 0.15, 0.1, 0.];
const ARC_SEGMENTS: u32 = 16;

#[derive(Component, Reflect)]
pub struct AimZone;

pub fn aim_zone(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    fire_origin: Vec2,
    angle_degrees: f32,
) -> impl Bundle {
    let half_angle = (angle_degrees / 2.0).to_radians();

    let angle_start = -half_angle;
    let angle_end = half_angle;
    let b = Vec2::new(
        angle_start.sin() * RETICLE_LENGTH,
        angle_start.cos() * RETICLE_LENGTH,
    );
    let c = Vec2::new(
        angle_end.sin() * RETICLE_LENGTH,
        angle_end.cos() * RETICLE_LENGTH,
    );

    (
        Name::new("AimZone"),
        AimZone,
        GameLayer::AimZone,
        Mesh2d(meshes.add(sector_mesh(half_angle))),
        MeshMaterial2d(materials.add(ColorMaterial::default())),
        Transform::from_translation(Vec3::new(fire_origin.x, fire_origin.y, 1.0))
            .with_scale(Vec3::splat(2.0)),
        (
            Mass(1.0),
            Collider::convex_hull(vec![
                Vec2::new(-BASE_HALF_WIDTH, 0.0),
                b,
                c,
                Vec2::new(BASE_HALF_WIDTH, 0.0),
            ])
            .unwrap(),
            CollidingEntities::default(),
            CollisionLayers::new(GameLayer::AimZone, [GameLayer::Enemy]),
            Sensor,
        ),
    )
}

fn update_aim_zone(
    weapon_dir: Single<&WeaponDirection>,
    mut aim_zone: Single<&mut Transform, With<AimZone>>,
) {
    let angle = weapon_dir.0.to_angle() - FRAC_PI_2;
    aim_zone.rotation = Quat::from_rotation_z(angle);
}

fn sector_mesh(half_angle: f32) -> Mesh {
    let vertex_count = (ARC_SEGMENTS + 3) as usize;
    let mut positions = Vec::with_capacity(vertex_count);
    let mut colors = Vec::with_capacity(vertex_count);
    let mut indices = Vec::with_capacity((ARC_SEGMENTS + 1) as usize * 3);

    positions.push([-BASE_HALF_WIDTH, 0.0, 0.0]);
    colors.push(PLAIN_RETICLE_COLOR);

    for i in 0..=ARC_SEGMENTS {
        let t = i as f32 / ARC_SEGMENTS as f32;
        let angle = -half_angle + t * 2.0 * half_angle;
        positions.push([
            angle.sin() * RETICLE_LENGTH,
            angle.cos() * RETICLE_LENGTH,
            0.0,
        ]);
        colors.push([RETICLE_COLOR[0], RETICLE_COLOR[1], RETICLE_COLOR[2], 0.0]);
    }

    positions.push([BASE_HALF_WIDTH, 0.0, 0.0]);
    colors.push(PLAIN_RETICLE_COLOR);

    let last = (positions.len() - 1) as u32;
    for i in 1..last {
        indices.extend_from_slice(&[0, i, i + 1]);
    }

    Mesh::new(PrimitiveTopology::TriangleList, default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_COLOR,
            VertexAttributeValues::Float32x4(colors),
        )
        .with_inserted_indices(Indices::U32(indices))
}
