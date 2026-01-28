use avian2d::prelude::LinearVelocity;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::{AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
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
    pub intent: Vec2,

    /// Maximum speed in world units per second.
    /// 1 world unit = 1 pixel when using the default 2D camera and no physics engine.
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
