use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::{AppSystems, PausableSystems};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        apply_movement
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MovementController {
    /// The direction the character wants to move in.
    pub direction: Vec2,

    /// Maximum speed in world units per second.
    /// 1 world unit = 1 pixel when using the default 2D camera and no physics engine.
    pub max_speed: f32,
}

impl Default for MovementController {
    fn default() -> Self {
        Self {
            direction: Vec2::ZERO,
            max_speed: 400.0,
        }
    }
}

fn apply_movement(
    mut movement_query: Query<
        (&MovementController, &mut LinearVelocity),
        Changed<MovementController>,
    >,
) {
    for (controller, mut linear_velocity) in &mut movement_query {
        let velocity = controller.max_speed * controller.direction;
        linear_velocity.0 = velocity;
    }
}
