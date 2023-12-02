use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::triangles::{
    Vertex,
    VertexSelector,
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_ui_state)
            .add_systems(Update, update_ui);
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Function {
    Create,
    Modify(Option<Entity>),
}

#[derive(Resource)]
pub struct UIState {
    pub function: Option<Function>,
    pub selected_triangle: Option<usize>,
    pub color_picker: [f32; 3],
    pub new_triangle: Vec<Vertex>,
}

fn setup_ui_state(
    mut commands: Commands,
) {
    commands.insert_resource(UIState {
        function: None,
        selected_triangle: None,
        color_picker: [1.0, 1.0, 1.0],
        new_triangle: Vec::new(),
    });
}

fn update_ui(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UIState>,
    mut vertex_selector_query: Query<(Entity, &VertexSelector)>,
) {
    egui::Window::new("Opções")
        .fixed_size([150.0, 200.0])
        .show(contexts.ctx_mut(), |ui| {
            if let Some(function) = &ui_state.function {
                match *function {
                    Function::Create => {
                        ui.label("Toque ou clique para adicionar pontos.");
                        ui.horizontal( |ui| {
                            ui.label("Cor:");
                            ui.color_edit_button_rgb(&mut ui_state.color_picker);
                        });
                        if ui.add(egui::Button::new("Voltar")).clicked() {
                            for (entity, _) in vertex_selector_query.iter() {
                                commands.entity(entity).despawn();
                            }
                            ui_state.function = None;
                            ui_state.new_triangle.clear();
                        }
                    },
                    Function::Modify(entity) => {
                        if let Some(entity) = entity {
                            ui.label(format!("Você está editando o triângulo {:?}", entity));
                        } else {
                            ui.label("Selecione um triângulo.");
                        }

                        if ui.add(egui::Button::new("Voltar")).clicked() {
                            for (entity, _) in vertex_selector_query.iter() {
                                commands.entity(entity).despawn();
                            }
                            ui_state.function = None;
                        }
                    },
                }
            } else {
                ui.label("O que você deseja fazer?");
                ui.horizontal( |ui| {
                    if ui.add(egui::Button::new("Adicionar")).clicked() {
                        ui_state.new_triangle.clear();
                        ui_state.function = Some(Function::Create);
                    }
                    if ui.add(egui::Button::new("Modificar")).clicked() {
                        ui_state.function = Some(Function::Modify(None));
                    }
                });
            }
        });
}