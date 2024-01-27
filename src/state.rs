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
    pub color_picker: [f32; 3],
    pub new_triangle: Vec<Vertex>,
    pub spawn_vertex_selectors: bool,
    pub triangles_count: usize,
    pub show_properties_window: bool,

    pub first_position_x_string: String,
    pub first_position_y_string: String,
    pub first_position_string_parsing_error: bool,

    pub middle_position_x_string: String,
    pub middle_position_y_string: String,
    pub middle_position_string_parsing_error: bool,

    pub last_position_x_string: String,
    pub last_position_y_string: String,
    pub last_position_string_parsing_error: bool,
}

fn setup_state(
    mut commands: Commands,
) {
    commands.insert_resource(State {
        function: Function::None,
        color_picker: [1.0, 0.0, 0.0],
        new_triangle: Vec::new(),
        spawn_vertex_selectors: false,
        selected_vertex: None,
        triangles_count: 1,

        show_properties_window: false,

        first_position_x_string: String::new(),
        first_position_y_string: String::new(),
        first_position_string_parsing_error: false,

        middle_position_x_string: String::new(),
        middle_position_y_string: String::new(),
        middle_position_string_parsing_error: false,

        last_position_x_string: String::new(),
        last_position_y_string: String::new(),
        last_position_string_parsing_error: false,
    });
}