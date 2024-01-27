use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use bevy_egui::{
    egui,
    EguiContexts,
};

use crate::{
    constants::{HEIGHT, WIDTH}, state::{
        Function,
        State,
    }, triangles::{
        Triangle, TriangleSprite, VertexOrder, VertexSelector
    }
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
    mut triangles_query: Query<&mut Triangle>,
    triangle_sprites_query: Query<&TriangleSprite>,
) {
    egui::Window::new("Opções")
        .fixed_size([150.0, 200.0])
        .show(contexts.ctx_mut(), |ui| {
            match state.function {
                Function::None => {
                    ui.label("Selecione uma das opções abaixo.");
                    ui.separator();
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
                    ui.label("Clique com o botão esquerdo do mouse para adicionar pontos.");
                    ui.separator();
                    ui.label("Use o seletor de cor abaixo para escolher a cor dos vértices.");
                    ui.horizontal( |ui| {
                        ui.label("Cor:");
                        ui.color_edit_button_rgb(&mut state.color_picker);
                    });
                    ui.separator();
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
                    ui.separator();
                    if ui.add(egui::Button::new("Voltar")).clicked() {
                        state.function = Function::None;
                    }
                }
                Function::Modify(entity) => {
                    if let Ok(triangle) = triangles_query.get(entity) {
                        ui.label(format!("Você está editando {}º triângulo.", triangle.index));

                        ui.separator();

                        ui.label("Para mover um vértice, clique com o botão esquerdo do mouse sobre um seletor para selecioná-lo. Depois, clique na nova posição.");

                        ui.separator();

                        ui.label("Para atribuir a cor abaixo, clique com o botão direito do mouse sobre um seletor.");
                        ui.horizontal( |ui| {
                            ui.label("Cor:");
                            ui.color_edit_button_rgb(&mut state.color_picker);
                        });

                        ui.separator();

                        ui.checkbox(&mut state.show_properties_window, "Exibir janela de propriedades");

                        ui.separator();

                        ui.horizontal( |ui| {
                            if ui.add(egui::Button::new("Voltar")).clicked() {
                                for (entity, _) in vertex_selector_query.iter() {
                                    commands.entity(entity).despawn();
                                }
                                state.function = Function::None;
                            }
                            if ui.add(egui::Button::new("Deletar")).clicked() {
                                // despawna seletores
                                for (entity, _) in vertex_selector_query.iter() {
                                    commands.entity(entity).despawn();
                                }
                                // despawna sprite do triângulo
                                if let Ok(triangle_sprite) = triangle_sprites_query.get(entity) {
                                    if let Some(entity) = triangle_sprite.0 {
                                        commands.entity(entity).despawn();
                                    }
                                }
                                // despawna sprite do triângulo
                                commands.entity(entity).despawn();
                                state.function = Function::None;
                            }
                        });
                    }
                },
            }
        });
    
    if state.show_properties_window {
        if let Function::Modify(entity) = state.function {
            if let Ok(mut triangle) = triangles_query.get_mut(entity) {
                egui::Window::new("Propriedades")
                    .fixed_size([150.0, 200.0])
                    .show(contexts.ctx_mut(), |ui| {
                        ui.vertical(|ui| {
                            ui.label("Primeiro vértice:");
                            ui.horizontal(|ui| {
                                ui.label("X:");
                                ui.add(egui::TextEdit::singleline(&mut state.first_position_x_string));
                            });
                            ui.horizontal(|ui| {
                                ui.label("Y:");
                                ui.add(egui::TextEdit::singleline(&mut state.first_position_y_string));
                            });
                            if state.first_position_string_parsing_error {
                                ui.label("Algo aqui não é ponto flutuante!");
                            }
                            ui.horizontal(|ui| {
                                if ui.button("Aplicar").clicked() {
                                    if let (
                                        Ok(mut xp),
                                        Ok(mut yp),
                                    ) = (
                                        state.first_position_x_string.parse::<f32>(),
                                        state.first_position_y_string.parse::<f32>(),
                                    ) {
                                        if xp >= WIDTH {
                                            xp = WIDTH - 1.0;
                                            state.first_position_x_string = format!("{}", xp);
                                        } else if xp < 0.0 {
                                            xp = 0.0;
                                            state.first_position_x_string = format!("{}", xp);
                                        }
                                        if yp >= HEIGHT {
                                            yp = HEIGHT - 1.0;
                                            state.first_position_y_string = format!("{}", yp);
                                        } else if yp < 0.0 {
                                            yp = 0.0;
                                            state.first_position_y_string = format!("{}", yp);
                                        }
                                        triangle.first.position[0] = xp;
                                        triangle.first.position[1] = yp;
                                        triangle.redraw = true;
                                        state.spawn_vertex_selectors = true;
                                        state.first_position_string_parsing_error = false;
                                    } else {
                                        state.first_position_string_parsing_error = true;
                                    }
                                }
                                if ui.button("Restaurar").clicked() {
                                    state.first_position_x_string = format!(
                                        "{}",
                                        triangle.first.position[0],
                                    );
                                    state.first_position_y_string = format!(
                                        "{}",
                                        triangle.first.position[1],
                                    );
                                }
                            });

                            ui.separator();

                            ui.label("Segundo vértice:");
                            ui.horizontal(|ui| {
                                ui.label("X:");
                                ui.add(egui::TextEdit::singleline(&mut state.middle_position_x_string));
                            });
                            ui.horizontal(|ui| {
                                ui.label("Y:");
                                ui.add(egui::TextEdit::singleline(&mut state.middle_position_y_string));
                            });
                            if state.middle_position_string_parsing_error {
                                ui.label("Algo aqui não é ponto flutuante!");
                            }
                            ui.horizontal(|ui| {
                                if ui.button("Aplicar").clicked() {
                                    if let (
                                        Ok(mut xp),
                                        Ok(mut yp),
                                    ) = (
                                        state.middle_position_x_string.parse::<f32>(),
                                        state.middle_position_y_string.parse::<f32>(),
                                    ) {
                                        if xp >= WIDTH {
                                            xp = WIDTH - 1.0;
                                            state.middle_position_x_string = format!("{}", xp);
                                        } else if xp < 0.0 {
                                            xp = 0.0;
                                            state.middle_position_x_string = format!("{}", xp);
                                        }
                                        if yp >= HEIGHT {
                                            yp = HEIGHT - 1.0;
                                            state.middle_position_y_string = format!("{}", yp);
                                        } else if yp < 0.0 {
                                            yp = 0.0;
                                            state.middle_position_y_string = format!("{}", yp);
                                        }
                                        triangle.middle.position[0] = xp;
                                        triangle.middle.position[1] = yp;
                                        triangle.redraw = true;
                                        state.spawn_vertex_selectors = true;
                                        state.middle_position_string_parsing_error = false;
                                    } else {
                                        state.middle_position_string_parsing_error = true;
                                    }
                                }
                                if ui.button("Restaurar").clicked() {
                                    state.middle_position_x_string = format!(
                                        "{}",
                                        triangle.middle.position[0],
                                    );
                                    state.middle_position_y_string = format!(
                                        "{}",
                                        triangle.middle.position[1],
                                    );
                                }
                            });

                            ui.separator();

                            ui.label("Terceiro vértice:");
                            ui.horizontal(|ui| {
                                ui.label("X:");
                                ui.add(egui::TextEdit::singleline(&mut state.last_position_x_string));
                            });
                            ui.horizontal(|ui| {
                                ui.label("Y:");
                                ui.add(egui::TextEdit::singleline(&mut state.last_position_y_string));
                            });
                            if state.last_position_string_parsing_error {
                                ui.label("Algo aqui não é ponto flutuante!");
                            }
                            ui.horizontal(|ui| {
                                if ui.button("Aplicar").clicked() {
                                    if let (
                                        Ok(mut xp),
                                        Ok(mut yp),
                                    ) = (
                                        state.last_position_x_string.parse::<f32>(),
                                        state.last_position_y_string.parse::<f32>(),
                                    ) {
                                        if xp >= WIDTH {
                                            xp = WIDTH - 1.0;
                                            state.last_position_x_string = format!("{}", xp);
                                        } else if xp < 0.0 {
                                            xp = 0.0;
                                            state.last_position_x_string = format!("{}", xp);
                                        }
                                        if yp >= HEIGHT {
                                            yp = HEIGHT - 1.0;
                                            state.last_position_y_string = format!("{}", yp);
                                        } else if yp < 0.0 {
                                            yp = 0.0;
                                            state.last_position_y_string = format!("{}", yp);
                                        }
                                        triangle.last.position[0] = xp;
                                        triangle.last.position[1] = yp;
                                        triangle.redraw = true;
                                        state.spawn_vertex_selectors = true;
                                        state.last_position_string_parsing_error = false;
                                    } else {
                                        state.last_position_string_parsing_error = true;
                                    }
                                }
                                if ui.button("Restaurar").clicked() {
                                    state.last_position_x_string = format!(
                                        "{}",
                                        triangle.last.position[0],
                                    );
                                    state.last_position_y_string = format!(
                                        "{}",
                                        triangle.last.position[1],
                                    );
                                }
                            });
                        });
                    });
            }
        }
    }
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
            Function::Create => {
                let mut order: VertexOrder = VertexOrder::First;
                let mut count: u8 = 0;
                let mut z: f32 = 100.0;

                for vertex in state.new_triangle.clone() {
                    if count == 1 {
                        order = VertexOrder::Middle;
                    } else if count == 1 {
                        order = VertexOrder::Last;
                    }
                    count += 1;

                    commands.spawn((
                        VertexSelector(order.clone()),
                        MaterialMesh2dBundle {
                            mesh: meshes.add(shape::Circle::new(9.0).into()).into(),
                            material: materials.add(ColorMaterial::from(Color::BLACK)),
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
                        VertexSelector(order.clone()),
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
                        VertexSelector(order.clone()),
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


            Function::Modify(entity) => {
                let triangle = triangles_query.get(entity).unwrap();
                let mut z: f32 = 100.0;

                // --------------------
                // seletor do primeiro vértice
                // --------------------

                commands.spawn((
                    VertexSelector(VertexOrder::First),
                    MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(9.0).into()).into(),
                        material: materials.add(ColorMaterial::from(Color::BLACK)),
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

                z += 1.0;

                // --------------------
                // seletor do vértice do meio
                // --------------------

                commands.spawn((
                    VertexSelector(VertexOrder::Middle),
                    MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(9.0).into()).into(),
                        material: materials.add(ColorMaterial::from(Color::BLACK)),
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

                z += 1.0;

                // --------------------
                // seletor do último vértice
                // --------------------

                commands.spawn((
                    VertexSelector(VertexOrder::Last),
                    MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(9.0).into()).into(),
                        material: materials.add(ColorMaterial::from(Color::BLACK)),
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