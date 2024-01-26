use bevy::{
    prelude::*,
    render::render_resource::*,
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

#[derive(Component)]
pub struct TriangleSprite(Option<Entity>);


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

            let entity = commands.spawn((
                Triangle {
                    first: sorted_triangle[0].clone(),
                    middle: sorted_triangle[1].clone(),
                    last: sorted_triangle[2].clone(),
                    redraw: true,
                },
                TriangleSprite(None),
            )).id();

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
                                triangle.redraw = true;
                            }
                            VertexOrder::Middle => {
                                triangle.middle.color = state.color_picker;
                                triangle.redraw = true;
                            }
                            VertexOrder::Last => {
                                triangle.last.color = state.color_picker;
                                triangle.redraw = true;
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
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut triangles_query: Query<(&mut Triangle, &mut TriangleSprite)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for (mut triangle, mut sprite) in triangles_query.iter_mut() {
        if triangle.redraw {
            let window = window_query.single();

            if let Some(entity) = sprite.0 {
                commands.entity(entity).despawn();
            }

            let mut image = Image::new_fill(
                Extent3d {
                    width: window.width() as u32,
                    height: window.height() as u32,
                    depth_or_array_layers: 1,
                },
                TextureDimension::D2,
                &[0, 0, 0, 0],
                TextureFormat::Rgba8Unorm,
            );
            image.texture_descriptor.usage =
                TextureUsages::COPY_DST | 
                TextureUsages::STORAGE_BINDING | 
                TextureUsages::TEXTURE_BINDING;
            
            render(&triangle, &mut image.data, window.width() as usize, window.height() as usize);

            let image = images.add(image);

            let entity = commands.spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(window.width(), window.height())),
                    ..default()
                },
                texture: image.clone(),
                transform: Transform::from_translation(Vec3::new(
                    window.width() / 2.0,
                    window.height() / 2.0,
                    triangle.first.position[2],
                )),
                ..default()
            }).id();

            sprite.0 = Some(entity);
            triangle.redraw = false;
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


fn render(
    triangle: &Triangle,
    image: &mut Vec<u8>,
    width: usize,
    height: usize,
) {
  

    for i in 0..3 {
        let (
            v0,
            v1,
        ) = if i == 0 {
            (
                triangle.first.clone(),
                triangle.middle.clone(),
            )
        } else if i == 1 {
            (
                triangle.middle.clone(),
                triangle.last.clone(),
            )
        } else {
            (
                triangle.last.clone(),
                triangle.first.clone(),
            )
        };

        let x0 = v0.position[0];
        let y0 = v0.position[1];
        let x1 = v1.position[0];
        let y1 = v1.position[1];

        let points = bresenham(x0, y0, x1, y1);
        let points_len = points.len() as f32;

        let mut count: f32 = 0.0;
        for (x, y) in points {
            let blend_factor = count / points_len;
            count += 1.0;
    
            let r = v0.color[0] + blend_factor * (v1.color[0] - v0.color[0]);
            let g = v0.color[1] + blend_factor * (v1.color[1] - v0.color[1]);
            let b = v0.color[2] + blend_factor * (v1.color[2] - v0.color[2]);
    
            let i = height - (y.round() as usize);
            let j = x.round() as usize;
            let index = (i * width + j) * 4;

            image[index] = (r * 255.0) as u8;
            image[index + 1] = (g * 255.0) as u8;
            image[index + 2] = (b * 255.0) as u8;
            image[index + 3] = 255;
        }
    }

    let x_min = triangle.last.position[0].min(triangle.middle.position[0].min(triangle.first.position[0])).round() as usize;
    let x_max = triangle.last.position[0].max(triangle.middle.position[0].max(triangle.first.position[0])).round() as usize;

    let y_min = triangle.last.position[1].min(triangle.middle.position[1].min(triangle.first.position[1])).round() as usize;
    let y_max = triangle.last.position[1].max(triangle.middle.position[1].max(triangle.first.position[1])).round() as usize;

    for i in y_min..=y_max {
        let i = height - i;

        let mut first_color: Option<[f32; 3]> = None;
        let mut first_color_j: usize = 0;
        let mut last_color: Option<[f32; 3]> = None;
        let mut last_color_j: usize = 0;
        let mut after_first_line = false;

        for j in x_min..=x_max {
            let index = (i * width + j) * 4;
            
            if image[index + 3] > 0 && after_first_line == false {
                first_color = Some(
                    [
                        image[index] as f32 / 255.0,
                        image[index + 1] as f32 / 255.0,
                        image[index + 2] as f32 / 255.0,
                    ]
                );
                first_color_j = j;
            } else if image[index + 3] == 0 && after_first_line == false && first_color.is_some() {
                after_first_line = true;
            } else if image[index + 3] > 0 && after_first_line == true && first_color.is_some() {
                last_color = Some(
                    [
                        image[index] as f32 / 255.0,
                        image[index + 1] as f32 / 255.0,
                        image[index + 2] as f32 / 255.0,
                    ]
                );
                last_color_j = j;
                break;
            }
        }

        if let (Some(first_color), Some(last_color)) = (first_color, last_color) {
            let points_len = (last_color_j - first_color_j) as f32;    
            let mut count: f32 = 0.0;
            let mut j = first_color_j + 1;
            while j < last_color_j {
                let blend_factor = count / points_len;
                count += 1.0;
        
                let r = first_color[0] + blend_factor * (last_color[0] - first_color[0]);
                let g = first_color[1] + blend_factor * (last_color[1] - first_color[1]);
                let b = first_color[2] + blend_factor * (last_color[2] - first_color[2]);

                let index = (i * width + j) * 4;
                image[index] = (r * 255.0) as u8;
                image[index + 1] = (g * 255.0) as u8;
                image[index + 2] = (b * 255.0) as u8;
                image[index + 3] = 255;

                j += 1;
            }
        }
    }

        /*

          // x, y, r, g, b
    let mut edges: Vec<Vec<(f32, f32, f32, f32, f32)>> = Vec::new();
        
        let mut edge: Vec<(f32, f32, f32, f32, f32)> = Vec::new();

        let mut count: f32 = 0.0;
        for (x, y) in points {
            let blend_factor = count / points_len;
            count += 1.0;
    
            let r = v0.color[0] + blend_factor * (v1.color[0] - v0.color[0]);
            let g = v0.color[1] + blend_factor * (v1.color[1] - v0.color[1]);
            let b = v0.color[2] + blend_factor * (v1.color[2] - v0.color[2]);

            edge.push((x, y, r, g, b));
        }

        // para deixar y decrescente
        if edge[0].1 < edge[edge.len() - 1].1 {
            edge.reverse();
        }

        // garante que os maiores y estejam no começo
        if edges.len() > 0 && edge[0].1 >= edges[0][0].1 {
            // garante que a aresta mais alta seja a primeira
            let edge_dy = edge[edge.len() - 1].1 - edge[0].1;
            let edges0_dy = edges[0][edges[0].len() - 1].1 - edges[0][0].1;
            if edge_dy.abs() > edges0_dy.abs() {
                edges.insert(0, edge);
            } else {
                edges.insert(1, edge);
            }
        } else {
            edges.push(edge);
        }
    }

    println!("{:?} {:?}", edges[0][0],  edges[0][edges[0].len() - 1]);
    println!("{:?} {:?}", edges[1][0],  edges[1][edges[1].len() - 1]);
    println!("{:?} {:?}", edges[2][0],  edges[2][edges[2].len() - 1]);

    for mut e0 in edges.remove(0) {
        let mut e1 = if edges[0].len() > 0 {
            edges[0].remove(0)
        } else {
            edges[1].remove(0)
        };

        if e0.0 > e1.0 {
            std::mem::swap(&mut e0, &mut e1);
        }

        let y = e0.1.round() as usize;
        let points_len = e1.0.round() - e0.0.round();

        let mut count: f32 = 0.0;
        for x in e0.0.round() as usize..e1.0.round() as usize {
            let blend_factor = count / points_len;
            count += 1.0;
    
            let r = e0.2 + blend_factor * (e1.2 - e0.2);
            let g = e0.3 + blend_factor * (e1.3 - e0.3);
            let b = e0.4 + blend_factor * (e1.4 - e0.4);

            let i = height - y;
            let j = x;
            let index = (i * width + j) * 4;

            image[index] = (r * 255.0) as u8;
            image[index + 1] = (g * 255.0) as u8;
            image[index + 2] = (b * 255.0) as u8;
            image[index + 3] = 255;
        }
    }*/


    /*
    let x_min = triangle.first.position[0].round() as usize;
    let x_max = triangle.last.position[0].round() as usize;

    let y_min = triangle.last.position[1].min(triangle.middle.position[1].min(triangle.first.position[1])).round() as usize;
    let y_max = triangle.last.position[1].max(triangle.middle.position[1].max(triangle.first.position[1])).round() as usize;

    for i in y_min..=y_max {
        let i = height - i;

        let mut first_color: Option<[f32; 3]> = None;
        let mut first_color_j: usize = 0;
        let mut last_color: Option<[f32; 3]> = None;
        let mut last_color_j: usize = 0;

        for j in x_min..=x_max {
            let index = (i * width + j) * 4;
            if image[index + 3] > 0 {
                if first_color.is_none() {
                    first_color = Some(
                        [
                            image[index] as f32 / 255.0,
                            image[index + 1] as f32 / 255.0,
                            image[index + 2] as f32 / 255.0,
                        ]
                    );
                    first_color_j = j;
                } else {
                    last_color = Some(
                        [
                            image[index] as f32 / 255.0,
                            image[index + 1] as f32 / 255.0,
                            image[index + 2] as f32 / 255.0,
                        ]
                    );
                    last_color_j = j;
                    break;
                }
            }
        }

        if let (Some(first_color), Some(last_color)) = (first_color, last_color) {
            let points_len = (last_color_j - first_color_j) as f32;    
            let mut count: f32 = 0.0;
            let mut j = first_color_j + 1;
            while j < last_color_j {
                let blend_factor = count / points_len;
                count += 1.0;
        
                let r = first_color[0] + blend_factor * (last_color[0] - first_color[0]);
                let g = first_color[1] + blend_factor * (last_color[1] - first_color[1]);
                let b = first_color[2] + blend_factor * (last_color[2] - first_color[2]);

                let index = (i * width + j) * 4;
                image[index] = (r * 255.0) as u8;
                image[index + 1] = (g * 255.0) as u8;
                image[index + 2] = (b * 255.0) as u8;
                image[index + 3] = 255;

                j += 1;
            }
        }
    }*/
}


fn bresenham(x0: f32, y0: f32, x1: f32, y1: f32) -> Vec<(f32, f32)> {
    let mut result = Vec::new();
    
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();

    let sx = if x0 < x1 { 1.0 } else { -1.0 };
    let sy = if y0 < y1 { 1.0 } else { -1.0 };

    let mut x = x0;
    let mut y = y0;

    let mut err = dx - dy;

    while (x - x1).abs() > 0.1 || (y - y1).abs() > 0.1 {
        result.push((x, y));

        let e2 = 2.0 * err;

        if e2 > -dy {
            err -= dy;
            x += sx;
        }

        if e2 < dx {
            err += dx;
            y += sy;
        }
    }

    result.push((x1, y1));
    result
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
    let (u, v, _w) = barycentric_coordinates(
        click, 
        (triangle.first.position[0], triangle.first.position[1]), 
        (triangle.middle.position[0], triangle.middle.position[1]), 
        (triangle.last.position[0], triangle.last.position[1]));

    u >= 0.0 && v >= 0.0 && u + v <= 1.0
}