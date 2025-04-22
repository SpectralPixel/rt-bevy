use bevy::prelude::*;

pub struct Ray {
    origin: Vec2,
    direction: Vec2, // is to be normalized
    length: f32,
}

impl Ray {
    pub fn new(origin: Vec2, target: Vec2) -> Self {
        let direction = (target - origin).normalize();
        assert_ne!(direction, Vec2::ZERO, "Ray direction cannot be zero!");
        Self {
            origin,
            direction,
            length: 0.,
        }
    }

    pub fn draw_gizmo(&self, mut gizmos: Gizmos, length: f32) {
        gizmos.arrow_2d(
            self.origin,
            self.origin + self.direction * length,
            Color::linear_rgb(1., 0., 0.),
        );
    }
}
