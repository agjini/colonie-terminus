use bevy::app::App;
use bevy::prelude::{Changed, Component, Query, Transform, Update, With};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, order_layer_sprites);
}

#[derive(Component)]
pub struct Layer(pub f32);

fn order_layer_sprites(
    mut sprites: Query<(&mut Transform, &Layer), (With<Layer>, Changed<Transform>)>,
) {
    for (mut transform, layer) in &mut sprites {
        transform.translation.z =
            layer.0 - (1.0f32 / (1.0f32 + (2.0f32.powf(-0.01 * transform.translation.y))));
    }
}
