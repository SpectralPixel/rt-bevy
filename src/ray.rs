use bevy::prelude::*;

pub struct Ray {
    pub position: Vec2,
    pub direction: Vec2,
}

impl Ray {
    pub fn new(position: Vec2, direction: Vec2) -> Self {
        Self {
            position,
            direction,
        }
    }

    pub fn draw_gizmo(&self, mut gizmos: Gizmos, length: f32) {
        gizmos.arrow_2d(
            self.position,
            self.position + self.direction * length,
            Color::linear_rgb(1., 0., 0.),
        );
    }
}
