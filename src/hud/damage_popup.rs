use crate::gameplay::GameLayer;
use bevy::prelude::*;
use bevy::time::Timer;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, update_damage_popups);
}

#[derive(Component)]
struct DamagePopup {
    timer: Timer,
    velocity: Vec2,
}

pub fn spawn_damage_popup(commands: &mut Commands, position: Vec3, damage: f32) {
    let offset_x = rand::random::<f32>() * 20.0 - 5.0;

    commands.spawn((
        GameLayer::AimZone,
        Text2d::new(damage.to_string()),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 1.0)),
        Transform::from_translation(position + Vec3::new(offset_x, 20.0, 1.0)),
        DamagePopup {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
            velocity: Vec2::new(offset_x * 0.5, 60.0),
        },
    ));
}

fn update_damage_popups(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DamagePopup, &mut Transform, &mut TextColor)>,
) {
    for (entity, mut popup, mut transform, mut color) in &mut query {
        popup.timer.tick(time.delta());
        let t = popup.timer.fraction();
        transform.translation.y += popup.velocity.y * time.delta_secs();
        let alpha = if t > 0.5 { 1.0 - (t - 0.5) * 2.0 } else { 1.0 };
        color.0.set_alpha(alpha);
        if popup.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
