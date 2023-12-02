use bevy::{
    prelude::*,
    window::PrimaryWindow,
    sprite::MaterialMesh2dBundle,
};
use bevy_egui::EguiContexts;
use std::cmp::Ordering;

use crate::ui::Function;
use crate::ui::UIState;

pub struct TrianglesPlugin;

impl Plugin for TrianglesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, associate_functions)
            .add_systems(Update, draw_triangles);
    }
}

#[derive(Component)]
pub struct VertexSelector;

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
}

fn associate_functions(
    mut commands: Commands,
    mut egui_contexts: EguiContexts,
    input: Res<Input<MouseButton>>,
    mut ui_state: ResMut<UIState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    triangles_query: Query<(Entity, &Triangle)>,
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
                                VertexSelector,
                                MaterialMesh2dBundle {
                                    mesh: meshes.add(shape::Circle::new(8.0).into()).into(),
                                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                                    transform: Transform::from_translation(Vec3::new(
                                        position[0],
                                        position[1], 
                                        position[2] + 10.0 * vertex_index,
                                    )),
                                    ..default()
                                },
                            ));
                            commands.spawn((
                                VertexSelector,
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
                                        position[2] + 10.0 * vertex_index + 1.0,
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
                    }).id();

                    ui_state.function = Some(Function::Modify(Some(entity)));

                    println!("{:?}", sorted_triangle);
                }
            }
            Function::Modify(entity) => {
                match entity {
                    Some(entity) => {
                        
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
}

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