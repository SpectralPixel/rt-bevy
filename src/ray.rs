use bevy::prelude::*;

const GRID_SIZE: u8 = 10;

pub struct Ray {
    pub position: Vec2,
    pub direction: Vec2,
}

impl Ray {
    pub fn new(position: Vec2, direction: Vec2) -> Self {
        Self { position, direction }
    }
}