//! Gestion des entrées du joueur et traduction en mouvement via un contrôleur de personnage.
//!
//! Un contrôleur de personnage est l'ensemble des systèmes qui régissent le mouvement des personnages.
//!
//! Dans notre cas, le contrôleur de personnage a la logique suivante :
//! - Définir l'intention de [`MovementController`] basée sur les entrées directionnelles du clavier.
//!   Ceci est fait dans le module `player`, car c'est spécifique au personnage joueur.
//! - Appliquer le mouvement basé sur l'intention de [`MovementController`] et la vitesse maximale.
//! - Appliquer la rotation du système de visée basée sur [`AimDirection`].
use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::demo::player::{AimDirection, AimRig, Player};
use crate::{AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (apply_movement, apply_aim_direction)
            .chain()
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

/// Paramètres de mouvement pour le contrôleur de personnage.
///
/// Pour l'instant, utilisé uniquement pour un seul joueur, mais pourrait également
/// contrôler des PNJ ou d'autres joueurs.
///
/// # Exemple
///
/// ```rust
/// let controller = MovementController {
///     intent: Vec2::new(1.0, 0.0), // Se déplace vers la droite
///     max_speed: 500.0,
/// };
/// ```
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MovementController {
    /// La direction dans laquelle le personnage veut se déplacer.
    ///
    /// Vecteur normalisé ou nul représentant l'intention de mouvement.
    pub intent: Vec2,

    /// Vitesse maximale en unités du monde par seconde.
    ///
    /// 1 unité du monde = 1 pixel lors de l'utilisation de la caméra 2D par défaut
    /// et sans moteur physique.
    pub max_speed: f32,
}

impl Default for MovementController {
    fn default() -> Self {
        Self {
            intent: Vec2::ZERO,
            // 400 pixels per second is a nice default, but we can still vary this per character.
            max_speed: 400.0,
        }
    }
}

fn apply_movement(mut movement_query: Query<(&MovementController, &mut LinearVelocity)>) {
    for (controller, mut linear_velocity) in &mut movement_query {
        let velocity = controller.max_speed * controller.intent;
        linear_velocity.0 = velocity;
    }
}

fn apply_aim_direction(
    aim_direction: Single<(&AimDirection, &Children), With<Player>>,
    mut aim_rig_query: Query<&mut Transform, With<AimRig>>,
) {
    let (aim_direction, children) = aim_direction.into_inner();
    for &child in children {
        if let Ok(mut rig_transform) = aim_rig_query.get_mut(child) {
            rig_transform.rotation = Quat::from_rotation_z(aim_direction.0);
        }
    }
}
