use crate::{AppSystems, PausableSystems};
use bevy::image::ImageSampler;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        update_reticle
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

const RETICLE_LENGTH: f32 = 64.0;
const RETICLE_THICKNESS: f32 = 2.0;
const LASER_COLOR: Color = Color::srgb(1.0, 0.15, 0.1);

#[derive(Component, Reflect)]
pub struct Reticle;

fn laser_gradient(images: &mut Assets<Image>) -> Handle<Image> {
    let width = 100u32;
    let pixel_size = 4u32;
    let mut data = vec![0u8; (width * pixel_size) as usize];
    for x in 0..width {
        let i = (x * pixel_size) as usize;
        data[i] = 255;
        data[i + 1] = 255;
        data[i + 2] = 255;
        data[i + 3] = 100 - x as u8;
    }
    let mut image = Image::new(
        Extent3d {
            width,
            height: 1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        default(),
    );
    image.sampler = ImageSampler::linear();
    images.add(image)
}

pub fn reticle(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    images: &mut Assets<Image>,
) -> impl Bundle {
    let texture = laser_gradient(images);
    (
        Name::new("Reticle"),
        Reticle,
        Mesh2d(meshes.add(Rectangle::new(RETICLE_LENGTH, RETICLE_THICKNESS))),
        MeshMaterial2d(materials.add(ColorMaterial {
            color: LASER_COLOR,
            texture: Some(texture),
            ..default()
        })),
        Transform::from_translation(Vec3::new(RETICLE_LENGTH / 2.0, 0.0, -1.0)),
    )
}

#[derive(Component, Reflect)]
pub struct WeaponDirection(pub Vec2);

fn update_reticle(
    weapon_dir: Single<&WeaponDirection>,
    mut reticle: Single<&mut Transform, With<Reticle>>,
) {
    let angle = weapon_dir.0.to_angle();
    let offset = weapon_dir.0.normalize_or_zero() * RETICLE_LENGTH / 2.0;
    reticle.rotation = Quat::from_rotation_z(angle);
    reticle.translation = offset.extend(reticle.translation.z);
}
