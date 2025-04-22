use bevy::prelude::*;
use crate::prelude::*;

#[derive(Component)]
pub struct RayViewport2D {
    position: Vec2,
    direction: Direction, // implement Into<Vec2> for Direction
    forward: Vec2,
    fov: f32,
    near_plane: f32, // near clipping plane distance
    ray_count: u16,
    left_edge_position: Vec2,
    right_edge_position: Vec2,
}

impl RayViewport2D {
    pub fn new(position: Vec2, direction: Direction, fov_degrees: f32, near_plane: f32, ray_count: u16) -> Self {
        let fov = fov_degrees.to_radians();
        let (forward, left_edge_position, right_edge_position) = Self::calculate_vectors(&position, &direction, fov_degrees, near_plane);
        Self {
            position,
            direction,
            forward,
            fov,
            near_plane,
            ray_count,
            left_edge_position,
            right_edge_position,
        }
    }

    pub fn cast_rays(&self, mut gizmos: Gizmos) {
        for i in 0..self.ray_count {
            let t = (i as f32 + 0.5) / self.ray_count as f32;
            let ray_position = self.left_edge_position.lerp(self.right_edge_position, t);
            let ray = Ray::new(self.position, ray_position);
            ray.draw_gizmo(&mut gizmos, 50.0);
        }
    }

    pub fn calculate_vectors(position: &Vec2, direction: &Direction, fov: f32, near_plane: f32) -> (Vec2, Vec2, Vec2) {
        let forward = Vec2::from_angle(direction.get()).normalize();
        let left = forward.perp();
        let right = -left;
        let left_edge_position = position + forward * near_plane + left * (fov / 2.) * near_plane;
        let right_edge_position = position + forward * near_plane + right * (fov / 2.) * near_plane;
        (forward, left_edge_position, right_edge_position)
    }

    pub fn recalculate_viewport(&mut self, position: Vec2, direction: Direction) {
        let (forward, left_edge_position, right_edge_position) = Self::calculate_vectors(&position, &direction, self.fov, self.near_plane);
        self.position = position;
        self.direction = direction;
        self.forward = forward;
        self.left_edge_position = left_edge_position;
        self.right_edge_position = right_edge_position;
    }
}
