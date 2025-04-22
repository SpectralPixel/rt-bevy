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
    }

    pub fn set_from_quat(&mut self, quat: Quat) {
        *self = Self::from(quat);
    }
}

impl From<Quat> for Direction {
    fn from(quat: Quat) -> Self {
        use std::f32::consts::TAU;
        let rot = quat.to_axis_angle();
        let p = rot.0.z;
        let q = rot.1;
        let angle = (-p).clamp(0., 1.) * TAU + q * p;
        Self::new(angle)
    }
}
