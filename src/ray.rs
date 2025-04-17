use bevy::prelude::*;

const GRID_SIZE: u8 = 10;

pub struct Ray {
    pub position: Vec2,
    pub direction: Vec2,
}