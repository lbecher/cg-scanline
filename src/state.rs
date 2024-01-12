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
    });
}