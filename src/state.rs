use bevy::prelude::*;

use crate::triangles::{
    Vertex,
    VertexOrder,
};

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_state);
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Function {
    None,
    Create,
    Select,
    Modify(Entity),
}

#[derive(Resource)]
pub struct State {
    pub function: Function,
    pub selected_vertex: Option<VertexOrder>,
    pub vertex_color_picker: [u8; 3],
    pub edges_color_picker: [u8; 3],
    pub constant_edges: bool,
    pub new_triangle: Vec<Vertex>,
    pub spawn_vertex_selectors: bool,
    pub triangles_count: usize,
    pub show_properties_window: bool,

    pub edges_color_r_string: String,
    pub edges_color_g_string: String,
    pub edges_color_b_string: String,
    pub edges_string_parsing_error: bool,

    pub first_position_x_string: String,
    pub first_position_y_string: String,
    pub first_color_r_string: String,
    pub first_color_g_string: String,
    pub first_color_b_string: String,
    pub first_vertex_string_parsing_error: bool,

    pub middle_position_x_string: String,
    pub middle_position_y_string: String,
    pub middle_color_r_string: String,
    pub middle_color_g_string: String,
    pub middle_color_b_string: String,
    pub middle_vertex_string_parsing_error: bool,

    pub last_position_x_string: String,
    pub last_position_y_string: String,
    pub last_color_r_string: String,
    pub last_color_g_string: String,
    pub last_color_b_string: String,
    pub last_vertex_string_parsing_error: bool,
}

fn setup_state(
    mut commands: Commands,
) {
    commands.insert_resource(State {
        function: Function::None,
        vertex_color_picker: [255, 0, 0],
        edges_color_picker: [0, 0, 0],
        constant_edges: false,
        new_triangle: Vec::new(),
        spawn_vertex_selectors: false,
        selected_vertex: None,
        triangles_count: 1,

        show_properties_window: false,

        edges_color_r_string: String::new(),
        edges_color_g_string: String::new(),
        edges_color_b_string: String::new(),
        edges_string_parsing_error: false,

        first_position_x_string: String::new(),
        first_position_y_string: String::new(),
        first_color_r_string: String::new(),
        first_color_g_string: String::new(),
        first_color_b_string: String::new(),
        first_vertex_string_parsing_error: false,

        middle_position_x_string: String::new(),
        middle_position_y_string: String::new(),
        middle_color_r_string: String::new(),
        middle_color_g_string: String::new(),
        middle_color_b_string: String::new(),
        middle_vertex_string_parsing_error: false,

        last_position_x_string: String::new(),
        last_position_y_string: String::new(),
        last_color_r_string: String::new(),
        last_color_g_string: String::new(),
        last_color_b_string: String::new(),
        last_vertex_string_parsing_error: false,
    });
}