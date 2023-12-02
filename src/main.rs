mod camera;
mod triangles;
mod ui;

use bevy::{
    prelude::*,
    window::WindowTheme,
};
use bevy_egui::EguiPlugin;

use crate::{
    camera::CameraPlugin,
    ui::UIPlugin,
    triangles::TrianglesPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Primeiro Trabalho de CG".into(),
                    window_theme: Some(WindowTheme::Dark),
                    resolution: (800.0, 600.0).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins(CameraPlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(TrianglesPlugin)
        .run();
}