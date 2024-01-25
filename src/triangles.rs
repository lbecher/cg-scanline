use bevy::{
    prelude::*,
    window::PrimaryWindow,
};
use bevy_egui::EguiContexts;
use std::cmp::Ordering;

use crate::state::{
    Function,
    State,
};

pub struct TrianglesPlugin;

impl Plugin for TrianglesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, creating)
            .add_systems(Update, modifying)
            .add_systems(Update, redrawing)
            .add_systems(Update, selecting);
    }
}

#[derive(Debug, Clone)]
pub enum VertexOrder {
    First,
    Middle,
    Last,
    Indifferent,
}

#[derive(Component)]
pub struct VertexSelector(pub VertexOrder);

#[derive(Debug, Clone)]
pub struct Vertex {
    pub color: [f32; 3],
    pub position: [f32; 3],
}

#[derive(Debug, Component)]
pub struct Triangle {
    pub first: Vertex,
    pub middle: Vertex,
    pub last: Vertex,
    pub redraw: bool,
}


fn creating(
    mut commands: Commands,
    mut egui_contexts: EguiContexts,
    input: Res<Input<MouseButton>>,
    mut state: ResMut<State>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Function::Create = state.function {
        let window = window_query.single();
        let ctx = egui_contexts.ctx_mut();

        // --------------------
        // se nem todos os vértices tiverem sido definidos,
        // adiciona-se os pontos de clique no vetor new_triangle
        // --------------------

        if state.new_triangle.len() < 3 {
            if input.just_pressed(MouseButton::Left) && !(ctx.is_using_pointer() || ctx.is_pointer_over_area()) {
                if let Some(cursor_position) = window.cursor_position() {
                    let position: [f32; 3] = [
                        cursor_position.x,
                        window.height() - cursor_position.y,
                        10.0,
                    ];
                    let color: [f32; 3] = state.color_picker.clone();

                    state.new_triangle.push(Vertex {
                        position,
                        color,
                    });

                    state.spawn_vertex_selectors = true;
                }
            }
        }

        // --------------------
        // quando todos os vértices já tiverem sido criados,
        // spawnamos o triângulo com suas informações
        // --------------------
        
        else {
            let mut sorted_triangle = state.new_triangle.clone();
            sorted_triangle.sort_by(|a, b| a.position[0].partial_cmp(&b.position[0]).unwrap_or(Ordering::Equal));

            let entity = commands.spawn(Triangle {
                first: sorted_triangle[0].clone(),
                middle: sorted_triangle[1].clone(),
                last: sorted_triangle[2].clone(),
                redraw: true,
            }).id();

            state.function = Function::Modify(entity);
            state.new_triangle.clear();
        }
    }
}


fn modifying(
    mut egui_contexts: EguiContexts,
    input: Res<Input<MouseButton>>,
    mut state: ResMut<State>,
    mut triangles_query: Query<(Entity, &mut Triangle)>,
    vertex_selector_query: Query<(Entity, &VertexSelector, &Transform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Function::Modify(entity) = state.function {
        let window = window_query.single();
        let ctx = egui_contexts.ctx_mut();

        if input.just_pressed(MouseButton::Right) && !(ctx.is_using_pointer() || ctx.is_pointer_over_area()) {
            if let Some(cursor_position) = window.cursor_position() {
                for (_, vertex_selector, transform) in vertex_selector_query.iter() {
                    let x_difference = cursor_position.x - transform.translation.x;
                    let y_difference = (window.height() - cursor_position.y) - transform.translation.y;

                    if x_difference.abs() < 8.0 && y_difference.abs() < 8.0 {
                        let (_, mut triangle) = triangles_query.get_mut(entity).unwrap();

                        match vertex_selector.0 {
                            VertexOrder::First => {
                                triangle.first.color = state.color_picker;
                            }
                            VertexOrder::Middle => {
                                triangle.middle.color = state.color_picker;
                            }
                            VertexOrder::Last => {
                                triangle.last.color = state.color_picker;
                            }
                            VertexOrder::Indifferent => {}
                        }

                        state.spawn_vertex_selectors = true;

                        break;
                    }
                }
            }
        }

        if input.just_pressed(MouseButton::Left) && !(ctx.is_using_pointer() || ctx.is_pointer_over_area()) {
            if let Some(cursor_position) = window.cursor_position() {
                let (_, mut triangle) = triangles_query.get_mut(entity).unwrap();

                if let Some(selected_vertex) = state.selected_vertex.clone() {
                    match selected_vertex {
                        VertexOrder::First => {
                            triangle.first.position[0] = cursor_position.x;
                            triangle.first.position[1] = window.height() - cursor_position.y;
                        }
                        VertexOrder::Middle => {
                            triangle.middle.position[0] = cursor_position.x;
                            triangle.middle.position[1] = window.height() - cursor_position.y;
                        }
                        VertexOrder::Last => {
                            triangle.last.position[0] = cursor_position.x;
                            triangle.last.position[1] = window.height() - cursor_position.y;
                        }
                        VertexOrder::Indifferent => {}
                    };

                    let mut vertices: [Vertex; 3] = [
                        triangle.first.clone(),
                        triangle.middle.clone(),
                        triangle.last.clone(),
                    ];

                    vertices.sort_by(|a, b| a.position[0].partial_cmp(&b.position[0]).unwrap_or(Ordering::Equal));
                    
                    triangle.first = vertices[0].clone();
                    triangle.middle = vertices[1].clone();
                    triangle.last = vertices[2].clone();
                    triangle.redraw = true;

                    state.spawn_vertex_selectors = true;
                    state.selected_vertex = None;
                }
                
                else {
                    let x_difference = cursor_position.x - triangle.first.position[0];
                    let y_difference = (window.height() - cursor_position.y) - triangle.first.position[1];

                    if x_difference.abs() < 8.0 && y_difference.abs() < 8.0 {
                        state.selected_vertex = Some(VertexOrder::First);
                    }

                    let x_difference = cursor_position.x - triangle.middle.position[0];
                    let y_difference = (window.height() - cursor_position.y) - triangle.middle.position[1];

                    if x_difference.abs() < 8.0 && y_difference.abs() < 8.0 {
                        state.selected_vertex = Some(VertexOrder::Middle);
                    }

                    let x_difference = cursor_position.x - triangle.last.position[0];
                    let y_difference = (window.height() - cursor_position.y) - triangle.last.position[1];

                    if x_difference.abs() < 8.0 && y_difference.abs() < 8.0 {
                        state.selected_vertex = Some(VertexOrder::Last);
                    }

                    if state.selected_vertex.is_some() {
                        state.spawn_vertex_selectors = true;
                    }
                }
            }
        }
    }
}


fn redrawing(
    mut egui_contexts: EguiContexts,
    input: Res<Input<MouseButton>>,
    mut state: ResMut<State>,
    mut triangles_query: Query<&mut Triangle>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for triangle in triangles_query.iter_mut() {
        if triangle.redraw {

        }
    }
}


fn selecting(
    mut egui_contexts: EguiContexts,
    input: Res<Input<MouseButton>>,
    mut state: ResMut<State>,
    triangles_query: Query<(Entity, &Triangle)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Function::Select = state.function {
        let window = window_query.single();
        let ctx = egui_contexts.ctx_mut();

        if input.just_pressed(MouseButton::Left) && !(ctx.is_using_pointer() || ctx.is_pointer_over_area()) {
            if let Some(cursor_position) = window.cursor_position() {
                let click = (cursor_position.x, window.height() - cursor_position.y);
                for (entity, triangle) in triangles_query.iter() {
                    if is_inside(click, triangle) {
                        state.function = Function::Modify(entity);
                        state.spawn_vertex_selectors = true;
                    }
                }
            }
        }
    }
}


fn barycentric_coordinates(p: (f32, f32), a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> (f32, f32, f32) {
    let v0 = (c.0 - a.0, c.1 - a.1);
    let v1 = (b.0 - a.0, b.1 - a.1);
    let v2 = (p.0 - a.0, p.1 - a.1);

    let dot00 = v0.0 * v0.0 + v0.1 * v0.1;
    let dot01 = v0.0 * v1.0 + v0.1 * v1.1;
    let dot02 = v0.0 * v2.0 + v0.1 * v2.1;
    let dot11 = v1.0 * v1.0 + v1.1 * v1.1;
    let dot12 = v1.0 * v2.0 + v1.1 * v2.1;

    let inv_denom = 1.0 / (dot00 * dot11 - dot01 * dot01);
    let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
    let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

    (u, v, 1.0 - u - v)
}


fn is_inside(click: (f32, f32), triangle: &Triangle) -> bool {
    let (u, v, w) = barycentric_coordinates(
        click, 
        (triangle.first.position[0], triangle.first.position[1]), 
        (triangle.middle.position[0], triangle.middle.position[1]), 
        (triangle.last.position[0], triangle.last.position[1]));

    u >= 0.0 && v >= 0.0 && u + v <= 1.0
}