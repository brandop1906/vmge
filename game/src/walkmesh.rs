use bevy::prelude::*;

pub struct WalkableMesh {
    min_x: f32,
    min_y: f32,
    max_x: f32,
    max_y: f32,
}

#[derive(Resource)]
pub struct WalkableArea {
    walkable_mesh: Vec<WalkableMesh>,
}

impl WalkableMesh {
    pub fn new(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Self {
        WalkableMesh {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }
}

impl WalkableArea {
    pub fn new() -> Self {
        WalkableArea {
            walkable_mesh: Vec::new(),
        }
    }

    pub fn add_walkable_mesh(&mut self, mesh: WalkableMesh) {
        self.walkable_mesh.push(mesh);
    }

    pub fn clamp_position(&self, position: Vec3) -> Vec3 {
        let mut clamped_position = position;
        let mesh = &self.walkable_mesh[0];
        clamped_position.x = position.x.clamp(mesh.min_x, mesh.max_x);
        clamped_position.y = position.y.clamp(mesh.min_y, mesh.max_y);
        clamped_position
    }
}