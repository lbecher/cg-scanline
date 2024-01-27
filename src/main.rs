mod state;
mod camera;
mod constants;
mod triangles;
mod ui;

use bevy::{
    prelude::*,
    window::WindowTheme,
};
use bevy_egui::EguiPlugin;
use constants::{HEIGHT, WIDTH};

use crate::{
    camera::CameraPlugin,
    state::StatePlugin,
    triangles::TrianglesPlugin,
    ui::UIPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Primeiro Trabalho de CG - Luiz Fernando".into(),
                    window_theme: Some(WindowTheme::Dark),
                    resolution: (WIDTH, HEIGHT).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins(CameraPlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(StatePlugin)
        .add_plugins(UIPlugin)
        .add_plugins(TrianglesPlugin)
        .run();
}