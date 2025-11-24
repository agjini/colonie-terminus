use crate::demo::player::Player;
use crate::PausableSystems;
use bevy::camera::Camera2d;
use bevy::prelude::{
    Commands, Component, IntoScheduleConfigs, Name, Query, Startup, Transform, Update, With,
    Without,
};

#[derive(Component)]
pub struct MainCamera;

pub(super) fn plugin(app: &mut bevy::prelude::App) {
    app.add_systems(Startup, spawn_camera)
        .add_systems(Update, camera_follow.in_set(PausableSystems));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2d,
        Transform::default(),
        MainCamera,
    ));
}

fn camera_follow(
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    if let Ok(player) = player_query.single()
        && let Ok(mut camera2d) = camera_query.single_mut()
    {
        camera2d.translation.x = player.translation.x;
        camera2d.translation.y = player.translation.y;
    }
}
