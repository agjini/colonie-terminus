use avian2d::prelude::PhysicsLayer;
use bevy::app::App;
use bevy::prelude::{Changed, Component, Query, Reflect, Transform, Update};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, order_layers);
}

#[derive(Component, PhysicsLayer, Default, Reflect)]
pub enum GameLayer {
    #[default]
    Ground,
    Enemy,
    Player,
}

impl GameLayer {
    fn z(&self) -> f32 {
        match self {
            GameLayer::Ground => -1.,
            GameLayer::Enemy => 10.,
            GameLayer::Player => 20.,
        }
    }
}

fn order_layers(mut sprites: Query<(&mut Transform, &GameLayer), Changed<Transform>>) {
    for (mut transform, layer) in &mut sprites {
        transform.translation.z =
            layer.z() - (1.0f32 / (1.0f32 + (2.0f32.powf(-0.01 * transform.translation.y))));
    }
}
