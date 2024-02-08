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

use crate::{
    camera::CameraPlugin,
    constants::{
        HEIGHT, 
        WIDTH,
    },
    state::StatePlugin,
    triangles::TrianglesPlugin,
    ui::UIPlugin,
};

fn main() {
    App::new()
        //.insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    title: "Primeiro Trabalho de CG - Luiz Fernando".into(),
                    resolution: (WIDTH, HEIGHT).into(),
                    resizable: false,
                    window_theme: Some(WindowTheme::Dark),
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