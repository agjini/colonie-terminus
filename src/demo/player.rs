//! Comportement spécifique au joueur.
//!
//! Ce module gère le joueur, son viseur (aim rig) et les entrées de direction.
//! Il fournit les composants et ressources nécessaires pour contrôler le personnage
//! du joueur et gérer sa visée avec la souris ou la manette.

use crate::demo::camera::MainCamera;
use crate::{
    asset_tracking::LoadResource, demo::movement::MovementController, AppSystems, PausableSystems,
};
use avian2d::prelude::{
    Collider, CollisionEventsEnabled, DebugRender, LinearVelocity, LockedAxes, RigidBody,
};
use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

const UP: [KeyCode; 2] = [KeyCode::KeyW, KeyCode::ArrowUp];
const DOWN: [KeyCode; 2] = [KeyCode::KeyS, KeyCode::ArrowDown];
const LEFT: [KeyCode; 2] = [KeyCode::KeyA, KeyCode::ArrowLeft];
const RIGHT: [KeyCode; 2] = [KeyCode::KeyD, KeyCode::ArrowRight];
const AIM_RADIUS: f32 = 75.;

/// Marqueur de composant pour l'entité joueur.
///
/// Utilisé pour identifier et requêter l'entité principale du joueur dans les systèmes.

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

/// Source de la direction de visée.
///
/// Détermine si la visée provient de la souris ou d'une manette de jeu.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Reflect)]
pub enum AimSource {
    /// Visée contrôlée par la souris.
    Mouse,
    /// Visée contrôlée par la manette.
    Gamepad,
}

/// Composant de viseur pour le personnage joueur.
///
/// Gère le cercle de visée et la croix de visée qui suivent la direction de la souris.
/// Le viseur tourne autour du joueur pour indiquer où le joueur vise.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct AimRig {
    /// Le rayon du cercle de visée en pixels.
    pub radius: f32,

    /// La source de la direction de visée (souris ou manette).
    pub source: AimSource,
}

/// Direction de visée du joueur en radians.
///
/// Représente l'angle de visée calculé à partir de la position de la souris
/// par rapport au joueur. L'angle 0 correspond à la droite (axe X positif).
#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub struct AimDirection(pub f32);

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>();

    app.add_systems(
        Update,
        (record_player_directional_input, record_aim_direction)
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );
}

/// Crée un bundle complet pour l'entité joueur.
///
/// # Arguments
///
/// * `max_speed` - Vitesse maximale de déplacement du joueur
/// * `materials` - Ressource des matériaux pour créer les visuels
/// * `meshes` - Ressource des meshes pour créer les formes
///
/// # Retour
///
/// Un bundle contenant tous les composants nécessaires au joueur :
/// - Composants de rendu (mesh, matériaux)
/// - Composants de physique (rigidbody, collider)
/// - Contrôleur de mouvement
/// - Système de visée (AimRig avec cercle et croix)
pub fn player(
    max_speed: f32,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
) -> impl Bundle {
    let body_mesh = meshes.add(Rectangle::new(32., 32.));
    let body_material = materials.add(ColorMaterial::from(Color::WHITE));
    let aim_circle_mesh = meshes.add(Circle::new(AIM_RADIUS));
    let aime_circle_material = materials.add(ColorMaterial::from(Color::srgba(1.0, 1.0, 1.0, 0.2)));
    let cross_horizontal_mesh = meshes.add(Rectangle::new(16., 2.));
    let cross_vertical_mesh = meshes.add(Rectangle::new(2., 16.));
    let cross_material = materials.add(ColorMaterial::from(Color::WHITE));
    (
        Name::new("Player"),
        Player,
        Mesh2d(body_mesh),
        MeshMaterial2d(body_material),
        Transform::from_scale(Vec2::splat(4.0).extend(1.0)),
        MovementController {
            max_speed,
            ..default()
        },
        AimDirection::default(),
        RigidBody::Dynamic,
        Collider::rectangle(32.0, 32.0),
        LinearVelocity::ZERO,
        LockedAxes::ROTATION_LOCKED,
        CollisionEventsEnabled,
        DebugRender::default().with_collider_color(Color::WHITE),
        children![(
            Name::new("AimRig"),
            Visibility::Inherited,
            Transform::default(),
            AimRig {
                radius: AIM_RADIUS,
                source: AimSource::Mouse,
            },
            children![
                (
                    Name::new("AimCircle"),
                    Visibility::Inherited,
                    Mesh2d(aim_circle_mesh),
                    MeshMaterial2d(aime_circle_material),
                    Transform::from_xyz(0.0, 0.0, -0.1),
                ),
                (
                    Name::new("AimCross"),
                    Visibility::Inherited,
                    Transform::from_translation(Vec3::new(AIM_RADIUS, 0., 0.1)),
                    children![
                        (
                            Visibility::Inherited,
                            Mesh2d(cross_horizontal_mesh),
                            MeshMaterial2d(cross_material.clone()),
                        ),
                        (
                            Visibility::Inherited,
                            Mesh2d(cross_vertical_mesh),
                            MeshMaterial2d(cross_material),
                        ),
                    ],
                )
            ]
        )],
    )
}

fn record_aim_direction(
    camera_query: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    window: Single<&Window>,
    player_query: Single<(&GlobalTransform, &mut AimDirection), With<Player>>,
) {
    let mouse_coords = window.cursor_position().map(|pos| {
        let (camera, camera_transform) = camera_query.into_inner();
        camera
            .viewport_to_world_2d(camera_transform, pos)
            .unwrap_or(vec2(0.0, 0.0))
    });

    let (player_transform, mut aim_direction) = player_query.into_inner();
    let player_pos = player_transform.translation().truncate();
    let aim_direction_vec = mouse_coords.unwrap_or_default() - player_pos;

    if aim_direction_vec != Vec2::ZERO {
        aim_direction.0 = aim_direction_vec.y.atan2(aim_direction_vec.x);
    }
}

fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController, With<Player>>,
) {
    let mut intent = Vec2::ZERO;
    if input.any_pressed(UP) {
        intent.y += 1.0;
    }
    if input.any_pressed(DOWN) {
        intent.y -= 1.0;
    }
    if input.any_pressed(LEFT) {
        intent.x -= 1.0;
    }
    if input.any_pressed(RIGHT) {
        intent.x += 1.0;
    }
    let intent = intent.normalize_or_zero();

    for mut controller in &mut controller_query {
        controller.intent = intent;
    }
}

/// Ressource contenant les assets du joueur.
///
/// Charge et stocke les images, sons et autres ressources utilisées par le joueur.
/// Cette ressource est automatiquement chargée au démarrage du plugin.
#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    ducky: Handle<Image>,

    /// Liste des sons de pas utilisés pour le déplacement du joueur.
    #[dependency]
    pub steps: Vec<Handle<AudioSource>>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            ducky: assets.load_with_settings(
                "images/ducky.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            steps: vec![
                assets.load("audio/sound_effects/step1.ogg"),
                assets.load("audio/sound_effects/step2.ogg"),
                assets.load("audio/sound_effects/step3.ogg"),
                assets.load("audio/sound_effects/step4.ogg"),
            ],
        }
    }
}
