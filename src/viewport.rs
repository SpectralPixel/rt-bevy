use bevy::prelude::*;
use crate::prelude::*;

pub struct RayViewport2D {
    origin: Vec2,
    direction: Direction, // implement Into<Vec2> for Direction
    forward: Vec2,
    fov: f32,
    near_plane: f32, // near clipping plane distance
    ray_count: u16,
    left_edge_position: Vec2,
    right_edge_position: Vec2,
}

impl RayViewport2D {
    pub fn new(origin: Vec2, direction: Direction, fov_degrees: f32, near_plane: f32, ray_count: u16) -> Self {
        let forward = Vec2::from_angle(direction.get()).normalize();
        let fov = fov_degrees.to_radians();
        let left = forward.perp();
        let right = -left;
        let left_edge_position = origin + forward * near_plane + left * (fov / 2.) * near_plane;
        let right_edge_position = origin + forward * near_plane + right * (fov / 2.) * near_plane;
        Self {
            origin,
            direction,
            forward,
            fov: fov_degrees.to_radians(),
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
            let ray = Ray::new(self.origin, ray_position);
            ray.draw_gizmo(&mut gizmos, 50.0);
        }
    }
}
