use bevy::prelude::*;
use crate::prelude::*;

pub struct RayViewport2D {
    origin: Vec2,
    direction: Direction, // implement Into<Vec2> for Direction
    forward: Vec2,
    fov: f32,
    ray_count: u16
}

impl RayViewport2D {
    pub fn new(origin: Vec2, direction: Direction, fov: f32, ray_count: u16) -> Self {
        let forward = Vec2::from_angle(direction.get());
        Self {
            origin,
            direction,
            forward,
            fov,
            ray_count
        }
    }
}
