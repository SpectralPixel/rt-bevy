use bevy::prelude::*;

#[derive(Component)]
pub struct Direction(f32);

impl Direction {
    pub fn new(angle: f32) -> Self {
        Self(angle)
    }

    pub fn get(&self) -> f32 {
        self.0
    }

    pub fn set(&mut self, angle: f32) {
        self.0 = angle;
        println!("Player rotation: {:?}", self.0);
    }
}
