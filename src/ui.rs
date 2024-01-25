use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use bevy_egui::{
    egui,
    EguiContexts,
};

use crate::{
    state::{
        Function,
        State,
    },
    triangles::{
        Triangle,
        VertexOrder,
        VertexSelector,
    },
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_ui)
            .add_systems(Update, spawn_vertex_selectors);
    }
}

fn update_ui(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut state: ResMut<State>,
    vertex_selector_query: Query<(Entity, &VertexSelector)>,
) {
    egui::Window::new("Opções")
        .fixed_size([150.0, 200.0])
        .show(contexts.ctx_mut(), |ui| {
            match state.function {
                Function::None => {
                    ui.label("O que você deseja fazer?");
                    ui.horizontal( |ui| {
                        if ui.add(egui::Button::new("Adicionar")).clicked() {
                            state.new_triangle.clear();
                            state.function = Function::Create;
                        }
                        if ui.add(egui::Button::new("Modificar")).clicked() {
                            state.function = Function::Select;
                        }
                    });
                }
                Function::Create => {
                    ui.label("Clique para adicionar pontos.");
                    ui.horizontal( |ui| {
                        ui.label("Cor:");
                        ui.color_edit_button_rgb(&mut state.color_picker);
                    });
                    if ui.add(egui::Button::new("Voltar")).clicked() {
                        for (entity, _) in vertex_selector_query.iter() {
                            commands.entity(entity).despawn();
                        }
                        state.function = Function::None;
                        state.new_triangle.clear();
                    }
                },
                Function::Select => {
                    ui.label("Selecione um triângulo.");
                    if ui.add(egui::Button::new("Voltar")).clicked() {
                        state.function = Function::None;
                    }
                }
                Function::Modify(entity) => {
                    ui.label(format!("Você está editando o triângulo {:?}", entity));
                    ui.label("Clique com o botão direito para atribuir a cor.");
                    ui.horizontal( |ui| {
                        ui.label("Cor:");
                        ui.color_edit_button_rgb(&mut state.color_picker);
                    });

                    if ui.add(egui::Button::new("Voltar")).clicked() {
                        for (entity, _) in vertex_selector_query.iter() {
                            commands.entity(entity).despawn();
                        }
                        state.function = Function::None;
                    }
                },
            }
        });
}


fn spawn_vertex_selectors(
    mut commands: Commands,
    mut state: ResMut<State>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    triangles_query: Query<&Triangle>,
    vertex_selector_query: Query<Entity, With<VertexSelector>>,
) {
    if state.spawn_vertex_selectors {
        for entity in vertex_selector_query.iter() {
            commands.entity(entity).despawn();
        }

        match state.function {
            
            // Spawna seletores de vértice para o triângulo em criação

            Function::Create => {
                let mut z: f32 = 100.0;

                for vertex in state.new_triangle.clone() {
                    commands.spawn((
                        VertexSelector(VertexOrder::Indifferent),
                        MaterialMesh2dBundle {
                            mesh: meshes.add(shape::Circle::new(8.0).into()).into(),
                            material: materials.add(ColorMaterial::from(Color::WHITE)),
                            transform: Transform::from_translation(Vec3::new(
                                vertex.position[0],
                                vertex.position[1], 
                                z,
                            )),
                            ..default()
                        },
                    ));

                    z += 1.0;

                    commands.spawn((
                        VertexSelector(VertexOrder::Indifferent),
                        MaterialMesh2dBundle {
                            mesh: meshes.add(shape::Circle::new(7.0).into()).into(),
                            material: materials.add(ColorMaterial::from(Color::Rgba { 
                                red: vertex.color[0], 
                                green: vertex.color[1], 
                                blue: vertex.color[2], 
                                alpha: 1.0, 
                            })),
                            transform: Transform::from_translation(Vec3::new(
                                vertex.position[0],
                                vertex.position[1],
                                z,
                            )),
                            ..default()
                        },
                    ));

                    z += 1.0;
                }
            }

            // Spawna seletores de vértice para o triângulo selecionado

            Function::Modify(entity) => {
                let triangle = triangles_query.get(entity).unwrap();
                let mut z: f32 = 100.0;

                // Seletor do primeiro vértice

                commands.spawn((
                    VertexSelector(VertexOrder::First),
                    MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(8.0).into()).into(),
                        material: materials.add(ColorMaterial::from(Color::WHITE)),
                        transform: Transform::from_translation(Vec3::new(
                            triangle.first.position[0],
                            triangle.first.position[1], 
                            z,
                        )),
                        ..default()
                    },
                ));

                z += 1.0;

                commands.spawn((
                    VertexSelector(VertexOrder::First),
                    MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(
                            if let Some(VertexOrder::First) = state.selected_vertex.clone() {
                                4.0
                            } else {
                                7.0
                            }
                        ).into()).into(),
                        material: materials.add(ColorMaterial::from(Color::Rgba { 
                            red: triangle.first.color[0], 
                            green: triangle.first.color[1], 
                            blue: triangle.first.color[2], 
                            alpha: 1.0, 
                        })),
                        transform: Transform::from_translation(Vec3::new(
                            triangle.first.position[0],
                            triangle.first.position[1],
                            z,
                        )),
                        ..default()
                    },
                ));

                // Seletor do vértice do meio

                z += 1.0;

                commands.spawn((
                    VertexSelector(VertexOrder::Middle),
                    MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(8.0).into()).into(),
                        material: materials.add(ColorMaterial::from(Color::WHITE)),
                        transform: Transform::from_translation(Vec3::new(
                            triangle.middle.position[0],
                            triangle.middle.position[1], 
                            z,
                        )),
                        ..default()
                    },
                ));

                z += 1.0;

                commands.spawn((
                    VertexSelector(VertexOrder::Middle),
                    MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(
                            if let Some(VertexOrder::Middle) = state.selected_vertex.clone() {
                                4.0
                            } else {
                                7.0
                            }
                        ).into()).into(),
                        material: materials.add(ColorMaterial::from(Color::Rgba { 
                            red: triangle.middle.color[0], 
                            green: triangle.middle.color[1], 
                            blue: triangle.middle.color[2], 
                            alpha: 1.0, 
                        })),
                        transform: Transform::from_translation(Vec3::new(
                            triangle.middle.position[0],
                            triangle.middle.position[1],
                            z,
                        )),
                        ..default()
                    },
                ));

                // Seletor do último vértice

                z += 1.0;

                commands.spawn((
                    VertexSelector(VertexOrder::Last),
                    MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(8.0).into()).into(),
                        material: materials.add(ColorMaterial::from(Color::WHITE)),
                        transform: Transform::from_translation(Vec3::new(
                            triangle.last.position[0],
                            triangle.last.position[1], 
                            z,
                        )),
                        ..default()
                    },
                ));

                z += 1.0;

                commands.spawn((
                    VertexSelector(VertexOrder::Last),
                    MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(
                            if let Some(VertexOrder::Last) = state.selected_vertex.clone() {
                                4.0
                            } else {
                                7.0
                            }
                        ).into()).into(),
                        material: materials.add(ColorMaterial::from(Color::Rgba { 
                            red: triangle.last.color[0], 
                            green: triangle.last.color[1], 
                            blue: triangle.last.color[2], 
                            alpha: 1.0, 
                        })),
                        transform: Transform::from_translation(Vec3::new(
                            triangle.last.position[0],
                            triangle.last.position[1],
                            z,
                        )),
                        ..default()
                    },
                ));
            }


            _ => {}
        }

        state.spawn_vertex_selectors = false;
    }
}