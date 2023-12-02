use bevy::{
    prelude::*,
    window::PrimaryWindow,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    let translation = Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0);
    let transform = Transform::from_translation(translation);
    commands.spawn(Camera2dBundle {
        transform,
        ..default()
    });
}