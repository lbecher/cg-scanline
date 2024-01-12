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
            .add_systems(Update, modifying);
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
    vertex_selector_query: Query<(Entity, &VertexSelector)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Function::Create = state.function {
        let window = window_query.single();
        let ctx = egui_contexts.ctx_mut();

        if state.new_triangle.len() < 3 {
            if input.just_pressed(MouseButton::Left) && !(ctx.is_using_pointer() || ctx.is_pointer_over_area()) {
                for (entity, _) in vertex_selector_query.iter() {
                    commands.entity(entity).despawn();
                }

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
        
        else {
            for (entity, _) in vertex_selector_query.iter() {
                commands.entity(entity).despawn();
            }

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
    mut commands: Commands,
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

                        for (entity, _, _) in vertex_selector_query.iter() {
                            commands.entity(entity).despawn();
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

                    for (entity, _, _) in vertex_selector_query.iter() {
                        commands.entity(entity).despawn();
                    }
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

                    for (entity, _, _) in vertex_selector_query.iter() {
                        commands.entity(entity).despawn();
                    }
                    state.spawn_vertex_selectors = true;
                }
            }
        }
    }
}

/*fn associate_functions(
    mut commands: Commands,
    mut egui_contexts: EguiContexts,
    input: Res<Input<MouseButton>>,
    mut ui_state: ResMut<UIState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut triangles_query: Query<(Entity, &mut Triangle)>,
    mut vertex_selector_query: Query<(Entity, &VertexSelector, &mut Transform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let ctx = egui_contexts.ctx_mut();
    let window = window_query.single();
    if let Some(function) = &ui_state.function {
        match function {
            Function::Create => {
                if ui_state.new_triangle.len() < 3 {
                    if input.just_pressed(MouseButton::Left) && !(ctx.is_using_pointer() || ctx.is_pointer_over_area()) {
                        if let Some(cursor_position) = window.cursor_position() {
                            let vertex_index = ui_state.new_triangle.len() as f32;
        
                            let position: [f32; 3] = [
                                cursor_position.x,
                                window.height() - cursor_position.y,
                                10.0,
                            ];
                            let color: [f32; 3] = ui_state.color_picker.clone();
        
                            commands.spawn((
                                VertexSelector(VertexOrder::Indifferent),
                                MaterialMesh2dBundle {
                                    mesh: meshes.add(shape::Circle::new(8.0).into()).into(),
                                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                                    transform: Transform::from_translation(Vec3::new(
                                        position[0],
                                        position[1], 
                                        position[2] + 2.0 * vertex_index,
                                    )),
                                    ..default()
                                },
                            ));
                            commands.spawn((
                                VertexSelector(VertexOrder::Indifferent),
                                MaterialMesh2dBundle {
                                    mesh: meshes.add(shape::Circle::new(7.0).into()).into(),
                                    material: materials.add(ColorMaterial::from(Color::Rgba { 
                                        red: color[0], 
                                        green: color[1], 
                                        blue: color[2], 
                                        alpha: 1.0, 
                                    })),
                                    transform: Transform::from_translation(Vec3::new(
                                        position[0],
                                        position[1], 
                                        position[2] + 2.0 * vertex_index + 1.0,
                                    )),
                                    ..default()
                                },
                            ));
        
                            ui_state.new_triangle.push(Vertex {
                                position,
                                color,
                            });
                        }
                    }
                } else {
                    let mut sorted_triangle = ui_state.new_triangle.clone();
                    sorted_triangle.sort_by(|a, b| a.position[0].partial_cmp(&b.position[0]).unwrap_or(Ordering::Equal));

                    ui_state.new_triangle.clear();

                    let entity = commands.spawn(Triangle {
                        first: sorted_triangle[0].clone(),
                        middle: sorted_triangle[1].clone(),
                        last: sorted_triangle[2].clone(),
                        redraw: true,
                    }).id();

                    ui_state.function = Some(Function::Modify(Some(entity)));

                    for (entity, _, _) in vertex_selector_query.iter() {
                        commands.entity(entity).despawn();
                    }

                    println!("{:?}", sorted_triangle);
                }
            }
            Function::Modify(entity) => {
                match entity {
                    Some(entity) => {
                        let triangle = commands.entity(*entity);
                        if !ui_state.vertex_selectors_spawned {
                            let (_, mut triangle) = triangles_query.get_mut(*entity).unwrap();
                            commands.spawn((
                                VertexSelector(VertexOrder::First),
                                MaterialMesh2dBundle {
                                    mesh: meshes.add(shape::Circle::new(8.0).into()).into(),
                                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                                    transform: Transform::from_translation(Vec3::new(
                                        triangle.first.position[0],
                                        triangle.first.position[1], 
                                        95.0,
                                    )),
                                    ..default()
                                },
                            ));
                            commands.spawn((
                                VertexSelector(VertexOrder::First),
                                MaterialMesh2dBundle {
                                    mesh: meshes.add(shape::Circle::new(7.0).into()).into(),
                                    material: materials.add(ColorMaterial::from(Color::Rgba { 
                                        red: triangle.first.color[0], 
                                        green: triangle.first.color[1], 
                                        blue: triangle.first.color[2], 
                                        alpha: 1.0, 
                                    })),
                                    transform: Transform::from_translation(Vec3::new(
                                        triangle.first.position[0],
                                        triangle.first.position[1],
                                        96.0,
                                    )),
                                    ..default()
                                },
                            ));

                            commands.spawn((
                                VertexSelector(VertexOrder::Middle),
                                MaterialMesh2dBundle {
                                    mesh: meshes.add(shape::Circle::new(8.0).into()).into(),
                                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                                    transform: Transform::from_translation(Vec3::new(
                                        triangle.middle.position[0],
                                        triangle.middle.position[1],
                                        97.0,
                                    )),
                                    ..default()
                                },
                            ));
                            commands.spawn((
                                VertexSelector(VertexOrder::Middle),
                                MaterialMesh2dBundle {
                                    mesh: meshes.add(shape::Circle::new(7.0).into()).into(),
                                    material: materials.add(ColorMaterial::from(Color::Rgba { 
                                        red: triangle.middle.color[0], 
                                        green: triangle.middle.color[1], 
                                        blue: triangle.middle.color[2], 
                                        alpha: 1.0, 
                                    })),
                                    transform: Transform::from_translation(Vec3::new(
                                        triangle.middle.position[0],
                                        triangle.middle.position[1],
                                        98.0,
                                    )),
                                    ..default()
                                },
                            ));

                            commands.spawn((
                                VertexSelector(VertexOrder::Last),
                                MaterialMesh2dBundle {
                                    mesh: meshes.add(shape::Circle::new(8.0).into()).into(),
                                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                                    transform: Transform::from_translation(Vec3::new(
                                        triangle.last.position[0],
                                        triangle.last.position[1],
                                        99.0,
                                    )),
                                    ..default()
                                },
                            ));
                            commands.spawn((
                                VertexSelector(VertexOrder::Last),
                                MaterialMesh2dBundle {
                                    mesh: meshes.add(shape::Circle::new(7.0).into()).into(),
                                    material: materials.add(ColorMaterial::from(Color::Rgba { 
                                        red: triangle.last.color[0], 
                                        green: triangle.last.color[1], 
                                        blue: triangle.last.color[2], 
                                        alpha: 1.0, 
                                    })),
                                    transform: Transform::from_translation(Vec3::new(
                                        triangle.last.position[0],
                                        triangle.last.position[1],
                                        100.0,
                                    )),
                                    ..default()
                                },
                            ));
                            
                            ui_state.vertex_selectors_spawned = true;
                        } else {
                            if input.just_pressed(MouseButton::Right) && !(ctx.is_using_pointer() || ctx.is_pointer_over_area()) {
                                if let Some(cursor_position) = window.cursor_position() {
                                    for (_, vertex_selector, transform) in vertex_selector_query.iter() {
                                        let x_difference = cursor_position.x - transform.translation.x;
                                        let y_difference = (window.height() - cursor_position.y) - transform.translation.y;

                                        if x_difference.abs() < 8.0 && y_difference.abs() < 8.0 {
                                            let (_, mut triangle) = triangles_query.get_mut(*entity).unwrap();
                                            match vertex_selector.0 {
                                                VertexOrder::First => {
                                                    triangle.first.color = ui_state.color_picker;
                                                }
                                                VertexOrder::Middle => {
                                                    triangle.middle.color = ui_state.color_picker;
                                                }
                                                VertexOrder::Last => {
                                                    triangle.last.color = ui_state.color_picker;
                                                }
                                                VertexOrder::Indifferent => {}
                                            }
                                            for (entity, _, _) in vertex_selector_query.iter() {
                                                commands.entity(entity).despawn();
                                            }
                                            ui_state.vertex_selectors_spawned = false;
                                            break;
                                        }
                                    }
                                }
                            }
                            if input.pressed(MouseButton::Left) && !(ctx.is_using_pointer() || ctx.is_pointer_over_area()) {
                                if let Some(cursor_position) = window.cursor_position() {
                                    for (_, mut triangle) in triangles_query.iter_mut() {
                                        let x_difference = cursor_position.x - triangle.first.position[0];
                                        let y_difference = (window.height() - cursor_position.y) - triangle.first.position[1];

                                        if x_difference.abs() < 8.0 && y_difference.abs() < 8.0 {
                                            for (_, vertex_selector, mut transform) in vertex_selector_query.iter_mut() {
                                                if let VertexOrder::First = vertex_selector.0 {
                                                    transform.translation.x = cursor_position.x;
                                                    transform.translation.y = window.height() - cursor_position.y;
                                                }
                                            }
                                            triangle.first.position[0] = cursor_position.x;
                                            triangle.first.position[1] = window.height() - cursor_position.y;
                                            triangle.redraw = true;
                                            break;
                                        }

                                        let x_difference = cursor_position.x - triangle.middle.position[0];
                                        let y_difference = (window.height() - cursor_position.y) - triangle.middle.position[1];

                                        if x_difference.abs() < 8.0 && y_difference.abs() < 8.0 {
                                            for (_, vertex_selector, mut transform) in vertex_selector_query.iter_mut() {
                                                if let VertexOrder::Middle = vertex_selector.0 {
                                                    transform.translation.x = cursor_position.x;
                                                    transform.translation.y = window.height() - cursor_position.y;
                                                }
                                            }
                                            triangle.middle.position[0] = cursor_position.x;
                                            triangle.middle.position[1] = window.height() - cursor_position.y;
                                            triangle.redraw = true;
                                            break;
                                        }

                                        let x_difference = cursor_position.x - triangle.last.position[0];
                                        let y_difference = (window.height() - cursor_position.y) - triangle.last.position[1];

                                        if x_difference.abs() < 8.0 && y_difference.abs() < 8.0 {
                                            for (_, vertex_selector, mut transform) in vertex_selector_query.iter_mut() {
                                                if let VertexOrder::Last = vertex_selector.0 {
                                                    transform.translation.x = cursor_position.x;
                                                    transform.translation.y = window.height() - cursor_position.y;
                                                }
                                            }
                                            triangle.last.position[0] = cursor_position.x;
                                            triangle.last.position[1] = window.height() - cursor_position.y;
                                            triangle.redraw = true;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    None => {
                        if input.just_pressed(MouseButton::Left) && !(ctx.is_using_pointer() || ctx.is_pointer_over_area()) {
                            if let Some(cursor_position) = window.cursor_position() {
                                let click = (cursor_position.x, window.height() - cursor_position.y);
                                for (entity, triangle) in triangles_query.iter() {
                                    if is_inside(click, triangle) {
                                        ui_state.function = Some(Function::Modify(Some(entity)));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}*/

fn draw_triangles(
    triangles_query: Query<&Triangle>,
) {
    for triangle in triangles_query.iter() {

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